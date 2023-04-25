#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use plugin_lib::{version, name, Item, export_items};
const _: () = {
    #[no_mangle]
    pub static __API_VERSION: ::plugin_lib::Version = ::plugin_lib::__api_version_inner();
};
const _: () = {
    #[no_mangle]
    pub static __PLUGIN_NAME: ::plugin_lib::ffi_types::FFIStaticStr = ::plugin_lib::ffi_types::FFIStaticStr::from_str(
        "Hello from Rust!",
    );
};
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
const _: () = {
    #[no_mangle]
    pub extern "C" fn __exported_items() -> ::plugin_lib::ffi_types::FFIVec {
        let mut vec: Vec<::plugin_lib::ffi_types::FFIStaticStr> = ::alloc::vec::Vec::new();
        vec.push(::plugin_lib::ffi_types::FFIStaticStr::from_str("Marker"));
        vec.push(::plugin_lib::ffi_types::FFIStaticStr::from_str("AlsoMarker"));
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
};
