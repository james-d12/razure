use chrono::NaiveDate;
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

type SpecificationFiles = HashMap<String, SpecificationFile>;

#[derive(Debug, Error)]
pub enum SpecificationFileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}

pub struct SpecificationFile {
    pub file_name: String,
    pub file_path: String,
    pub domain_name: String,
    pub date: NaiveDate,
}

fn get_domain_name_for_file(path_item: &Path) -> Option<String> {
    let great_grand_parent = path_item.parent()?.parent()?.parent()?;
    let domain_name = great_grand_parent.file_name()?.to_str()?.to_string();
    Some(domain_name)
}

fn is_valid_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let path_str = path.to_string_lossy();

    if path_str.contains("example") || path_str.contains("preview") {
        return false;
    }

    if path.extension() != Some(std::ffi::OsStr::new("json")) {
        return false;
    }

    true
}

fn get_json_files_for_directory(
    directory: &PathBuf,
) -> Result<SpecificationFiles, SpecificationFileError> {
    let mut specification_files_hashmap: HashMap<String, SpecificationFile> = HashMap::new();

    for file in fs::read_dir(directory)?.flatten() {
        let path = file.path();

        if path.is_dir() {
            let json_files = get_json_files_for_directory(&path)?;
            specification_files_hashmap.extend(json_files);
        }

        if is_valid_file(&path) {
            let full_path_str = path.to_str().unwrap();

            let parent_file_name = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|f| f.to_str());

            if let Some(parent_file_name) = parent_file_name {
                if let Ok(date) = NaiveDate::parse_from_str(parent_file_name, "%Y-%m-%d") {
                    let file_name = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or_default()
                        .to_lowercase();
                    let domain_name = get_domain_name_for_file(&path)
                        .unwrap_or_default()
                        .to_lowercase();
                    let key = format!("{domain_name}-{file_name}");

                    let current_specification_file = specification_files_hashmap.get(&key);

                    match current_specification_file {
                        Some(current_specification_file) => {
                            if date > current_specification_file.date {
                                specification_files_hashmap.insert(
                                    key,
                                    SpecificationFile {
                                        file_name,
                                        file_path: full_path_str.to_string(),
                                        domain_name: domain_name.to_string(),
                                        date,
                                    },
                                );
                            }
                        }
                        None => {
                            specification_files_hashmap.insert(
                                key,
                                SpecificationFile {
                                    file_name,
                                    file_path: full_path_str.to_string(),
                                    domain_name: domain_name.to_string(),
                                    date,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(specification_files_hashmap)
}

fn validate_output_path(output_path: &str) -> Result<(), SpecificationFileError> {
    let output_path_exists = Path::new(output_path).try_exists()?;

    if !output_path_exists {
        println!("Creating path: {0}.", output_path);
        fs::create_dir(output_path)?;
    }

    Ok(())
}

pub fn get_latest_stable_specifications(
    output_path: &str,
) -> Result<SpecificationFiles, SpecificationFileError> {
    validate_output_path(output_path)?;

    println!("Getting latest Stable Azure Specifications");
    let specification_path = PathBuf::from(output_path).join("specification");

    if specification_path.exists() {
        println!("Azure Repository already downloaded.");
        return get_json_files_for_directory(&specification_path);
    }

    Repository::clone(
        "https://github.com/Azure/azure-rest-api-specs.git",
        output_path,
    )?;
    get_json_files_for_directory(&specification_path)
}
