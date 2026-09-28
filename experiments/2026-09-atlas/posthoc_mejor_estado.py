#!/usr/bin/env python3
"""Análisis POST HOC (no pre-registrado): reconstruye, reaplicando los
bloques de conversacion.json sobre la línea base, el último estado de cada
réplica que pasó la comprobación de su condición, y lo evalúa con el
oráculo congelado. Comprueba antes que la reaplicación reproduce el estado
final guardado en work/.  Uso: python posthoc_mejor_estado.py
"""
import json, os, pathlib, re, subprocess, sys, tempfile, shutil
os.environ["PYTHONUTF8"] = "1"
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import ejecutar as E

def replay(rid):
    task, cond, rep = rid.split("-")
    d = HERE / "resultados" / rid
    conv = json.loads((d / "conversacion.json").read_text(encoding="utf-8"))
    meta = json.loads((d / "meta.json").read_text(encoding="utf-8"))
    tmp = pathlib.Path(tempfile.mkdtemp())
    E.material(cond, tmp)
    mejor = None
    asist = [m["content"] for m in conv if m["role"] == "assistant"]
    for texto, turno in zip(asist, meta["turnos"]):
        E.aplicar(texto, tmp, cond)
        if (turno.get("check") or {}).get("codigo") == 0:
            if mejor: shutil.rmtree(mejor)
            mejor = pathlib.Path(tempfile.mkdtemp()); shutil.copytree(tmp, mejor, dirs_exist_ok=True)
    rels = E.FILES_A if cond == "A" else ["cronometro.trz"]
    iguales = all((tmp / r).read_text(encoding="utf-8").replace("\r\n", "\n") ==
                  (d / "work" / r).read_text(encoding="utf-8").replace("\r\n", "\n") for r in rels)
    return task, cond, iguales, mejor

def evaluar(task, cond, dirw):
    base = E.base_dir(cond) / ("frontend" if cond == "A" else "cronometro_full.trz")
    cand = dirw / ("frontend" if cond == "A" else "cronometro.trz")
    ev = subprocess.run([sys.executable, str(E.EXP / "oraculo/evaluar.py"), task, cond, str(base), str(cand)],
                        capture_output=True, text=True, encoding="utf-8", errors="replace")
    return json.loads(ev.stdout.strip().splitlines()[-1])

filas = []
for d in sorted((HERE / "resultados").glob("T*-*-*")):
    task, cond, iguales, mejor = replay(d.name)
    final = json.loads((d / "evaluacion.json").read_text(encoding="utf-8"))["correcta"]
    m = evaluar(task, cond, mejor)["correcta"] if mejor else None
    filas.append((d.name, iguales, final, m))
    print(d.name, "reaplicación fiel" if iguales else "REAPLICACIÓN DISTINTA", "final", final, "mejor estado", m, flush=True)
for c in "AB":
    s = [f for f in filas if f"-{c}-" in f[0]]
    print(f"{c}: final {sum(bool(f[2]) for f in s)}/{len(s)}; último estado que pasó la comprobación "
          f"{sum(bool(f[3]) for f in s)}/{len(s)}; reaplicaciones fieles {sum(f[1] for f in s)}/{len(s)}")
