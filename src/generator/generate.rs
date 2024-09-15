use crate::filesystem::SpecificationFile;
use crate::generator::{
    format_as_file_name, generate_cargo_project, generate_definitions, generate_parameters,
};
use crate::parser::parse_specification_file;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

fn create_file(
    file_path: &String,
    parameter_structs: &HashMap<String, String>,
) -> Result<(), Error> {
    let mut parameters_file = File::create(file_path)?;

    for (_, parameter_struct) in parameter_structs {
        let mut str = parameter_struct.clone();
        str.push('\n');
        parameters_file.write_all(str.as_ref())?;
    }

    Ok(())
}

fn create_lib_file(output_path: &str, file_names: &BTreeMap<String, String>) -> Result<(), Error> {
    let lib_file_path = format!("{output_path}/src/lib.rs");
    let mut lib_file = File::create_new(lib_file_path)?;

    for (_, file_mod_statement) in file_names {
        lib_file.write_all(file_mod_statement.as_ref())?
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

pub fn generate(output_path: &str, specifications: &HashMap<String, SpecificationFile>) {
    let output_src_path: String = format!("{output_path}/src");

    match create_project(output_path) {
        Ok(_) => {
            let mut file_mod_statements: BTreeMap<String, String> = BTreeMap::new();
            for (_, specification_file) in specifications.iter() {
                let swagger = parse_specification_file(specification_file);

                if let Some(swagger) = swagger {
                    let file_name = format_as_file_name(specification_file.file_name.as_str());
                    let domain_file_name =
                        format_as_file_name(specification_file.domain_name.as_str());
                    let mut data: HashMap<String, String> = HashMap::new();

                    if let Some(parameters) = generate_parameters(&swagger) {
                        data.extend(parameters);
                    }

                    if let Some(definitions) = generate_definitions(&swagger) {
                        data.extend(definitions);
                    }

                    if data.len() <= 0 {
                        println!("Skipping file: {0} as it has no content.", file_name);
                        continue;
                    }

                    let full_name = format!("{}_{}", domain_file_name, file_name);
                    let file_path = format!("{}/{}.rs", output_src_path, full_name);

                    match create_file(&file_path, &data) {
                        Ok(()) => {
                            let file_mod_statement = format!("pub mod {full_name};\n");
                            file_mod_statements.insert(file_name, file_mod_statement);
                        }
                        Err(error) => eprintln!(
                            "Could not create file: {0} due to error: {error}",
                            &file_path
                        ),
                    }
                }
            }

            match create_lib_file(output_path, &file_mod_statements) {
                Ok(_) => println!("Successfully created lib.rs file!"),
                Err(error) => eprintln!("Could not create lib.rs file due to error: {error}"),
            }
        }
        Err(error) => eprintln!("error: {error}"),
    }
}
