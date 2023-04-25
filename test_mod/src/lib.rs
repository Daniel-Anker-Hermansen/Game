use plugin_lib::{version, name, Item, export_item};

version!();

name!("Hello from Rust!");

struct Marker;

impl Item for Marker {
    fn id() -> i64 {
        12
    }
}

export_item!(Marker);
