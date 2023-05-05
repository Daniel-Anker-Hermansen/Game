#![feature(thread_spawn_unchecked)]

use std::sync::atomic::compiler_fence;

use plugin_lib::{ffi_types::{load_slice_from_lib, FFIString}, ItemVTable, Metadata, UserEvent};
use winit::{event_loop::{EventLoopBuilder, EventLoopProxy}, window::WindowBuilder, event::{Event, DeviceEvent, ElementState, WindowEvent}};

use crate::state::State;

mod state;

static mut VTABLES: Vec<ItemVTable> = Vec::new();

fn vtables() -> &'static [ItemVTable] {
    unsafe { &VTABLES }
}

const __METADATA: &[u8] = b"__METADATA\0";
const __EXPORTED_ITEMS: &[u8] = b"__EXPORTED_ITEMS\0";

fn main() {
    env_logger::init();
    // Immitate regular segfault for now. Later change to show popup or something.
    extern fn segfault(signum: libc::c_int) {
        eprintln!("segmentation fault (core dumped)");
        std::process::exit(signum as i32);
    }

    unsafe { libc::signal(libc::SIGSEGV, segfault as libc::size_t) };
    
    let plugins = std::fs::read_dir("plugins").unwrap();
    let mut libs = vec![];
    let mut items = vec![];
    
    let event_loop = EventLoopBuilder::with_user_event()
        .build();

    for file in plugins {
        match file {
            Ok(file) => {
                let path = file.path();
                let lib = unsafe { libloading::Library::new(path).unwrap() };
                let meta = unsafe { lib.get::<&'static Metadata>(__METADATA).unwrap() };
                let name: &str = meta.plugin_name.into();
                println!("Plugin name: {}", name);
                println!("{}.{}.{}", meta.plugin_version.major, meta.plugin_version.minor, meta.plugin_version.patch);
                let proxy = Box::leak(Box::new(event_loop.create_proxy())) as *mut EventLoopProxy<_>;
                (meta.constructor)(panic, proxy as _);
                unsafe { items.extend_from_slice(load_slice_from_lib::<ItemVTable>(&lib, __EXPORTED_ITEMS)) };
                libs.push(lib);
            },
            Err(e) => eprintln!("{e}"),
        }
    }
    unsafe {
        VTABLES = items;
    }

    // I do not think this is necessary but better be on the safe side. 
    // The idea is to ensure that VTABLES is written to before continuing.
    compiler_fence(std::sync::atomic::Ordering::AcqRel);
    for item in vtables() {
        println!("{}", item.name());
    }

    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let mut state = State::new(window);
    for vtable in vtables() {
        state.add_item(vtable.new());
    }

    event_loop.run(move |event, _target, _flow| {
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Button { button: 1, state: ElementState::Pressed } => {
                    for _ in 0..20 {
                        state.iterate();
                    }
                    state.render().unwrap();
                },
                _ => (),
            },
            Event::WindowEvent { window_id, event: WindowEvent::Resized(new_size) } if window_id == state.window().id() => state.resize(new_size),
            Event::RedrawRequested(window_id) if window_id == state.window().id() => state.render().unwrap(), 
            Event::UserEvent(UserEvent::Panic(msg)) => state.panic(msg),
            _ => (),
        }
    });
}

/// Panic function for plugins so that blue screen of death is shown.
extern fn panic(msg: FFIString, proxy: usize) {
    let proxy: *mut EventLoopProxy<UserEvent> = unsafe { std::mem::transmute(proxy) };
    unsafe { &*proxy }.send_event(UserEvent::Panic(String::from(msg))).unwrap();
    std::panic::set_hook(Box::new(|_| ()));
    panic!();
}
