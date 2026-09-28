// Benchmark reproducible del tiempo de parseo y verificación.
//
// Uso:  cargo run --release -p trenza-core --example bench_verify
//       (o scripts/bench-verify.sh, que además registra el entorno)
//
// Responde a la revisión B de Onward! 2026: "i would have appreciated more
// information on the performance evaluation, e.g., technical setup etc.
// leading to the claimed verification time of <100ms".
//
// Mide por separado parseo (pest → AST) y verificación (validator::verify),
// en microsegundos, como mediana y p95 de N repeticiones tras un
// calentamiento. No incluye arranque del proceso ni generación de código.

use std::time::Instant;
use trenza_core::{parser, validator};

const WARMUP: usize = 20;
const RUNS: usize = 200;

fn stats(mut xs: Vec<f64>) -> (f64, f64) {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let med = xs[xs.len() / 2];
    let p95 = xs[(xs.len() * 95) / 100];
    (med, p95)
}

fn bench(name: &str, src: &str) {
    let mut parse_us = Vec::with_capacity(RUNS);
    let mut verify_us = Vec::with_capacity(RUNS);
    let mut n_diags = 0;
    for i in 0..WARMUP + RUNS {
        let t0 = Instant::now();
        let program = parser::parse_file(src).expect("parse");
        let t1 = Instant::now();
        let res = validator::verify(&program);
        let t2 = Instant::now();
        n_diags = res.err().map(|d| d.len()).unwrap_or(0);
        if i >= WARMUP {
            parse_us.push((t1 - t0).as_secs_f64() * 1e6);
            verify_us.push((t2 - t1).as_secs_f64() * 1e6);
        }
    }
    let (pm, pp) = stats(parse_us);
    let (vm, vp) = stats(verify_us);
    let lines = src.lines().count();
    println!(
        "| {name:<40} | {lines:>6} | {n_diags:>5} | {pm:>10.0} | {pp:>9.0} | {vm:>10.0} | {vp:>9.0} |"
    );
}

/// Especificación sintética: `k` contextos en anillo, `m` roles por contexto,
/// un evento; completa (pasa R1-R8), así que la verificación recorre todo.
fn synthetic(k: usize, m: usize) -> String {
    let mut s = String::from("data D:\n    x: Id\n\nsystem S:\n    initial: C0\n    contexts:\n");
    for i in 0..k {
        s += &format!("        C{i}\n");
    }
    for i in 0..k {
        s += &format!("\ncontext C{i}:\n");
        for j in 0..m {
            if j == 0 {
                s += &format!("    role r{j}: D\n        on e -> go{i}\n");
            } else {
                s += &format!("    role r{j}: D\n        on e -> ignored\n");
            }
        }
        s += &format!("    transitions:\n        on go{i} -> C{}\n", (i + 1) % k);
    }
    s
}

fn main() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let crono = std::fs::read_to_string(format!(
        "{root}/examples/cronometro-wasm/src/cronometro_full.trz"
    ))
    .expect("cronometro_full.trz");
    let crono_sin_comodin: String = crono
        .lines()
        .filter(|l| !l.contains("role *"))
        .collect::<Vec<_>>()
        .join("\n");

    println!("Tiempos en microsegundos; mediana y p95 de {RUNS} repeticiones tras {WARMUP} de calentamiento.\n");
    println!("| especificación                           | líneas | diags | parse med. | parse p95 | verif. med. | verif. p95 |");
    println!("|------------------------------------------|-------:|------:|-----------:|----------:|-----------:|----------:|");
    bench("cronometro_full.trz", &crono);
    bench("cronometro_full.trz sin `role *`", &crono_sin_comodin);
    for (k, m) in [(10, 10), (50, 10), (100, 10), (200, 20), (500, 20)] {
        bench(&format!("sintética k={k} contextos, m={m} roles"), &synthetic(k, m));
    }
}
