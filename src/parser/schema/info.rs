use serde::Deserialize;
use crate::parser::schema::contact::Contact;
use crate::parser::schema::license::License;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[cfg(test)]
mod test {
    use crate::parser::schema::Info;
    use serde_json::from_str;
    use crate::parser::schema::contact::Contact;
    use crate::parser::schema::license::License;

    #[test]
    fn deserialize_info() {
        let json_string = r#"{
            "title": "Test Title",
            "version": "2021-10-01",
            "description": "Test Description",
            "contact": {
                "name": "Test Contact Name",
                "url": "http://www.test.com",
                "email": "support@test.io"
            },
            "license": {
                "name": "Test License Name",
                "url": "http://www.test.com"
            },
            "termsOfService": "Test TOS"
        }"#;

        let info: Info = from_str(json_string).unwrap();

        let expected_contact: Contact = Contact {
            name: Some("Test Contact Name".to_string()),
            url: Some("http://www.test.com".to_string()),
            email: Some("support@test.io".to_string()),
        };

        let expected_license: License = License {
            name: Some("Test License Name".to_string()),
            url: Some("http://www.test.com".to_string()),
        };

        assert_eq!(info.title, "Test Title");
        assert_eq!(info.version, "2021-10-01");
        assert_eq!(info.description.unwrap(), "Test Description");
        assert_eq!(info.contact.unwrap(), expected_contact);
        assert_eq!(info.license.unwrap(), expected_license);
        assert_eq!(info.terms_of_service.unwrap(), "Test TOS");
    }
}
