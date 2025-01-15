mod commands;
mod helpers;
use crate::commands::create::create;
use crate::commands::deploy::deploy_contract;
use crate::commands::register::{devnet_register, mainnet_register};
use crate::helpers::helpers::check_dependencies_installed;
use clap::{Parser, Subcommand};
use colored::Colorize;
use enum_iterator::{all, Sequence};
use helpers::helpers::check_deploymet_args;
use std::error::Error;

/// A CLI tool to interact with Web3.Storage
#[derive(Parser)]
#[command(author = "Idogwu Chinonso", version, about = "Bootstrap and deploy cartesi coprocesor programs easily from your CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build and run all necessary steps to register your program with co-processor
    #[command(
        about = "Build and run all necessary steps to register your program with co-processor"
    )]
    Register {
        /// Email address for logging in
        #[arg(short, long, help = "Your email address registered with Web3.Storage")]
        email: String,

        /// Network your program will be deplyed to
        #[arg(
            short,
            long,
            help = "Environment where your program will be deployed to, e.g. Devnet, Mainnet or Testnet"
        )]
        network: String,
    },
    /// Bootstrap a new directiry for your coprocessor program
    #[command(
        about = "Bootstrap a new directiry for your program",
        long_about = "Bootstrap a new directiry for your coprocessor program, this would contain both the cartesi template and also the solidity template"
    )]
    Create {
        /// Name of your program
        #[arg(short, long, help = "Name of your program")]
        dapp_name: String,

        /// Language you intend to build with
        #[arg(short, long, help = "Language you intend to build with")]
        template: String,
    },

    /// Deploy the solidity code for your coprocessor program to any network of choice.
    #[command(
        about = "Deploy the solidity code for your coprocessor program to any network of choice.",
        long_about = "Deploy the solidity code for your coprocessor program to any network of choice, by running the default deploy script (Deploy.s.sol)"
    )]
    Deploy {
        /// Network your program will be deplyed to
        #[arg(
            short,
            long,
            help = "Environment where your program will be deployed to, e.g. Devnet, Mainnet or Testnet"
        )]
        network: String,

        /// Private key for deploying to selected network
        #[arg(short, long, help = "Private key for deploying to selected network")]
        private_key: Option<String>,

        /// RPC for deploying to network of choice
        #[arg(short, long, help = "RPC for deploying to network of choice")]
        rpc: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Sequence)]
enum DeploymentOptions {
    Devnet,
    Testnet,
    Mainnet,
}

impl DeploymentOptions {
    fn to_string(&self) -> String {
        match self {
            DeploymentOptions::Devnet => "Devnet".to_string(),
            DeploymentOptions::Testnet => "Testnet".to_string(),
            DeploymentOptions::Mainnet => "Mainnet".to_string(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match check_dependencies_installed() {
        false => Ok(()),
        true => match cli.command {
            Commands::Register { email, network } => {
                println!("Registering progam with co-processor...");
                check_registration_environment(network, email);
                Ok(())
            }
            Commands::Create {
                template,
                dapp_name,
            } => {
                create(dapp_name, template);
                Ok(())
            }
            Commands::Deploy {
                network,
                private_key,
                rpc,
            } => {
                check_deployment_environment(network, private_key, rpc);
                Ok(())
            }
        },
    }
}

fn check_registration_environment(network: String, email: String) {
    let mut environment: Option<DeploymentOptions> = None;

    for option in all::<DeploymentOptions>().collect::<Vec<_>>() {
        if network.to_lowercase() == option.to_string().to_lowercase() {
            println!(
                "Deployment environment {} is available",
                option.to_string().to_lowercase()
            );

            environment = Some(option);
        }
    }

    if environment.is_none() {
        println!(
            "{}",
            "Invalid network environment, please select either, devnet, mainnet, or testnet".red()
        );
        return;
    }

    if let Some(deployment_env) = environment {
        match deployment_env {
            DeploymentOptions::Devnet => {
                devnet_register(email);
            }
            DeploymentOptions::Testnet => {
                println!("Sorry Testnet integration is not available at the moment!!",);
            }
            DeploymentOptions::Mainnet => {
                mainnet_register(email);
            }
        }
    }
}

fn check_deployment_environment(network: String, private_key: Option<String>, rpc: Option<String>) {
    match network.to_lowercase().as_str() {
        "mainnet" => {
            if check_deploymet_args(&network, private_key.clone(), rpc.clone()) {
                return;
            } else {
                deploy_contract(private_key.unwrap(), rpc.unwrap());
            }
        }
        "testnet" => {
            if check_deploymet_args(&network, private_key.clone(), rpc.clone()) {
                return;
            } else {
                deploy_contract(private_key.unwrap(), rpc.unwrap());
            }
        }
        "devnet" => {
            deploy_contract(
                String::from("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"),
                String::from("http://127.0.0.1:8545"),
            );
        }
        _ => {
            println!(
                "{}",
                "Invalid network environment, please select either, devnet, mainnet, or testnet"
                    .red()
            );
            return;
        }
    }
}
