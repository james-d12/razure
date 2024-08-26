use std::fmt::{Display, Formatter};
use crate::parser::schema::reference::Reference;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
    None
}

impl Display for ParameterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ParameterType::String => "string",
            ParameterType::Number => "number",
            ParameterType::Integer => "integer",
            ParameterType::Boolean => "boolean",
            ParameterType::Array => "array",
            ParameterType::File => "file",
            ParameterType::None => "none"
        }; 
        write!(f, "{0}", str)
    }
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    name: Option<String>,
    #[serde(rename = "in")]
    location: Option<String>,
    required: Option<bool>,
    schema: Option<Reference>,
    #[serde(rename = "type")]
    parameter_type: Option<ParameterType>,
}

impl Parameter {
    pub fn print(&self) {
        println!(
            " Name: {0}\n Description: {1}\n Required: {2}\n Type: {3}",
            self.name.as_deref().unwrap_or(""),
            self.location.as_deref().unwrap_or(""),
            self.required.unwrap_or(false),
            self.parameter_type.as_ref().unwrap_or(&ParameterType::None)
        )
    }
}
