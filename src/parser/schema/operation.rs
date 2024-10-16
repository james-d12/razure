use crate::parser::schema::{ParameterType, Response};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub id: String,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub parameters: Option<Vec<ParameterType>>,
    pub responses: HashMap<String, Response>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::schema::{Parameter, PropertyType, Reference};
    use serde_json::from_str;

    #[test]
    fn deserialize_operation_with_reference() {
        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
            "summary": "Test Summary",
            "parameters": [
                {{
                    "$ref": "{0}"
                }}
            ],
            "tags": [
                "test-tag1",
                "test-tag2"
            ],
            "responses": {{
                "200": {{
                  "description": "OK. The request has succeeded.",
                  "schema": {{
                    "$ref": "{0}"
                  }}
                }}   
            }}
        }}"#,
            parameter_reference
        );

        let operation: Operation = from_str(json_string.as_str()).unwrap();

        let expected_reference: Reference = Reference {
            path: parameter_reference.to_string(),
        };
        let expected_parameters: Vec<ParameterType> =
            vec![ParameterType::Reference(expected_reference)];

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.summary.unwrap(), "Test Summary");
        assert_eq!(operation.tags.unwrap(), vec!["test-tag1", "test-tag2"]);
    }

    #[test]
    fn deserialize_operation_with_parameter() {
        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
            "summary": "Test Summary",
            "parameters": [
                {{
                     "name": "Test Name",
                     "description": "Test Description",
                     "in": "query",
                     "required": true,
                     "type": "string",
                     "minLength": 5,
                     "maxLength": 64 
                }}
            ],
            "tags": [
                "test-tag1",
                "test-tag2"
            ],
            "responses": {{
                "200": {{
                  "description": "OK. The request has succeeded.",
                  "schema": {{
                    "$ref": "{0}"
                  }}
                }}   
            }}
        }}"#,
            parameter_reference
        );

        let operation: Operation = from_str(json_string.as_str()).unwrap();

        let expected_parameter: Parameter = Parameter {
            name: Some("Test Name".to_string()),
            description: Some("Test Description".to_string()),
            location: Some("query".to_string()),
            required: Some(true),
            min_length: Some(5),
            max_length: Some(64),
            property_type: Some(PropertyType::String),
            schema: None,
            pattern: None,
        };
        let expected_parameters: Vec<ParameterType> =
            vec![ParameterType::Parameter(expected_parameter)];

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.summary.unwrap(), "Test Summary");
        assert_eq!(operation.tags.unwrap(), vec!["test-tag1", "test-tag2"]);
    }

    #[test]
    fn deserialize_operation_with_many_parameters() {
        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
            "summary": "Test Summary",
            "parameters": [
                {{
                    "$ref": "{0}"
                }},
                {{
                     "name": "Test Name",
                     "description": "Test Description",
                     "in": "query",
                     "required": true,
                     "type": "string",
                     "minLength": 5,
                     "maxLength": 64 
                }}
            ],
            "tags": [
                "test-tag1",
                "test-tag2"
            ],
            "responses": {{
                "200": {{
                  "description": "OK. The request has succeeded.",
                  "schema": {{
                    "$ref": "{0}"
                  }}
                }}   
            }}
        }}"#,
            parameter_reference
        );

        let operation: Operation = from_str(json_string.as_str()).unwrap();

        let expected_parameter: Parameter = Parameter {
            name: Some("Test Name".to_string()),
            description: Some("Test Description".to_string()),
            location: Some("query".to_string()),
            required: Some(true),
            min_length: Some(5),
            max_length: Some(64),
            property_type: Some(PropertyType::String),
            schema: None,
            pattern: None,
        };

        let expected_reference: Reference = Reference {
            path: parameter_reference.to_string(),
        };

        let expected_parameters: Vec<ParameterType> = vec![
            ParameterType::Reference(expected_reference),
            ParameterType::Parameter(expected_parameter),
        ];

        //todo! - This test fails if the reference is added after parameter to expected_parameters. We need to sort and then compare.

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.summary.unwrap(), "Test Summary");
        assert_eq!(operation.tags.unwrap(), vec!["test-tag1", "test-tag2"]);
    }
}
