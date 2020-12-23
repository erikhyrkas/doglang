#Rust Snippets
Displays the type of an object (debug use only):

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
```

#Dog Snippets
