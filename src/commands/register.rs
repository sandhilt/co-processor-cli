use crate::helpers::helpers::{
    check_available_space, check_if_logged_in, get_machine_hash, read_file,
};
use colored::Colorize;
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread, time};

fn set_active_space(space_name: String) {
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
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    // Collect lines from stdout first
    let lines: Vec<Result<String, std::io::Error>> = stdout.lines().collect();

    // Process collected lines to extract available spaces
    for line in lines {
        if line.is_ok() {
            println!("Switched to space: {}", space_name);
            println!("Space ID: {}", line.unwrap());
            return;
        } else {
            eprintln!("Failed to switch to space: {}", space_name);
            return;
        }
    }

    thread::spawn(move || {
        for line in stderr.lines() {
            match line {
                Ok(output) => {
                    println!("{} {}", "WEB3STORAGE::".red(), output.red());
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });
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

fn upload_car_file(file_path: String) -> bool {
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
                return true;
            } else {
                eprintln!("{}", "upload process failed.".red());
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

fn build_program() -> bool {
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
                return true;
            } else {
                eprintln!("{}", "build process failed.".red());
                return false;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the build process.");
    return false;
}

fn run_carize_container() -> bool {
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
                return true;
            } else {
                eprintln!("{}", "car file generation process failed.".red());
                return false;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the generatioon process.");
    return false;
}

fn register_program_with_coprocessor() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let output_cid = current_dir.join("output.cid");
    let output_size = current_dir.join("output.size");

    let cid = read_file(
        output_cid
            .to_str()
            .expect("error converting path to string"),
        "CID",
    );
    let size = read_file(
        output_size
            .to_str()
            .expect("error converting path to string"),
        "SIZE",
    );
    let machine_hash = get_machine_hash();

    let curl_status = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg(format!(
            "https://cartesi-coprocessor-solver.fly.dev/ensure/{}/{}/{}",
            cid, machine_hash, size
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute curl POST command")
        .wait_with_output()
        .expect("Failed to wait for curl command to finish");

    if curl_status.status.success() {
        println!("{}", "Successfully sent request to co-processor.".green());
        let stdout = String::from_utf8_lossy(&curl_status.stdout);
        println!("{} {}", "RESPONSE::".green(), stdout.green());
    } else {
        eprintln!("Failed to send POST request.");
        let stderr = String::from_utf8_lossy(&curl_status.stderr);
        eprintln!("Error: {}", stderr);
    }
}

fn login(email: String) -> bool {
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
                return true;
            } else {
                eprintln!("Login process timed out. Please verify the email within the specified timeout.");
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

fn check_and_create_space(space: String) -> bool {
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
            return false;
        } else {
            set_active_space(new_space[0].clone());
            return true;
        }
    } else {
        set_active_space(existing_space[0].clone());
        return true;
    }
}

fn check_and_upload() -> bool {
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
        return false;
    }

    // Convert the path to a string
    let car_file = car_file_path
        .to_str()
        .expect("Failed to convert path to string");

    println!("{} CAR file found: {}", "INFO::".green(), car_file);
    match upload_car_file(car_file.to_string()) {
        true => return true,
        false => return false,
    };
}

pub fn register(email: String) {
    match check_if_logged_in(email.clone()) {
        true => {}
        false => {
            let _is_logged_in = login(email.clone());
        }
    };
    match build_program() {
        true => match run_carize_container() {
            true => match check_and_create_space("cartesi-coprocessor-programs".to_string()) {
                true => match check_and_upload() {
                    true => {
                        register_program_with_coprocessor();
                    }
                    false => return,
                },
                false => return,
            },
            false => {
                return;
            }
        },
        false => {
            return;
        }
    }
}
