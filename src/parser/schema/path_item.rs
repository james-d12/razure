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
               "get": {0},
               "put": {0},
               "post": {0},
               "delete": {0},
               "options": {0},
               "head": {0},
               "patch": {0},
               "parameters": [
                    {{
                      "name": "createUpdateParameters",
                      "in": "body",
                      "description": "The Service resource parameters.",
                      "required": true,
                      "schema": {{
                        "$ref": "/#/parameters/accountNameParameter"
                      }}
                    }}
               ]
        }}"#,
            generate_endpoint_operation_json_string()
        );

        let expected_post_operation_response = Response {
            description: Some("OK. The request has succeeded.".to_string()),
            schema: Some(ParameterType::Reference(Reference {
                path: parameter_reference.to_string(),
            })),
        };

        let expected_operation_parameter = Parameter {
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

        let mut expected_operation_responses: HashMap<String, Response> = HashMap::new();
        expected_operation_responses.insert("200".to_string(), expected_post_operation_response);

        let expected_operation_parameters: Vec<ParameterType> =
            vec![ParameterType::Parameter(expected_operation_parameter)];

        let expected_operation: Operation = Operation {
            id: "Test_OperationId".to_string(),
            description: Some("Test Description".to_string()),
            summary: Some("Test Summary".to_string()),
            parameters: Some(expected_operation_parameters),
            responses: expected_operation_responses,
            tags: None,
        };

        let expected_parameters: Vec<ParameterType> = vec![ParameterType::Parameter(Parameter {
            name: Some("createUpdateParameters".to_string()),
            description: Some("The Service resource parameters.".to_string()),
            location: Some("body".to_string()),
            required: Some(true),
            schema: Some(Reference {
                path: "/#/parameters/accountNameParameter".to_string(),
            }),
            property_type: None,
            min_length: None,
            max_length: None,
            pattern: None,
        })];

        let path_item: PathItem = from_str(json_string.as_str()).unwrap();

        assert_eq!(path_item.get.unwrap(), expected_operation);
        assert_eq!(path_item.put.unwrap(), expected_operation);
        assert_eq!(path_item.post.unwrap(), expected_operation);
        assert_eq!(path_item.delete.unwrap(), expected_operation);
        assert_eq!(path_item.options.unwrap(), expected_operation);
        assert_eq!(path_item.head.unwrap(), expected_operation);
        assert_eq!(path_item.patch.unwrap(), expected_operation);
        assert_eq!(path_item.parameters.unwrap(), expected_parameters)
    }

    fn generate_endpoint_operation_json_string() -> String {
        format!(
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
            }}],
            "responses": {{
                "200": {{
                  "description": "OK. The request has succeeded.",
                  "schema": {{
                    "$ref": "{0}"
                  }}
                }}   
            }}
        }}"#,
            "#/parameters/subscriptionIdParameter"
        )
    }
}
