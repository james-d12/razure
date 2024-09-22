use crate::parser::schema::ParameterType;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub description: Option<String>,
    pub schema: Option<ParameterType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::schema::{Parameter, PropertyType, Reference};
    use serde_json::from_str;

    #[test]
    fn deserialize_operation_with_reference() {
        let reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
                "description": "OK. The request has succeeded.",
                "schema": {{
                  "$ref": "{reference}"
                }}
        }}"#
        );

        let response: Response = from_str(json_string.as_str()).unwrap();

        let expected_schema = ParameterType::Reference(Reference {
            path: reference.to_string(),
        });

        assert_eq!(
            response.description.unwrap(),
            "OK. The request has succeeded."
        );
        assert_eq!(response.schema.unwrap(), expected_schema);
    }

    #[test]
    fn deserialize_operation_with_parameter() {
        let json_string = r#"{
                "description": "OK. The request has succeeded.",
                "schema": {
                    "name": "Test Name",
                    "description": "Test Description",
                    "in": "Test Location",
                    "required": false,
                    "type": "object",
                    "minLength": 10,
                    "maxLength": 16,
                    "pattern": "Test Pattern"
                }
        }"#;

        let response: Response = from_str(json_string).unwrap();

        let expected_parameter = ParameterType::Parameter(Parameter {
            name: Some("Test Name".to_string()),
            description: Some("Test Description".to_string()),
            location: Some("Test Location".to_string()),
            required: Some(false),
            schema: None,
            property_type: Some(PropertyType::Object),
            min_length: Some(10),
            max_length: Some(16),
            pattern: Some("Test Pattern".to_string()),
        });

        assert_eq!(
            response.description.unwrap(),
            "OK. The request has succeeded."
        );
        assert_eq!(response.schema.unwrap(), expected_parameter);
    }
}
