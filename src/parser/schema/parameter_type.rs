use crate::parser::schema::parameter::Parameter;
use crate::parser::schema::reference::Reference;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ParameterType {
    Parameter(Parameter), // For inline parameters with more details
    Reference(Reference), // For `$ref` parameters
}
