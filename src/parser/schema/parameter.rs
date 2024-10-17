use crate::parser::schema::Schema;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
    File,
}

impl Display for PropertyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PropertyType::String => "string",
            PropertyType::Number => "number",
            PropertyType::Integer => "integer",
            PropertyType::Boolean => "boolean",
            PropertyType::Array => "array",
            PropertyType::File => "file",
            PropertyType::Object => "object",
        };
        write!(f, "{0}", str)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Parameter {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
    #[serde(rename = "type")]
    pub property_type: Option<PropertyType>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u16>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u16>,
    pub pattern: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::schema::SchemaType;
    use rstest::rstest;
    use serde_json::from_str;

    #[rstest]
    #[case::string("string", PropertyType::String)]
    #[case::number("number", PropertyType::Number)]
    #[case::integer("integer", PropertyType::Integer)]
    #[case::boolean("boolean", PropertyType::Boolean)]
    #[case::array("array", PropertyType::Array)]
    #[case::file("file", PropertyType::File)]
    fn deserialize_parameter_with_parameter_types(
        #[case] parameter_type_str: String,
        #[case] expected_type: PropertyType,
    ) {
        let json_string = format!(
            r#"{{
            "name": "Test Name",
            "in": "query",
            "required": true,
            "type": "{0}",
            "description": "Test Description",
            "minLength": 5,
            "maxLength": 64
        }}"#,
            parameter_type_str
        );

        let parameter: Parameter = from_str(json_string.as_str()).unwrap();

        assert_eq!(parameter.name.unwrap(), "Test Name");
        assert_eq!(parameter.description.unwrap(), "Test Description");
        assert_eq!(parameter.location.unwrap(), "query");
        assert!(parameter.required.unwrap());
        assert_eq!(parameter.property_type.unwrap(), expected_type);
        assert_eq!(parameter.min_length.unwrap(), 5);
        assert_eq!(parameter.max_length.unwrap(), 64);
    }

    #[test]
    fn deserialize_parameter_with_schema_ref() {
        let schema_reference = "#/definitions/SubscriptionName";
        let json_string = format!(
            r#"{{
            "name": "Test Name",
            "in": "body",
            "required": true,
            "description": "Test Description",
            "schema": {{
                "$ref": "{0}"
            }}
        }}"#,
            schema_reference
        );

        let parameter: Parameter = from_str(json_string.as_str()).unwrap();

        assert_eq!(parameter.name.unwrap(), "Test Name");
        assert_eq!(parameter.location.unwrap(), "body");
        assert!(parameter.required.unwrap());
        assert_eq!(parameter.description.unwrap(), "Test Description");
        assert_eq!(
            parameter.schema.unwrap().reference.unwrap(),
            schema_reference.to_string()
        );
    }

    #[test]
    fn deserialize_parameter_with_schema() {
        let schema_reference = "#/definitions/SubscriptionName";
        let json_string = format!(
            r#"{{
            "name": "Test Name",
            "in": "body",
            "description": "Test Description",
            "required": true,
            "schema": {{
              "type": "object",
              "format": "file"
            }}
        }}"#
        );

        let parameter: Parameter = from_str(json_string.as_str()).unwrap();

        let expected_schema = Schema {
            reference: None,
            format: Some("file".to_string()),
            title: None,
            description: None,
            default: None,
            multiple_of: None,
            maximum: None,
            exclusive_maximum: None,
            minimum: None,
            exclusive_minimum: None,
            max_length: None,
            min_length: None,
            pattern: None,
            max_items: None,
            min_items: None,
            unique_items: None,
            max_properties: None,
            min_properties: None,
            required: None,
            schema_type: Some(SchemaType::Object),
        };

        assert_eq!(parameter.name.unwrap(), "Test Name");
        assert_eq!(parameter.location.unwrap(), "body");
        assert!(parameter.required.unwrap());
        assert_eq!(parameter.description.unwrap(), "Test Description");
        assert_eq!(parameter.schema.unwrap(), expected_schema);
    }
}
