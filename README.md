# Merging two jsons together
Small rust library just to merge two jsons

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
