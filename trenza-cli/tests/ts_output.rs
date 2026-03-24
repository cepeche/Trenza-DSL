/// Integration test: los .ts generados deben pasar tsc --strict --noEmit
///
/// Invariante: para cualquier spec .trz de referencia, el TS generado
/// no debe tener errores de tipos bajo tsc --strict.
///
/// Ejecutar: cargo test --test ts_output
/// Requisito: npx disponible en PATH (Node.js instalado)

use std::process::Command;
use std::path::Path;

fn tsc_strict(ts_path: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .join(ts_path);

    assert!(path.exists(), "Archivo no encontrado: {}", path.display());

    // En Windows el shebang de npx requiere .cmd
    let npx = if cfg!(windows) { "npx.cmd" } else { "npx" };

    let status = Command::new(npx)
        .args(["tsc", "--strict", "--noEmit", "--target", "ES2020"])
        .arg(&path)
        .status()
        .expect("No se pudo ejecutar npx. ¿Node.js instalado?");

    assert!(
        status.success(),
        "tsc --strict falló en: {}",
        path.display()
    );
}

#[test]
fn cronometro_psp_ts_pasa_tsc() {
    tsc_strict("spec/reference/cronometro-psp/generated/CronometroPSP_out.ts");
}

#[test]
fn trenza_cli_ts_pasa_tsc() {
    tsc_strict("spec/reference/trenza-cli/generated/CLI_Trenza_out.ts");
}
