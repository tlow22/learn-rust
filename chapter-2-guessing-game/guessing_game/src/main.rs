use std::io; 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // generate random number 
    println!("Generating secret number between 1-100");
    let secret_number = rand::thread_rng().gen_range(1..101);

    // user can guess the number until correct 
    println!("You will have unlimited tries. Type 'quit' to exit game");

    loop {
        println!("Guess the number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_)  => continue,
        };

        // compare guess and generated number 
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    
        println!("You guessed: {}", guess);    
    }
    
}
