use crate::definitions::generate_definitions;
use crate::parameters::generate_parameters;
use crate::string_formatter::format_as_file_name;
use filesystem::filesystem::SpecificationFile;
use parser::parser::parse_specification_file;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};

fn create_file(name: &String, parameter_structs: &HashMap<String, String>) -> Result<(), Error> {
    let mut parameters_file =
        File::create(format!("C:/Users/User/Downloads/razure-output/{name}.rs"))?;

    for (name, parameter_struct) in parameter_structs {
        let mut str = parameter_struct.clone();
        str.push('\n');
        parameters_file.write_all(str.as_ref())?;
    }

    Ok(())
}

pub fn generate(specifications: &HashMap<String, SpecificationFile>) {
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

            match create_file(&file_name, &data) {
                Ok(()) => {}
                Err(_) => eprintln!("Could not create file: {0}.", &file_name)
            }
        }
    }
}
