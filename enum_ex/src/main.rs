struct Pserson<T>{
    id: T,
}


enum User{
    Pserson,
    Node(usize),
}

fn search(user: User) -> i32{
    let a= match user {
        User::Pserson => 0,
        User::Node((_)) => 1,
    };

    a
}
fn main() {
    let mut root= User::Pserson;
    let root_node= User::Node;
    let a= search(root);
    let b= search(root_node(1));

    println!("a: {}, b: {}", a, b);
}
