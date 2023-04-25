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
