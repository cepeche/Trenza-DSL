#!/usr/bin/env python3
"""Agrega resultados/*/ en tablas (Markdown en stdout)."""
import json, pathlib, statistics as st
HERE = pathlib.Path(__file__).resolve().parent
rows = []
for d in sorted((HERE / "resultados").iterdir()):
    ev = json.loads((d / "evaluacion.json").read_text()); me = json.loads((d / "meta.json").read_text())
    t, c, r = d.name.split("-")
    rows.append(dict(t=t, c=c, r=int(r), ok=ev.get("correcta"), arranca=ev.get("arranca"),
                     fallos=len(ev.get("fallos_expectativa", [])), regr=len(ev.get("regresiones", [])),
                     tok=me["tokens"], ms=me["ms"], tools=me["tool_uses"], diff=me["lineas_diff"]))
def cell(sel):
    n = len(sel); ok = sum(1 for x in sel if x["ok"])
    return n, ok, st.median(x["tok"] for x in sel), st.median(x["ms"] for x in sel) / 1000, st.median(x["tools"] for x in sel), st.median(x["diff"] for x in sel)
print("| Tarea | Cond. | Correctas | Tokens (mediana) | Duración s (mediana) | Llamadas a herramientas (mediana) | Líneas de diff (mediana) |")
print("|---|---|---:|---:|---:|---:|---:|")
for t in ["T1", "T2", "T3", "T4", "Total"]:
    for c in ["A", "B"]:
        sel = [x for x in rows if x["c"] == c and (t == "Total" or x["t"] == t)]
        n, ok, tok, s, tools, diff = cell(sel)
        name = f"**{t}**" if t == "Total" else t
        print(f"| {name} | {c} | {ok}/{n} | {tok:,.0f} | {s:.0f} | {tools:.0f} | {diff:.0f} |".replace(",", "."))
print()
print("Réplicas no correctas:")
for x in rows:
    if not x["ok"]:
        print(f"- {x['t']}-{x['c']}-{x['r']}: arranca={x['arranca']} fallos={x['fallos']} regresiones={x['regr']}")
tot = sum(x["tok"] for x in rows)
print(f"\nTokens totales de los subagentes: {tot:,}".replace(",", "."))
