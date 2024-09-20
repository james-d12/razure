use crate::filesystem::SpecificationFile;
use crate::generator::rust::{
    create_file, create_lib_file, create_project, create_struct, create_struct_simple_type,
    format_as_file_name,
};
use crate::generator::{ConversionType, Generator};
use crate::parser::parse_specification_file;
use crate::parser::schema::{Definition, DefinitionType, Parameter, PropertyType};
use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
pub struct RustGenerator {
    pub structs: HashMap<String, String>,
    pub definitions: HashMap<String, String>,
}

impl RustGenerator {
    fn generate_definitions(&mut self, definitions: &HashMap<String, Definition>) {
        for (name, definition) in definitions {
            match &definition.schema {
                DefinitionType::Object { properties } => {
                    let struct_str = create_struct(name, properties);
                    self.structs.insert(name.to_string(), struct_str);
                }
                _ => {}
            }
        }
    }

    fn generate_parameters(&mut self, parameters: &HashMap<String, Parameter>) {
        for (name, parameter) in parameters {
            if let Some(property_type) = &parameter.property_type {
                match property_type {
                    PropertyType::String | PropertyType::Integer | PropertyType::Number => {
                        let property_type_string = property_type.get_type_as_string();

                        if let Some(property_type_string) = property_type_string {
                            let struct_string =
                                create_struct_simple_type(name, property_type_string.to_string());
                            self.structs.insert(name.to_string(), struct_string);
                        }
                    }
                    PropertyType::Object => {
                        println!("Object type for {0}", name)
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Generator for RustGenerator {
    fn generate(&mut self, output_path: &str, specifications: &HashMap<String, SpecificationFile>) {
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

                        if let Some(parameters) = &swagger.parameters {
                            self.generate_parameters(parameters);
                        }

                        if let Some(definitions) = &swagger.definitions {
                            self.generate_definitions(definitions);
                        }

                        if self.is_empty() {
                            println!("Skipping file: {0} as it has no content.", file_name);
                            continue;
                        }

                        let full_name = format!("{}_{}", domain_file_name, file_name);
                        let file_path = format!("{}/{}.rs", output_src_path, full_name);

                        match create_file(&file_path, self) {
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

    fn is_empty(&self) -> bool {
        self.structs.is_empty() && self.definitions.is_empty()
    }
}
