use crate::parser::schema::definition::Definition;
use crate::parser::schema::info::Info;
use crate::parser::schema::parameter::Parameter;
use crate::parser::schema::parameter_type::ParameterType;
use crate::parser::schema::path_item::PathItem;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    swagger: String,
    info: Option<Info>,
    schemes: Option<Vec<String>>,
    host: Option<String>,
    consumes: Option<Vec<String>>,
    produces: Option<Vec<String>>,
    paths: Option<HashMap<String, PathItem>>,
    parameters: Option<HashMap<String, Parameter>>,
    definitions: Option<HashMap<String, Definition>>,
}

impl Swagger {
    fn print_overview(&self) {
        if let Some(info) = &self.info {
            println!("{0}", info.title);
            if let Some(description) = &info.description {
                println!("{0}", description);
            }
        }
    }

    fn print_parameters(&self) {
        match &self.parameters {
            Some(parameters) => {
                for (key, parameter) in parameters {
                    println!(" Parameter: {0}", key);
                    parameter.print();
                }
            }
            None => {}
        }
    }

    fn print_paths(&self) {
        for (endpoint, path) in self.paths.as_ref().unwrap() {
            println!(" Path: {0}", endpoint);

            for (name, operation) in path.get_operations().iter() {
                println!("  Method: {0}", name);
                println!("  Id: {0}", operation.id);

                if let Some(description) = &operation.description {
                    println!("  Description: {0}", description);
                }

                println!("  Parameters:");
                for parameter in &operation.parameters {
                    match parameter {
                        ParameterType::Reference(reference) => {}
                        ParameterType::Parameter(inline) => {
                            if let Some(name) = &inline.name {
                                println!("   {0}:", name);
                            }
                        }
                    }
                }
            }
        }
    }

    fn print_definitions(&self) {
        let definitions_iter = self.definitions.as_ref();

        if let Some(definitions) = definitions_iter {
            for (name, schema_definition) in definitions.iter() {
                println!(" Definition: {0}", name);

                if let Some(description) = &schema_definition.description {
                    println!("  Description: {0}", description);
                }
            }
        }
    }

    pub fn walk(&self) {
        self.print_overview();
        //self.print_paths();
        //self.print_parameters();
        self.print_definitions();
    }
}
