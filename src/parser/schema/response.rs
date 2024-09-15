use crate::parser::schema::ParameterType;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub description: Option<String>,
    pub schema: Option<ParameterType>,
}
