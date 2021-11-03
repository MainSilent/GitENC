use std::fs;
use tar::Builder;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

pub fn commit() {
    compress().unwrap();
}

fn compress() -> Result<(), std::io::Error> {
    let filename = "enc.tar.gz";

    // remove last compression
    fs::remove_file(filename).unwrap_or(());

    let tar_gz = File::create(filename)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all("bardir", ".").unwrap();
    Ok(())
}