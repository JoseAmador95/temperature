//! This is the main file for the temperature conversion cli tool.

use clap::Parser;

mod cli;
mod temperature;

/// The possible units for temperature conversion.
#[derive(Debug)]
enum Unit {
    /// Celsius unit.
    Celcius,
    /// Fahrenheit unit.
    Farenheit,
    /// Kelvin unit.
    Kelvin,
}

impl Unit {
    /// Parse a string to a unit.
    ///
    /// # Errors
    /// Returns an error if the string is not a valid unit.
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            cli::CELCIUS_SHORT_NAME => Ok(Unit::Celcius),
            cli::FARENHEIT_SHORT_NAME => Ok(Unit::Farenheit),
            cli::KELVIN_SHORT_NAME => Ok(Unit::Kelvin),
            _ => Err(format!("Invalid unit: {unit}", unit = s)),
        }
    }

    /// Get the string representation of the unit.
    fn to_str_unit(&self) -> &'static str {
        match self {
            Unit::Celcius => "°C",
            Unit::Farenheit => "°F",
            Unit::Kelvin => "K",
        }
    }
}

/// Convert a temperature from one unit to another.
///
/// # Errors
/// Returns an error if the conversion is not possible.
fn convert_temp(temp: f64, input_unit: &Unit, output_unit: &Unit) -> f64 {
    match (input_unit, output_unit) {
        (Unit::Celcius, Unit::Celcius)
        | (Unit::Farenheit, Unit::Farenheit)
        | (Unit::Kelvin, Unit::Kelvin) => temp,
        (Unit::Celcius, Unit::Farenheit) => temperature::celcius_to_fahrenheit(temp),
        (Unit::Celcius, Unit::Kelvin) => temperature::celcius_to_kelvin(temp),
        (Unit::Farenheit, Unit::Celcius) => temperature::fahrenheit_to_celcius(temp),
        (Unit::Farenheit, Unit::Kelvin) => temperature::fahrenheit_to_kelvin(temp),
        (Unit::Kelvin, Unit::Celcius) => temperature::kelvin_to_celcius(temp),
        (Unit::Kelvin, Unit::Farenheit) => temperature::kelvin_to_fahrenheit(temp),
    }
}

/// The main function for the cli tool.
///
/// # Errors
/// Returns an error if the conversion is not possible.
fn main() -> Result<(), String> {
    let args = cli::Args::parse_from(std::env::args());
    let input_unit = Unit::from_str(&args.input_unit)?;
    let output_unit = Unit::from_str(&args.output_unit)?;

    println!(
        "input={temp:?}, input_unit={input_unit:?}, output_unit={output_unit:?}",
        temp = args.temperature
    );
    let output = convert_temp(args.temperature, &input_unit, &output_unit);
    println!(
        "output={output:?}{unit}",
        output = output,
        unit = output_unit.to_str_unit()
    );

    Ok(())
}
