use crate::definitions::{create_definitions_file, generate_definitions};
use crate::parameters::{create_parameters_file, generate_parameters};
use filesystem::filesystem::SpecificationFile;
use parser::parser::parse_specification_file;
use std::collections::HashMap;

pub fn generate(specifications: &HashMap<String, SpecificationFile>) {
    let mut all_parameters: HashMap<String, String> = HashMap::new();
    let mut all_definitions: HashMap<String, String> = HashMap::new();

    for (_, specification_file) in specifications.iter() {
        let result = parse_specification_file(specification_file);

        match result {
            Some(swagger) => {
                let parameters = generate_parameters(&swagger);
                let definitions = generate_definitions(&swagger);

                if let Some(parameters) = parameters {
                    all_parameters.extend(parameters)
                }

                if let Some(definitions) = definitions {
                    all_definitions.extend(definitions);
                }
            }
            None => {}
        }
    }

    let parameters_result = create_parameters_file(&all_parameters);
    let definitions_result = create_definitions_file(&all_definitions);

    match parameters_result {
        Ok(()) => println!("Created Parameters.rs successfully!"),
        Err(error) => eprintln!("Could not create Parameters.rs file: {error}"),
    }

    match definitions_result {
        Ok(()) => println!("Created Definitions.rs successfully!"),
        Err(error) => eprintln!("Could not create Definitions.rs file: {error}"),
    }
}
