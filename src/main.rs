mod cli;
mod endpoints;
mod file_handler;
mod go_parser;
mod template;
mod utils;

use endpoints::create_endpoints;
use std::io;
use std::path::PathBuf;
use utils::extract_project_name;

fn main() -> io::Result<()> {
    println!("Welcome to the Endpoint Generator!");

    let project_name = extract_project_name(&PathBuf::from("./go.mod")).expect("go.mod not found");

    create_endpoints(&project_name).unwrap();

    println!("Successfully created endpoint files!");

    Ok(())
}
