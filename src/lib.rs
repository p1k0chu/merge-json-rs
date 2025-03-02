//! Merge two jsons together.
//!
//! # Examples
//! ```
//! use serde_json::json;
//! use merge_json::Merge;
//!
//! let mut a = json!({"name": {"first": "John", "last": "Doe"}, "age": 18});
//! let b = json!({"name": {"first": "John"}, "age": 19});
//!
//! a.merge(&b);
//!
//! let output = json!({"age": 19, "name": {"first": "John", "last": "Doe"}});
//!
//! assert_eq!(&a, &output);
//! ```

use serde_json::Value;

/// Merge two objects
pub trait Merge {
    /// `second` remains unchanged, `self` will be the merged version
    fn merge(&mut self, second: &Self);
}

/// merges two objects [`serde_json::Value`]
impl Merge for Value {
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use merge_json::Merge;
    /// #
    /// let mut a = json!({"name": {"first": "John", "last": "Doe"}, "age": 18});
    /// let b = json!({"name": {"first": "John"}, "age": 19});
    ///
    /// a.merge(&b);
    ///
    /// let output = json!({"age": 19, "name": {"first": "John", "last": "Doe"}});
    ///
    /// assert_eq!(&a, &output);
    /// ```
    fn merge(&mut self, second: &Self) {
        match (self, second) {
            (Value::Object(first), Value::Object(second)) => {
                for (k, new_value) in second {
                    match first.get_mut(k) {
                        Some(og_value) => {
                            og_value.merge(new_value);
                        }
                        None => {
                            first.insert(k.clone(), new_value.clone());
                        }
                    }
                }
            }
            (Value::Array(first), Value::Array(second)) => {
                first.extend(second.clone());
            }
            (a, b) => {
                *a = b.clone();
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_test() {
        let mut a = json!({"name": {"first": "John", "last": "Doe"}, "age": 70, "firstdo": ["task1", "task2"], "items": {"thing": {"s": 69, "c": 3}}});
        let b = json!({"name": {"first": "John"}, "age": 69, "firstdo": ["task2", "task3"], "items": {"thing": {"s": 1, "b": 2}}});

        a.merge(&b);

        let output = json!({"name": {"first": "John", "last": "Doe"}, "age": 69, "firstdo":["task1", "task2", "task2", "task3"], "items": {"thing": {"s": 1, "c": 3, "b": 2}}});

        assert_eq!(&a, &output);
    }
}
