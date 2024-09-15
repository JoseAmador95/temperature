use clap::Parser;

mod cli;
mod temperature;

#[derive(Debug)]
enum Unit {
    Celcius,
    Farenheit,
    Kelvin,
}

impl Unit {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            cli::CELCIUS_SHORT_NAME => Ok(Unit::Celcius),
            cli::FARENHEIT_SHORT_NAME => Ok(Unit::Farenheit),
            cli::KELVIN_SHORT_NAME => Ok(Unit::Kelvin),
            _ => Err(format!("Invalid unit: {unit}", unit = s)),
        }
    }

    fn to_str_unit(&self) -> &'static str {
        match self {
            Unit::Celcius => "°C",
            Unit::Farenheit => "°F",
            Unit::Kelvin => "K",
        }
    }
}

fn convert_temp(
    temp: temperature::Temperature,
    input_unit: &Unit,
    output_unit: &Unit,
) -> temperature::Temperature {
    match (input_unit, output_unit) {
        (Unit::Celcius, Unit::Celcius) => temp,
        (Unit::Celcius, Unit::Farenheit) => temperature::celcius_to_fahrenheit(temp),
        (Unit::Celcius, Unit::Kelvin) => temperature::celcius_to_kelvin(temp),
        (Unit::Farenheit, Unit::Farenheit) => temp,
        (Unit::Farenheit, Unit::Celcius) => temperature::fahrenheit_to_celcius(temp),
        (Unit::Farenheit, Unit::Kelvin) => temperature::fahrenheit_to_kelvin(temp),
        (Unit::Kelvin, Unit::Kelvin) => temp,
        (Unit::Kelvin, Unit::Celcius) => temperature::kelvin_to_celcius(temp),
        (Unit::Kelvin, Unit::Farenheit) => temperature::kelvin_to_fahrenheit(temp),
    }
}

fn main() {
    let args = cli::Args::parse_from(std::env::args());
    let input_unit = Unit::from_str(&args.input_unit).unwrap();
    let output_unit = Unit::from_str(&args.output_unit).unwrap();
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
}
