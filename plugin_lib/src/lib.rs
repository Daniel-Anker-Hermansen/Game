pub mod ffi_types;

#[repr(C)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[macro_export]
macro_rules! version {
    () => {
        mod __api_version {        
            use plugin_lib::{Version, __api_version_inner};
            #[no_mangle]
            pub static __API_VERSION: Version = __api_version_inner();
        }
    };
}

#[inline]
pub const fn __api_version_inner() -> Version {
    Version {
        major: 0,
        minor: 1,
        patch: 0,
    }
}

/// Represents &'static str for ffi boundry.
#[repr(C)]
pub struct StaticString {
    ptr: *const u8,
    len: usize,
}

unsafe impl Sync for StaticString { }
unsafe impl Send for StaticString { }

impl From<&'static str> for StaticString {
    fn from(value: &'static str) -> Self {
        from_str(value)
    }
}

impl<'a> From<&'a StaticString> for &'static str {
    fn from(value: &'a StaticString) -> Self {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(value.ptr, value.len)) }
    }
}

pub const fn from_str(value: &str) -> StaticString {
    StaticString { ptr: value.as_ptr(), len: value.len() } 
} 

#[macro_export]
macro_rules! name {
    ($name:expr) => {
        mod __plugin_name {
            use plugin_lib::{StaticString, from_str};
            #[no_mangle]
            pub static __PLUGIN_NAME: StaticString = from_str($name);
        }
    };
}

pub trait Item {
    fn id() -> i64;
}

pub use concat_idents::concat_idents;

#[macro_export]
macro_rules! __export_item {
    ($t:ty) => {
        $crate::concat_idents!(fn_name = __item_id_, $t {
            #[no_mangle]
            pub extern "C" fn fn_name() -> i64 {
                <$t>::id()
            }
        });
    };
}

#[macro_export]
macro_rules! export_items {
    ($($t:ty), *) => {
        mod __export_items {
            use super::*;
            // The inner type of FFIVec is FFIString.
            #[no_mangle]
            pub extern "C" fn __exported_items() -> $crate::ffi_types::FFIVec {
                let mut vec: Vec<$crate::ffi_types::FFIString> = vec![];
                $(
                    vec.push(stringify!($t).to_string().into());
                ) *
                vec.into()
            }
            $(
                $crate::__export_item!($t);
            ) *
        }
    };
}
