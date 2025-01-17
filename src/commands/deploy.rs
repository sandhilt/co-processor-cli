use chrono::Local;
use colored::Colorize;
use serde_json::json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// @notice Function to deploy a smart contract with constructor arguments using Forge
/// @param private_key The private of thye account to deploy with
/// @param rpc The rpc of the chain to deploy to
/// @param constructor_args Constructor arguments for the smart contract
/// @param contract_name The name of the smart contract
pub fn deploy_contract(
    private_key: String,
    rpc: String,
    constructor_args: Option<Vec<String>>,
    contract_name: String,
) {
    match constructor_args {
        Some(args) => {
            let forge_status = {
                let mut command = Command::new("forge");
                command
                    .arg("create")
                    .arg(contract_name)
                    .arg("--rpc-url")
                    .arg(rpc.clone())
                    .arg("--private-key")
                    .arg(private_key)
                    .arg("--broadcast")
                    .arg("--constructor-args");

                // Add the constructor arguments dynamically
                for arg in args {
                    command.arg(arg);
                }

                command
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute forge deploy command")
                    .wait_with_output()
                    .expect("Failed to wait for forge command to finish")
            };

            if forge_status.status.success() {
                let stdout = String::from_utf8_lossy(&forge_status.stdout);
                println!("{} {}", "FORGE::RESPONSE::".green(), stdout.green());
                register_deployment(stdout.to_string(), rpc);
            } else {
                eprintln!("❌ Failed to deploy contract with Forge.");
                let stderr = String::from_utf8_lossy(&forge_status.stderr);
                if stderr.contains("error sending request for url (http://127.0.0.1:8545/)") {
                    println!("Please ensure you have a devnet environment. Run the stop and start devnet commands.");
                } else {
                    eprintln!("Error: {}", stderr);
                }
            }
        }
        None => deploy_without_args(private_key, rpc, contract_name),
    }
}

// @notice Function to deploy a smart contract without constructor arguments using Forge
/// @param private_key The private of thye account to deploy with
/// @param rpc The rpc of the chain to deploy to
/// @param contract_name The name of the smart contract
fn deploy_without_args(private_key: String, rpc: String, contract_name: String) {
    let forge_status = Command::new("forge")
        .arg("create")
        .arg(contract_name)
        .arg("--rpc-url")
        .arg(rpc.clone())
        .arg("--private-key")
        .arg(private_key)
        .arg("--broadcast")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute forge deploy command")
        .wait_with_output()
        .expect("Failed to wait for forge command to finish");

    if forge_status.status.success() {
        let stdout = String::from_utf8_lossy(&forge_status.stdout);
        println!("{} {}", "FORGE::RESPONSE::".green(), stdout.green());
        register_deployment(stdout.to_string(), rpc);
    } else {
        eprintln!("❌ Failed to deploy contract with Forge.");
        let stderr = String::from_utf8_lossy(&forge_status.stderr);
        if stderr.contains("error sending request for url (http://127.0.0.1:8545/)") {
            println!("Please ensure you have a devnet environment. Run the stop and start devnet commands.");
        } else {
            eprintln!("Error: {}", stderr);
        }
    }
}

/// @notice Function to register each deployment handled by the cli
/// @param deployment data This contains deployment logs and info from foundry after a deploymentß
/// @param rpc The rpc of the chain deployed to
fn register_deployment(deployment_data: String, rpc: String) {
    let exe_dir = env::current_dir().expect("Failed to get current directory");

    // Path to the cartesi-coprocessor folder relative to the CLI tool's directory
    let copro_path = exe_dir.join("deployment_history");
    let path = copro_path
        .to_str()
        .expect("Unable to decode path to deployment history")
        .to_string();

    // Check if the folder exists
    if !copro_path.exists() {
        println!(
            "Creating directory to record deployments at {:?}",
            copro_path
        );
        fs::create_dir_all(&copro_path).expect("Failed to create directory for deployment history");
    }
    match save_deployment_info(&deployment_data, &path, rpc) {
        Ok(_) => println!("✅ {}", "Deployment info saved successfully.".green()),
        Err(err) => eprintln!("❌ Error saving deployment info: {}", err),
    }
}

/// @notice Function to store a log for each deployment handled by the cli
/// @param log data This contains deployment logs and info from foundry after a deployment
/// @param path This is the path to the folser where the logs are stored
/// @param rpc The rpc of the chain deployed to
/// @return An option containing the status of the process
fn save_deployment_info(log: &str, path: &str, rpc: String) -> std::io::Result<()> {
    // Extract relevant information using simple string parsing
    let deployer = log
        .lines()
        .find(|line| line.starts_with("Deployer:"))
        .map(|line| line.replace("Deployer: ", "").trim().to_string())
        .unwrap_or_default();

    let deployed_to = log
        .lines()
        .find(|line| line.starts_with("Deployed to:"))
        .map(|line| line.replace("Deployed to: ", "").trim().to_string())
        .unwrap_or_default();

    let transaction_hash = log
        .lines()
        .find(|line| line.starts_with("Transaction hash:"))
        .map(|line| line.replace("Transaction hash: ", "").trim().to_string())
        .unwrap_or_default();

    // Get the current date and time for the file title
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    // Create the JSON object
    let deployment_info = json!({
        "deployer": deployer,
        "deployed_to": deployed_to,
        "rpc_url": rpc,
        "transaction_hash": transaction_hash,
    });

    // Define the file path and name
    let file_path = Path::new(path).join(format!("deployment_{}.json", timestamp));

    // Write the JSON to a file
    let mut file = File::create(file_path)?;
    file.write_all(deployment_info.to_string().as_bytes())?;

    Ok(())
}
