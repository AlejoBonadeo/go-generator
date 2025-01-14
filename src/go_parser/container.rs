use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use regex::Regex;

use crate::{template::generate_container_content, utils::camel_to_snake_case};

pub fn update_container_file(
    path: &Path,
    module: &str,
    endpoint: &str,
    project_name: &str,
) -> io::Result<()> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;
    let import_module = camel_to_snake_case(module);

    if !content.contains(&format!(
        "{import_module}h \"{project_name}/src/entrypoints/rest/{import_module}\""
    )) {
        content = add_import(&content, &import_module, project_name);
    }

    let struct_pattern = format!(r"type {module}Container struct {{");
    let new_field = format!("    {} {}h.{}", endpoint, module, endpoint);

    if let Some(struct_pos) = content.find(&struct_pattern) {
        let closing_brace = content[struct_pos..].find("}").unwrap() + struct_pos;
        content.insert_str(closing_brace, &format!("\n{}\n", new_field));
    }

    let constructor_pattern = format!(r"func New{module}Container\((.*?)\)");
    let constructor_regex = Regex::new(&constructor_pattern).unwrap();

    if let Some(caps) = constructor_regex.find(&content) {
        let existing_params = &content[caps.start()..caps.end()];
        let new_param = format!("{} {}h.{}", endpoint, module, endpoint);

        let updated_params = if existing_params.contains(")") {
            existing_params.replace(")", &format!(", {new_param})"))
        } else {
            format!("{}, {new_param})", existing_params)
        };

        content = content.replace(existing_params, &updated_params);

        let return_pattern = format!(r"return {module}Container{{");
        if let Some(return_pos) = content.find(&return_pattern) {
            let closing_brace = content[return_pos..].find("}").unwrap() + return_pos;
            content.insert_str(
                closing_brace,
                &format!("\n        {}: {},\n", endpoint, endpoint),
            );
        }
    }

    fs::write(path, content)?;
    println!("Updated container file: {}", path.display());
    Ok(())
}

fn add_import(content: &str, module: &str, project_name: &str) -> String {
    let import_end = content.find(")").unwrap();
    let new_import = format!("    {module}h \"{project_name}/src/entrypoints/rest/{module}\"\n");
    let mut new_content = content.to_string();
    new_content.insert_str(import_end, &new_import);
    new_content
}

pub fn create_new_container_file(
    path: &Path,
    module: &str,
    endpoint: &str,
    project_name: &str,
) -> io::Result<()> {
    let content = generate_container_content(module, endpoint, project_name);
    fs::write(path, content)?;
    println!("Created new container file: {}", path.display());
    Ok(())
}
