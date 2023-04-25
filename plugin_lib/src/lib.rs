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
            pub extern "C" fn __api_version() -> Version {
                __api_version_inner()
            }
        }
    };
}

/// Represents &'static str for ffi boundry.
#[repr(C)]
pub struct StaticString {
    ptr: *const u8,
    len: usize,
}

impl From<&'static str> for StaticString {
    fn from(value: &'static str) -> Self {
        StaticString { ptr: value.as_ptr(), len: value.len() } 
    }
}

impl From<StaticString> for &'static str {
    fn from(value: StaticString) -> Self {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(value.ptr, value.len)) }
    }
}

#[macro_export]
macro_rules! name {
    ($name:expr) => {
        mod __plugin_name {
            use plugin_lib::StaticString;
            #[no_mangle]
            pub extern "C" fn __plugin_name() -> StaticString {
                StaticString::from($name)
            }
        }
    };
}

#[inline]
pub fn __api_version_inner() -> Version {
    Version {
        major: 0,
        minor: 1,
        patch: 0,
    }
}

pub trait Item {
    fn id() -> i64;
}

pub use concat_idents::concat_idents;

#[macro_export]
macro_rules! export_item {
    ($t:ty) => {
        plugin_lib::concat_idents!(fn_name = __item_id_, $t {
            #[no_mangle]
            pub extern "C" fn fn_name() -> i64 {
                <$t>::id()
            }
        });
    };
}
