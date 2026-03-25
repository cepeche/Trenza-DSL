use wasm_bindgen::prelude::*;
use crate::parser;
use crate::validator;
use crate::ast;
use crate::interpreter::Interpreter;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn verify_wasm(source: &str) -> JsValue {
    internal_verify_single(source)
}

#[wasm_bindgen]
pub fn verify_project_wasm(files_js: JsValue) -> JsValue {
    let files: HashMap<String, String> = serde_wasm_bindgen::from_value(files_js).unwrap_or_default();
    let mut all_definitions = Vec::new();
    let mut all_errors = Vec::new();

    for (filename, content) in files {
        match parser::parse_file(&content) {
            Ok(program) => {
                all_definitions.extend(program.definitions);
            },
            Err(e) => {
                let diag = pest_to_diagnostic(e, &filename);
                all_errors.push(diag);
            }
        }
    }

    if !all_errors.is_empty() {
        return serde_wasm_bindgen::to_value(&all_errors).unwrap();
    }

    let program_ast = ast::Program { definitions: all_definitions };
    match validator::verify(&program_ast) {
        Ok(_) => serde_wasm_bindgen::to_value(&Vec::<validator::Diagnostic>::new()).unwrap(),
        Err(diags) => serde_wasm_bindgen::to_value(&diags).unwrap(),
    }
}

#[wasm_bindgen]
pub struct InterpreterWasm {
    inner: Interpreter,
}

#[wasm_bindgen]
impl InterpreterWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<InterpreterWasm, JsValue> {
        match parser::parse_file(source) {
            Ok(program) => Ok(InterpreterWasm { inner: Interpreter::new(program) }),
            Err(e) => Err(JsValue::from_str(&format!("{}", e))),
        }
    }

    pub fn dispatch(&mut self, event: &str, payload: &str) -> JsValue {
        let res = self.inner.dispatch(event, payload);
        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.state).unwrap()
    }
}

fn internal_verify_single(source: &str) -> JsValue {
    match parser::parse_file(source) {
        Ok(program) => {
            match validator::verify(&program) {
                Ok(_) => serde_wasm_bindgen::to_value(&Vec::<validator::Diagnostic>::new()).unwrap(),
                Err(diags) => serde_wasm_bindgen::to_value(&diags).unwrap(),
            }
        },
        Err(e) => {
            let diag = pest_to_diagnostic(e, "input.trz");
            serde_wasm_bindgen::to_value(&vec![diag]).unwrap()
        }
    }
}

fn pest_to_diagnostic(e: pest::error::Error<crate::Rule>, filename: &str) -> validator::Diagnostic {
    let (line, col) = match e.line_col {
        pest::error::LineColLocation::Pos((l, c)) => (l, c),
        pest::error::LineColLocation::Span((l, c), _) => (l, c),
    };
    
    validator::Diagnostic {
        span: ast::Span {
            start: ast::Pos { line, col },
            end: ast::Pos { line, col: col + 1 },
        },
        message: format!("[{}] {}", filename, e),
        severity: "error".to_string(),
        code: "syntax".to_string(),
    }
}
