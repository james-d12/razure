use filesystem::filesystem::get_latest_stable_specifications;
use parser::parser::parse_specification_file;

#[test]
fn download_azure_gets_latest_specifications() {
    let current_directory = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let output_path = format!("{0}\\tests\\output", current_directory);
    let specifications = get_latest_stable_specifications(output_path.as_str());

    let mut failed_count = 0;
    for (_, specification_file) in &specifications {
        let parsed = parse_specification_file(&specification_file);
        match parsed {
            Some(_swagger) => {}
            None => failed_count += 1,
        }
    }

    assert_ne!(specifications.len(), 0);
    assert!(failed_count <= 17);
}
