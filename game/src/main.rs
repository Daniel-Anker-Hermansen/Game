use plugin_lib::{Version, StaticString};

const __API_VERSION: &[u8] = b"__api_version";
const __PLUGIN_NAME: &[u8] = b"__plugin_name";

fn main() {
    let plugins = std::fs::read_dir("plugins").unwrap();
    for file in plugins {
        match file {
            Ok(file) => {
                let path = file.path();
                let lib = unsafe { libloading::Library::new(path).unwrap() };
                let __api_version = unsafe { lib.get::<extern "C" fn() -> Version>(__API_VERSION).unwrap() };
                let __plugin_name = unsafe { lib.get::<extern "C" fn() -> StaticString>(__PLUGIN_NAME).unwrap() };
                let __item_id_marker = unsafe { lib.get::<extern "C" fn() -> i64>(b"__item_id_Marker").unwrap() };
                let name: &str = __plugin_name().into();
                let version = __api_version();
                let id = __item_id_marker();
                println!("Plugin name: {}", name);
                println!("{}.{}.{}", version.major, version.minor, version.patch);
                println!("Marker id: {id}");
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}
