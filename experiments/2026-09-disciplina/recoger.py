#!/usr/bin/env python3
"""Evalúa una réplica terminada y guarda sus resultados en resultados/<id>/.

Uso: recoger.py <T> <C> <r> [--tokens N] [--ms N] [--resumen archivo]
Guarda: diff.patch (frente a la línea base), evaluacion.json, meta.json y,
si se da, el resumen final del agente.
"""
import argparse, json, pathlib, shutil, subprocess

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE_A = ROOT / "history/inspirations/cronometro-psp-original/frontend"
BASE_B = ROOT / "examples/cronometro-wasm/src/cronometro_full.trz"

ap = argparse.ArgumentParser()
ap.add_argument("task"); ap.add_argument("cond"); ap.add_argument("rep")
ap.add_argument("--tokens", type=int); ap.add_argument("--ms", type=int)
ap.add_argument("--tool-uses", type=int); ap.add_argument("--resumen")
a = ap.parse_args()
rid = f"{a.task}-{a.cond}-{a.rep}"
work = pathlib.Path("/tmp/trenza-exp") / rid
out = HERE / "resultados" / rid
out.mkdir(parents=True, exist_ok=True)
if a.cond == "A":
    base, cand = BASE_A, work / "frontend"
    diff = subprocess.run(["diff", "-ruN", str(base), str(cand)], capture_output=True, text=True).stdout
else:
    base, cand = BASE_B, work / "cronometro.trz"
    diff = subprocess.run(["diff", "-uN", str(base), str(cand)], capture_output=True, text=True).stdout
(out / "diff.patch").write_text(diff.replace(str(ROOT) + "/", "").replace("/tmp/trenza-exp/", ""))
ev = subprocess.run(["python3", str(HERE / "oraculo/evaluar.py"), a.task, a.cond, str(base), str(cand)],
                    capture_output=True, text=True, timeout=1800)
try:
    res = json.loads(ev.stdout.strip().splitlines()[-1])
except Exception:
    res = {"correcta": False, "arranca": False, "error": "evaluador", "detail": (ev.stdout + ev.stderr)[-2000:]}
(out / "evaluacion.json").write_text(json.dumps(res, ensure_ascii=False, indent=1))
meta = {"id": rid, "tokens": a.tokens, "ms": a.ms, "tool_uses": a.tool_uses,
        "lineas_diff": sum(1 for l in diff.splitlines() if l[:1] in "+-" and l[:3] not in ("+++", "---"))}
(out / "meta.json").write_text(json.dumps(meta, indent=1))
if a.resumen:
    shutil.copy2(a.resumen, out / "resumen-agente.md")
print(json.dumps({"id": rid, "correcta": res.get("correcta"), "arranca": res.get("arranca"),
                  "fallos": len(res.get("fallos_expectativa", [])), "regresiones": len(res.get("regresiones", [])),
                  "forbidden_como_nada": res.get("forbidden_como_nada"), **meta}, ensure_ascii=False))
