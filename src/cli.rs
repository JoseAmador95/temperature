//! Command line interface module
use clap::Parser;

/// The short name for Celsius unit.
pub const CELCIUS_SHORT_NAME: &str = "c";

/// The short name for Fahrenheit unit.
pub const FARENHEIT_SHORT_NAME: &str = "f";

/// The short name for Kelvin unit.
pub const KELVIN_SHORT_NAME: &str = "k";

/// The array of all supported unit short names.
const SUPPORTED_UNIT_SHORT: [&str; 3] =
    [CELCIUS_SHORT_NAME, FARENHEIT_SHORT_NAME, KELVIN_SHORT_NAME];

/// The command line arguments.
#[derive(Parser, Debug)]
#[clap(
    version,
    about = "A simple command line tool to convert temperatures between Celsius, Fahrenheit and Kelvin.",
    long_about = None
)]
pub struct Args {
    /// The temperature to convert.
    #[arg(required = true)]
    pub temperature: f64,

    /// The unit of the input temperature.
    #[arg(long, short, required = true, value_parser = SUPPORTED_UNIT_SHORT)]
    pub input_unit: String,

    /// The unit to convert the input temperature to.
    #[arg(long, short, required = true, value_parser = SUPPORTED_UNIT_SHORT)]
    pub output_unit: String,
}
