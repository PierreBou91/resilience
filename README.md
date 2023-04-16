# Retry Function

This repository contains a basic implementation of a retry function in Rust. It is a simple and minimal example, designed to help you understand the basic concept behind retrying a function until it succeeds or reaches a maximum number of retries.

## Disclaimer

This implementation is for educational purposes and may not be suitable for production use. For a more feature-rich and production-ready implementation, we recommend using the [backoff crate](https://github.com/ihrwein/backoff).

## Usage

The retry function takes a closure or function that returns a `Result<T, Box<dyn std::error::Error>>` and the maximum number of times the function should be retried. The retry function will keep executing the provided function until it succeeds or reaches the maximum number of retries.

```rust
use resilience::retry;

let mut count = 0;
let result = retry(5, || {
    count += 1;
    if count < 4 {
        Err("Failed".into())
    } else {
        Ok(count)
    }
});

assert!(result.is_ok());
assert_eq!(result.unwrap(), 4);
```

For more details and examples, please refer to the documentation comments and test cases in the source code.
