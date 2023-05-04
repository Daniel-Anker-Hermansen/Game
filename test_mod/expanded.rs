#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use plugin_lib::{version, name, Item, export_items};
const _: () = {
    #[no_mangle]
    pub static __API_VERSION: ::plugin_lib::Version = ::plugin_lib::Version {
        major: 0,
        minor: 1,
        patch: 0,
    };
};
const _: () = {
    #[no_mangle]
    pub static __PLUGIN_NAME: ::plugin_lib::ffi_types::FFIStaticStr = ::plugin_lib::ffi_types::FFIStaticStr::from_str(
        "Hello from Rust!",
    );
};
pub struct Marker {
    id: i64,
}
impl Item for Marker {
    fn id(&mut self) -> i64 {
        self.id += 1;
        self.id
    }
    fn new() -> Self {
        Marker { id: 0 }
    }
}
pub struct Fibonacci {
    prev1: i64,
    prev2: i64,
}
impl Item for Fibonacci {
    fn id(&mut self) -> i64 {
        let tmp = self.prev1 + self.prev2;
        self.prev1 = self.prev2;
        self.prev2 = tmp;
        self.prev1
    }
    fn new() -> Self {
        Fibonacci { prev1: 0, prev2: 1 }
    }
}
const _: () = {
    #[no_mangle]
    pub static __EXPORTED_ITEMS: &::plugin_lib::ffi_types::FFISlice = &::plugin_lib::ffi_types::slice_to_ffi(
        &[
            ::plugin_lib::item_vtable::<Marker>(),
            ::plugin_lib::item_vtable::<Fibonacci>(),
        ],
    );
};
