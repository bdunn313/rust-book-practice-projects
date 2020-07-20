use std::io;

fn main() {
    // Temperature converter project
    temperature_converter();
}

fn temperature_converter() {
    enum Temperature {
        Celsius,
        Farenheit,
    };
    println!("Temperature converter.");
    loop {
        println!("Is your temperature in [C]elcius or [F]arenheit?");
        let mut temperature_mode = String::new();
        io::stdin()
            .read_line(&mut temperature_mode)
            .expect("Couldn't read stdin!");
        let temperature_mode = match temperature_mode.trim() {
            "c" | "C" => Temperature::Celsius,
            "f" | "F" => Temperature::Farenheit,
            _ => continue,
        };
        println!(
            "You've selected {}",
            match temperature_mode {
                Temperature::Celsius => "Celsius",
                Temperature::Farenheit => "Farenheit",
            }
        );
        println!("Enter your temperature");
        let mut temperature_degrees = String::new();
        io::stdin()
            .read_line(&mut temperature_degrees)
            .expect("Couldn't read stdin!");
        let temperature_degrees: f64 = match temperature_degrees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let (converted_temperature, mode_symbol) = match temperature_mode {
            Temperature::Celsius => (celcius_to_farenheit(temperature_degrees), 'F'),
            Temperature::Farenheit => (farenheit_to_celcius(temperature_degrees), 'C'),
        };
        println!("----------------------------");
        println!(
            "Start: {}°{}",
            temperature_degrees,
            match temperature_mode {
                Temperature::Celsius => 'C',
                Temperature::Farenheit => 'F',
            }
        );
        println!("Converted: {}°{}", converted_temperature, mode_symbol);
        break;
    }
}

fn celcius_to_farenheit(temp_in_celcius: f64) -> f64 {
    (temp_in_celcius * 9.0 / 5.0) + 32.0
}

fn farenheit_to_celcius(temp_in_farenheit: f64) -> f64 {
    (temp_in_farenheit - 32.0) * 5.0 / 9.0
}
