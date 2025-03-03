# Merging two jsons together
Small rust library just to merge two jsons

# Installation
Add it as dependency in your crate's `Cargo.toml` file
```toml
[dependencies]
merge_json = { git = "https://github.com/p1k0chu/merge-json-rs.git" }
```
After this, you can view docs:
```
cargo doc -p merge_json --open
```

# Example
```rust
use serde_json::json;
use merge_json::Merge;

let mut a = json!({"name": {"first": "John", "last": "Doe"}, "age": 18});
let b = json!({"name": {"first": "John"}, "age": 19});

a.merge(&b);

let output = json!({"age": 19, "name": {"first": "John", "last": "Doe"}});

assert_eq!(&a, &output);
```
