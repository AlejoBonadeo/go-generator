use std::{io, path::PathBuf};

use crate::{
    cli::get_user_input,
    file_handler::{
        create_directory_if_not_exists, create_file_if_not_exists, create_file_with_content,
    },
    go_parser::{
        create_new_container_file, update_container_file, update_functions_definitions,
        update_web_handler_container, update_wire_file,
    },
    template::{
        generate_codes_content, generate_errors_content, generate_rest_content,
        generate_test_content, generate_usecase_content,
    },
    utils::{camel_to_snake_case, capitalize_first_letter},
};

pub fn create_endpoints(project_name: &str) -> io::Result<()> {
    let module_name = get_user_input("Please enter the module name:")?;
    let endpoint_name = get_user_input("Please enter the endpoint name:")?;

    let capitalized_endpoint = capitalize_first_letter(&endpoint_name);
    let module_name_snake = camel_to_snake_case(&module_name);
    let endpoint_name_snake = camel_to_snake_case(&endpoint_name);

    let usecase_path = PathBuf::from("./src/core/usecases").join(&module_name_snake);
    create_directory_if_not_exists(&usecase_path)?;

    let usecase_content = generate_usecase_content(&module_name, &capitalized_endpoint);
    let test_content = generate_test_content(&module_name, &capitalized_endpoint);

    let main_file = usecase_path.join(format!("{}.go", endpoint_name_snake));
    let test_file = usecase_path.join(format!("{}_test.go", endpoint_name_snake));

    create_file_with_content(&main_file, &usecase_content)?;
    create_file_with_content(&test_file, &test_content)?;

    let errors_file = usecase_path.join("errors.go");
    let errors_content = generate_errors_content(&module_name);
    create_file_if_not_exists(&errors_file, &errors_content)?;

    let rest_base_path = PathBuf::from("./src/entrypoints/rest").join(&module_name_snake);
    create_directory_if_not_exists(&rest_base_path)?;

    let rest_file = rest_base_path.join(format!("{}.go", endpoint_name_snake));
    create_file_with_content(
        &rest_file,
        &generate_rest_content(&module_name, &capitalized_endpoint, &project_name),
    )?;

    let codes_file = rest_base_path.join("codes.go");
    let codes_content = generate_codes_content(&module_name);
    create_file_if_not_exists(&codes_file, &codes_content)?;

    let container_file_path =
        PathBuf::from("./src/entrypoints").join(format!("{}.go", module_name_snake));

    if container_file_path.exists() {
        update_container_file(
            &container_file_path,
            &module_name,
            &capitalized_endpoint,
            &project_name,
        )?;
    } else {
        create_new_container_file(
            &container_file_path,
            &module_name,
            &capitalized_endpoint,
            &project_name,
        )?;
    }

    let wire_file_path = PathBuf::from("./src/infrastructure/dependencies/wire.go");
    if wire_file_path.exists() {
        update_wire_file(
            &wire_file_path,
            &module_name,
            &capitalized_endpoint,
            &project_name,
        )?;
    } else {
        println!("Warning: wire.go file not found at expected location");
    }

    let containers_file_path = PathBuf::from("./src/infrastructure/dependencies/containers.go");
    if containers_file_path.exists() {
        update_web_handler_container(&containers_file_path, &module_name)?;
    } else {
        println!("Warning: containers.go file not found at expected location");
    }

    let functions_file_path = PathBuf::from("./functions.go");
    if functions_file_path.exists() {
        update_functions_definitions(&functions_file_path, &module_name, &endpoint_name)?;
    } else {
        println!("Warning: functions.go file not found at expected location");
    }
    return Ok(());
}
