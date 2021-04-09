use std::io;
use std::cmp::Ordering; // an enum that's returned from the cmp method of all comparable type
use rand::Rng; // this is a create provided by the community (crates.io)

// the first cargo build (or run) downloads all dependencies from the registry (crates.io)
// actual used versions are written in Cargo.lock (to ensure reproducable builds: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file)
// as long as dependencies don't change, consecutive build won't download the crates unnecessarilly


// cargo has a very useful feature: cargo doc --open:
// generates documentation for all used creates of current project (dependencies), and
// opens the docs in browser (html)

fn get_random() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

fn get_guess() -> String {
    println!("Enter your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    return guess;
}

fn main() {

    let mut num_tries = 5;
    loop { // an infinite loop
        println!("Guess the number! [{} tries remaining]", num_tries);

        let guess = get_guess();

        // Note: you can use variable shadowing in this case (it's actually recommended)
        //       so instead of guess_str / guess or guess_int, you can simply use the same symbol to
        //       refer to the integer value.
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(why) => {
                println!("Failed to parse integer: {}", why);
                continue;
            },
        };
                        
        let random_number = get_random();
        println!("The generated number: {}", random_number);
        println!("Your guess: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            },

            _ => println!("Try again"),
        }

        num_tries -= 1;
        if num_tries == 0 {
            println!("Tough luck :/");
            break;
        }
    }
}
