use plugin_lib::{meta, Item, export_items};

meta!("Hello from Rust!", 0, 1, 0);

pub struct Marker {
    id: i64,
}

impl Item for Marker {
    fn new() -> Self {
        Marker { id: 0 }
    }

    fn name() -> &'static str {
        "Counter"
    }

    fn id(&mut self) -> i64 {
        self.id += 1;
        self.id
    }
}

pub struct Fibonacci {
    prev1: i64,
    prev2: i64,
}

impl Item for Fibonacci {
    fn new() -> Self {
        Fibonacci { prev1: 0, prev2: 1 }
    }

    fn name() -> &'static str {
        "Fibonacci"
    }

    fn id(&mut self) -> i64 {
        let tmp = self.prev1 + self.prev2;
        self.prev1 = self.prev2;
        self.prev2 = tmp;
        self.prev1
    }
}

pub struct Panic;

impl Item for Panic {
    fn new() -> Self {
        Panic
    }

    fn name() -> &'static str {
        "Panic"
    }

    fn id(&mut self) -> i64 {
        panic!("Haha i made a bad plugin");
    }
}

export_items!(Marker, Fibonacci);
