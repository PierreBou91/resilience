#![warn(rust_2018_idioms, missing_docs)]

//! This module provides a `retry` function that retries executing a given function
//! for a specified number of attempts.

/// Retry a function a number of times.
///
/// Executes a provided function up to `num_retries` times until it succeeds,
/// and returns the successful result wrapped in `Ok`. If the function fails
/// on all attempts, the last error is returned wrapped in `Err`.
///
/// # Arguments
///
/// * `num_retries` - The maximum number of times to retry executing the function.
/// * `function` - A function or closure that returns a `Result<T, Box<dyn std::error::Error>>`.
///
/// # Examples
///
/// ```
/// use resilience::retry;
/// let mut count = 0;
/// let result = retry(5, || {
///     count += 1;
///     if count < 4 {
///         Err("Failed".into())
///     } else {
///         Ok(count)
///     }
/// });
///
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 4);
/// ```
pub fn retry<F, T>(num_retries: u32, mut function: F) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnMut() -> Result<T, Box<dyn std::error::Error>>,
{
    for i in 0..num_retries {
        match function() {
            Ok(v) => return Ok(v),
            Err(e) => {
                println!("Tried {} times, got error: {:?}", i + 1, e);
                continue;
            }
        }
    }
    function()
}

/// Tests for the `retry` function.
#[cfg(test)]
mod tests {
    use super::retry;

    /// Test whether the `retry` function succeeds on the first try.
    #[test]
    fn test_success_on_first_try() {
        let count = 1;
        let result = retry(3, || Ok(count));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    /// Test whether the `retry` function fails after all attempts.
    #[test]
    fn test_always_fail() {
        let result: Result<u32, Box<dyn std::error::Error>> =
            retry(3, || Err("Always fail".to_string().into()));
        assert!(result.is_err());
    }

    /// Test whether the `retry` function succeeds after a few attempts.
    #[test]
    fn test_success_after_retries() {
        let mut count = 0;
        let result = retry(5, || {
            count += 1;
            if count < 4 {
                Err(format!("Failed on attempt {}", count).into())
            } else {
                Ok(count)
            }
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    /// Test whether the `retry` function barely succeeds within the allowed number of retries.
    #[test]
    fn test_barely_succeeds() {
        let mut count = 0;
        let result = retry(5, || {
            count += 1;
            if count < 5 {
                Err(format!("Failed on attempt {}", count).into())
            } else {
                Ok(count)
            }
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }
}
