use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("==========Guess the number by grqphical07==========");

    // Generate the number to guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Main game loop
    loop {
        println!("Please input your guess:");

        // Initalize an empty string variable and get input from the user
        let mut guess= String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

        // Create an unsigned int version of the guess to compare with the number generated above
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Prints the guess to the stdout
        println!("You guessed: {guess}");  

        // Match expression to check if the guess was too big, too small or correct
        // If it's correct we break the loop and exit the game
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Got It");
                break;
            }
        }
    }

      
}
