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
        eprintln!("Uso: trenza-cli <archivo.trz>");
        std::process::exit(1);
    }
    
    let filepath = &args[1];
    let unparsed_file = fs::read_to_string(filepath).expect("No se pudo leer el archivo");

    let successful_parse = TrenzaParser::parse(Rule::program, &unparsed_file);
    
    match successful_parse {
        Ok(_parsed) => {
            println!("✅ Archivo '{}' parseado correctamente.", filepath);
            println!("- IEFBR14 completado: El programa nulo (o con comentarios) se ha validado.");
            println!("- El AST generado es una hebra limpia.");
        },
        Err(e) => {
            eprintln!("❌ Error de sintaxis en '{}':\n{}", filepath, e);
            std::process::exit(1);
        }
    }
}
