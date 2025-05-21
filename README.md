Convenient conversion of bool states

# Examples

```rust
# use to_true::ToTrue;
let mut state = false;
let mut n = 0;

assert_eq!(state.to_true(|| n += 1), Some(()));
assert_eq!((n, state), (1, true));

assert_eq!(state.to_true(|| n += 1), None);
assert_eq!((n, state), (1, true));

assert_eq!(state.to_false(|| n += 1), Some(()));
assert_eq!((n, state), (2, false));

assert_eq!(state.to_false(|| n += 1), None);
assert_eq!((n, state), (2, false));
```
