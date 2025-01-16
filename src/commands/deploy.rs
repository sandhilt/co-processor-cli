use colored::Colorize;
use std::process::{Command, Stdio};

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
                    .arg(rpc)
                    .arg("--private-key")
                    .arg(private_key)
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
            } else {
                eprintln!("❌ Failed to deploy contract with Forge.");
                let stderr = String::from_utf8_lossy(&forge_status.stderr);
                eprintln!("Error: {}", stderr);
            }
        }
        None => deploy_without_args(private_key, rpc, contract_name),
    }
}

fn deploy_without_args(private_key: String, rpc: String, contract_name: String) {
    let forge_status = Command::new("forge")
        .arg("create")
        .arg(contract_name)
        .arg("--rpc-url")
        .arg(rpc)
        .arg("--private-key")
        .arg(private_key)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute forge deploy command")
        .wait_with_output()
        .expect("Failed to wait for forge command to finish");

    if forge_status.status.success() {
        let stdout = String::from_utf8_lossy(&forge_status.stdout);
        println!("{} {}", "FORGE::RESPONSE::".green(), stdout.green());
    } else {
        eprintln!("❌ Failed to deploy contract with Forge.");
        let stderr = String::from_utf8_lossy(&forge_status.stderr);
        eprintln!("Error: {}", stderr);
    }
}
