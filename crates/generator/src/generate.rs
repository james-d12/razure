use crate::definitions::{create_definitions_file, generate_definitions};
use crate::parameters::{create_parameters_file, generate_parameters};
use crate::string_formatter::format_as_file_name;
use filesystem::filesystem::SpecificationFile;
use parser::parser::parse_specification_file;
use std::collections::HashMap;

pub fn generate(specifications: &HashMap<String, SpecificationFile>) {
    for (name, specification_file) in specifications.iter() {
        let result = parse_specification_file(specification_file);

        match result {
            Some(swagger) => {
                let parameters = generate_parameters(&swagger);
                let definitions = generate_definitions(&swagger);

                if let Some(parameters) = parameters {
                    let file_name = format!("{0}_parameters", format_as_file_name(name));
                    let result = create_parameters_file(file_name.as_str(), &parameters);
                }

                if let Some(definitions) = definitions {
                    let file_name = format!("{0}_definitions", format_as_file_name(name));
                    let result = create_definitions_file(file_name.as_str(), &definitions);
                }
            }
            None => {}
        }
    }
}
