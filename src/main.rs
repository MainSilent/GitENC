use std::env;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1][..];

    match command {
        "push" => commands::push(),
        _ => {
            eprintln!("Please pass an argument")
        }
    }
}
