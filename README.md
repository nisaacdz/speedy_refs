# SPEEDY_REFS

A collection of simple, Fast, and light smart pointers for rust.

Contains faster and lighter alternatives to std smart pointers and much more.

# FEATURES

- **Rc** - Faster and lighter alternative to the std reference counting smart pointer
- **RefCell** - Faster and lighter alternative to the std RefCell
- **Arc** - Lighter alternative the std arc with equivalent performance
- **Cell** - Extremely fast and smart pointer for mutable access from an immutable context
- **Reon** - Read only static pointer based on the `Arc` and implements `Sync` and `Send`

# DEPENDENCIES



# INSTALLATION

* Cargo command:
`cargo add speedy_refs`

* From Cargo.toml: 
```
[dependencies]
speedy_refs = "0.2.0"
```

# Example
```
use std::thread;
use std::sync::{Arc, Barrier};
use speedy_refs::Reon;

fn main() {
    let x = Reon::new(42);
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut threads = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let x = x.clone();
        let barrier = Arc::clone(&barrier);
        let thread = thread::spawn(move || {
            barrier.wait();
            println!("Thread {:?} sees value: {}", thread::current().id(), *x);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}


```

# LICENSE
**MIT license**