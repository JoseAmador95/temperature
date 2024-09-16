//! Temperature conversion functions.
//!
//! The functions in this module convert temperatures between Celcius, Fahrenheit and Kelvin.
//!

/// Convert Celcius to Fahrenheit.
pub fn celcius_to_fahrenheit<T: Into<f64>>(c: T) -> f64 {
    (c.into() * 9.0 / 5.0) + 32.0
}

/// Convert Fahrenheit to Celcius.
pub fn fahrenheit_to_celcius<T: Into<f64>>(f: T) -> f64 {
    (f.into() - 32.0) * 5.0 / 9.0
}

/// Convert Kelvin to Celcius.
pub fn kelvin_to_celcius<T: Into<f64>>(k: T) -> f64 {
    k.into() - 273.15
}

/// Convert Celcius to Kelvin.
pub fn celcius_to_kelvin<T: Into<f64>>(c: T) -> f64 {
    c.into() + 273.15
}

/// Convert Fahrenheit to Kelvin.
pub fn fahrenheit_to_kelvin<T: Into<f64>>(f: T) -> f64 {
    celcius_to_kelvin(fahrenheit_to_celcius(f))
}

/// Convert Kelvin to Fahrenheit.
pub fn kelvin_to_fahrenheit<T: Into<f64>>(k: T) -> f64 {
    celcius_to_fahrenheit(kelvin_to_celcius(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: Temperature = 1e-10;

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert!((celcius_to_fahrenheit(0.0) - 32.0).abs() < EPSILON);
        assert!((celcius_to_fahrenheit(100.0) - 212.0).abs() < EPSILON);
        assert!((celcius_to_fahrenheit(-20.0) + 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_fahrenheit_to_celcius() {
        assert!((fahrenheit_to_celcius(32.0) - 0.0).abs() < EPSILON);
        assert!((fahrenheit_to_celcius(212.0) - 100.0).abs() < EPSILON);
        assert!((fahrenheit_to_celcius(-4.0) + 20.0).abs() < EPSILON);
    }

    #[test]
    fn test_kelvin_to_celcius() {
        assert!((kelvin_to_celcius(273.15) - 0.0).abs() < EPSILON);
        assert!((kelvin_to_celcius(373.15) - 100.0).abs() < EPSILON);
        assert!((kelvin_to_celcius(233.15) + 40.0).abs() < EPSILON);
    }

    #[test]
    fn test_celcius_to_kelvin() {
        assert!((celcius_to_kelvin(0.0) - 273.15).abs() < EPSILON);
        assert!((celcius_to_kelvin(100.0) - 373.15).abs() < EPSILON);
        assert!((celcius_to_kelvin(-20.0) - 253.15).abs() < EPSILON);
    }
}
