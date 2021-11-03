use std::io;
use std::process::{Command, Stdio};

pub fn clone(url: &str) {
    println!("Initializing git...");
    init(url);
}

fn init(url: &str) {
    let mut branch = String::new();
    print!("\nPlease enter a branch name: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    io::stdin().read_line(&mut branch).unwrap();

    Command::new("git")
        .args(["init", "."]).output().unwrap();

    Command::new("git")
        .args(["remote", "add", "origin", url]).output().unwrap();
    
    println!("Clonning...");
    Command::new("git")
        .args(["pull", "origin", &branch])
        .stdout(Stdio::inherit())
        .output().unwrap();
}