use proc_macro2::TokenStream;
use quote::ToTokens;
use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use syn::{visit::Visit, Item};
use walkdir::WalkDir;

struct Collector(Vec<String>);

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, item: &'ast Item) {
        let mut tokens = TokenStream::new();
        item.to_tokens(&mut tokens);
        self.0.push(tokens.to_string());
    }
}

fn process_file(path: &Path, collector: &mut Collector) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;

    // Append the file name as a comment for clarity in the output.
    collector
        .0
        .push(format!("\n//### File: {:?}", path.file_name().unwrap()));

    // Check if the file is a Rust source file or a TOML file
    match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("rs") => {
            // Parse the Rust source file and collect its tokens.
            let ast = syn::parse_file(&content).map_err(|err| {
                eprintln!("Error parsing Rust file {:?}: {}", path.display(), err);
                std::io::Error::new(std::io::ErrorKind::InvalidData, "lex error")
            })?;
            collector.visit_file(&ast);
        }
        _ => {
            // If the file extension is not .rs, log it and skip.
            eprintln!("Unsupported file extension: {:?}", path);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut collector = Collector(Vec::new());
    // Precompile the regex outside of the loop
    let end_with_pattern = Regex::new(r"(\.rs)$").expect("Invalid regex pattern");
    let exclude_pattern = Regex::new(r"(schema\.rs|tests\.rs)$").expect("Invalid regex pattern");

    for entry in WalkDir::new("./demo") {
        let entry = entry.expect("Error walking the directory");
        let entry_path = entry.path();

        // Check if it's a file before processing
        if entry.file_type().is_file() {
            if let Some(path_str) = entry_path.to_str() {
                if end_with_pattern.is_match(path_str) && !exclude_pattern.is_match(path_str) {
                    process_file(entry_path, &mut collector)?; // Use ? to propagate errors
                }
            } else {
                eprintln!("Path contains invalid Unicode: {:?}", entry_path);
            }
        }
    }

    let output_file = Path::new("output.rs");
    let mut file = fs::File::create(&output_file)?;
    file.write_all(collector.0.join("\n").as_bytes())?;

    Ok(())
}
