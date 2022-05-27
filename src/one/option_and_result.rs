/// `Option<T>` and `Result<T, E>` are two of the most common
/// enums you work with in Rust:

/// These are powerful concepts that are built upon incredibly simple enums.
///
/// ```rust
/// enum Option<T> {
///     Some(T),
///     None
/// }
///
/// enum Result<T, E> {
///     Ok(T),
///     Err(e),
/// }
/// ```

/// Please look over their wonderful documentation in
/// https://doc.rust-lang.org/std/option/
/// and
/// https://doc.rust-lang.org/std/result/

/// `Option<T>` is used in place of `T | null` or `T | undefined`.
pub fn positive_n(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

/// `Result<T, E>` provides a way to manage errors in a structured way while avoiding exceptions
pub fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("division by zero".to_owned())
    } else {
        Ok(x / y)
    }
}
