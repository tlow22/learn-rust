use std::cmp::Ordering;

// prints out the christams carol "Twelve Days of Christmas"
fn main() {
    let nth = ["first", "second", "third", "fourth", "fifth", "sixth", 
               "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];
    let lyrics = ["a partridge in a pear tree",
                  "two turtle doves", 
                  "three French hens",
                  "four calling birds",
                  "five gold rings",
                  "six geese a laying",
                  "seven swans a swimming",
                  "eight maids a milking",
                  "nine ladies dancing",
                  "ten lords a leaping",
                  "eleven pipers piping",
                  "twelve drummers drumming"];
    for i in 0..nth.len() {
        println!("On the {} day of Christmas my true love gave to me", nth[i]);
        
        match i.cmp(&0) {
            Ordering::Equal => println!("{}", lyrics[0]),
            _ => {
                // intermediate phrases 
                let mut counter = 0; 
                for j in (1..i+1).rev(){
                    print!("{},", lyrics[j]);
                    
                    // newline tidiness
                    counter += 1; 
                    if counter % 3 == 0 {
                        counter = 0; 
                        println!("");
                    }
                }
                // last phrase 
                print!("and {}", lyrics[0]);
                println!("");
            } 
        }

        println!("");
    }
}