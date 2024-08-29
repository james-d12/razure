use crate::schema::definition::Definition;
use crate::schema::info::Info;
use crate::schema::parameter::Parameter;
use crate::schema::parameter_type::ParameterType;
use crate::schema::path_item::PathItem;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    pub swagger: String,
    pub info: Option<Info>,
    pub schemes: Option<Vec<String>>,
    pub host: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub paths: Option<HashMap<String, PathItem>>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub definitions: Option<HashMap<String, Definition>>,
}

impl Swagger {
    fn print_overview(&self) {
        println!("Swagger: {0}", self.swagger);
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

                if let Some(parameters) = &operation.parameters {
                    for parameter in parameters {
                        match parameter {
                            ParameterType::Reference(_) => {}
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
        self.print_paths();
        self.print_parameters();
        self.print_definitions();
    }
}
