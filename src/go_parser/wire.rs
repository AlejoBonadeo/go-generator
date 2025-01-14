use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use crate::utils::{camel_to_snake_case, capitalize_first_letter};

pub fn update_wire_file(
    path: &Path,
    module: &str,
    endpoint: &str,
    project_name: &str,
) -> io::Result<()> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;

    let module_import = camel_to_snake_case(module);
    let module_container = capitalize_first_letter(module);

    if !content.contains(&format!(
        "{}uc \"{}/src/core/usecases/{}\"",
        module_import, project_name, module_import
    )) {
        content = add_wire_import(
            &content,
            &format!(
                "{}uc \"{}/src/core/usecases/{}\"",
                module_import, project_name, module_import
            ),
        );
    }
    if !content.contains(&format!(
        "{}h \"{}/src/entrypoints/rest/{}\"",
        module_import, project_name, module_import
    )) {
        content = add_wire_import(
            &content,
            &format!(
                "{}h \"{}/src/entrypoints/rest/{}\"",
                module_import, project_name, module_import
            ),
        );
    }

    let builder_func_name = format!("build{}Container", module_container);

    if content.contains(&builder_func_name) {
        content = add_to_existing_builder(&content, &module_import, endpoint, &module_container);
    } else {
        content = add_builder_to_wire_build(&content, &module_container);
        content = add_new_builder_function(&content, &module_import, &module_container, endpoint);
    }

    fs::write(path, content)?;
    println!("Updated wire.go file");

    Ok(())
}

fn add_wire_import(content: &str, import_line: &str) -> String {
    let import_end = content
        .find(")")
        .map(|i| content[..i].rfind('\n').unwrap_or(i))
        .unwrap_or(0);

    let mut new_content = content.to_string();
    new_content.insert_str(import_end, &format!("\n\t{}", import_line));
    new_content
}

fn add_builder_to_wire_build(content: &str, module_container: &str) -> String {
    let pattern = "wire.Struct(new(WebHandlerContainer), \"*\"),";
    if let Some(pos) = content.find(pattern) {
        let builder_ref = format!("\n\t\tbuild{}Container,\n\t\t", module_container);
        let mut new_content = content.to_string();
        new_content.insert_str(pos, &builder_ref);
        new_content
    } else {
        content.to_string()
    }
}

fn add_to_existing_builder(
    content: &str,
    module_import: &str,
    endpoint: &str,
    module_container: &str,
) -> String {
    let builder_pattern = format!("func build{}Container", module_container);
    if let Some(pos) = content.find(&builder_pattern) {
        if let Some(build_pos) = content[pos..].find("wire.Build(") {
            let repository_pos =
                content[pos + build_pos..].find("repositorySet,").unwrap() + pos + build_pos;
            let new_lines = format!(
                "\t\t{}h.New{},\n\t\t{}uc.New{},\n\t\t",
                module_import, endpoint, module_import, endpoint
            );
            let mut new_content = content.to_string();
            new_content.insert_str(repository_pos, &new_lines);
            new_content
        } else {
            content.to_string()
        }
    } else {
        content.to_string()
    }
}

fn add_new_builder_function(
    content: &str,
    module_import: &str,
    module_container: &str,
    endpoint: &str,
) -> String {
    format!(
        "{}\n\nfunc build{}Container(conf *config.Config, logger *zap.Logger, db *gorm.DB) entrypoints.{}Container {{\n\twire.Build(\n\t\tentrypoints.New{}Container,\n\t\t{}h.New{},\n\t\t{}uc.New{},\n\t\trepositorySet,\n\t)\n\treturn entrypoints.{}Container{{}}\n}}",
        content,
        module_container, module_container, module_container,
        module_import, endpoint, module_import, endpoint,
        module_container
    )
}
