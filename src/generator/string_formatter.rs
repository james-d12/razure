
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
pub fn format_name_as_valid_struct_identifier(name: &String) -> String {
    
    // Capitalize first letter
    // Remove hyphens, underscores and white space.
    
    let mut formatted_name = name
        .trim()
        .replace("-", "")
        .replace("_", "")
        .replace(".", "");
    
    formatted_name = capitalize(formatted_name.as_str());
    formatted_name
}