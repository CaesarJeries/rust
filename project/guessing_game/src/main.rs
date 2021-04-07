use std::io;
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
    println!("Guess the number!");

   let guess = get_guess();
   println!("Your guess: {}", guess);

   let random_number = get_random();
   println!("The generated number: {}", random_number);

   let guess_int = guess.trim()
                   .parse::<i32>()
                   .expect("Failed to parse integer from string");

   let result = guess_int == random_number;

   match result {
       true => println!("You won"),
       false => println!("You lose")
   }
}
