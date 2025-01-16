mod commands;
mod helpers;
use crate::commands::create::create;
use crate::helpers::helpers::check_dependencies_installed;
use clap::{Parser, Subcommand};
use helpers::helpers::{check_deployment_environment, check_registration_environment};
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
    #[command(
        about = "Build and run all necessary steps to register your program with co-processor"
    )]
    Register {
        #[arg(short, long, help = "Your email address registered with Web3.Storage")]
        email: String,

        #[arg(
            short,
            long,
            help = "Environment where your program will be deployed to, e.g. Devnet, Mainnet or Testnet"
        )]
        network: String,
    },
    #[command(
        about = "Bootstrap a new directiry for your program",
        long_about = "Bootstrap a new directiry for your coprocessor program, this would contain both the cartesi template and also the solidity template"
    )]
    Create {
        #[arg(short, long, help = "Name of your program")]
        dapp_name: String,

        #[arg(short, long, help = "Language you intend to build with")]
        template: String,
    },

    #[command(
        about = "Deploy the solidity code for your coprocessor program to any network of choice.",
        long_about = "Deploy the solidity code for your coprocessor program to any network of choice, by running the default deploy script (Deploy.s.sol)"
    )]
    Deploy {
        #[arg(short, long, help = "Name of your contract file")]
        contract_name: String,

        #[arg(
            short,
            long,
            help = "Environment where your program will be deployed to, e.g. Devnet, Mainnet or Testnet"
        )]
        network: String,

        #[arg(short, long, help = "Private key for deploying to selected network")]
        private_key: Option<String>,

        #[arg(short, long, help = "RPC for deploying to network of choice")]
        rpc: Option<String>,

        #[arg(
        short = 'a',
        long,
        help = "Constructor arguments to pass to the contract",
        num_args = 0..,
        value_delimiter = ' '
        )]
        constructor_args: Option<Vec<String>>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match check_dependencies_installed() {
        false => Ok(()),
        true => match cli.command {
            Commands::Register { email, network } => {
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
                contract_name,
                network,
                private_key,
                rpc,
                constructor_args,
            } => {
                // let contract_path = check_contract_exists(&file_name);
                check_deployment_environment(
                    network,
                    private_key,
                    rpc,
                    constructor_args,
                    contract_name,
                );
                Ok(())
            }
        },
    }
}
