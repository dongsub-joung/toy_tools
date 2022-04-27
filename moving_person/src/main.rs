struct Person{
    x: f64,
    y: f64,
    z: f64,
}

impl Person {
    fn set() -> Person{
        Person { x: 0.0, y: 0.0, z: 0.0 }
    }
}

enum Move{
    X,
    Y,
    Z,
}

fn main() {
    println!("Hello, world!");
}
