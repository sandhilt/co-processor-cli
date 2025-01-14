mod commands;
mod helpers;
use crate::commands::create::create;
use crate::commands::register::{devnet_register, mainnet_register};
use crate::helpers::helpers::check_dependencies_installed;
use clap::{Parser, Subcommand, ValueEnum};
use enum_iterator::{all, Sequence};
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
                check_deployment_environment(network, email);
                // register(email);
                Ok(())
            }
            Commands::Create {
                template,
                dapp_name,
            } => {
                create(dapp_name, template);
                Ok(())
            }
        },
    }
}

fn check_deployment_environment(network: String, email: String) {
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
