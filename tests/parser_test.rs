use razure::filesystem::get_latest_stable_specifications;
use razure::parser::parse_specification_file;
use std::collections::HashMap;

#[test]
fn download_azure_gets_latest_specifications() {
    let current_directory = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let output_path = format!("{0}/tests/output", current_directory);
    let specifications = get_latest_stable_specifications(output_path.as_str()).unwrap();

    let mut failed_count = 0;
    let mut failed_files = HashMap::new();

    for specification_file in specifications.values() {
        let parsed = parse_specification_file(specification_file);
        match parsed {
            Ok(_swagger) => {}
            Err(error) => {
                failed_count += 1;
                failed_files.insert(&specification_file.file_path, error.to_string());
            }
        }
    }

    assert_ne!(specifications.len(), 0);
    println!("Failed count: {failed_count}");

    println!("Failed Files: ");
    for (file, error) in failed_files {
        println!("{0} with error {1}", file, error);
    }

    assert!(failed_count <= 4);
}
