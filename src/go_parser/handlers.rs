use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use regex::Regex;

use crate::utils::{camel_to_snake_case, capitalize_first_letter};

pub fn update_web_handler_container(path: &Path, module: &str) -> io::Result<()> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;

    let module = capitalize_first_letter(module);

    let struct_pattern = "type WebHandlerContainer struct {";
    if let Some(struct_start) = content.find(struct_pattern) {
        let struct_end = content[struct_start..]
            .find('}')
            .map(|i| struct_start + i)
            .unwrap_or(content.len());

        let new_field = format!("\t{} entrypoints.{}Container\n", module, module);

        if !content[struct_start..struct_end].contains(&new_field) {
            content.insert_str(struct_end, &new_field);
        }
    } else {
        eprintln!(
            "Error: `WebHandlerContainer` struct not found in {}",
            path.display()
        );
    }

    fs::write(path, content)?;
    println!("Updated WebHandlerContainer in: {}", path.display());
    Ok(())
}

pub fn update_functions_definitions(path: &Path, module: &str, endpoint: &str) -> io::Result<()> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;

    let module_upper = capitalize_first_letter(module);
    let module_snake = camel_to_snake_case(module);
    let endpoint_upper = capitalize_first_letter(endpoint);
    let endpoint_snake = camel_to_snake_case(endpoint);

    let pattern = r"(?P<method>GET|POST|PUT|DELETE|PATCH)\(fn\.Handlers\.\w+\.\w+\.Handle\(\)\)\)";
    let regex = Regex::new(pattern).unwrap();

    let mut new_content = content.clone();

    if let Some(matched) = regex.find(&content) {
        let match_start = matched.start();
        let match_end = matched.end();
        let match_str = &content[match_start..match_end];

        if let Some(_) = regex.captures(match_str) {
            // let http_method = caps.name("method").unwrap().as_str();

            let new_handler = format!(
                "\nfunctions.HTTP(\"/api/v1/{}/{}\", fn.\n        Public().\n        Path(\"/api/v1/{}/{}\").\n        POST(fn.{}.{}.Handle()))",
                module_snake, endpoint_snake, module_snake, endpoint_snake, module_upper, endpoint_upper
            );

            new_content.insert_str(match_end, &new_handler);
        }
    } else {
        eprintln!(
            "No matching HTTP method handler found in {}",
            path.display()
        );
    }

    fs::write(path, new_content)?;
    println!("Updated WebHandlerContainer in: {}", path.display());
    Ok(())
}
