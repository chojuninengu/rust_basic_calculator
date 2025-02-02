pub fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err("Factorial is only defined for non-negative integers.".to_string());
    }
    Ok(factorial_recursive(n as u64) as f64)
}

fn factorial_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}