pub mod ast;
pub mod parser;
pub mod validator;
pub mod generator;
pub mod runtime;
pub mod interpreter;
pub mod pub_surface;

#[cfg(feature = "wasm")]
pub mod wasm;

#[macro_use]
extern crate pest_derive;


#[derive(Parser)]
#[grammar = "trenza.pest"]
pub struct TrenzaParser;
