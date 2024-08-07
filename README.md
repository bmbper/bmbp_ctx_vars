# BMBP_CONTEXT_VARS

```rust
(&*BMBP_CONTEXT_VARS).set_value("A".to_string(), "B".to_string());
assert_eq!(
   &*BMBP_CONTEXT_VARS.get_value("A".to_string()),
   "B".to_string()
);
assert_eq!(
   &*BMBP_CONTEXT_VARS.get_value("D".to_string()),
   "".to_string()
);
```
