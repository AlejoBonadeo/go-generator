use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn create_directory_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        println!("Created directory: {}", path.display());
    }
    Ok(())
}

pub fn create_file_with_content(path: &Path, content: &str) -> io::Result<()> {
    if path.exists() {
        println!("Warning: File {} already exists!", path.display());
        println!("Do you want to overwrite it? (y/n):");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Skipping file creation.");
            return Ok(());
        }
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    println!("Created file: {}", path.display());
    Ok(())
}

pub fn create_file_if_not_exists(path: &Path, content: &str) -> io::Result<()> {
    if !path.exists() {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        println!("Created file: {}", path.display());
    }
    Ok(())
}
