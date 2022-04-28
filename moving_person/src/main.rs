#[derive(Debug)]
struct Person{
    x: f64,
    y: f64,
    z: f64,
}

impl Person {
    fn set() -> Person{
        Person { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    fn moving(mut self: Person, m: Move) -> Person{
        match m {
            Move::X => {
                self.x += 1.0;
            },
            Move::Y => {
                self.y += 1.0;
            },
            Move::Z => {
                self.z += 1.0;
            },
            _ => { println!("None"); }
        }
        
        self
    }
}

enum Move{
    X,
    Y,
    Z,
}

fn main() {
    let mut person= Person::set();
    let person= person.moving(Move::X);
    println!("{:#?}", person);
}
