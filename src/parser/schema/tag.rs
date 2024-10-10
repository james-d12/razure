use crate::parser::schema::external_documentation::ExternalDocumentation;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

#[cfg(test)]
mod test {
    use crate::parser::schema::external_documentation::ExternalDocumentation;
    use crate::parser::schema::tag::Tag;
    use serde_json::from_str;

    #[test]
    fn deserialize_tag() {
        let json_string = r#"{
	        "name": "pet",
	        "description": "Pets operations",
	        "externalDocs": {
	            "url": "https://swagger.io",
                "description": "Find more info here"
            }
        }"#;

        let tag: Tag = from_str(json_string).unwrap();

        let expected_external_docs: ExternalDocumentation = ExternalDocumentation {
            url: String::from("https://swagger.io"),
            description: Some(String::from("Find more info here")),
        };

        assert_eq!(tag.name, "pet");
        assert_eq!(tag.description.unwrap(), "Pets operations");
        assert_eq!(tag.external_docs.unwrap(), expected_external_docs);
    }
}
