pub mod ffi_types;
pub use concat_idents::concat_idents;
#[cfg(feature = "game")]
use libloading::Library;
use libloading::Symbol;

#[repr(C)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[macro_export]
macro_rules! version {
    () => {
        const _: () = {        
            #[no_mangle]
            pub static __API_VERSION: $crate::Version = $crate::Version {
                major: 0,
                minor: 1,
                patch: 0,
            };
        };
    };
}

#[macro_export]
macro_rules! name {
    ($name:expr) => {
        const _: () = {
            #[no_mangle]
            pub static __PLUGIN_NAME: $crate::ffi_types::FFIStaticStr = $crate::ffi_types::FFIStaticStr::from_str($name);
        };
    };
}

pub trait Item {
    fn new() -> Self;

    fn id(&mut self) -> i64;
}

#[cfg(feature = "game")]
pub trait ItemGame {
    fn id(&mut self) -> i64;
}

#[cfg(feature = "game")]
pub struct ItemVTable<'lib> {
    this: &'lib mut (),
    id: Symbol<'lib, extern "C" fn(&mut ()) -> i64>,
    drop: Symbol<'lib, extern "C" fn(&mut ())>,
}

#[cfg(feature = "game")]
impl Drop for ItemVTable<'_> {
    fn drop(&mut self) {
        (self.drop)(self.this)
    }
}

#[cfg(feature = "game")]
impl ItemGame for ItemVTable<'_> {
    fn id(&mut self) -> i64 {
        (self.id)(self.this)
    }
}

#[cfg(feature = "game")]
pub struct ItemConstructor<'lib> {
    constructor: Symbol<'lib, extern "C" fn() -> &'lib mut ()>,
    id: Symbol<'lib, extern "C" fn(&mut ()) -> i64>,
    drop: Symbol<'lib, extern "C" fn(&mut ())>,
}

#[cfg(feature = "game")]
impl<'lib> ItemConstructor<'lib> {
    pub fn load_from_lib(type_name: &str, lib: &'lib Library) -> ItemConstructor<'lib> {
        let ident = format!("__item_new_{}", type_name);
        let constructor = unsafe { lib.get(ident.as_bytes()) }.unwrap();
        let ident = format!("__item_id_{}", type_name);
        let id = unsafe { lib.get(ident.as_bytes()) }.unwrap();
        let ident = format!("__item_drop_{}", type_name);
        let drop = unsafe { lib.get(ident.as_bytes()) }.unwrap();
        ItemConstructor { constructor, id, drop }
    }

    pub fn construct_item(&self) -> ItemVTable<'lib> {
        ItemVTable { this: (self.constructor)(), id: self.id.clone(), drop: self.drop.clone() } 
    }
}

pub fn alloc<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

pub fn drop<T>(ptr: *mut T) {
    unsafe {
        std::mem::drop(Box::from_raw(ptr));
    }
}

#[macro_export]
macro_rules! __export_item {
    ($t:ty) => {
        $crate::concat_idents!(fn_name = __item_new_, $t {
            #[no_mangle]
            pub extern "C" fn fn_name() -> *mut $t {
                $crate::alloc(<$t>::new())
            }
        });
        $crate::concat_idents!(fn_name = __item_id_, $t {
            #[no_mangle]
            pub extern "C" fn fn_name(this: *mut $t) -> i64 {
                (unsafe { &mut *this }).id()
            }
        });
        $crate::concat_idents!(fn_name = __item_drop_, $t {
            #[no_mangle]
            pub extern "C" fn fn_name(this: *mut $t) {
                $crate::drop(this);
            }
        });
    };
}

#[macro_export]
macro_rules! export_items {
    ($($t:ty), *) => {
        const _: () = {
            // The inner type of FFIVec is FFIStaticStr.
            #[no_mangle]
            pub extern "C" fn __exported_items() -> $crate::ffi_types::FFIVec {
                let mut vec: Vec<$crate::ffi_types::FFIStaticStr> = vec![];
                $(
                    vec.push($crate::ffi_types::FFIStaticStr::from_str(stringify!($t)));
                ) *
                vec.into()
            }
            $(
                $crate::__export_item!($t);
            ) *
        };
    };
}
