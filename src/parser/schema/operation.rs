use crate::parser::schema::{ParameterType, Reference, Response};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub id: String,
    #[serde(rename = "x-ms-examples")]
    pub examples: Option<HashMap<String, Reference>>,
    pub description: Option<String>,
    pub parameters: Option<Vec<ParameterType>>,
    pub responses: HashMap<String, Response>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::schema::{Parameter, PropertyType};
    use serde_json::from_str;

    #[test]
    fn deserialize_operation_with_reference() {
        let schema_reference = "#/definitions/SubscriptionName";

        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
            "parameters": [
                {{
                    "$ref": "{0}"
                }}
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
        let mut expected_parameters: Vec<ParameterType> = Vec::new();
        expected_parameters.push(ParameterType::Reference(expected_reference));

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.examples, None);
    }

    #[test]
    fn deserialize_operation_with_parameter() {
        let schema_reference = "#/definitions/SubscriptionName";

        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
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
        let mut expected_parameters: Vec<ParameterType> = Vec::new();
        expected_parameters.push(ParameterType::Parameter(expected_parameter));

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.examples, None);
    }

    #[test]
    fn deserialize_operation_with_many_parameters() {
        let schema_reference = "#/definitions/SubscriptionName";

        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "operationId": "Test_OperationId",
            "description": "Test Description",
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

        let mut expected_parameters: Vec<ParameterType> = Vec::new();
        expected_parameters.push(ParameterType::Reference(expected_reference));
        expected_parameters.push(ParameterType::Parameter(expected_parameter));

        //todo! - This test fails if the reference is added after parameter to expected_parameters. We need to sort and then compare.

        assert_eq!(operation.id, "Test_OperationId");
        assert_eq!(operation.description.unwrap(), "Test Description");
        assert_eq!(operation.parameters.unwrap(), expected_parameters);
        assert_eq!(operation.examples, None);
    }
}
