# SPEEDY_REFS

A collection of simple, Fast, and light smart pointers for rust.4

Contains faster and lighter alternatives to std smart pointers.

# FEATURES

- **Rc** - Faster and lighter alternative to the std reference counting smart pointer
- **RefCell** - Faster and lighter alternative to the std RefCell
- **Arc** - Lighter alternative the std arc with equivalent performance
- **Cell** - Extremely fast and smart pointer for mutable access from an immutable context
- **Reon** - Read only smart pointer based on the `Arc` and implements `Sync` and `Send`
- **Rajax** - A more bare-metal implementation of `Reon` and contains static data that should not be used with any type of interior mutability

# DEPENDENCIES



# INSTALLATION

* Cargo command:
`cargo add speedy_refs`

* From Cargo.toml: 
```
[dependencies]
speedy_refs = "0.1.0"
```


# LICENSE
**MIT license**