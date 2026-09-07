const FORMULA: f64 = 9. / 5.;

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) / FORMULA
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * FORMULA) + 32.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahrenheit_to_celsius_works() {
        let result = fahrenheit_to_celsius(-459.67);
        assert_eq!(result, -273.15);
    }
    #[test]
    fn celsius_to_fahrenheit_works() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }
}
