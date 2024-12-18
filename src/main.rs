mod commands;
mod helpers;
use crate::commands::create::create;
use crate::commands::register::register;
use crate::helpers::helpers::check_dependencies_installed;
use clap::{Parser, Subcommand};
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match check_dependencies_installed() {
        false => Ok(()),
        true => match cli.command {
            Commands::Register { email } => {
                println!("Registering progam with co-processor...");
                register(email);
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
