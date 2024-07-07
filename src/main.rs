use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing the Argument: {}", err);
        process::exit(1);
    });

    println!("Searching for filename {}", config.filename);
    println!("In file query {} \n", config.query);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contains =
        fs::read_to_string(config.filename)?;
        //.expect("Something went wrong to read this file");

    println!("{}", contains);

    Ok(())
}


impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}