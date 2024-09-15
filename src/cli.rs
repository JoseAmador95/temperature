use clap::Parser;

pub const CELCIUS_SHORT_NAME: &str = "c";
pub const FARENHEIT_SHORT_NAME: &str = "f";
pub const KELVIN_SHORT_NAME: &str = "k";
const SUPPORTED_UNIT_SHORT: [&str; 3] =
    [CELCIUS_SHORT_NAME, FARENHEIT_SHORT_NAME, KELVIN_SHORT_NAME];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    pub temperature: f64,

    #[arg(long, short, required = true, value_parser = SUPPORTED_UNIT_SHORT)]
    pub input_unit: String,

    #[arg(long, short, required = true, value_parser = SUPPORTED_UNIT_SHORT)]
    pub output_unit: String,
}
