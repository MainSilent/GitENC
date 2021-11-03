use std::fs;
use tar::Builder;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::process::Command;

pub fn commit() {
    println!("Compressing...");
    compress().unwrap();

    println!("\nEncrypting...");
    encrypt().unwrap();

    println!("\nDone.");
}

fn compress() -> Result<(), std::io::Error> {
    let output = "enc.tar.gz";
    let ignore = [".env", ".git", ".gitignore", "node_modules", "__pycache__", output];
    
    // Remove last compression
    fs::remove_file(output).unwrap_or(());

    // Create archive
    let tar_gz = File::create(output)?;
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
    tar.finish()?;
    Ok(())
}

fn encrypt() -> Result<(), std::io::Error> {
    let password = "dummy";
    
    Command::new("openssl")
        .args(["aes-256-cbc", "-a", "-salt", "-in", password, "-out", "sdf.enc"])
        .output().unwrap();
    Ok(())
}