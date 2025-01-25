use clone_into_closure::clone_into_closure;
use std::sync::{Arc, Mutex};

#[clone_into_closure]
fn main() {
    let a = Arc::new(Mutex::new("a"));
    let b = Arc::new(Mutex::new("b"));
    let c = Arc::new(Mutex::new("c"));
    test(move || {
        clone(a, b, c);
        println!("{}", a.lock().unwrap());
        println!("{}", b.lock().unwrap());
        println!("{}", c.lock().unwrap());
    });
}

fn test(f: impl Fn()) {
    f();
}
