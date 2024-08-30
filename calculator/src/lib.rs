pub mod calculator {
    pub fn parse_to_float(number: &str) -> Result<f64, String> {
        match number.trim().parse::<f64>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("Error: {} is not a valid number", number.trim())),
        }
    }
}