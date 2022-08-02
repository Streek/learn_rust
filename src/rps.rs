// import
use rand::Rng;
use std::io;

pub fn main() {
    let mut user_input = String::new();
    let mut user_score: u32 = 0;
    let mut computer_score: u32 = 0;
    let mut _computer_pick: u32 = rand::thread_rng().gen_range(1..4);

    while user_input != "r" && user_input != "p" && user_input != "s" {
        user_input = String::new();
        _computer_pick = rand::thread_rng().gen_range(1..4);

        //print user and computer score
        println!(
            "User Score: {} Computer Score: {}",
            user_score, computer_score
        );
        println!("Enter Rock, Paper, or Scissors (r,p, and s work as well), or q to quit: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // get first character of input as lowercase character exit if error
        let input_char: char = match user_input.trim().chars().next() {
            Some(c) => c.to_lowercase().next().unwrap(),
            None => {
                println!("Your input was invalid. Closing...");
                return;
            }
        };

        if input_char == 'q' {
            break;
        }

        // make computer pick r,p,s

        // match input_char with r,p, s
        match input_char {
            'r' => println!("Rock"),
            'p' => println!("Paper"),
            's' => println!("Scissors"),
            _ => println!("Invalid input"),
        };

        // print computer pick
        match _computer_pick {
            1 => println!("vs. Rock"),
            2 => println!("vs. Paper"),
            3 => println!("vs. Scissors"),
            _ => println!("vs. Invalid input"),
        };

        // see if user wins and increment score
        if input_char == 'r' && _computer_pick == 3
            || input_char == 'p' && _computer_pick == 1
            || input_char == 's' && _computer_pick == 2
        {
            println!("You win!");
            user_score += 1;
        } else if input_char == 'r' && _computer_pick == 1
            || input_char == 'p' && _computer_pick == 2
            || input_char == 's' && _computer_pick == 3
        {
            println!("It's a tie!");
        } else {
            println!("You lose!");
            computer_score += 1;
        }
    }
    // end program
    println!("Thanks for playing!");
}
