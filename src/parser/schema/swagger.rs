use crate::parser::schema::external_documentation::ExternalDocumentation;
use crate::parser::schema::{Definition, Info, Parameter, PathItem, SecuritySchema, Tag};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    pub swagger: String,
    pub info: Option<Info>,
    pub schemes: Option<Vec<String>>,
    pub host: Option<String>,
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub paths: Option<HashMap<String, PathItem>>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub definitions: Option<HashMap<String, Definition>>,
    #[serde(rename = "securityDefinitions")]
    pub security_definitions: Option<HashMap<String, SecuritySchema>>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

#[cfg(test)]
mod tests {
    use crate::parser::schema::{Info, Swagger};
    use serde_json::from_str;

    #[test]
    fn deserialize_schema() {
        let json_string = r#"{
            "swagger": "2.0",
            "info": {
                "title": "Cosmos DB",
                "description": "Azure Cosmos DB Database Service Resource Provider REST API",
                "version": "2024-08-15"
            },
            "basePath": "/basepath",
            "host": "management.azure.com",
            "schemes": [
              "https"
            ],
            "consumes": [
              "application/json"
            ],
            "produces": [
              "application/json"
            ]
        }"#;

        let swagger: Swagger = from_str(json_string).unwrap();
        let expected_info: Info = Info {
            title: "Cosmos DB".to_string(),
            version: "2024-08-15".to_string(),
            description: Some(
                "Azure Cosmos DB Database Service Resource Provider REST API".to_string(),
            ),
            terms_of_service: None,
            contact: None,
            license: None,
        };

        assert_eq!(swagger.swagger, "2.0");
        assert_eq!(swagger.info.unwrap(), expected_info);
        assert_eq!(swagger.base_path.unwrap(), "/basepath");
        assert_eq!(swagger.host.unwrap(), "management.azure.com");
        assert_eq!(swagger.schemes.as_ref().unwrap().len(), 1);
        assert_eq!(swagger.schemes.as_ref().unwrap().first().unwrap(), "https");
        assert_eq!(swagger.consumes.as_ref().unwrap().len(), 1);
        assert_eq!(
            swagger.consumes.as_ref().unwrap().first().unwrap(),
            "application/json"
        );
        assert_eq!(swagger.produces.as_ref().unwrap().len(), 1);
        assert_eq!(
            swagger.produces.as_ref().unwrap().first().unwrap(),
            "application/json"
        );
    }
}
