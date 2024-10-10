use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::parser::schema::license::License;
    use serde_json::from_str;

    #[test]
    fn deserialize_license() {
        let json_string = r#"{
            "name": "Test License Name",
            "url": "http://www.test.com"
        }"#;

        let license: License = from_str(json_string).unwrap();

        assert_eq!(license.name.unwrap(), "Test License Name");
        assert_eq!(license.url.unwrap(), "http://www.test.com");
    }
}
