use crate::parser::schema::ParameterType;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DefinitionPropertyType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DefinitionType {
    Object {
        properties: HashMap<String, DefinitionProperty>,
    },
    Array {
        items: Vec<DefinitionType>,
    },
    Other {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DefinitionProperty {
    #[serde(flatten)]
    pub schema: DefinitionType,
    pub description: Option<String>,
    pub pattern: Option<String>,
    #[serde(rename = "type")]
    pub definition_property_type: Option<DefinitionPropertyType>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(flatten)]
    pub schema: DefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<ParameterType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Display for DefinitionPropertyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let property_type_str = match self {
            DefinitionPropertyType::Object => "object",
            DefinitionPropertyType::String => "string",
            DefinitionPropertyType::Number => "number",
            DefinitionPropertyType::Integer => "integer",
            DefinitionPropertyType::Boolean => "boolean",
            DefinitionPropertyType::Array => "array",
        };
        write!(f, "{0}", property_type_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn deserialize_definition() {
        let schema_reference = "#/definitions/SubscriptionName";
        let json_string = r#"{
            "description": "Properties for the database account",
            "type": "object",
            "allOf": [
              {
                "$ref": ".../../cosmos-db.json#/definitions/ARMProxyResource"
              }  
            ],
            "properties": {
                "sqlDedicatedGatewayEndpoint": {
                    "type": "string",
                    "description": "SqlDedicatedGateway endpoint for the service."
                }
            }
        }"#;

        let definition: Definition = from_str(json_string).unwrap();

        let mut expected_schema_properties: HashMap<String, DefinitionProperty> = HashMap::new();
        expected_schema_properties.insert("sqlDedicatedGatewayEndpoint".to_string(), DefinitionProperty {
            schema: DefinitionType::Other { additional: HashMap::new() },
            description: Some("SqlDedicatedGateway endpoint for the service.".to_string()),
            pattern: None,
            definition_property_type: Some(DefinitionPropertyType::String),
            reference: None,
            read_only: None,
        });

        let expected_schema = DefinitionType::Object {
            properties: expected_schema_properties
        };

        assert_eq!(definition.schema, expected_schema);
    }
}

