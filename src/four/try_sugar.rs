/// Result can be incredibly powerful, but as we'll show, managing chains of fallible computations can be cumbersome.
/// Rust provides syntactic sugar for short-circuiting evaluation with the `?`  
/// This is informally called the `yeet` operator by some.
///
/// Javascript has a '?' which is similar, but more specific. It is used for short-circuiting chains of property references.
/// It is very useful, but also limited in scope.
///
/// Rust's `?` can be used in more places, but is also quite abrupt.  Instead of short-ciruiting an expression. It short-curcuits the function.
///  
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn take_1(x: f64, y: f64) -> MathResult {
        let ratio = match div(x, y) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        let log = match ln(ratio) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        sqrt(log)
    }

    // Intermediate function
    fn take_2(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match take_2(x, y) {
            Err(why) => panic!(
                "{}",
                match why {
                    MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                    MathError::DivisionByZero => "division by zero",
                    MathError::NegativeSquareRoot => "square root of negative number",
                }
            ),
            Ok(value) => println!("{}", value),
        }
    }
}

fn run() {
    checked::op(1.0, 10.0);
}
