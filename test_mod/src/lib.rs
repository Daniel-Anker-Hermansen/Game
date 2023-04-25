use plugin_lib::{version, name, Item, export_items};

version!();

name!("Hello from Rust!");

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

export_items!(Marker, AlsoMarker);
