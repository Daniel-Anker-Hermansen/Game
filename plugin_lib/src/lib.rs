pub mod ffi_types;
use ffi_types::FFIStaticStr;

#[repr(C)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[repr(C)]
pub struct Metadata {
    pub api_version: Version,
    pub plugin_version: Version,
    pub plugin_name: FFIStaticStr,
    pub constructor: extern fn(),
}

#[cfg(feature = "plugin")]
#[macro_export]
macro_rules! meta {
    ($name:expr, $major:expr, $minor:expr, $patch:expr) => {
        const _: () = {        
            extern fn init() {
                std::panic::set_hook(Box::new(|panic_info| {
                    eprintln!("Game panicked inside plugin: {}, version: {}.{}.{}.", $name, $major, $minor, $patch);
                    eprintln!("This is most likely a bug in that plugin. Please send a bug report including the following:");
                    if let Some(loc) = panic_info.location() {
                        eprintln!("Panic occured in file: '{}' at line {}.", loc.file(), loc.line());
                    }
                    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                        eprintln!("Reason for panic: {s:?}.");
                    } 
                    else {
                        eprintln!("Unknown reason for panic.");
                    }
                    // In future version this can callback to game so that the error can be
                    // displayed in window and perhaps have an automatic bug report submission
                    // field. For now exit with -1 error code to avoid core dumped from
                    // unsuccesful catching of panic from game.
                    std::process::exit(-1);
                }));
            }

            #[no_mangle]
            pub static __METADATA: $crate::Metadata = $crate::Metadata {
                api_version: $crate::Version {
                    major: 0,
                    minor: 1,
                    patch: 0,
                },
                plugin_version: $crate::Version {
                    major: $major,
                    minor: $minor,
                    patch: $patch,
                },
                plugin_name: $crate::ffi_types::FFIStaticStr::from_str($name),
                constructor: init,
            };
        };
    };
}

pub trait Item: Sized {
    fn new() -> Self;

    fn name() -> &'static str;

    fn id(&mut self) -> i64;
}

#[cfg(feature = "plugin")]
trait ItemRawAdapter: Item {
    extern fn new() -> *mut () {
        alloc(<Self as Item>::new()).cast()
    }

    extern fn name() -> FFIStaticStr {
        FFIStaticStr::from_str(<Self as Item>::name())
    }

    extern fn id(ptr: *mut ()) -> i64 {
        unsafe { &mut *ptr.cast::<Self>() }.id()
    }

    extern fn drop(ptr: *mut ()) {
        drop(ptr);
    }
}

#[cfg(feature = "plugin")]
impl<T> ItemRawAdapter for T where T: Item { }

#[cfg(feature = "plugin")]
pub const fn item_vtable<T>() -> ItemVTable where T: Item {
    ItemVTable { 
        new: <T as ItemRawAdapter>::new,
        name: <T as ItemRawAdapter>::name,
        id: <T as ItemRawAdapter>::id, 
        drop: <T as ItemRawAdapter>::drop, 
    }
}

#[cfg(feature = "game")]
pub trait ItemGame {
    fn id(&mut self) -> i64;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ItemVTable {
    new: extern fn() -> *mut (),
    name: extern fn() -> FFIStaticStr,
    id: extern fn(*mut ()) -> i64 ,
    drop: extern fn(*mut ()),
}

#[cfg(feature = "game")]
impl ItemVTable {
    pub fn new(&self) -> ItemInstance<'_> {
        ItemInstance { this: (self.new)(), vtable: self }
    }

    pub fn name(&self) -> &str {
        (self.name)().into()
    } 
}

#[cfg(feature = "game")]
pub struct ItemInstance<'lib> {
    this: *mut (),
    vtable: &'lib ItemVTable,
}

#[cfg(feature = "game")]
impl Drop for ItemInstance<'_> {
    fn drop(&mut self) {
        (self.vtable.drop)(self.this)
    }
}

#[cfg(feature = "game")]
impl ItemGame for ItemInstance<'_> {
    fn id(&mut self) -> i64 {
        (self.vtable.id)(self.this)
    }
}

#[cfg(feature = "game")]
impl ItemInstance<'_> {
    pub fn vtable(&self) -> &ItemVTable {
        self.vtable
    }
}

pub fn alloc<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

pub fn drop<T>(ptr: *mut T) {
    unsafe {
        std::ptr::drop_in_place(ptr);
    }
}

#[macro_export]
macro_rules! export_items {
    ($($t:ty), *) => {
        const _: () = {
            #[no_mangle]
            pub static __EXPORTED_ITEMS: $crate::ffi_types::FFISlice = $crate::ffi_types::slice_to_ffi(&[
                $(
                    $crate::item_vtable::<$t>(),
                ) *
            ]);
        };
    };
}
