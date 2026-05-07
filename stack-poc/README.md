# Stack POC

## Output

```
cargo run

Stack POC!
Pushing...
Stack { elements: [10, 20, 30] }
Stack Length: 3
Pop: Some(30)
After pop: Stack { elements: [10, 20] }
Popped: Some(20)
Popped: Some(10)
Stack Length: 0
Empty: true
```

## Tests
```
cargo test

running 2 tests
test stack::tests::should_init_with_empty_elements ... ok
test stack::tests::should_push_and_pop ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```