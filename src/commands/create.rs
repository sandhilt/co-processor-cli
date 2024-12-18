use colored::Colorize;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{thread, time};

fn create_contract_template(work_dir: &PathBuf) {
    let contract_template = include_str!("../../contract-template/src/MyContract.sol");
    let work_dir = work_dir.join("src/MyContract.sol");

    match fs::write(work_dir, contract_template) {
        Ok(_) => println!("{}", "Successfully created contract template.".green(),),
        Err(e) => eprintln!("Error creating contract template: {}", e),
    };
}

fn install_base_contract(work_dir: &PathBuf) {
    let work_dir = work_dir.join("contracts");

    let mut child = Command::new("forge")
        .arg("install")
        .arg("https://github.com/Mugen-Builders/coprocessor-base-contract")
        .arg("--no-commit")
        .current_dir(work_dir.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'forge install' command");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    // Handle initial output and extract keyphrase
    thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                println!("{} {}", "FORGE:: ".green(), line.green());
            }
        }
        return;
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "FORGE::".yellow(), line.yellow());
            } else if let Err(e) = line {
                eprintln!("{} {}", "FORGE::ERROR::".red(), e);
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 50 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("{}", "Successfully initialized base contract.".green());
                create_contract_template(&work_dir);
                break;
            } else {
                eprintln!("error installing base contract.");
                break;
            }
        }

        // Poll every 2 seconds
        thread::sleep(time::Duration::from_secs(2));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the base contract installation process.");
}

fn bootstrap_foundry(project_name: &str) {
    // Create the Foundry project directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let work_dir = current_dir.join(project_name);

    let mut child = Command::new("forge")
        .arg("init")
        .arg("contracts")
        .arg("--no-commit")
        .current_dir(work_dir.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'forge init' command");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    // Handle initial output and extract keyphrase
    thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                println!("{} {}", "FORGE:: ".green(), line.green());
            }
        }
        return;
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "FORGE::".yellow(), line.yellow());
            } else if let Err(e) = line {
                eprintln!("{} {}", "FORGE::ERROR::".red(), e);
            }
        }
    });

    // Wait for email verification or timeout
    while start.elapsed().as_secs() < 50 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!("{}", "Successfully initialized foundry project.".green());
                install_base_contract(&work_dir);
                break;
            } else {
                eprintln!("error initializing a new forge project.");
                break;
            }
        }

        // Poll every 2 seconds
        thread::sleep(time::Duration::from_secs(2));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the foundry project initialization process.");
}

fn create_template(dapp_name: String, template: String) {
    let mut child = Command::new("cartesi")
        .arg("create")
        .arg(dapp_name.clone())
        .arg(format!("--template={}", template))
        .arg("--branch")
        .arg("wip/coprocessor")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'cartesi create command'");

    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
    let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));

    let start = time::Instant::now();

    // Handle output in separate threads
    thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                println!("{} {}", "CARTESI:: ".green(), line.green());
            }
        }
    });

    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "CARTESI::NOTE::".yellow(), line.yellow());
            } else if let Err(e) = line {
                eprintln!("{} {}", "CARTESI::ERROR::".red(), e);
            }
        }
    });

    while start.elapsed().as_secs() < 300 {
        if let Some(status) = child.try_wait().expect("Failed to check process status") {
            if status.success() {
                println!(
                    "{}",
                    "CARTESI:: Successfully created dapp template.".green()
                );
                bootstrap_foundry(&dapp_name);
                return;
            } else {
                eprintln!("Template creation process failed.");
                return;
            }
        }

        // Poll every 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }

    // If timeout occurs
    child
        .kill()
        .expect("Failed to terminate the login process.");
    eprintln!("Template creation process timed out. Please verify the email within the specified timeout.");
}

pub fn create(dapp_name: String, template: String) {
    create_template(dapp_name, template);
}
