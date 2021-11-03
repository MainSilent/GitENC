use std::env;
use dotenv::dotenv;
mod commands;

fn main() {
    dotenv().expect("Failed to read .env file");

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Please pass an argument");
        std::process::exit(101)
    }

    let command = &args[1][..];
    match command {
        "commit" => commands::commit(),
        "clone" => commands::clone(&args[2]),
        _ => {
            eprintln!("Please pass a valid argument")
        }
    }
}
