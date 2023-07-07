use proc_macro2::TokenStream;
use quote::ToTokens;
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
    let ast = match syn::parse_file(&content) {
        Ok(ast) => ast,
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err.to_string(),
            ))
        }
    };

    collector
        .0
        .push(format!("\n//### File: {:?}", path.file_name().unwrap()));
    collector.visit_file(&ast);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut collector = Collector(Vec::new());

    for entry in WalkDir::new("./demo") {
        let entry = entry.expect("Error walking the directory");
        if entry.path().extension().unwrap_or_default() == "rs" {
            process_file(entry.path(), &mut collector)?;
        }
    }

    let output_file = Path::new("output.rs");
    let mut file = fs::File::create(&output_file)?;
    file.write_all(collector.0.join("\n").as_bytes())?;

    Ok(())
}
