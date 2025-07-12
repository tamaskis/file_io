//! Placeholder module.

/// Example function.
///
/// # Arguments
///
/// * `x` - Floating-point number, $x$.
///
/// # Returns
///
/// $x^{2}$
pub fn example_function(x: f64) -> f64 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_function() {
        assert_eq!(example_function(2.0), 4.0);
    }
}
