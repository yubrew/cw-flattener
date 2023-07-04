use quote::ToTokens;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use syn::{visit::Visit, Item};

struct Collector(Vec<String>);

impl Collector {
    fn push_str(&mut self, s: &str) {
        self.0.push(s.to_owned());
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, item: &'ast Item) {
        let mut tokens = proc_macro2::TokenStream::new();
        item.to_tokens(&mut tokens);
        self.0.push(tokens.to_string());
        syn::visit::visit_item(self, item);
    }
}

fn main() {
    let mut collector = Collector(Vec::new());

    let entries = fs::read_dir("./demo").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        if entry.path().extension().is_some() && entry.path().extension().unwrap() == "rs" {
            print!("entry: {:?}\n", entry.file_name().to_ascii_lowercase());
            let file_name = entry.file_name().to_string_lossy().into_owned();
            collector.push_str(&format!("\n// File: {}", file_name));

            let content = fs::read_to_string(&entry.path()).unwrap();
            let ast = syn::parse_file(&content).unwrap();
            collector.visit_file(&ast);
        }
    }

    let output_file = PathBuf::from("output.rs");
    let mut file = fs::File::create(&output_file).unwrap();
    file.write_all(collector.0.join("\n").as_bytes()).unwrap();
}

// let path = entry.path();
// let file_name = path.file_name().unwrap().to_str().unwrap();
// content = format!("// File Name: {}\n{}", file_name, content);
