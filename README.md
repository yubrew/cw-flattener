# Cosmwasm smart-contract flattened source file generation

## Utility to combine all source files to one flattened .rs file

One long file provides full context for Large Language Model (LLM) ingestion
Flattens by combining all \*.rs files into one file. It does not import Cargo.toml dependencies.

## Config

Copy paste repo into demo folder.
`cargo run` to generate flattened source file to `output.rs`.

Original inspiration https://github.com/poanetwork/solidity-flattener
