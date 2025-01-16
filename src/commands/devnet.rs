use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread, time, time::Duration};

pub fn start_devnet() {
    let coprocessor_path = clone_coprocessor_repo();

    match coprocessor_path {
        Some(path) => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            spinner.set_message("Starting devnet containers...");

            // Start the spinner
            spinner.enable_steady_tick(Duration::from_millis(100));

            // Run Cartesi-Coprocessor in the background
            let docker_status = Command::new("docker")
                .arg("compose")
                .arg("-f")
                .arg("docker-compose-devnet.yaml")
                .arg("up")
                .arg("--wait")
                .arg("-d")
                .current_dir(path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start Cartesi-Coprocessor devnet environment")
                .wait_with_output()
                .expect("Failed to complete git status check");

            if docker_status.status.success() {
                spinner.finish_and_clear();
                println!(
                    "✅ {}",
                    "Cartesi-Coprocessor devnet environment started.".green()
                )
            } else {
                spinner.finish_and_clear();
                eprintln!(
                    "{} \n{}",
                    "❌ Failed to start devnet containers:".red(),
                    String::from_utf8_lossy(&docker_status.stderr).red()
                );
                return;
            }
        }
        None => {
            eprintln!("❌ Failed to clone Cartesi-Coprocessor repository.");
            return;
        }
    }
}

fn clone_coprocessor_repo() -> Option<String> {
    // Get the directory where the CLI executable resides
    let exe_dir = std::env::current_exe()
        .expect("Failed to get the path of the current executable")
        .parent()
        .expect("Failed to get the parent directory of the executable")
        .to_path_buf();

    // Path to the cartesi-coprocessor folder relative to the CLI tool's directory
    let copro_path = exe_dir.join("cartesi-coprocessor-repo");
    let path = copro_path
        .to_str()
        .expect("Unable to decode path to co-processor")
        .to_string();

    // Check if the folder exists
    if !copro_path.exists() {
        println!(
            "Creating directory for Cartesi-Coprocessor at {:?}",
            copro_path
        );
        fs::create_dir_all(&copro_path)
            .expect("Failed to create directory for Cartesi-Coprocessor");
    }

    // Check if the repository is already cloned
    let git_dir = copro_path.join(".git");
    if git_dir.exists() {
        println!(
            "Cartesi-Coprocessor repository already cloned at {:?}",
            copro_path
        );
        check_git_status(path.clone());
        return Some(path);
    }

    // Clone the repository
    println!("Cloning Cartesi-Coprocessor repository...");
    let clone_status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/zippiehq/cartesi-coprocessor")
        .arg(&copro_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute git clone command")
        .wait_with_output()
        .expect("Failed to complete repository cloning");

    if clone_status.status.success() {
        println!(
            "✅ {} {:?}",
            "Successfully cloned Cartesi-Coprocessor repository into".green(),
            format!("{:?}", copro_path).green()
        );
        update_submodules(path.clone());
        return Some(path.clone());
    } else {
        eprintln!("❌ Failed to clone Cartesi-Coprocessor repository.");
        let stderr = String::from_utf8_lossy(&clone_status.stderr);
        println!("{} {}", "GIT::RESPONSE::".red(), stderr.red());
        return None;
    }
}

fn check_git_status(path: String) {
    let status_output = Command::new("git")
        .arg("status")
        .current_dir(path.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute git status command")
        .wait_with_output()
        .expect("Failed to complete git status check");

    if status_output.status.success() {
        let response = String::from_utf8_lossy(&status_output.stdout);
        if response.contains("Your branch is behind 'origin/main'") {
            println!("🔄 Updates are available. Pulling latest changes...");
            pull_latest_changes(path);
        } else {
            println!("Cartesi-Coprocessor repository is up to date")
        }
    } else {
        eprintln!(
            "❌ Failed to check repository status: {}",
            String::from_utf8_lossy(&status_output.stderr)
        );
        return;
    }
}

fn pull_latest_changes(path: String) {
    let pull_status = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg("main")
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute git pull command")
        .wait_with_output()
        .expect("Failed to complete git pull");

    if pull_status.status.success() {
        println!(
            "✅ {}",
            "Successfully pulled latest changes from the 'origin/main' branch.".green()
        );
    } else {
        eprintln!("❌ Failed to pull latest changes from the 'origin/main' branch.");
        let stderr = String::from_utf8_lossy(&pull_status.stderr);
        println!("{} {}", "GIT::RESPONSE::".red(), stderr.red());
    }
}

fn update_submodules(path: String) {
    let mut update_status = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute git submodule update command");

    let stdout = BufReader::new(
        update_status
            .stdout
            .take()
            .expect("Failed to capture stdout"),
    );
    let stderr = BufReader::new(
        update_status
            .stderr
            .take()
            .expect("Failed to capture stderr"),
    );
    // Handle output in separate threads
    thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                println!("{} {}", "GIT:: ".green(), line.green());
            }
        }
    });

    let start = time::Instant::now();
    thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                eprintln!("{} {}", "GIT::NOTE::".yellow(), line.yellow());
            } else if let Err(e) = line {
                eprintln!("{} {}", "GIT::ERROR::".red(), e);
            }
        }
    });

    while start.elapsed().as_secs() < 30000 {
        if let Some(status) = update_status
            .try_wait()
            .expect("Failed to update submodules")
        {
            if status.success() {
                println!("✅  Successfully updated submodules.");
                return;
            } else {
                eprintln!("❌ Failed to update submodules.");
                return;
            }
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}

pub fn stop_devnet() {
    let coprocessor_path = clone_coprocessor_repo();

    match coprocessor_path {
        Some(path) => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            spinner.set_message("Stoping devnet containers...");

            // Start the spinner
            spinner.enable_steady_tick(Duration::from_millis(100));

            // Run Cartesi-Coprocessor in the background
            let docker_status = Command::new("docker")
                .arg("compose")
                .arg("-f")
                .arg("docker-compose-devnet.yaml")
                .arg("down")
                .arg("-v")
                .current_dir(path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start Cartesi-Coprocessor devnet environment")
                .wait_with_output()
                .expect("Failed to complete git status check");

            if docker_status.status.success() {
                spinner.finish_and_clear();
                println!(
                    "✅ {}",
                    "Cartesi-Coprocessor devnet environment stoped.".green()
                )
            } else {
                spinner.finish_and_clear();
                eprintln!(
                    "{} \n{}",
                    "❌ Failed to stop devnet containers:".red(),
                    String::from_utf8_lossy(&docker_status.stderr).red()
                );
                return;
            }
        }
        None => {
            eprintln!("❌ Failed to clone Cartesi-Coprocessor repository.");
            return;
        }
    }
}
