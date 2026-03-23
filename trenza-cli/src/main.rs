pub mod ast;
pub mod parser;
pub mod generator;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;
use std::env;

#[derive(Parser)]
#[grammar = "trenza.pest"]
pub struct TrenzaParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: trenza-cli [generate] [--profile=pro|pre] <archivo.trz>");
        std::process::exit(1);
    }
    
    let mut is_generate = false;
    let mut profile = "pre".to_string();
    let mut filepath = "".to_string();

    for arg in &args[1..] {
        if arg == "generate" {
            is_generate = true;
        } else if arg.starts_with("--profile=") {
            profile = arg.split('=').nth(1).unwrap().to_string();
        } else {
            filepath = arg.to_string();
        }
    }

    if filepath.is_empty() {
        eprintln!("Debe especificar un archivo .trz");
        std::process::exit(1);
    }

    let mut unparsed_file = fs::read_to_string(&filepath).expect("No se pudo leer el archivo");
    unparsed_file = unparsed_file.trim_start_matches('\u{feff}').to_string();

    match parser::parse_file(&unparsed_file) {
        Ok(ast) => {
            println!("✅ Archivo '{}' parseado correctamente.", filepath);
            if is_generate {
                let rust_code = generator::generate_rust(&ast, &profile);
                let out_file = format!("{}_out.rs", filepath);
                fs::write(&out_file, rust_code).expect("Unable to write file");
                println!("- ✅ Código Strand 1 (Rust) generado en: {}", out_file);
            } else {
                println!("- IEFBR14 completado: El programa es válido.");
                println!("- El AST generado es una hebra limpia.");
            }
        },
        Err(e) => {
            eprintln!("❌ Error de sintaxis en '{}':\n{}", filepath, e);
            std::process::exit(1);
        }
    }
}
