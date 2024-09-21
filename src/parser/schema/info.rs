use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::parser::schema::Info;
    use serde_json::from_str;

    #[test]
    fn deserialize_info() {
        let json_string = r#"{
            "title": "Test Title",
            "version": "2021-10-01",
            "description": "Test Description",
            "summary": "Test Summary",
            "termsOfService": "Test TOS"
        }"#;

        let info: Info = from_str(json_string).unwrap();

        assert_eq!(info.title, "Test Title");
        assert_eq!(info.version, "2021-10-01");
        assert_eq!(info.description.unwrap(), "Test Description");
        assert_eq!(info.summary.unwrap(), "Test Summary");
        assert_eq!(info.terms_of_service.unwrap(), "Test TOS");
    }
}
