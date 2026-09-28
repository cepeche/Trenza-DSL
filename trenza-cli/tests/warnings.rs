/// `trenza-cli check`: los avisos no hacen fallar la verificación salvo con
/// `--deny-warnings` (decisión 3a, 2026-09-28).

use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static N: AtomicUsize = AtomicUsize::new(0);

fn check(args: &[&str], src: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("trenza-warn-{}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst)));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("spec.trz");
    std::fs::write(&file, src).unwrap();
    let ok = Command::new(env!("CARGO_BIN_EXE_trenza-cli"))
        .arg("check")
        .args(args)
        .arg(Path::new(&file))
        .output()
        .unwrap()
        .status
        .success();
    let _ = std::fs::remove_dir_all(&dir);
    ok
}

const CON_AVISO: &str = "
data D:
    x: Id

system S:
    initial: A
    contexts:
        A
        B

context A:
    role r: D
        on e -> go
    transitions:
        on go -> B
        on nunca -> B

context B:
    role r: D
        on e -> back
    transitions:
        on back -> A
";

#[test]
fn un_aviso_no_hace_fallar_check() {
    assert!(check(&[], CON_AVISO));
}

#[test]
fn deny_warnings_convierte_los_avisos_en_fallo() {
    assert!(!check(&["--deny-warnings"], CON_AVISO));
}

#[test]
fn un_error_sigue_haciendo_fallar_check() {
    assert!(!check(&[], &CON_AVISO.replace("        on back -> A\n", "")));
}
