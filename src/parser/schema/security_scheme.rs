use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub enum SecuritySchemaLocation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum SecuritySchemaType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "apiKey")]
    ApiKey,
    #[serde(rename = "oauth2")]
    OAuth2,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum SecuritySchemaFlow {
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "accessCode")]
    AccessCode,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SecuritySchema {
    #[serde(rename = "type")]
    pub security_type: SecuritySchemaType,
    pub name: String,
    #[serde(rename = "in")]
    pub location: SecuritySchemaLocation,
    pub flow: SecuritySchemaFlow,
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    pub scopes: HashMap<String, String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::parser::schema::security_scheme::{
        SecuritySchema, SecuritySchemaFlow, SecuritySchemaLocation, SecuritySchemaType,
    };
    use serde_json::from_str;
    use std::collections::HashMap;

    #[test]
    fn deserialize_schema() {
        let json_string = r#"{
            "type": "oauth2",
            "name": "oauth2",
            "in": "header",
            "flow": "implicit",
            "authorizationUrl": "http://swagger.io/api/oauth/dialog",
            "tokenUrl": "https://token-url.com",
            "scopes": {
              "write:pets": "modify pets in your account",
              "read:pets": "read your pets"
            }
        }"#;

        let security_schema: SecuritySchema = from_str(json_string).unwrap();

        let mut expected_scopes: HashMap<String, String> = HashMap::new();
        expected_scopes.insert(
            String::from("write:pets"),
            String::from("modify pets in your account"),
        );
        expected_scopes.insert(String::from("read:pets"), String::from("read your pets"));

        assert_eq!(security_schema.security_type, SecuritySchemaType::OAuth2);
        assert_eq!(security_schema.name, "oauth2");
        assert_eq!(security_schema.location, SecuritySchemaLocation::Header);
        assert_eq!(security_schema.flow, SecuritySchemaFlow::Implicit);
        assert_eq!(
            security_schema.authorization_url,
            "http://swagger.io/api/oauth/dialog"
        );
        assert_eq!(security_schema.token_url, "https://token-url.com");
        assert_eq!(security_schema.scopes, expected_scopes)
    }
}
