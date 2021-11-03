use std::io;
use std::{fs, env};
use std::process::{Command, Stdio};

pub fn clone(url: &str) {
    println!("Initializing git...");
    init(url);

    println!("\nDecrypting...");
    decrypt();

    println!("Uncompressing...");
    //uncompress();
}

fn init(url: &str) {
    let mut branch = String::new();
    print!("\nPlease enter a branch name: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    io::stdin().read_line(&mut branch).unwrap();
    branch = branch.trim().to_string();

    Command::new("git")
        .args(["init", "."]).output().unwrap();

    Command::new("git")
        .args(["remote", "add", "origin", url]).output().unwrap();
    
    println!("Clonning...");
    Command::new("git")
        .args(["pull", "origin", &branch])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output().unwrap();
}

fn decrypt() {
    let password = env::var("GITENC_PASSWORD").unwrap();

    Command::new("openssl")
        .args(["enc", "-d", "-aes-256-cbc", "-pbkdf2", "-a", "-in", "./data.enc", "-out", "./enc.tar.gz", "-k", &password])
        .output().unwrap();
}