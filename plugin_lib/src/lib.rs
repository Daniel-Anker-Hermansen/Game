pub mod ffi_types;
use std::marker::PhantomData;

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

pub trait Item: Sized {
    fn new() -> Self;

    fn id(&mut self) -> i64;
}

#[cfg(feature = "plugin")]
trait ItemRawAdapter: Item {
    extern fn new() -> *mut () {
        alloc(<Self as Item>::new()).cast()
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
pub const fn item_vtable<T>() -> ItemVTable<'static> where T: Item {
    ItemVTable { 
        new: <T as ItemRawAdapter>::new, 
        id: <T as ItemRawAdapter>::id, 
        drop: <T as ItemRawAdapter>::drop, 
        _phantom: PhantomData 
    }
}

#[cfg(feature = "game")]
pub trait ItemGame {
    fn id(&mut self) -> i64;
}

#[repr(C)]
pub struct ItemVTable<'lib> {
    new: extern fn() -> *mut (),
    id: extern fn(*mut ()) -> i64 ,
    drop: extern fn(*mut ()),
    _phantom: PhantomData<&'lib ()>,
}

#[cfg(feature = "game")]
impl<'lib> ItemVTable<'lib> {
    pub fn new(&'lib self) -> ItemInstance<'lib> {
        ItemInstance { this: (self.new)(), vtable: self }
    }
}

#[cfg(feature = "game")]
pub struct ItemInstance<'lib> {
    this: *mut (),
    vtable: &'lib ItemVTable<'lib>,
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

pub fn alloc<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

pub fn drop<T>(ptr: *mut T) {
    unsafe {
        std::ptr::drop_in_place(ptr);
    }
}

/*#[macro_export]
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
}*/

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
