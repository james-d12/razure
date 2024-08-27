use crate::parser::schema::reference::Reference;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
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
        };
        write!(f, "{0}", str)
    }
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Reference>,
    #[serde(rename = "type")]
    pub parameter_type: Option<ParameterType>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u16>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u16>,
}

impl Parameter {
    pub fn print(&self) {
        println!(
            "  Name: {0}\n  Description: {1}\n  Required: {2}\n  Type: {3}",
            self.name.as_deref().unwrap_or(""),
            self.location.as_deref().unwrap_or(""),
            self.required.unwrap_or(false),
            self.parameter_type
                .as_ref()
                .unwrap_or(&ParameterType::String)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use serde_json::from_str;

    #[rstest]
    #[case("string", ParameterType::String)]
    #[case("number", ParameterType::Number)]
    #[case("integer", ParameterType::Integer)]
    #[case("boolean", ParameterType::Boolean)]
    #[case("array", ParameterType::Array)]
    #[case("file", ParameterType::File)]
    fn deserialize_parameter_parameter_type_correct(
        #[case] parameter_type_str: String,
        #[case] expected_type: ParameterType,
    ) {
        let json_string = format!(
            r#"{{
            "name": "Test Name",
            "in": "query",
            "required": true,
            "type": "{0}",
            "description": "Test Description"
        }}"#,
            parameter_type_str
        );

        let parameter: Parameter = from_str(json_string.as_str()).unwrap();

        assert_eq!(parameter.name.unwrap(), "Test Name");
        assert_eq!(parameter.description.unwrap(), "Test Description");
        assert_eq!(parameter.location.unwrap(), "query");
        assert_eq!(parameter.required.unwrap(), true);
        assert_eq!(parameter.parameter_type.unwrap(), expected_type)
    }
}
