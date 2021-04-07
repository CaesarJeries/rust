use std::io;

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
}