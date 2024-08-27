use crate::parser::schema::info::Info;
use crate::parser::schema::parameter::Parameter;
use crate::parser::schema::parameter_type::ParameterType;
use serde::Deserialize;
use std::collections::HashMap;
use crate::parser::schema::path_item::PathItem;

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
}

impl Swagger {
    pub fn print_parameters(&self) {
        match &self.parameters {
            Some(parameters) => {
                for (key, parameter) in parameters {
                    println!("{0}", key);
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

                for parameter in &operation.parameters {
                    match parameter {
                        ParameterType::Reference(reference) => {
                            println!("  Ref Parameter");
                            println!("    Path: {0}", reference.path);
                        }
                        ParameterType::Parameter(inline) => {
                            println!("  Inline Parameter: ");
                        }
                    }
                }
            }
        }
    }

    pub fn walk(&self) {
        self.print_paths();
        self.print_parameters()
    }
}