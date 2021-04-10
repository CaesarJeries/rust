use std::io;

fn get_user_input() -> u8 {
    println!("Enter the index of the fibonacci number you wish to calculate (starting at 0)");
  
    loop {
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read from standard input");

        let answer = match answer.trim().parse::<u8>() {
            Ok(num) => num,
            Err(why) => {
                println!("Failed to parse integer: {}", why);
                println!("Try again.");
                continue;
            }
        };
        
        // reached if match succeeds
        return answer;
    }
}

fn fibonacci(n: u8) -> i128 {
    if n <= 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let n = get_user_input();
    let fib = fibonacci(n);

    println!("The fibonacci number at index {} is: {}", n, fib);
}
