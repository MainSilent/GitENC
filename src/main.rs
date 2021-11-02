use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1][..];
    
    match command {
        _ => {
            eprintln!("Please pass an argument")
        }
    }
}
