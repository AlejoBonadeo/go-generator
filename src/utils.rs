use std::{
    fs,
    io::{self, Read},
    path::Path,
};

pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn camel_to_snake_case(input: &str) -> String {
    let mut snake_case = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            snake_case.push('_');
        }
        snake_case.push(c.to_ascii_lowercase());
    }
    snake_case
}

pub fn extract_project_name(go_mod_path: &Path) -> io::Result<String> {
    let mut content = String::new();
    fs::File::open(go_mod_path)?.read_to_string(&mut content)?;

    if let Some(line) = content.lines().find(|line| line.starts_with("module ")) {
        let package_name = line.trim_start_matches("module ").trim();
        return Ok(package_name.to_string());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Module directive not found in go.mod",
    ))
}
