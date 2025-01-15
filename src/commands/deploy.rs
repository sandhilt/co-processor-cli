use colored::Colorize;
use regex::Regex;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread, time, time::Duration};

use crate::helpers::helpers::read_file;

// /// @notice Function to call the co-processor task manager to register the machine, hash, grogram cid etc.
// fn devnet_register_program_with_coprocessor() {
//     let current_dir = env::current_dir().expect("Failed to get current directory");
//     let output_cid = current_dir.join("output.cid");
//     let output_size = current_dir.join("output.size");

//     let cid = read_file(
//         output_cid
//             .to_str()
//             .expect("error converting path to string"),
//         "CID",
//     );
//     let size = read_file(
//         output_size
//             .to_str()
//             .expect("error converting path to string"),
//         "SIZE",
//     );
//     let machine_hash = get_machine_hash();

//     let curl_status = Command::new("curl")
//         .arg("-X")
//         .arg("POST")
//         .arg(format!(
//             "http://127.0.0.1:3034/ensure/{}/{}/{}",
//             cid, machine_hash, size
//         ))
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()
//         .expect("Failed to execute curl POST command")
//         .wait_with_output()
//         .expect("Failed to wait for curl command to finish");

//     if curl_status.status.success() {
//         println!(
//             "✅ {}",
//             "Successfully sent request to co-processor.".green()
//         );
//         let stdout = String::from_utf8_lossy(&curl_status.stdout);
//         println!("✅ {} {}", "RESPONSE::".green(), stdout.green());
//     } else {
//         eprintln!("Failed to send POST request.");
//         let stderr = String::from_utf8_lossy(&curl_status.stderr);
//         eprintln!("Error: {}", stderr);
//     }
// }

pub fn deploy_contract(private_key: String, rpc: String) {
    let forge_status = Command::new("forge")
        .arg("script")
        .arg("script/Deploy.s.sol:Deploy")
        .arg("--fork-url")
        .arg(rpc)
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
        extract_deployment_address(stdout.to_string());
    } else {
        eprintln!("❌ Failed to deploy contract with Forge.");
        let stderr = String::from_utf8_lossy(&forge_status.stderr);
        eprintln!("Error: {}", stderr);
    }
}

fn extract_deployment_address(response: String) {
    let chain_id_pattern = Regex::new(r"Chain (\d+)").unwrap();
    let mut chain_id: String = String::new();

    // Search for the pattern in the content
    if let Some(captures) = chain_id_pattern.captures(&response) {
        // Extract the chain ID from the first capture group
        match captures.get(1).map(|m| m.as_str().to_string()) {
            Some(chain) => chain_id = chain,
            None => eprintln!("Failed to extract chain ID from response."),
        }
    }

    if chain_id != String::new() {
        let exec_file = format!("broadcast/Deploy.s.sol/{}/run-latest.json", chain_id);
        let file_content = read_file(&exec_file, "address");
        match get_contract_address(&file_content) {
            Some(address) => println!(
                "✅ {} {}",
                "Deployed contract address:".green(),
                address.green()
            ),
            None => eprintln!(
                "❌ {}",
                "Failed to extract deployed contract address.".red()
            ),
        }
    }
}

fn get_contract_address(json_content: &str) -> Option<String> {
    let parsed: Value = serde_json::from_str(json_content).ok()?;
    parsed["transactions"]
        .get(0)?
        .get("contractAddress")?
        .as_str()
        .map(String::from)
}
