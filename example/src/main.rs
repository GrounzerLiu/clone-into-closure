use clone_into_closure::clone_into_closure;
use std::sync::{Arc, Mutex};

#[clone_into_closure]
fn main() {
    let a = Arc::new(Mutex::new("a"));
    let b = Arc::new(Mutex::new("b"));
    let c = Arc::new(Mutex::new("c"));
    test(move |(a, b, c), _| {
        println!("{}", a.lock().unwrap());
        println!("{}", b.lock().unwrap());
        println!("{}", c.lock().unwrap());
    }, 0.0);

    *a.lock().unwrap() = "A";
    *b.lock().unwrap() = "B";
    *c.lock().unwrap() = "C";

    println!("{}", a.lock().unwrap());
    println!("{}", b.lock().unwrap());
    println!("{}", c.lock().unwrap());
}

fn test(f: impl Fn(f32), num:f32) {
    f(0.0);
}
