use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ExternalDocumentation {
    pub url: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::parser::schema::external_documentation::ExternalDocumentation;
    use serde_json::from_str;

    #[test]
    fn deserialize_external_documentation() {
        let json_string = r#"{
            "url": "https://swagger.io",
            "description": "Find more info here"
        }"#;

        let external_documentation: ExternalDocumentation = from_str(json_string).unwrap();

        assert_eq!(external_documentation.url, "https://swagger.io");
        assert_eq!(
            external_documentation.description.unwrap(),
            "Find more info here"
        )
    }

    #[test]
    fn deserialize_external_documentation_without_description() {
        let json_string = r#"{
            "url": "https://swagger.io"
        }"#;

        let external_documentation: ExternalDocumentation = from_str(json_string).unwrap();

        assert_eq!(external_documentation.url, "https://swagger.io");
        assert_eq!(external_documentation.description, None)
    }
}
