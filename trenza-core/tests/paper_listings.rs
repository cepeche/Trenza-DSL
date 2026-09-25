// Los listados .trz que aparecen en el paper (paper/onward2027/listings/)
// se verifican aquí con el compilador real. Si un listado deja de producir
// exactamente los diagnósticos que el texto afirma, este test falla.
//
// Motivo: la revisión de Onward! 2026 señaló que el paper no mostraba
// Trenza y que sus afirmaciones de verificación no eran comprobables.
// Cada afirmación del paper sobre un listado concreto debe tener aquí
// su contrapartida ejecutable.

use std::fs;
use std::path::PathBuf;
use trenza_core::{parser, validator};

fn listing(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../paper/onward2027/listings")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {}", path.display(), e))
}

/// Códigos de diagnóstico que produce el verificador, ordenados.
fn codes(name: &str) -> Vec<String> {
    let program = parser::parse_file(&listing(name)).expect("el listado debe parsear");
    let mut codes = match validator::verify(&program) {
        Ok(()) => vec![],
        Err(diags) => diags.into_iter().map(|d| d.code).collect(),
    };
    codes.sort();
    codes
}

#[test]
fn listado_1_modos_verifica_sin_diagnosticos() {
    assert_eq!(codes("modos.trz"), Vec::<String>::new());
}

#[test]
fn listado_2_olvido_produce_exactamente_un_error_de_completitud() {
    assert_eq!(codes("modos_olvido.trz"), vec!["completeness"]);
}

/// Limitación conocida, documentada en el paper: `role *: ignored` exime
/// al contexto de las Reglas 1 y 5, de modo que el mismo olvido del
/// listado 2 pasa sin diagnóstico. Si se cambia el diseño del comodín,
/// este test debe actualizarse junto con el texto del paper.
#[test]
fn listado_3_comodin_silencia_el_olvido() {
    assert_eq!(codes("modos_comodin.trz"), Vec::<String>::new());
}
