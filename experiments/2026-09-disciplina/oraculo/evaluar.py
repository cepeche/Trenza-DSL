#!/usr/bin/env python3
"""Oráculo: evalúa una réplica contra su línea base.

Uso: evaluar.py <T1..T4> <A|B> <linea_base> <candidato>
  A: rutas a directorios frontend/;  B: rutas a archivos .trz
Salida: JSON con {correcta, arranca, fallos_expectativa[], regresiones[], forbidden_como_nada}
"""
import json, subprocess, sys, pathlib, tempfile

HERE = pathlib.Path(__file__).resolve().parent
EXP = json.loads((HERE / "expectativas.json").read_text())
NOOP = "__noop__"
STATE_FIELDS = {"A": ["mode", "overlays", "tab"], "B": ["base", "overlays", "concurrent"]}
COMPARE_FIELDS = {"A": ["mode", "overlays", "tab", "mutations", "missing"],
                  "B": ["base", "overlays", "concurrent", "action", "forbidden", "missing"]}

def run(cond, target, scenarios):
    f = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
    json.dump(scenarios, f); f.close()
    if cond == "A":
        cmd = ["node", str(HERE.parent / "harness-a/run.js"), target, f.name]
    else:
        cmd = [sys.executable, str(HERE.parent / "harness-b/run.py"), target, f.name]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=900)
    try:
        return json.loads(p.stdout.strip().splitlines()[-1])
    except Exception:
        return {"error": "harness", "detail": (p.stdout + p.stderr)[-2000:]}

def main():
    task, cond, base, cand = sys.argv[1:5]
    t = EXP["tasks"][task]
    prefixes = dict(EXP["prefixes"]); prefixes.update(t["extra_prefixes"])
    elements = EXP["base_elements"] + t["extra_elements"]
    keys = []
    for p, seq in prefixes.items():
        keys.append((p, NOOP, seq + [NOOP]))
        for e in elements:
            keys.append((p, e, seq + [e]))
    scen = [k[2] for k in keys]
    res = run(cond, cand, scen)
    if isinstance(res, dict):
        print(json.dumps({"correcta": False, "arranca": False, "error": res}, ensure_ascii=False)); return
    obs = {(k[0], k[1]): o for k, o in zip(keys, res)}
    if cond == "A" and any(o.get("errors", 0) > 0 for o in res):
        print(json.dumps({"correcta": False, "arranca": False, "error": "errores JS"}, ensure_ascii=False)); return

    fallos, forb = [], 0
    covered = set()
    for ex in t["expect"]:
        key = (ex["pre"], ex["tap"]); covered.add(key)
        o = obs[key]
        ok = True
        if o.get("missing"):
            ok = False
        elif ex["kind"] == "state":
            ok = all(o.get(f) == v for f, v in ex[cond].items())
        elif ex["kind"] == "nothing":
            before = obs[(ex["pre"], NOOP)]
            ok = all(o.get(f) == before.get(f) for f in STATE_FIELDS[cond])
            if cond == "A":
                ok = ok and o.get("mutations") == 0
            else:
                ok = ok and (o.get("action") is None)
                if ok and o.get("forbidden"): forb += 1
        elif ex["kind"] == "like":
            ref = obs[(ex["like_pre"], ex["tap"])]
            over = ex[cond]
            ok = all(o.get(f) == (over[f] if f in over else ref.get(f)) for f in COMPARE_FIELDS[cond])
        if not ok:
            fallos.append({"pre": ex["pre"], "tap": ex["tap"], "obs": o})

    base_keys = [(p, e) for p in EXP["prefixes"] for e in EXP["base_elements"] if (p, e) not in covered]
    bres = run(cond, base, [prefixes[p] + [e] for p, e in base_keys])
    regres = []
    for (p, e), b in zip(base_keys, bres):
        o = obs[(p, e)]
        diff = {f: [b.get(f), o.get(f)] for f in COMPARE_FIELDS[cond] if b.get(f) != o.get(f)}
        if diff:
            regres.append({"pre": p, "tap": e, "diff": diff})
    print(json.dumps({"correcta": not fallos and not regres, "arranca": True,
                      "fallos_expectativa": fallos, "regresiones": regres,
                      "forbidden_como_nada": forb}, ensure_ascii=False))

main()
