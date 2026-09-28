#!/usr/bin/env python3
"""Arnés B: ejecuta escenarios de taps sobre el Rust generado desde un .trz.

Uso: run.py <spec.trz> <escenarios.json>   → JSON en stdout

escenarios.json: lista de listas de nombres de rol (cada tap es `<rol>.tap`).
Para cada escenario se crea un System nuevo (contexto inicial del sistema),
se aplican los taps y se devuelve la observación tras el último:
  {base, overlays, concurrent, action, forbidden, missing}
Si el .trz no verifica o el Rust no compila: {"error": "..."}.
"""
import json, os, re, subprocess, sys, tempfile, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
CLI = ROOT / "target/release/trenza-cli"
TARGET = pathlib.Path(os.environ.get("HARNESS_B_TARGET", "/tmp/harness-b-target"))

MAIN = r'''
use std::panic::{catch_unwind, AssertUnwindSafe};
fn obs(sys: &System, action: Option<&'static str>, forbidden: bool, missing: bool) -> serde_json::Value {
    let mut conc: Vec<String> = sys.concurrent.iter().map(|c| format!("{:?}", c)).collect();
    conc.sort();
    serde_json::json!({
        "base": format!("{:?}", sys.base),
        "overlays": sys.overlay_stack.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>(),
        "concurrent": conc,
        "action": action, "forbidden": forbidden, "missing": missing,
    })
}
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let scenarios: Vec<Vec<String>> = serde_json::from_str(&input).unwrap();
    let mut out = Vec::new();
    for sc in scenarios {
        let effects = RecordingEffects::new();
        let mut sys = System::new(INITIAL, &effects);
        let mut last = serde_json::Value::Null;
        for role in &sc {
            let r = catch_unwind(AssertUnwindSafe(|| tap(&mut sys, role)));
            last = match r {
                Ok(Some(a)) => obs(&sys, a, false, false),
                Ok(None) => obs(&sys, None, false, true),
                Err(_) => obs(&sys, None, true, false),
            };
        }
        out.push(last);
    }
    println!("{}", serde_json::to_string(&out).unwrap());
}
'''

def main():
    spec, scen = sys.argv[1], sys.argv[2]
    work = pathlib.Path(tempfile.mkdtemp(prefix="hb-"))
    chk = subprocess.run([str(CLI), "check", spec], capture_output=True, text=True)
    if chk.returncode != 0:
        print(json.dumps({"error": "check", "detail": (chk.stdout + chk.stderr)[-2000:]})); return
    gen = subprocess.run([str(CLI), "generate", "--lang=rust", "--profile=pro", f"--out-dir={work}/out", spec], capture_output=True, text=True)
    outs = list((work / "out").glob("*_out.rs"))
    if gen.returncode != 0 or not outs:
        print(json.dumps({"error": "generate", "detail": (gen.stdout + gen.stderr)[-2000:]})); return
    code = outs[0].read_text()
    initial = None
    for line in pathlib.Path(spec).read_text(encoding="utf-8", errors="replace").splitlines():
        m = re.match(r"\s*initial:\s*(\w+)", line)
        if m: initial = m.group(1); break
    disp = re.findall(r"pub fn dispatch_(\w+?)_tap\(&mut self, \w+: &(\w+)\)", code)
    arms = "\n".join(
        f'        "{r}" => {{ let d: {t} = Default::default(); Some(sys.dispatch_{r}_tap(&d)) }}' for r, t in disp)
    driver = f'''
const INITIAL: Contexto = Contexto::{initial};
fn tap(sys: &mut System, role: &str) -> Option<Option<&'static str>> {{
    match role {{
{arms}
        _ => None,
    }}
}}
'''
    (work / "src").mkdir()
    (work / "Cargo.toml").write_text('[package]\nname = "hb"\nversion = "0.0.0"\nedition = "2021"\n'
        '[dependencies]\nserde = { version = "1.0", features = ["derive"] }\nserde_json = "1.0"\n[workspace]\n')
    (work / "src/main.rs").write_text("#![allow(warnings)]\n" + code + driver + MAIN)
    b = subprocess.run(["cargo", "build", "--quiet", "--release"], cwd=work, capture_output=True, text=True,
                       env={**os.environ, "CARGO_TARGET_DIR": str(TARGET)})
    if b.returncode != 0:
        print(json.dumps({"error": "compile", "detail": b.stderr[-3000:]})); return
    r = subprocess.run([str(TARGET / "release/hb"), scen], capture_output=True, text=True)
    if r.returncode != 0:
        print(json.dumps({"error": "run", "detail": r.stderr[-2000:]})); return
    print(r.stdout.strip())

main()
