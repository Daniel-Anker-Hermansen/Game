#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use plugin_lib::{version, name, Item, export_items};
mod __api_version {
    use plugin_lib::{Version, __api_version_inner};
    #[no_mangle]
    pub static __API_VERSION: Version = __api_version_inner();
}
mod __plugin_name {
    use plugin_lib::{StaticString, from_str};
    #[no_mangle]
    pub static __PLUGIN_NAME: StaticString = from_str("Hello from Rust!");
}
struct Marker;
impl Item for Marker {
    fn id() -> i64 {
        12
    }
}
struct AlsoMarker;
impl Item for AlsoMarker {
    fn id() -> i64 {
        23
    }
}
mod __export_items {
    use super::*;
    #[no_mangle]
    pub extern "C" fn __exported_items() -> ::plugin_lib::ffi_types::FFIVec {
        let mut vec: Vec<::plugin_lib::ffi_types::FFIString> = ::alloc::vec::Vec::new();
        vec.push("Marker".to_string().into());
        vec.push("AlsoMarker".to_string().into());
        vec.into()
    }
    #[no_mangle]
    pub extern "C" fn __item_id_Marker() -> i64 {
        <Marker>::id()
    }
    #[no_mangle]
    pub extern "C" fn __item_id_AlsoMarker() -> i64 {
        <AlsoMarker>::id()
    }
}
