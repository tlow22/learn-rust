use std::io;

fn main() {
    // prompt user for temperature 
    println!("Please specify temperature value"); 
    let mut value   = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read this.");

    let value: f64 = value.trim().parse().expect("Please input a number");

    // prompt user for units 
    println!("Please specify units (F/C):"); 
    let mut units = String::new();

    io::stdin()
        .read_line(&mut units)
        .expect("Failed to read this.");
    
    // perform temperature conversion
    let units = units.trim().to_lowercase();

    match units.as_ref() {
        "c"    => {
            let value = fahrenheit_to_celsius(value);
            println!("Equivalent temperature = {} F.", value);
        },
        "f"    => {
            let value = celsius_to_fahrenheit(value);
            println!("Equivalent temperature = {} C.", value);
        },
        _ => println!("Invalid units"),
    }
}


fn fahrenheit_to_celsius(temp: f64) -> f64 {
    return (32.0*temp - 32.0) * 5.0/9.0;
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    return (temp * 9.0/5.0) + 32.0; 
}