use std::fs;
use std::error::Error;

// Config struct and implementation 
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient arguments. 2 required");
        }
        else {
            let query = args[1].clone();
            let filename = args[2].clone();
            return Ok(Config {query, filename})
        }
    }
}

// functions 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    return Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results
}

// unit tests 
#[cfg(test)] 
mod test {
    use super::*; 

    #[test] 
    fn one_result() {
        let query = "duct"; 
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
