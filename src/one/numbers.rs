// Let's start easy: numbers.
// Where TypeScript has a single `number` type, Rust distinguishes between
// various types of numbers.

// All number types and more information on them can be found at
// https://doc.rust-lang.org/reference/types/numeric.html

/// # Example
pub fn add_a(a: i64, b: i64) -> i64 {
    let result = a + b;
    result
}

// An alternate return style
// Not often used in functions with a single termination
pub fn add_a_alternate(a: i64, b: i64) -> i64 {
    let result = a + b;
    return result;
}

pub fn add_b(a: u32, b: u16) -> u64 {
    let result = a + (b as u32);
    result as u64
}

pub fn add_c(a: u128, b: u128) -> i16 {
    let result = a + b;
    result as i16
}
