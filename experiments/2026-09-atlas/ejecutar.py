#!/usr/bin/env python3
"""Réplica del experimento de disciplina con el modelo local de ATLAS.

Ejecuta las réplicas sin pasar por Claude: habla directamente con Ollama
(/api/chat), aplica los bloques BUSCAR/REEMPLAZAR que devuelve el modelo,
le enseña la salida de la comprobación de su condición (hasta --rondas
correcciones) y evalúa el resultado con el oráculo del primer experimento.

Todo lo que el primer experimento fijó (tareas, línea base, guía, oráculo,
arneses y trenza-cli) se toma del commit del pre-registro 2689bcb, en un
worktree congelado; el verificador actual no se usa. Ver PREREGISTRO.md.

Uso (desde la raíz del repositorio):
  python experiments/2026-09-atlas/ejecutar.py --preparar
  python experiments/2026-09-atlas/ejecutar.py --solo T1-B-1        (piloto)
  python experiments/2026-09-atlas/ejecutar.py                      (las 40)
  python experiments/2026-09-atlas/ejecutar.py --resumen
Las réplicas ya evaluadas se saltan, así que se puede interrumpir y seguir.

Variables: ATLAS_URL (defecto http://ATLAS-A9:11434), ATLAS_MODEL
(defecto gpt-oss-120b:latest). Requiere git, cargo, node y npm.
"""
import argparse, difflib, json, os, pathlib, re, shutil, subprocess, sys, time, urllib.request

os.environ["PYTHONUTF8"] = "1"          # los scripts del oráculo leen UTF-8 también en Windows
HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
COMMIT = "2689bcb"                      # sello del pre-registro del primer experimento
FROZEN = ROOT / ".exp-congelado"        # worktree en ese commit (ignorado por git)
EXP = FROZEN / "experiments/2026-09-disciplina"
RES = HERE / "resultados"
TASKS = {"T1": "T1-pausa.md", "T2": "T2-reordenar.md", "T3": "T3-pestanas.md", "T4": "T4-nuevo.md"}
FILES_A = ["frontend/index.html", "frontend/js/app.js", "frontend/js/api-client.js"]
EXE = ".exe" if os.name == "nt" else ""
BLOCK = re.compile(r"<{7} BUSCAR[ \t]+(\S+)[ \t]*\n(.*?)\n?={7}[ \t]*\n(.*?)\n?>{7} REEMPLAZAR", re.S)


def sh(cmd, **kw):
    print("  $", " ".join(map(str, cmd)))
    subprocess.run(cmd, check=True, **kw)


def preparar():
    if not FROZEN.exists():
        sh(["git", "-C", str(ROOT), "worktree", "add", "--detach", str(FROZEN), COMMIT])
    sh(["cargo", "build", "--release", "-p", "trenza-cli"], cwd=FROZEN)
    if not (EXP / "harness-a/node_modules").exists():
        sh(["npm" + (".cmd" if os.name == "nt" else ""), "ci"], cwd=EXP / "harness-a")
    print("Preparado:", FROZEN)


def cli():
    return FROZEN / f"target/release/trenza-cli{EXE}"


def base_dir(cond):
    if cond == "A":
        return FROZEN / "history/inspirations/cronometro-psp-original"
    return FROZEN / "examples/cronometro-wasm/src"


def material(cond, work):
    """Copia el material de la condición a work/ y devuelve {ruta: texto} para el prompt."""
    if cond == "A":
        orig = base_dir("A")
        shutil.copytree(orig / "frontend", work / "frontend")
        shutil.copytree(orig / "tests/js", work / "tests/js")
        return {f: (work / f).read_text(encoding="utf-8") for f in FILES_A}
    shutil.copy2(base_dir("B") / "cronometro_full.trz", work / "cronometro.trz")
    return {"GUIA-TRENZA.md": (EXP / "GUIA-TRENZA.md").read_text(encoding="utf-8"),
            "cronometro.trz": (work / "cronometro.trz").read_text(encoding="utf-8")}


def tarea(task, cond):
    # Mismo filtrado que preparar.py del primer experimento.
    otra = "B" if cond == "A" else "A"
    out, skip = [], False
    for l in (EXP / "tareas" / TASKS[task]).read_text(encoding="utf-8").splitlines():
        if l.startswith("- Condición "):
            skip = l.startswith(f"- Condición {otra}:")
        elif not l.startswith("  "):
            skip = False
        if not skip:
            out.append(l.replace(f"- Condición {cond}:", "-"))
    return "\n".join(out)


def prompt(task, cond, files):
    comun = re.sub(r"<!--.*?-->\n", "", (EXP / "tareas/comun.md").read_text(encoding="utf-8"), flags=re.S)
    partes = [comun, (HERE / f"tareas/condicion-{cond}.md").read_text(encoding="utf-8"),
              tarea(task, cond), (HERE / "tareas/formato.md").read_text(encoding="utf-8")]
    for ruta, texto in files.items():
        partes.append(f'<archivo ruta="{ruta}">\n{texto}\n</archivo>')
    return "\n\n".join(partes)


def aplicar(texto, work, cond):
    permitidos = FILES_A if cond == "A" else ["cronometro.trz"]
    informe, n = [], 0
    for ruta, buscar, reemplazo in BLOCK.findall(texto):
        n += 1
        if ruta not in permitidos:
            informe.append(f"Bloque {n}: {ruta} no es un archivo que puedas cambiar.")
            continue
        f = work / ruta
        actual = f.read_text(encoding="utf-8")
        veces = actual.count(buscar) if buscar else 0
        if veces != 1:
            informe.append(f"Bloque {n} ({ruta}): el texto de BUSCAR aparece {veces} veces; no se aplicó.")
            continue
        f.write_text(actual.replace(buscar, reemplazo, 1), encoding="utf-8")
        informe.append(f"Bloque {n} ({ruta}): aplicado.")
    # D1: una cabecera BUSCAR que no forma bloque válido cuenta como bloque fallido
    malas = len(re.findall(r"^<{7} BUSCAR", texto, re.M)) - n
    if malas > 0:
        n += malas
        informe.append(f"{malas} bloque(s) con la cabecera mal formada; no se aplicaron. "
                       "Tras «<<<<<<< BUSCAR» va solo la ruta del archivo, y el texto a buscar "
                       "empieza en la línea siguiente.")
    return n, informe


def comprobar(cond, work):
    if cond == "A":
        cmd = ["node", "tests/js/runner.js"]
    else:
        cmd = [str(cli()), "check", "cronometro.trz"]
    p = subprocess.run(cmd, cwd=work, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=300)
    salida = (p.stdout + p.stderr).replace(str(work), ".")
    lineas = salida.splitlines()
    if cond == "B":   # la lista de archivos parseados no aporta nada
        lineas = [l for l in lineas if "leido y parseado" not in l]
    return p.returncode, "\n".join(lineas)[-4000:]


def chat(url, model, messages, num_ctx):
    body = json.dumps({"model": model, "messages": messages, "stream": False,
                       "options": {"num_ctx": num_ctx}}).encode()
    req = urllib.request.Request(f"{url}/api/chat", data=body, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=3600) as r:
        return json.loads(r.read())


def replica(task, cond, rep, a):
    rid = f"{task}-{cond}-{rep}"
    out = RES / rid
    if (out / "evaluacion.json").exists():
        print(f"{rid}: ya evaluada"); return
    if out.exists():
        shutil.rmtree(out)
    work = out / "work"
    work.mkdir(parents=True)
    files = material(cond, work)
    messages = [{"role": "user", "content": prompt(task, cond, files)}]
    turnos, t0 = [], time.monotonic()
    print(f"{rid}: ", end="", flush=True)
    for ronda in range(a.rondas + 1):
        r = chat(a.url, a.model, messages, a.num_ctx)
        msg = r.get("message", {})
        texto = msg.get("content", "")
        n, informe = aplicar(texto, work, cond)
        turno = {k: r.get(k) for k in ("prompt_eval_count", "eval_count", "total_duration", "done_reason")}
        turno.update(bloques=n, informe=informe, thinking_chars=len(msg.get("thinking") or ""))
        messages.append({"role": "assistant", "content": texto})
        if n == 0:
            turno["check"] = None
            turnos.append(turno)
            print("·", end="", flush=True)
            break
        code, salida = comprobar(cond, work)
        turno["check"] = {"codigo": code, "salida": salida}
        turnos.append(turno)
        print(f"{n}b{'✓' if code == 0 else '✗'} ", end="", flush=True)
        if ronda == a.rondas:
            break
        messages.append({"role": "user", "content":
            "Resultado de aplicar tus bloques:\n" + "\n".join(informe) +
            f"\n\nResultado de la comprobación (código de salida {code}):\n{salida}\n\n"
            "Si hay que corregir algo, responde con más bloques. Si el cambio está terminado, "
            "responde solo TERMINADO."})
    segundos = time.monotonic() - t0
    (out / "conversacion.json").write_text(json.dumps(messages, ensure_ascii=False, indent=1), encoding="utf-8")
    # diff frente a la línea base
    diff = []
    for rel in (FILES_A if cond == "A" else ["cronometro.trz"]):
        orig = base_dir(cond) / (rel if cond == "A" else "cronometro_full.trz")
        diff += difflib.unified_diff(orig.read_text(encoding="utf-8").splitlines(True),
                                     (work / rel).read_text(encoding="utf-8").splitlines(True), rel, rel)
    (out / "diff.patch").write_text("".join(diff), encoding="utf-8")
    base = base_dir(cond) / ("frontend" if cond == "A" else "cronometro_full.trz")
    cand = work / ("frontend" if cond == "A" else "cronometro.trz")
    ev = subprocess.run([sys.executable, str(EXP / "oraculo/evaluar.py"), task, cond, str(base), str(cand)],
                        capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=3600)
    try:
        res = json.loads(ev.stdout.strip().splitlines()[-1])
    except Exception:
        res = {"correcta": False, "arranca": False, "error": "evaluador", "detail": (ev.stdout + ev.stderr)[-2000:]}
    meta = {"id": rid, "modelo": a.model, "num_ctx": a.num_ctx, "segundos": round(segundos, 1),
            "turnos": turnos, "lineas_diff": sum(1 for l in diff if l[:1] in "+-" and l[:3] not in ("+++", "---")),
            "max_prompt_tokens": max((t["prompt_eval_count"] or 0) for t in turnos)}
    (out / "meta.json").write_text(json.dumps(meta, ensure_ascii=False, indent=1), encoding="utf-8")
    (out / "evaluacion.json").write_text(json.dumps(res, ensure_ascii=False, indent=1), encoding="utf-8")
    print(f"→ {'correcta' if res.get('correcta') else 'NO correcta'} ({segundos:.0f} s)")


def resumen():
    filas = []
    for d in sorted(RES.glob("T*-*-*")):
        if not (d / "evaluacion.json").exists():
            continue
        ev = json.loads((d / "evaluacion.json").read_text(encoding="utf-8"))
        me = json.loads((d / "meta.json").read_text(encoding="utf-8"))
        filas.append((d.name, ev, me))
    lineas = ["| Réplica | Correcta | Arranca | Expectativas fallidas | Regresiones | Turnos | Máx. tokens prompt | s |",
              "|---|---|---|---|---|---|---|---|"]
    for rid, ev, me in filas:
        lineas.append(f"| {rid} | {'sí' if ev.get('correcta') else 'no'} | {'sí' if ev.get('arranca') else 'no'} | "
                      f"{len(ev.get('fallos_expectativa', []))} | {len(ev.get('regresiones', []))} | "
                      f"{len(me['turnos'])} | {me['max_prompt_tokens']} | {me['segundos']} |")
    for c in "AB":
        sel = [ev for rid, ev, _ in filas if f"-{c}-" in rid]
        if sel:
            lineas.append(f"\n**{c}:** {sum(bool(e.get('correcta')) for e in sel)}/{len(sel)} correctas.")
    texto = "\n".join(lineas)
    (HERE / "RESUMEN.md").write_text("# Resumen automático\n\n" + texto + "\n", encoding="utf-8")
    print(texto)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--preparar", action="store_true")
    ap.add_argument("--resumen", action="store_true")
    ap.add_argument("--solo", help="una réplica, p. ej. T1-B-1")
    ap.add_argument("--reps", type=int, default=5)
    ap.add_argument("--rondas", type=int, default=3, help="rondas de corrección tras la primera respuesta")
    ap.add_argument("--num-ctx", type=int, default=40960)
    ap.add_argument("--url", default=os.environ.get("ATLAS_URL", "http://ATLAS-A9:11434").removesuffix("/v1").rstrip("/"))
    ap.add_argument("--model", default=os.environ.get("ATLAS_MODEL", "gpt-oss-120b:latest"))
    a = ap.parse_args()
    if a.preparar:
        return preparar()
    if a.resumen:
        return resumen()
    if not cli().exists():
        sys.exit("Falta el worktree congelado: ejecuta primero con --preparar")
    if a.solo:
        t, c, r = a.solo.split("-")
        return replica(t, c, r, a)
    # Mismo orden que el primer experimento: por réplica y tarea, alternando la condición inicial.
    for rep in range(1, a.reps + 1):
        for i, task in enumerate(TASKS):
            conds = "AB" if (rep + i) % 2 else "BA"
            for cond in conds:
                replica(task, cond, str(rep), a)
    resumen()


if __name__ == "__main__":
    main()
