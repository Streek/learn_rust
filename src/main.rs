// import
use std::io;
// import main from guessing.rs
mod guessing;
mod rps;
mod web_server;

fn main() {
    // take user input
    let mut input = String::new();

    // get input from user
    println!("1 - Guessing Game");
    println!("2 - Rock Paper Scissors");
    println!("3 - Web Server (Port 3000)");
    println!("4 - Quit");
    println!("Enter your choice: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // convert input to number
    let input_num = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Your input was invalid. Closing...");
            return;
        }
    };

    match input_num {
        // run guessing game
        1 => guessing::main(),
        // run rock paper scissors
        2 => rps::main(),
        // run rock paper scissors
        3 => web_server::main(),
        // quit
        4 => println!("Thanks for playing!"),
        _ => println!("Bye!"),
    };

    return;
}
