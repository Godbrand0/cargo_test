use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number - 2 Player Turn-Based Game!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut current_player = 1;
    
    loop {
        println!("Player {}, enter your guess:", current_player);
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Player {} wins! The number was {}.", current_player, secret_number);
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
        
        // Switch to the other player
        current_player = if current_player == 1 { 2 } else { 1 };
    }
}
