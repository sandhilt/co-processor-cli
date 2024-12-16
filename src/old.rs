use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{stdin, BufRead, BufReader};
use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
};
use std::{thread, time};

/// A CLI tool to interact with Web3.Storage
#[derive(Parser)]
#[command(author = "Your Name", version, about = "Manage Web3.Storage easily from your CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Log in to Web3.Storage
    #[command(about = "Log in using your email address")]
    Login {
        /// Email address for logging in
        #[arg(short, long, help = "Your email address registered with Web3.Storage")]
        email: String,
    },
    /// Create a new storage space in Web3.Storage
    #[command(about = "Create a new named storage space")]
    CreateSpace {
        /// Name of the storage space
        #[arg(short, long, help = "A unique name for your storage space")]
        space: String,
    },
    /// Upload a CAR file to Web3.Storage
    #[command(about = "Upload a CAR file to Web3.Storage")]
    Upload {
        /// Path to the CAR file
        #[arg(short, long, help = "Path to the CAR file to be uploaded")]
        car_file: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("command: {:?}", cli.command);

    match cli.command {
        Commands::Login { email } => {
            println!("Logging into Web3.Storage...");
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
                        return Ok(());
                    } else {
                        eprintln!("Login process failed.");
                        return Ok(());
                    }
                }

                // Poll every 5 seconds
                thread::sleep(time::Duration::from_secs(5));
            }

            // If timeout occurs
            child
                .kill()
                .expect("Failed to terminate the login process.");
            eprintln!(
                "Login process timed out. Please verify the email within the specified timeout."
            );
            Ok(())
        }

        Commands::CreateSpace { space } => {
            let mut child = Command::new("w3")
                .arg("space")
                .arg("create")
                .arg(space.clone())
                .arg("--no-recovery")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute 'w3 space creation'");

            println!("one");
            let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));
            let stderr = BufReader::new(child.stderr.take().expect("Failed to capture stderr"));
            let mut stdin = child.stdin.take().expect("Failed to capture stdin");

            let start = time::Instant::now();
            println!("two");

            // Handle output in separate threads
            // thread::spawn(move || {
            //     for line in stdout.lines() {
            //         if let Ok(line) = line {
            //             println!("{} {}", "W3STORAGE:: ".green(), line.green());
            //             if line == "" {
            //                 println!("W3STORAGE:: Storage space '{}' already exists.", space);
            //             }
            //         }
            //     }
            // });

            thread::spawn(move || {
                for line in stdout.lines() {
                    match line {
                        Ok(output) => {
                            println!("Child Output: {}", output);

                            // Detect a prompt for input
                            if output.contains("enter a response") {
                                println!("Detected input request...");
                            }
                        }
                        Err(e) => eprintln!("Error reading stdout: {}", e),
                    }
                }
            });
            println!("three");

            thread::spawn(move || {
                for line in stderr.lines() {
                    if let Ok(line) = line {
                        eprintln!("{} {}", "W3STORAGE::NOTE::".yellow(), line.yellow());
                    } else if let Err(e) = line {
                        eprintln!("{} {}", "W3STORAGE::ERROR::".red(), e);
                    }
                }
            });
            println!("four");

            // let out = child.try_wait().expect("failed to wait");
            // match out {
            //     Some(status) => {
            //         if !status.success() {
            //             eprintln!("Failed to create storage space");
            //             return Ok(());
            //         }
            //     }
            //     None => {
            //         println!(
            //             "{:?} lines",
            //             BufReader::new(child.stdout.take().expect("Failed to capture stderr"))
            //                 .lines()
            //         );
            //         stdin.write_all(b"y\n").expect("Failed to write to stdin");
            //         eprintln!("Failed to create storage space 2");
            //     }
            // }
            // println!("before {:?}", out);

            // let out = child.wait_with_output().expect("failed to wait");
            // println!("{:?} test out", out);
            // child.stdin.insert(value);
            // let out = child.try_wait().expect("failed to wait");
            // println!("After {:?}", out);

            let status = child.wait()?;
            println!("Child process exited with status: {}", status);

            println!("check");
            println!("check again...");
            // Wait for email verification or timeout
            while start.elapsed().as_secs() < 100 {
                if let Some(status) = child.try_wait().expect("Failed to check process status") {
                    if status.success() {
                        println!("Successfully logged in to Web3.Storage.");
                        return Ok(());
                    } else {
                        eprintln!("Login process failed.");
                        return Ok(());
                    }
                }

                // Poll every 2 seconds
                thread::sleep(time::Duration::from_secs(2));
            }

            println!("five");

            // If timeout occurs
            child
                .kill()
                .expect("Failed to terminate the login process.");
            eprintln!(
                "Login process timed out. Please verify the email within the specified timeout."
            );

            println!("six");

            Ok(())
        }
        Commands::Upload { car_file } => {
            println!("Uploading CAR file...");
            let status = Command::new("w3")
                .arg("up")
                .arg("--car")
                .arg(car_file.clone())
                .status()
                .expect("Failed to execute 'w3 up'");

            if !status.success() {
                eprintln!("Failed to upload CAR file.");
            } else {
                println!("CAR file '{}' uploaded successfully.", car_file);
            }
            Ok(())
        }
    }
}
