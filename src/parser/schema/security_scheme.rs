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
    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<SecuritySchemaLocation>,
    pub flow: Option<SecuritySchemaFlow>,
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: Option<String>,
    #[serde(rename = "tokenUrl")]
    pub token_url: Option<String>,
    pub scopes: Option<HashMap<String, String>>,
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
    fn deserialize_security_schema() {
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
        assert_eq!(security_schema.name.unwrap(), "oauth2");
        assert_eq!(
            security_schema.location.unwrap(),
            SecuritySchemaLocation::Header
        );
        assert_eq!(security_schema.flow.unwrap(), SecuritySchemaFlow::Implicit);
        assert_eq!(
            security_schema.authorization_url.unwrap(),
            "http://swagger.io/api/oauth/dialog"
        );
        assert_eq!(security_schema.token_url.unwrap(), "https://token-url.com");
        assert_eq!(security_schema.scopes.unwrap(), expected_scopes)
    }
}
