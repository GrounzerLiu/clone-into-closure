# Clone Into Closure

`Clone Into Closure` is a procedural macro for Rust that simplifies working with closures by automatically cloning variables into closures


Here is an example demonstrating how to use the `clone_into_closure` attribute macro:

### Example

```rust
use clone_into_closure::clone_into_closure;
use std::sync::{Arc, Mutex};

#[clone_into_closure]
fn main() {
    let a = Arc::new(Mutex::new("a"));

    test(move || {
        clone(a, b, c);
        println!("{}", a.lock().unwrap());
    });
}

fn test(f: impl Fn()) {
    f();
}
```

### What Happens

The macro automatically transforms the code into:

```rust
test({
move || {
        clone(a, b, c);
        println!("{}", a.lock().unwrap());
    }});
```
