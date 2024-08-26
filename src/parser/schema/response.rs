use crate::parser::schema::parameter_type::ParameterType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    description: Option<String>,
    schema: Option<ParameterType>,
}
