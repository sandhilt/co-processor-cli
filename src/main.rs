use clap::{Parser, Subcommand};
use colored::Colorize;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::{
    error::Error,
    process::{Command, Stdio},
};
use std::{thread, time};

/// A CLI tool to interact with Web3.Storage
#[derive(Parser)]
#[command(author = "Your Name", version, about = "Manage Web3.Storage easily from your CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Log in to Web3.Storage
    #[command(about = "Log in using your email address")]
    Login {
        /// Email address for logging in
        #[arg(short, long, help = "Your email address registered with Web3.Storage")]
        email: String,
    },
    /// Create a new storage space in Web3.Storage
    #[command(about = "Create a new named storage space")]
    CreateSpace {
        /// Name of the storage space
        #[arg(short, long, help = "A unique name for your storage space")]
        space: String,
    },
    /// Upload a CAR file to Web3.Storage
    #[command(about = "Upload a CAR file to Web3.Storage")]
    Upload,
    Build,
    carize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("command: {:?}", cli.command);

    match cli.command {
        Commands::Login { email } => {
            println!("Logging into Web3.Storage...");
            let mut child = Command::new("w3")
                .arg("login")
                .arg(email)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute 'w3 login'");

            let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
            let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

            let start = time::Instant::now();

            // Handle output in separate threads
            thread::spawn(move || {
                for line in stdout.lines() {
                    if let Ok(line) = line {
                        println!("{} {}", "W3STORAGE:: ".green(), line.green());
                    }
                }
            });

            thread::spawn(move || {
                for line in stderr.lines() {
                    if let Ok(line) = line {
                        eprintln!("{} {}", "W3STORAGE::NOTE::".yellow(), line.yellow());
                    } else if let Err(e) = line {
                        eprintln!("{} {}", "W3STORAGE::ERROR::".red(), e);
                    }
                }
            });

            // Wait for email verification or timeout
            while start.elapsed().as_secs() < 300 {
                if let Some(status) = child.try_wait().expect("Failed to check process status") {
                    if status.success() {
                        println!("Successfully logged in to Web3.Storage.");
                        return Ok(());
                    } else {
                        eprintln!("Login process failed.");
                        return Ok(());
                    }
                }

                // Poll every 5 seconds
                thread::sleep(time::Duration::from_secs(5));
            }

            // If timeout occurs
            child
                .kill()
                .expect("Failed to terminate the login process.");
            eprintln!(
                "Login process timed out. Please verify the email within the specified timeout."
            );
            Ok(())
        }

        Commands::CreateSpace { space } => {
            let available_spaces = check_available_space();

            let existing_space: Vec<String> = available_spaces
                .into_iter()
                .filter(|space_name| space_name.to_lowercase() == space.to_lowercase())
                .collect();
            // println!("{:?} match is:", existing_space);

            if existing_space.is_empty() {
                println!("Creating space {}", space);
                create_space(space.clone());

                let new_available_spaces = check_available_space();
                let new_space: Vec<String> = new_available_spaces
                    .into_iter()
                    .filter(|space_name| space_name.to_lowercase() == space.to_lowercase())
                    .collect();

                if new_space.is_empty() {
                    println!("{}", "ERROR creating space".red());
                    return Ok(());
                } else {
                    set_active_space(new_space[0].clone());
                }
            } else {
                set_active_space(existing_space[0].clone());
            }

            Ok(())
        }
        Commands::Upload {} => {
            let car_file_name = "output.car";
            // Get the current directory
            let current_dir = env::current_dir().expect("Failed to get current directory");

            // Build the full path to the CAR file
            let car_file_path = current_dir.join(car_file_name);

            // Check if the file exists
            if !car_file_path.exists() {
                eprintln!(
                    "{} The CAR file '{}' was not found in the current directory '{}'.",
                    "Error::".red(),
                    car_file_name,
                    current_dir.display()
                );
                return Ok(());
            }

            // Convert the path to a string
            let car_file = car_file_path
                .to_str()
                .expect("Failed to convert path to string");

            println!("{} CAR file found: {}", "INFO::".green(), car_file);
            upload_car_file(car_file.to_string());
            Ok(())
        }

        Commands::Build {} => {
            println!("Building Cartesi Program...");
            build_program();
            Ok(())
        }

        Commands::carize {} => {
            println!("Generating Carize file..");
            run_carize_container();
            Ok(())
        }
    }
}

fn check_available_space() -> Vec<String> {
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

fn set_active_space(space_name: String) -> bool {
    let mut child = Command::new("w3")
        .arg("space")
        .arg("use")
        .arg(space_name.clone())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'w3 space creation'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));

    // Collect lines from stdout first
    let lines: Vec<Result<String, std::io::Error>> = stdout.lines().collect();

    // Process collected lines to extract available spaces
    for line in lines {
        if line.is_ok() {
            println!("Switched to space: {}", space_name);
            println!("Space ID: {}", line.unwrap());
            return true;
        } else {
            eprintln!("Failed to switch to space: {}", space_name);
            return false;
        }
    }
    return false;
}

fn create_space(space: String) {
    println!("Creating a new storage space: {}", space);
    let mut child = Command::new("w3")
        .arg("space")
        .arg("create")
        .arg(&space.clone())
        .arg("--no-recovery")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'w3 space create'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    // Handle initial output and extract keyphrase
    thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                println!("{} {}", "W3STORAGE::OUTPUT:: ".green(), line.green());
            }
        }
        return;
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "W3STORAGE::NOTE::".yellow(), line.yellow());
                if line.contains("- Waiting for payment plan to be selected") {
                    println!("{}", "W3STORAGE::INSTRUCTION:: Login to your W3 storage dashboard and complete your payment plan selection".yellow());
                    break;
                }
            } else if let Err(e) = line {
                eprintln!("{} {}", "W3STORAGE::ERROR::".red(), e);
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 50 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("Successfully created your space.");
                break;
            } else {
                eprintln!("space creation process timed out. Please select your payment plan within the specified timeout.");
                break;
            }
        }

        // Poll every 2 seconds
        thread::sleep(time::Duration::from_secs(2));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the space creation process.");
}

fn upload_car_file(file_path: String) {
    println!("{}", "Uploading CAR file...".yellow());

    let mut child = Command::new("w3")
        .arg("up")
        .arg("--car")
        .arg(file_path.clone())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'w3 up'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));

    let start = time::Instant::now();

    thread::spawn(move || {
        for line in stdout.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "W3STORAGE::".green(), output.green());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 300 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("{}", "Successfully uploaded file to Web3.Storage.".green());
                break;
            } else {
                eprintln!("{}", "upload process failed.".red());
                break;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the login process.");
}

fn build_program() {
    println!("{}", "Building Cartesi Program...".yellow());
    let mut child = Command::new("cartesi")
        .arg("build")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'cartesi build' command");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    thread::spawn(move || {
        for line in stdout.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "CARTESI::".green(), output.green());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "CARTESI::".green(), output.red());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 300 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("{}", "Cartesi Program built successfully.".green());
                break;
            } else {
                eprintln!("{}", "build process failed.".red());
                break;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the build process.");
}

fn run_carize_container() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    println!("{}", "Running Cartesi Container...".yellow());
    let mut child = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("-v")
        .arg(format!(
            "{}:/data",
            current_dir
                .join(".cartesi/image")
                .to_str()
                .expect("failed to convert path to string")
        ))
        .arg("-v")
        .arg(format!(
            "{}:/output",
            current_dir
                .to_str()
                .expect("Failed to get current directory")
        ))
        .arg("carize:latest")
        .arg("/carize.sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'cartesi build' command");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    thread::spawn(move || {
        for line in stdout.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "CARIZE::".green(), output.green());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "CARIZE::".green(), output.red());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 300 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("{}", "CARIZE generated successfully.".green());
                break;
            } else {
                eprintln!("{}", "car file generation process failed.".red());
                break;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the generatioon process.");
}
