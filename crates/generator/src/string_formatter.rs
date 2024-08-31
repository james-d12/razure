pub trait RustType {
    fn get_type_as_string(&self) -> Option<&str>;
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result.replace("-", "_")
}

pub fn format_field_as_valid_field_identifier(name: &String) -> String {
    if name == "type" {
        return "property_type".to_string();
    }

    to_snake_case(name)
}

pub fn format_name_as_valid_struct_identifier(name: &String) -> String {
    let mut formatted_name = name
        .trim()
        .replace("-", "")
        .replace("_", "")
        .replace("$", "")
        .replace(".", "");

    formatted_name = capitalize(formatted_name.as_str());
    formatted_name
}

pub fn format_as_file_name(name: &String) -> String {
    to_snake_case(name.replace(".json", "").as_str())
}
