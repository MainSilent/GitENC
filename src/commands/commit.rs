use std::fs;
use tar::Builder;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

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

    // List all files
    let files = fs::read_dir("./").unwrap();
    for file in files {
        let path = file.unwrap().path();
        let name = path.display().to_string();

        // Check if the file is ignored
        if !ignore.iter().any(|f| name.ends_with(f)) {
            println!("Adding {}", name);
        }
    }

    // Remove last compression
    fs::remove_file(output).unwrap_or(());

    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all("enc", ".").unwrap();
    Ok(())
}

fn encrypt() -> Result<(), std::io::Error> {
    Ok(())
}