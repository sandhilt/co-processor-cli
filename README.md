# Cartesi Co-Processor CLI Tool

## Overview

The Cartesi Co-Processor CLI Tool simplifies bootstrapping, registering, and deploying Cartesi co-processor programs. This tool streamlines the development workflow for Cartesi-based applications, allowing developers to focus on building their logic instead of wrestling with setup and deployment processes.

## Features

- **Bootstrap Projects**: Quickly initialize a Cartesi project with a simple Foundry project inheriting the co-processor base contracts.
- **Simplified Registration**: Easily build, generate .car files and finally update the co-processor task issuer with your program/machine details.
- **Simplified Deployment**: Automate the deployment and registration of co-processor programs.

## Installation

### Using Cargo

Ensure you have Rust and Cargo installed. You can then install the CLI tool directly from crates.io:

```bash
cargo install cartesi-coprocessor
```

### From Source

Clone the repository and build the tool manually:

```bash
git clone https://github.com/Mugen-Builders/co-processor-cli
cd cartesi-cli-tool
cargo install --path .
```

## Usage

Run the CLI tool with:

```bash
cartesi-cli [COMMAND] [OPTIONS]
```

### Commands

#### Bootstrap a Project

Initialize a new Foundry project with Cartesi base contracts:

```bash
coprocessor-cli create --dapp-name <project_name> --template <language template>
```

#### Register a Co-Processor Program

Register your Cartesi co-processor program:

```bash
coprocessor-cli register --email <w3 storage account email>
```

## Example Workflow

1. **Bootstrap a Project**

```bash
coprocessor-cli create --dapp-name my-cartesi-project --template rust
cd my-cartesi-project
```

2. **Add Logic to the Child Contract**
   Edit the pre-generated child contract in the `src` folder to implement your business logic.

3. **Register the Program**

```bash
coprocessor-cli register --email test@gmail.com
```

## Contributing

We welcome contributions! To get started:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-name`
3. Make your changes and test them.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Support

If you encounter issues or have feature requests, please open an issue on [GitHub](https://github.com/Mugen-Builders/co-processor-cli/issues).
Or drop a message on Cartesi discord under the Co-Procesor thread.

---
