use colored::Colorize;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{thread, time};

/// @notice Helper Function to check all the required dependencies are installed
/// @returns a boolean value indicating whether or not all dependencies are installed
pub fn check_dependencies_installed() -> bool {
    let required_tools = vec!["forge", "w3", "cartesi", "docker", "curl"];

    for tool in &required_tools {
        if !check_installed(tool.to_string()).unwrap_or(false) {
            eprintln!(
                "{} is not installed. Please install {} and try again.",
                tool.bright_red(),
                tool
            );
            return false;
        }
    }

    true
}

/// @notice Internal Function to ensure that a dependency exists
/// @param tool The name of the tool to check if installed
/// @returns a Result to tell of the operation was sucessful
pub fn check_installed(tool: String) -> Result<bool, String> {
    let output = Command::new("which")
        .arg(tool)
        .output()
        .map_err(|e| format!("Failed to execute 'which': {}", e))?;

    Ok(output.status.success())
}

/// @notice Function to read the contents of a file
/// @param path The path to file to be read from
/// @param var_name: The name of the file to be read from
pub fn read_file(path: &str, var_name: &str) -> String {
    if !Path::new(path).exists() {
        eprintln!(
            "{} {} file '{}' does not exist.",
            "Error::".red(),
            var_name,
            path
        );
        std::process::exit(1);
    }
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read {} file '{}'", var_name, path));
    content.trim().to_string()
}

/// @notice Function to get all available spaces
/// @return a verc of string representing all the available spaces
pub fn check_available_space() -> Vec<String> {
    let mut child = Command::new("w3")
        .arg("space")
        .arg("ls")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'w3 space creation'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));

    let mut available_spaces: Vec<String> = Vec::new();

    // Collect lines from stdout first
    let lines: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.ok()) // Filter out errors
        .collect();

    // Process collected lines to extract available spaces
    for line in lines {
        let response: Vec<&str> = line.split_ascii_whitespace().collect();
        available_spaces.push((response[response.len() - 1]).to_string());
    }

    // print the available spaces
    // for space in available_spaces.clone() {
    //     println!("Available space: {}", space);
    // }

    return available_spaces;
}

/// @notice Helper Function to check if a particulr email is loged in on w3 storage
/// @param email the email address to check if logged in
/// @returns a boolean value indicating whether or not the execution was successfull
pub fn check_if_logged_in(email: String) -> bool {
    let mut child = Command::new("w3")
        .arg("account")
        .arg("ls")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'w3 account ls'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    let mut logged_in: Vec<String> = Vec::new();
    let email_name: Vec<&str> = email.split("@").collect();

    // Collect lines from stdout first
    let lines: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.ok()) // Filter out errors
        .collect();

    // Process collected lines to extract available spaces
    for line in lines {
        if line.starts_with("did:mailto:gmail.com:") {
            // println!("{} {}", "--W3STORAGE:: ".green(), line[21..].green());
            logged_in.push(line[21..].to_string());
        }
    }

    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "W3STORAGE::NOTE::".yellow(), line.yellow());
            } else if let Err(e) = line {
                eprintln!("{} {}", "W3STORAGE::ERROR::".red(), e);
            }
        }
    });

    while start.elapsed().as_secs() < 300 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                for name in logged_in.clone() {
                    if email_name[0].to_lowercase() == name.to_lowercase() {
                        return true;
                    }
                }
            } else {
                return false;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the login process.");
    return false;
}

/// @notice Function to get the machine hash
pub fn get_machine_hash() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let image_hash = current_dir.join(".cartesi/image/hash");

    let output = Command::new("xxd")
        .arg("-p")
        .arg(image_hash)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'xxd' command")
        .stdout
        .expect("Failed to capture xxd output");

    let output = BufReader::new(output)
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<_>>()
        .concat();

    println!("MACHINE HASH::{}", output.trim().to_string());
    return output.trim().to_string();
}
