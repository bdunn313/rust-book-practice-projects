use std::io;

fn main() {
    sing_carol();
}

// Temperature
// -----------------------
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

// nth Fibonnaci sequence
// ---------------------------------
fn nth_fib(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }
    let mut prev;
    let mut curr = 1;
    for _ in 1..n {
        prev = curr;
        curr = curr + prev
    }
    curr
}

// On the twelfth day of Christmas
// My true love gave to me
// 12 drummers drumming
// Eleven pipers piping
// Ten lords a leaping
// Nine ladies dancing
// Eight maids a milking
// Seven swans a swimming
// Six geese a laying
// Five gold rings, badam-pam-pam
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

fn sing_carol() {
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming",
    ];
    for (idx, _) in gifts.iter().enumerate() {
        println!("On the {} day of Christmas", match idx {
            0 => String::from("1st"),
            1 => String::from("2nd"),
            2 => String::from("3rd"),
            _ => format!("{}th", idx +1)
        });
        println!("My true love gave to me");
        for (idx2, gift) in gifts[..=idx].iter().enumerate().rev() {
            println!("{} {}", match (idx,idx2) {
                (0,0) => String::from("A"),
                (_,0) => String::from("And a"),
                _ => (idx2 + 1).to_string(),
            }, gift);
        }
        println!("");
    }
}
