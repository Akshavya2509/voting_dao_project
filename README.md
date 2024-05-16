# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── voting_dao
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

# VotingContract Smart Contract
This project contains a smart contract written in Rust using the soroban_sdk for the Soroban blockchain platform. The smart contract implements a voting system where users can cast votes for candidates, and the contract keeps track of total votes and votes per candidate.

- Features

Allows users to register and vote for candidates.
Maintains a record of total votes and votes per candidate.
Provides functions to retrieve voting statistics.

# Getting Started
- Prerequisites
Before running or testing this smart contract, ensure you have the following installed:

Rust and Cargo (Rust's package manager)
Soroban SDK
Installation

- Clone this repository to your local machine:
```
git clone <repository_url>
```
- Navigate to the project directory:
```
cd VotingContract
```
# Usage
To use this smart contract, follow these steps:

- Import the Contract:

Include the VotingContract in your Soroban application.

- Register Voters:

Use the register_voter function to register users before allowing them to vote.

- Cast Votes:

Use the vote function to allow registered users to cast their votes for specific candidates.

- Testing
To test the smart contract, run the following command:

```
cargo test
```
This command executes the test suite defined in the test.rs file, which includes tests for each contract function.

