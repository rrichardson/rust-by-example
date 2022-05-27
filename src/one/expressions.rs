// In Rust, almost everything is an expression.
// If/else blocks, functions, match statements
// This is one of the most simple, yet powerful
// differentiators in Rust (and other functional languages)

// if / else
fn check(num: i64) -> &'static str {
    let result = if num < 0 {
        "negative"
    } else if num > 0 {
        "positive"
    } else {
        "zero"
    };

    result
}

fn check_again(num: i64) -> &'static str {
    if num < 0 {
        "negative"
    } else if num > 0 {
        "positive"
    } else {
        "zero"
    }
}

fn switch_it_up(num: i64) -> &'static str {
    match num {
        x if x < 0 => "negative",
        x if x > 0 => "positive",
        _ => "zero",
    }
}

// advanced
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
