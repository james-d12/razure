fn rename_keyword(s: &str) -> &str {
    let lowercase = s.to_lowercase();
    match lowercase.as_str() {
        "type" => "r#type",
        "enum" => "r#enum",
        "ref" => "r#ref",
        "loop" => "r#loop",
        "true" => "r#true",
        "false" => "r#false",
        "self" => "r#this",
        "for" => "r#for",
        "as" => "r#as",
        "use" => "r#use",
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

fn to_lower_case(s: &str) -> String {
    s.to_lowercase().replace(['-', '_', '_', '.'], "")
}

pub fn format_field_as_valid_field_identifier(name: &String) -> String {
    if name == "type" {
        return "property_type".to_string();
    }

    let formatted_name = rename_keyword(name.as_str());
    to_lower_case(formatted_name).replace(['$', '@', '.'], "")
}

pub fn format_name_as_valid_struct_identifier(name: &str) -> String {
    let mut formatted_name = name.trim().replace(
        [
            '-', '_', '$', '.', '[', ']', '(', ')', '{', '}', ',', '`', '`',
        ],
        "",
    );

    formatted_name = capitalize(formatted_name.as_str());
    formatted_name
}

pub fn format_as_file_name(s: &str) -> String {
    let mut name = s.replace(".json", "");
    name = to_lower_case(&name);
    name = name.replace("a_p_i", "api");
    //.replace("__", "_");
    name
}
