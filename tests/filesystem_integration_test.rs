use razure::filesystem::get_latest_stable_specifications;

#[test]
fn download_azure_gets_latest_specifications() {
    let output_path = "./tests/output";
    let specifications = get_latest_stable_specifications(output_path);
    assert_ne!(specifications.len(), 0);
}
