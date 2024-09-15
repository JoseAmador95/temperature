use clap::Parser;

mod cli;
mod temperature;

#[derive(Debug)]
enum Unit {
    Celcius(temperature::Temperature),
    Farenheit(temperature::Temperature),
    Kelvin(temperature::Temperature),
}

fn parse_unit(unit: &String, temp: temperature::Temperature) -> Result<Unit, String> {
    match unit.as_str() {
        cli::CELCIUS_SHORT_NAME => Ok(Unit::Celcius(temp)),
        cli::FARENHEIT_SHORT_NAME => Ok(Unit::Farenheit(temp)),
        cli::KELVIN_SHORT_NAME => Ok(Unit::Kelvin(temp)),
        _ => Err(format!("Invalid unit: {unit}", unit = unit)),
    }
}

fn convert_temp(input: Unit, output: Unit) -> Unit {
    match (input, output) {
        (Unit::Celcius(c), Unit::Celcius(_)) => Unit::Celcius(c),
        (Unit::Celcius(c), Unit::Farenheit(_)) => {
            Unit::Farenheit(temperature::celcius_to_fahrenheit(c))
        }
        (Unit::Celcius(c), Unit::Kelvin(_)) => Unit::Kelvin(temperature::celcius_to_kelvin(c)),
        (Unit::Farenheit(f), Unit::Farenheit(_)) => Unit::Farenheit(f),
        (Unit::Farenheit(f), Unit::Celcius(_)) => {
            Unit::Celcius(temperature::fahrenheit_to_celcius(f))
        }
        (Unit::Farenheit(f), Unit::Kelvin(_)) => Unit::Kelvin(temperature::fahrenheit_to_kelvin(f)),
        (Unit::Kelvin(k), Unit::Kelvin(_)) => Unit::Kelvin(k),
        (Unit::Kelvin(k), Unit::Celcius(_)) => Unit::Celcius(temperature::kelvin_to_celcius(k)),
        (Unit::Kelvin(k), Unit::Farenheit(_)) => {
            Unit::Farenheit(temperature::kelvin_to_fahrenheit(k))
        }
    }
}

fn main() {
    let args = cli::Args::parse_from(std::env::args());
    let input = parse_unit(&args.input_unit, args.temperature).unwrap();
    let output_unit = parse_unit(&args.output_unit, 0.0).unwrap();
    println!("input={input:?}, output_unit={output_unit:?}");
    let output = convert_temp(input, output_unit);
    println!("output={output:?}");
}
