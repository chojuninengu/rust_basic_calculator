pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}