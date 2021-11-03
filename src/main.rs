use std::env;
use dotenv::dotenv;
mod commands;

fn main() {
    dotenv().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Please pass an argument");
        std::process::exit(101)
    }

    let command = &args[1][..];
    match command {
        "commit" => commands::commit(),
        _ => {
            eprintln!("Please pass a valid argument")
        }
    }
}
