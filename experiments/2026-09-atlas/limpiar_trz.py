#!/usr/bin/env python3
"""Genera material/cronometro_limpio.trz para B′ a partir de la línea base
congelada (2689bcb). Solo cambia la presentación, nunca la semántica:
  1. quita las cabeceras de origen de la concatenación: `-- <ruta>.trz ...`
     y `-- Convertido de ... a .trz (...)`;
  2. repara el mojibake (UTF-8 leído como Latin-1/CP1252, incluso doble);
  3. quita los espacios finales;
  4. quita las líneas en blanco, salvo una antes de cada línea que empieza
     en la columna 0 (definición o comentario de nivel superior).
La equivalencia se comprueba con el oráculo (ver PREREGISTRO-BPRIMA.md).
"""
import pathlib, re, subprocess, sys
HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
def desmojibake(l):
    for _ in range(3):
        n = None
        for enc in ("cp1252", "latin-1"):   # 0x8D (Í) no existe en cp1252
            try:
                n = l.encode(enc).decode("utf-8")
                break
            except (UnicodeEncodeError, UnicodeDecodeError):
                pass
        if n is None:
            return l
        if n == l:
            return l
        l = n
    return l

def limpiar(src):
    lineas = []
    for l in src.splitlines():
        l = desmojibake(l).rstrip()
        if re.match(r"--\s+[\w/]+\.trz\b", l) or re.match(r"--\s+Convertido de ", l):
            continue
        lineas.append(l)
    out = []
    for i, l in enumerate(lineas):
        if not l:
            sig = next((x for x in lineas[i + 1:] if x), "")
            if not out or out[-1] == "" or sig[:1] in (" ", "\t", ""):
                continue
        out.append(l)
    while out and not out[-1]:
        out.pop()
    return "\n".join(out) + "\n"


if __name__ == "__main__":
    src = subprocess.run(["git", "-C", str(ROOT), "show", "2689bcb:examples/cronometro-wasm/src/cronometro_full.trz"],
                         capture_output=True, check=True).stdout.decode("utf-8")
    texto = limpiar(src)
    dest = HERE / "material/cronometro_limpio.trz"
    dest.write_text(texto, encoding="utf-8", newline="\n")
    print(f"{len(src.splitlines())} → {len(texto.splitlines())} líneas; {dest}")
