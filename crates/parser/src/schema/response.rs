use crate::schema::parameter_type::ParameterType;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub description: Option<String>,
    pub schema: Option<ParameterType>,
}
