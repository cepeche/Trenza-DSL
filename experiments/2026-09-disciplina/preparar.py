#!/usr/bin/env python3
"""Prepara el directorio de trabajo de una réplica e imprime su prompt.

Uso: preparar.py <T1..T4> <A|B> <réplica>
Crea /tmp/trenza-exp/<T>-<C>-<r>/ (fuera del repositorio) con el material de
su condición y escribe prompt.txt. El binario trenza-cli se copia a
/tmp/trenza-exp/bin/ para que la condición B no necesite el repositorio.
"""
import pathlib, re, shutil, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE = pathlib.Path("/tmp/trenza-exp")
TASKS = {"T1": "T1-pausa.md", "T2": "T2-reordenar.md", "T3": "T3-pestanas.md", "T4": "T4-nuevo.md"}

def main():
    task, cond, rep = sys.argv[1], sys.argv[2], sys.argv[3]
    work = BASE / f"{task}-{cond}-{rep}"
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    cli = BASE / "bin/trenza-cli"
    cli.parent.mkdir(exist_ok=True)
    shutil.copy2(ROOT / "target/release/trenza-cli", cli)
    orig = ROOT / "history/inspirations/cronometro-psp-original"
    if cond == "A":
        shutil.copytree(orig / "frontend", work / "frontend")
        shutil.copytree(orig / "tests/js", work / "tests/js")
    else:
        shutil.copy2(ROOT / "examples/cronometro-wasm/src/cronometro_full.trz", work / "cronometro.trz")
        shutil.copy2(HERE / "GUIA-TRENZA.md", work / "GUIA-TRENZA.md")
    otra = "B" if cond == "A" else "A"
    # quitar la viñeta de la otra condición y sus líneas de continuación
    out, skip = [], False
    for l in (HERE / "tareas" / TASKS[task]).read_text(encoding="utf-8").splitlines():
        if l.startswith("- Condición "):
            skip = l.startswith(f"- Condición {otra}:")
        elif not l.startswith("  "):
            skip = False
        if not skip:
            out.append(l.replace(f"- Condición {cond}:", "-"))
    tarea = "\n".join(out)
    bloque = (HERE / "tareas" / f"condicion-{cond}.md").read_text(encoding="utf-8")
    comun = re.sub(r"<!--.*?-->\n", "", (HERE / "tareas/comun.md").read_text(encoding="utf-8"), flags=re.S)
    prompt = (comun + "\n" + bloque + "\n" + tarea).replace("{WORKDIR}", str(work)).replace("{CLI}", str(cli))
    (work / "prompt.txt").write_text(prompt, encoding="utf-8")
    print(prompt)

main()
