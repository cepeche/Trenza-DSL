pub mod ast;
pub mod parser;
pub mod generator;
pub mod validator;

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
            println!("✅ Archivo '{}' leido y parseado.", filepath);
            
            match validator::verify(&ast) {
                Ok(_) => {
                    println!("- ✅ Verificación Semántica: Superada impecablemente.");
                    if is_generate {
                        let rust_code = generator::generate_rust(&ast, &profile);
                        let mermaid_code = generator::generate_mermaid(&ast);
                        let audit_doc = generator::generate_audit(&ast);

                        let out_rust = format!("{}_out.rs", filepath);
                        let out_mermaid = format!("{}_out.mermaid", filepath);
                        let out_audit = format!("{}_out_audit.md", filepath);

                        fs::write(&out_rust, rust_code).expect("No se pudo escribir el archivo de salida Rust");
                        fs::write(&out_mermaid, mermaid_code).expect("No se pudo escribir el archivo de salida Mermaid");
                        fs::write(&out_audit, audit_doc).expect("No se pudo escribir el informe de auditoría");

                        println!("- ✅ Código Strand 1 (Rust) generado en: {}", out_rust);
                        println!("- ✅ Código Strand 3 (Mermaid) generado en: {}", out_mermaid);
                        println!("- ✅ Informe Strand 4 (Auditoría) generado en: {}", out_audit);
                    } else {
                        println!("- IEFBR14 completado: El programa es válido y la hebra es limpia.");
                    }
                },
                Err(errores) => {
                    eprintln!("❌ Verificación Semántica Fallida:");
                    for err in errores {
                        eprintln!("  {}", err);
                    }
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Error de sintaxis en '{}':\n{}", filepath, e);
            std::process::exit(1);
        }
    }
}
