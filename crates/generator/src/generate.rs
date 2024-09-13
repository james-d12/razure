use crate::definitions::generate_definitions;
use crate::parameters::generate_parameters;
use crate::string_formatter::format_as_file_name;
use crate::terminal::generate_cargo_project;
use filesystem::filesystem::SpecificationFile;
use parser::parser::parse_specification_file;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use std::fs;

fn create_file(file_path: &String, parameter_structs: &HashMap<String, String>) -> Result<(), Error> {
    let mut parameters_file =
        File::create(file_path)?;

    for (_, parameter_struct) in parameter_structs {
        let mut str = parameter_struct.clone();
        str.push('\n');
        parameters_file.write_all(str.as_ref())?;
    }

    Ok(())
}

fn create_project(output_path: &str) -> Result<bool, Error> {
    let path = Path::new(output_path);

    if !path.exists() {
        println!("Path: {output_path} does not exist, creating now.");
        fs::create_dir(output_path)?
    }
    
    Ok(generate_cargo_project(output_path))
}

pub fn generate(specifications: &HashMap<String, SpecificationFile>) {
    let output_path: &str = "C:/Users/User/Downloads/razure-output";
    let output_src_path: String = format!("{output_path}/src");
    
    match create_project(output_path) {
        Ok(_) => {
            for (name, specification_file) in specifications.iter() {
                let swagger = parse_specification_file(specification_file);

                if let Some(swagger) = swagger {
                    let file_name = format!("{0}", format_as_file_name(name));
                    let mut data: HashMap<String, String> = HashMap::new();

                    if let Some(parameters) = generate_parameters(&swagger) {
                        data.extend(parameters);
                    }

                    if let Some(definitions) = generate_definitions(&swagger) {
                        data.extend(definitions);
                    }

                    let file_path = format!("{output_src_path}/{file_name}.rs");

                    match create_file(&file_path, &data) {
                        Ok(()) => {}
                        Err(error) => eprintln!("Could not create file: {0} due to error: {error}", &file_path),
                    }
                }
            }
        }
        Err(error) => eprintln!("error: {error}")
    }

}
