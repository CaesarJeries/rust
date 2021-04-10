#![allow(unused_labels)]
use std::io;


// convert temperatures between Celsius and Fahrenheit units
fn main() {
    let (unit, temp) = get_user_input();

    println!("Your input: {}{}", temp, unit);

    let (unit, temp) = convert(unit, temp);
    println!("Converted value: {:.4}{}", temp, unit);
}

fn get_user_input() -> (char, f64) {
    'outer: loop {
        println!("Enter source units [F / C]:");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read from standard input");

        let answer = match answer.trim().parse::<char>() {
            Ok(c) => c,
            Err(why) => {
                println!("Failed to parse character from input: {}", why);
                continue;
            }
        };

        if answer != 'C' && answer != 'F' {
            println!("Error: the input must be either F or C");
            continue;
        }

        let src_unit = answer;
       
        // prompt the user for the temperature
        let mut answer = String::new();
        'inner: loop {
            println!("Enter temperature:");
            io::stdin().read_line(&mut answer).expect("Failed to read from standard input");

            let answer = match answer.trim().parse::<f64>() {
                Ok(num) => num,
                Err(why) => {
                    println!("Failed to parse a floating-point number: {}", why);
                    continue;
                }
            };

            return (src_unit, answer);
        }
    }
}

// C = (F - 32) / 1.8
// F = 1.8*C + 32
fn convert(unit: char, temp: f64) -> (char, f64) {
    match unit {
        'C' => {
            ('F', 1.8 * temp + 32.)
        },
        'F' => {
            ('C', (temp - 32.) / 1.8)
        },
        _ => {
            ('E', 0.)
        }
    }
}
