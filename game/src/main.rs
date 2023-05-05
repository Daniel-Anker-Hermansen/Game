use plugin_lib::{ffi_types::load_slice_from_lib, ItemGame, ItemVTable, Metadata};
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, DeviceEvent, ElementState}};

const __METADATA: &[u8] = b"__METADATA\0";
const __EXPORTED_ITEMS: &[u8] = b"__EXPORTED_ITEMS\0";

fn main() {
    // Immitate regular segfault for now. Later change to show popup or something.
    extern fn segfault(signum: libc::c_int) {
        eprintln!("segmentation fault (core dumped)");
        std::process::exit(signum as i32);
    }

    unsafe { libc::signal(libc::SIGSEGV, segfault as libc::size_t) };
    
    let plugins = std::fs::read_dir("plugins").unwrap();
    let mut libs = vec![];
    let mut items = vec![];

    for file in plugins {
        match file {
            Ok(file) => {
                let path = file.path();
                let lib = unsafe { libloading::Library::new(path).unwrap() };
                let meta = unsafe { lib.get::<&'static Metadata>(__METADATA).unwrap() };
                let name: &str = meta.plugin_name.into();
                println!("Plugin name: {}", name);
                println!("{}.{}.{}", meta.plugin_version.major, meta.plugin_version.minor, meta.plugin_version.patch);
                (meta.constructor)();
                unsafe { items.extend_from_slice(load_slice_from_lib::<ItemVTable>(&lib, __EXPORTED_ITEMS)) };
                libs.push(lib);
            },
            Err(e) => eprintln!("{e}"),
        }
    }

    for item in &items {
        println!("{}", item.name());
    }

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .build(&event_loop);
    let items = items.leak();

    let mut item_instances: Vec<_> = items.iter().map(|vtable| vtable.new()).collect();

    event_loop.run(move |event, _target, _flow| {
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Button { button: 1, state: ElementState::Pressed } => {
                    for item in item_instances.iter_mut() {
                        let id = item.id();
                        println!("{}: {}", item.vtable().name(), id)
                    }
                },
                _ => (),
            },
            _ => (),
        }
    });
}
