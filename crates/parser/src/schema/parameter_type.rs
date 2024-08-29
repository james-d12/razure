use serde::Deserialize;
use crate::schema::parameter::Parameter;
use crate::schema::reference::Reference;
// todo! Order of them matters: https://serde.rs/enum-representations.html#untagged

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ParameterType {
    Reference(Reference), // For `$ref` parameters
    Parameter(Parameter), // For inline parameters with more details
}
