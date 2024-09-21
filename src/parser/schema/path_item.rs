use crate::parser::schema::Operation;
use crate::parser::schema::ParameterType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PathItem {
    #[serde(rename = "$ref")]
    reference: Option<String>,
    get: Option<Operation>,
    put: Option<Operation>,
    post: Option<Operation>,
    delete: Option<Operation>,
    options: Option<Operation>,
    head: Option<Operation>,
    patch: Option<Operation>,
    parameters: Option<Vec<ParameterType>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parameter, PropertyType, Reference, Response};
    use serde_json::from_str;
    use std::collections::HashMap;

    #[test]
    fn deserialize_path_item() {
        let parameter_reference = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "post": {{
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
                }}],
                "responses": {{
                    "200": {{
                      "description": "OK. The request has succeeded.",
                      "schema": {{
                        "$ref": "{parameter_reference}"
                      }}
                    }}   
                }}
            }}
        }}"#
        );

        let expected_post_operation_response = Response {
            description: Some("OK. The request has succeeded.".to_string()),
            schema: Some(ParameterType::Reference(Reference {
                path: parameter_reference.to_string(),
            })),
        };

        let expected_post_operation_parameter = Parameter {
            name: Some("Test Name".to_string()),
            description: Some("Test Description".to_string()),
            location: Some("query".to_string()),
            required: Some(true),
            schema: None,
            property_type: Some(PropertyType::String),
            min_length: Some(5),
            max_length: Some(64),
            pattern: None,
        };

        let mut expected_post_operation_responses: HashMap<String, Response> = HashMap::new();
        expected_post_operation_responses
            .insert("200".to_string(), expected_post_operation_response);

        let expected_post_operation_parameters: Vec<ParameterType> =
            vec![ParameterType::Parameter(expected_post_operation_parameter)];

        let expected_post_operation: Operation = Operation {
            id: "Test_OperationId".to_string(),
            description: Some("Test Description".to_string()),
            parameters: Some(expected_post_operation_parameters),
            responses: expected_post_operation_responses,
            examples: None,
        };

        let path_item: PathItem = from_str(json_string.as_str()).unwrap();

        assert_eq!(path_item.post.unwrap(), expected_post_operation);
    }
}
