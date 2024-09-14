pub trait RustType {
    fn get_type_as_string(&self) -> Option<&str>;
}

fn rename_keyword(s: &str) -> &str {
    let lowercase = s.to_lowercase();
    match lowercase.as_str() {
        "type" => "type_",
        "enum" => "enum_",
        "ref" => "ref_",
        "loop" => "loop_",
        "true" => "true_",
        "false" => "false_",
        "self" => "self_",
        "for" => "for_",
        "as" => "as_",
        &_ => s,
    }
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

    let formatted_name = rename_keyword(name.as_str());
    to_snake_case(formatted_name)
        .replace("$", "")
        .replace("@", "")
        .replace(".", "_")
}

pub fn format_name_as_valid_struct_identifier(name: &String) -> String {
    let mut formatted_name = name
        .trim()
        .replace("-", "")
        .replace("_", "")
        .replace("$", "")
        .replace(".", "")
        .replace("[", "")
        .replace("]", "")
        .replace("(", "")
        .replace(")", "")
        .replace("{", "")
        .replace("}", "")
        .replace(",", "")
        .replace("`", "");

    formatted_name = capitalize(formatted_name.as_str());
    formatted_name
}

pub fn format_as_file_name(s: &String) -> String {
    let mut name = s.replace(".json", "");
    name = to_snake_case(name.as_str());
    name = name
        .replace(".", "_")
        .replace("a_p_i", "api")
        .replace("__", "_");
    name
}
