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

use std::path::{Path, PathBuf};

fn get_all_trz_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_trz_files(&path));
            } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("trz") {
                files.push(path);
            }
        }
    }
    files
}

#[derive(Parser)]
#[grammar = "trenza.pest"]
pub struct TrenzaParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: trenza-cli [generate] [--profile=pro|pre] [--concurrency=composite|threads] <archivo_o_directorio>");
        std::process::exit(1);
    }
    
    let mut is_generate = false;
    let mut profile = "pre".to_string();
    let mut concurrency = "composite".to_string();
    let mut filepath = "".to_string();

    for arg in &args[1..] {
        if arg == "generate" {
            is_generate = true;
        } else if arg.starts_with("--profile=") {
            profile = arg.split('=').nth(1).unwrap().to_string();
        } else if arg.starts_with("--concurrency=") {
            concurrency = arg.split('=').nth(1).unwrap().to_string();
        } else {
            filepath = arg.to_string();
        }
    }

    if filepath.is_empty() {
        eprintln!("Debe especificar una ruta .trz o un directorio");
        std::process::exit(1);
    }

    let path = Path::new(&filepath);
    let mut all_definitions = Vec::new();

    let mut files_to_parse = if path.is_dir() {
        get_all_trz_files(path)
    } else {
        vec![path.to_path_buf()]
    };

    if files_to_parse.is_empty() {
        eprintln!("No se encontraron archivos .trz en la ruta especificada.");
        std::process::exit(1);
    }

    files_to_parse.sort_by_key(|p| {
        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
        if name == "system.trz" { 0 } else { 1 }
    });

    for file in &files_to_parse {
        let mut unparsed_file = fs::read_to_string(file).expect("No se pudo leer un archivo");
        unparsed_file = unparsed_file.trim_start_matches('\u{feff}').to_string();
        match parser::parse_file(&unparsed_file) {
            Ok(ast) => {
                println!("✅ Archivo '{}' leido y parseado.", file.display());
                all_definitions.extend(ast.definitions);
            },
            Err(e) => {
                eprintln!("❌ Error de sintaxis en '{}':\n{}", file.display(), e);
                std::process::exit(1);
            }
        }
    }

    let program_ast = ast::Program { definitions: all_definitions };

    match validator::verify(&program_ast) {
        Ok(_) => {
            println!("- ✅ Verificación Semántica: Superada impecablemente para {} archivos.", files_to_parse.len());
            if is_generate {
                let rust_code = generator::generate_rust(&program_ast, &profile, &concurrency);
                let mermaid_code = generator::generate_mermaid(&program_ast);
                let audit_doc = generator::generate_audit(&program_ast);

                let mut system_name = "project".to_string();
                for def in &program_ast.definitions {
                    if let ast::Definition::System(sys) = def {
                        system_name = sys.name.clone();
                        break;
                    }
                }

                let out_rust = format!("{}_out.rs", system_name);
                let out_mermaid = format!("{}_out.mermaid", system_name);
                let out_audit = format!("{}_out_audit.md", system_name);

                fs::write(&out_rust, rust_code).expect("No se pudo escribir el archivo Rust");
                fs::write(&out_mermaid, mermaid_code).expect("No se pudo escribir el archivo Mermaid");
                fs::write(&out_audit, audit_doc).expect("No se pudo escribir el archivo Audit");

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
}
