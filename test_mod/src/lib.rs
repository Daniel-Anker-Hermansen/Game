use plugin_lib::{version, name, Item, export_items};

version!();

name!("Hello from Rust!");

/*pub struct Marker {
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

export_items!(Marker, Fibonacci);*/
