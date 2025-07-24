pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    f / 1.8 - 32
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(-459.67);
        let result1 = celsius_to_fahrenheit(0.0);
        assert_eq!(result, -273.15);
        assert_eq!(result1, 32.0);
    }
}
