# SPEEDY_REFS

A collection of simple, Fast, and useful smart pointers for rust.

# FEATURES

- **Rc** -> Blazingly fast alternative to the std `Rc` smart pointer.
- **RefCell** -> Blazingly fast alternative to the std `RefCell`.
- **Arc** - Lighter alternative the std `Arc` with equivalent performance
- **HeapCell** - Similar to `NonNull` with simpler type `deallocation` and `dropping`
- **Reon** - Read only static pointer that implements `Sync` and `Send`
- **RcCell** - Simple and more concise version of `Rc<RefCell>`
- **SharedCell** - For Shared ownership without borrow checking.
- **Borrow** - A cloneable shared ownership without borrow checking. Like how references are used in languages like java, go, python, etc.

# Upcoming

- **Atomic** - Uses atomic operations to control mutable and immutable access to any type for multithread syncing.
- **Hazard** - A hazard pointer implementation.

# DEPENDENCIES

# INSTALLATION

- **Cargo command** ->

```
cargo add speedy_refs
```

- **From Cargo.toml** ->

```
[dependencies]
speedy_refs = "0.2.6"
```

# Example

## Reon

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

## Borrow

```
use speedy_refs::Borrow;

#[derive(Debug, PartialEq, Eq)]
struct Data(String, usize, bool, Vec<Self>);

fn main() {
    // Create a new variable
    let data = Data(String::from("Hello, World"), 100, false, vec![]);
    // Create a Borrow (a reference) with the variable
    let mut data_ref = Borrow::new(data);
    // Create another reference to the same variable
    let mut clone = Borrow::clone(&data_ref);
    // Use the Borrow seemlessly
    data_ref.0.push('!');
    clone.1 += 55;
    data_ref.2 = true;
    clone.3.push(Data("".into(), 0, false, Vec::new()));

    // Debug for JavaCell is same as that for Data
    println!("{:?}", clone);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])
    println!("{:?}", data_ref);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])

    assert_eq!(
        *data_ref,
        Data(
            String::from("Hello, World!"),
            155,
            true,
            vec![Data("".into(), 0, false, vec![])]
        )
    );
    assert_eq!(
        *clone,
        Data(
            String::from("Hello, World!"),
            155,
            true,
            vec![Data("".into(), 0, false, vec![])]
        )
    );
    // Borrow implements AsRef and Deref of T
    print(&data_ref);
}

fn print<T: std::fmt::Debug>(data: &T) {
    // do something
    println!("{:?}", data);
}

```

# LICENSE

**MIT license**
