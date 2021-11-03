use std::{fs, env};
use tar::Builder;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::process::Command;

pub fn commit() {
    println!("Compressing...");
    compress();

    println!("\nEncrypting...");
    encrypt();

    // Track files
    Command::new("git")
        .args(["add", "."]).output().unwrap();

    Command::new("git")
        .args(["commit", "-m", "\"Update encrypted data\""]).output().unwrap();

    println!("\nCommitted successfully.");
}

fn compress() {
    let output = "enc.tar.gz";
    let ignore = [".env", ".git", ".gitignore", "node_modules", "__pycache__", output];
    
    // Remove last compression
    fs::remove_file(output).unwrap_or(());

    // Create archive
    let tar_gz = File::create(output).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    
    // List all files
    let files = fs::read_dir("./").unwrap().map(|f| f.unwrap().path().display().to_string());
    for file in files {

        // Check if the file is ignored
        if !ignore.iter().any(|n| file.ends_with(n)) {
            println!("Adding {}", file);
            tar.append_path(file).unwrap();
        }
    }
    tar.finish().unwrap();
}

fn encrypt() {
    let password = env::var("GITENC_PASSWORD").unwrap();

    Command::new("openssl")
        .args(["enc", "-aes-256-cbc", "-a", "-salt", "-in", "./enc.tar.gz", "-out", "./data.enc", "-k", &password, "-pbkdf2"])
        .output().unwrap();
    
    fs::remove_file("data.enc").ok();
    fs::remove_file("enc.tar.gz").ok();
}