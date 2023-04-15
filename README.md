# SPEEDY_REFS

A collection of simple, Fast, and light smart pointers for rust.

Contains faster and lighter alternatives to std smart pointers and much more.

# FEATURES

- **Rc** -> Blazingly fast alternative to the std `Rc` smart pointer.
- **RefCell** -> Blazingly fast alternative to the std `RefCell`.
- **Arc** - Lighter alternative the std `Arc` with equivalent performance
- **HeapCell** - Similar to `NonNull` with simpler type deallocation and destruction
- **Reon** - Read only static pointer that implements `Sync` and `Send`

# Upcoming
- **Atomic** - Uses atomic operations to control mutable and immutable access to any type for multithread syncing.
- **Hazard** - A hazard pointer implementation.


# DEPENDENCIES



# INSTALLATION

* **Cargo command** -> 
```
cargo add speedy_refs
```

* **From Cargo.toml** -> 
```
[dependencies]
speedy_refs = "0.2.3"
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