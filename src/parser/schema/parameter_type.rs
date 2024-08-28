use crate::parser::schema::parameter::Parameter;
use crate::parser::schema::reference::Reference;
use serde::Deserialize;

// todo! Order of them matters: https://serde.rs/enum-representations.html#untagged

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ParameterType {
    Reference(Reference), // For `$ref` parameters
    Parameter(Parameter), // For inline parameters with more details
}
