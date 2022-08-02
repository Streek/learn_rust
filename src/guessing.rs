// import
use rand::Rng;
use std::io;

pub fn main() {
    let mut guess = String::new();

    // loop program until user presses q
    while guess != "q" {
        guess = String::new();

        // set a random number between 1 and 10
        let secret_number = rand::thread_rng().gen_range(1, 11);
        println!("Guess a number between 1 and 10!  Or input q to quit.");

        //read the user input into the mutable variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess != "q" {
                    println!("Your input was invalid. Closing...");
                } else {
                    // quit
                    break;
                }
                return;
            }
        };

        if guess == secret_number {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }
    // end program
    println!("Thanks for playing!");
}
