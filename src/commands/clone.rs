use std::io;
use std::{fs, env};
use std::process::{Command, Stdio};
use tar::Archive;
use std::fs::File;
use flate2::read::GzDecoder;

pub fn clone(url: &str) {
    fs::remove_dir_all("*").ok();
    fs::remove_dir_all(".git").ok();
    fs::remove_file(".gitignore").ok();

    println!("Initializing git...");
    init(url);

    println!("\nDecrypting...");
    decrypt();

    println!("Extracting...");
    extract();
    println!("Extracted successfully.");
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

    let cmd = Command::new("openssl")
        .args(["enc", "-d", "-aes-256-cbc", "-pbkdf2", "-a", "-in", "./data.enc", "-out", "./enc.tar.gz", "-k", &password])
        .output().unwrap();

    if cmd.status.code().unwrap() != 0 {
        eprintln!("Failed to decrypt");
        std::process::exit(102);
    }

    fs::remove_file("data.enc").ok();
}

fn extract() {
    let path = "enc.tar.gz";
    let tar_gz = File::open(path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".").unwrap();

    fs::remove_file(path).ok();
}