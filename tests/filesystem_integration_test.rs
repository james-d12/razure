use razure::filesystem::get_latest_stable_specifications;

#[test]
fn download_azure_gets_latest_specifications() {
    let current_directory = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let output_path = format!("{0}/tests/output", current_directory);
    let specifications = get_latest_stable_specifications(output_path.as_str());
    assert_ne!(specifications.len(), 0);
}
