# Flatten Cosmwasm Smart Contracts into One File

## Utility to combine all source files to one flattened .rs file

Large Language Models (LLM) like GPT-4 ingest single files for better results.

Flattens by combining all \*.rs files into one file. Does not import Cargo.toml dependencies.

## Config

Copy paste repo into demo folder.
`cargo run` to generate flattened source file to `output.rs`.

Original inspiration https://github.com/poanetwork/solidity-flattener
