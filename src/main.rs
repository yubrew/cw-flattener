use quote::ToTokens;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use syn::{visit::Visit, Item};

struct Collector(Vec<String>);

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
        if entry.path().extension().unwrap() == "rs" {
            let content = fs::read_to_string(&entry.path()).unwrap();
            let ast = syn::parse_file(&content).unwrap();
            collector.visit_file(&ast);
        }
    }

    let output_file = PathBuf::from("output.rs");
    let mut file = fs::File::create(&output_file).unwrap();
    file.write_all(collector.0.join("\n").as_bytes()).unwrap();
}
