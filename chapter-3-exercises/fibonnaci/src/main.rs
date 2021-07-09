use std::io;

// generates the n-th fibonnaci number 
fn main() {
    // get user input 
    println!("Enter n-th Fibonnaci number desired"); 

    let mut idx = String::new();
    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read");

    let idx:u32 = match idx.trim().parse() {
        Ok(num) => num, 
        Err(_)  => 0
    };

    // generate specified Fibonnaci number 
    match idx {
        0 => println!("Invalid input detected. No calculation will be done"),
        _ => println!("The {}-th Fibonnaci number is {}", idx, fibonnaci(idx)),
    }
}

fn fibonnaci(n: u32) -> u32 {
    if n  <= 1 {
        return n; 
    } 

    return fibonnaci(n-1) + fibonnaci(n-2);
}
