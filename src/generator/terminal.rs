use log::error;
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
pub fn generate_cargo_project(path: &str) -> bool {
    let command = format!("cd {path} && cargo init");
    let result = Command::new("cmd").args(["/C", command.as_str()]).status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            error!("Error: {0} whilst trying to execute {1}.", error, command);
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn check_executable_exists(executable: &str) -> bool {
    let result = Command::new("cmd")
        .args(["/C", executable])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to check executable {1} exists.",
                error, executable
            );
            false
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn generate_cargo_project(path: &str) -> bool {
    let command = format!("cd {path} && cargo init");
    let result = Command::new("sh").arg("-c").arg(command.as_str()).status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to execute {1}.",
                error.to_string(),
                command
            );
            false
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn check_executable_exists(executable: &str) -> bool {
    let result = Command::new("sh")
        .arg("-c")
        .arg(executable)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to check executable {1} exists.",
                error.to_string(),
                executable
            );
            false
        }
    }
}
