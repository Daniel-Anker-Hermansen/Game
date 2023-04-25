use std::ops::Deref;

use plugin_lib::{Version, StaticString, ffi_types::{FFIString, FFIVec}};

const __API_VERSION: &[u8] = b"__API_VERSION";
const __PLUGIN_NAME: &[u8] = b"__PLUGIN_NAME";

fn main() {
    let plugins = std::fs::read_dir("plugins").unwrap();
    for file in plugins {
        match file {
            Ok(file) => {
                let path = file.path();
                let lib = unsafe { libloading::Library::new(path).unwrap() };
                let version = unsafe { lib.get::<&'static Version>(__API_VERSION).unwrap() };
                let name = unsafe { lib.get::<&'static StaticString>(__PLUGIN_NAME).unwrap() };
                let name: &str = (*name.deref()).into();
                println!("Plugin name: {}", name);
                println!("{}.{}.{}", version.major, version.minor, version.patch);
                if let Ok(exported_items) = unsafe { lib.get::<extern "C" fn() -> FFIVec>(b"__exported_items") } {
                    let ffi_vec = exported_items();
                    let vec: Vec<FFIString> = unsafe { ffi_vec.to_vec() };
                    for item in vec {
                        let string: String = item.into();
                        let ident = format!("__item_id_{}", string);
                        let item_id = unsafe { lib.get::<extern "C" fn() -> i64>(ident.as_bytes()).unwrap() };
                        let id = item_id();
                        println!("{} has id: {}", string, id);
                    }
                }
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}
