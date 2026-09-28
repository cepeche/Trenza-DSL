"""Edición estructurada de un .trz (experimento B″).

El modelo no copia texto: escribe operaciones, una por línea, y este módulo
las aplica sobre el archivo. Sintaxis (la misma forma que en Trenza):

  CONTEXTO <base|overlay|concurrent> <Nombre>
  MANEJADOR <Contexto> <rol>: <Tipo> on <evento> -> <destino>
  TRANSICION <Contexto> on <acción> -> <destino>
  QUITAR_MANEJADOR <Contexto> <rol> <evento>
  QUITAR_TRANSICION <Contexto> <acción>

MANEJADOR crea el rol si el contexto no lo tiene, o sustituye el destino
si ya existe `on <evento>`. TRANSICION crea la sección `transitions:` si
falta, o sustituye el destino de `on <acción>`. CONTEXTO lo añade a la
lista del bloque `system` y crea `context <Nombre>:` vacío al final.
Solo toca los roles directos del contexto, no los de bloques `fills`.
"""
import re

LISTA = {"base": "contexts:", "overlay": "overlays:", "concurrent": "concurrent:"}
OPS = re.compile(r"^\s*`*\s*(CONTEXTO|MANEJADOR|TRANSICION|QUITAR_MANEJADOR|QUITAR_TRANSICION)\b(.*?)`*\s*$", re.M)


def ind(l):
    return len(l) - len(l.lstrip(" "))


def es_codigo(l):
    s = l.strip()
    return bool(s) and not s.startswith("--")


class Error(Exception):
    pass


def bloque(lines, nombre):
    """(inicio, fin, sangría base) del contexto; fin es exclusivo."""
    ini = next((i for i, l in enumerate(lines) if re.match(rf"context\s+{re.escape(nombre)}\s*:", l)), None)
    if ini is None:
        raise Error(f"no existe el contexto {nombre}")
    fin = ini + 1
    while fin < len(lines) and not (lines[fin][:1] not in (" ", "\t", "") ):
        fin += 1
    while fin > ini + 1 and not lines[fin - 1].strip():
        fin -= 1
    base = next((ind(l) for l in lines[ini + 1:fin] if es_codigo(l)), 4)
    return ini, fin, base


def fin_sub(lines, i, fin, nivel):
    """Índice tras las líneas más sangradas que `nivel` que siguen a i."""
    j = i + 1
    while j < fin and (not lines[j].strip() or ind(lines[j]) > nivel):
        j += 1
    while j > i + 1 and not lines[j - 1].strip():
        j -= 1
    return j


def seccion(lines, ini, fin, base, cabecera):
    return next((i for i in range(ini + 1, fin) if ind(lines[i]) == base and lines[i].strip() == cabecera), None)


def manejador(lines, ctx, rol, tipo, evento, destino):
    ini, fin, base = bloque(lines, ctx)
    r = next((i for i in range(ini + 1, fin) if ind(lines[i]) == base
              and re.match(rf"(pub\s+)?role\s+{re.escape(rol)}\s*:", lines[i].strip())), None)
    if r is None:
        antes = next((i for i in range(ini + 1, fin) if ind(lines[i]) == base
                      and re.match(r"(transitions|effects):|fills\s", lines[i].strip())), fin)
        lines[antes:antes] = [" " * base + f"role {rol}: {tipo}", " " * (base + 4) + f"on {evento} -> {destino}"]
        return f"rol {rol} creado en {ctx} con on {evento}"
    decl = re.match(rf"(pub\s+)?role\s+{re.escape(rol)}\s*:\s*(\w+)", lines[r].strip())
    if decl and decl.group(2) != tipo:
        raise Error(f"{rol} ya es de tipo {decl.group(2)} en {ctx}, no {tipo}")
    f = fin_sub(lines, r, fin, base)
    h = next((i for i in range(r + 1, f) if re.match(rf"on\s+{re.escape(evento)}\s*->", lines[i].strip())), None)
    if h is not None:
        lines[h] = " " * ind(lines[h]) + f"on {evento} -> {destino}"
        return f"{ctx}.{rol} on {evento} sustituido"
    sang = next((ind(lines[i]) for i in range(r + 1, f) if es_codigo(lines[i])), base + 4)
    lines[f:f] = [" " * sang + f"on {evento} -> {destino}"]
    return f"{ctx}.{rol} on {evento} añadido"


def transicion(lines, ctx, accion, destino):
    ini, fin, base = bloque(lines, ctx)
    t = seccion(lines, ini, fin, base, "transitions:")
    if t is None:
        lines[fin:fin] = [" " * base + "transitions:", " " * (base + 4) + f"on {accion} -> {destino}"]
        return f"transitions: creada en {ctx} con on {accion}"
    f = fin_sub(lines, t, fin, base)
    h = next((i for i in range(t + 1, f) if re.match(rf"on\s+{re.escape(accion)}\s*->", lines[i].strip())), None)
    if h is not None:
        lines[h] = " " * ind(lines[h]) + f"on {accion} -> {destino}"
        return f"{ctx} on {accion} sustituida"
    sang = next((ind(lines[i]) for i in range(t + 1, f) if es_codigo(lines[i])), base + 4)
    lines[f:f] = [" " * sang + f"on {accion} -> {destino}"]
    return f"{ctx} on {accion} añadida"


def contexto(lines, clase, nombre):
    if clase not in LISTA:
        raise Error(f"clase {clase} desconocida (base, overlay o concurrent)")
    if any(re.match(rf"context\s+{re.escape(nombre)}\s*:", l) for l in lines):
        raise Error(f"ya existe el contexto {nombre}")
    s = next((i for i, l in enumerate(lines) if re.match(r"system\s+\w+\s*:", l)), None)
    if s is None:
        raise Error("no hay bloque system")
    fs = s + 1
    while fs < len(lines) and (not lines[fs].strip() or lines[fs][:1] in (" ", "\t")):
        fs += 1
    lst = next((i for i in range(s + 1, fs) if lines[i].strip() == LISTA[clase]), None)
    if lst is None:
        lines[fs:fs] = ["    " + LISTA[clase], "        " + nombre]
    else:
        f = fin_sub(lines, lst, fs, ind(lines[lst]))
        sang = next((ind(lines[i]) for i in range(lst + 1, f) if es_codigo(lines[i])), ind(lines[lst]) + 4)
        lines[lst + 1:lst + 1] = [" " * sang + nombre]
    while lines and not lines[-1].strip():
        lines.pop()
    lines += ["", f"context {nombre}:"]
    return f"contexto {clase} {nombre} creado"


def quitar(lines, ctx, cabecera_o_rol, clave, es_rol):
    ini, fin, base = bloque(lines, ctx)
    if es_rol:
        r = next((i for i in range(ini + 1, fin) if ind(lines[i]) == base
                  and re.match(rf"(pub\s+)?role\s+{re.escape(cabecera_o_rol)}\s*:", lines[i].strip())), None)
    else:
        r = seccion(lines, ini, fin, base, "transitions:")
    if r is None:
        raise Error(f"{ctx} no tiene {cabecera_o_rol}")
    f = fin_sub(lines, r, fin, base)
    h = next((i for i in range(r + 1, f) if re.match(rf"on\s+{re.escape(clave)}\s*->", lines[i].strip())), None)
    if h is None:
        raise Error(f"{ctx}: no hay on {clave} en {cabecera_o_rol}")
    del lines[h]
    return f"{ctx}: on {clave} quitado"


def aplicar_ops(texto, trz):
    """Aplica las operaciones de `texto` a la cadena `trz`. Devuelve (nuevo, n, informe)."""
    lines = trz.split("\n")
    informe = []
    ops = OPS.findall(texto)
    for n, (op, resto) in enumerate(ops, 1):
        resto = resto.strip()
        try:
            if op == "CONTEXTO":
                m = re.fullmatch(r"(\w+)\s+(\w+)", resto)
                if not m: raise Error("forma: CONTEXTO <base|overlay|concurrent> <Nombre>")
                msg = contexto(lines, m.group(1), m.group(2))
            elif op == "MANEJADOR":
                m = re.fullmatch(r"(\w+)\s+(\w+)\s*:\s*(\w+)\s+on\s+(\S+)\s*->\s*(.+)", resto)
                if not m: raise Error("forma: MANEJADOR <Contexto> <rol>: <Tipo> on <evento> -> <destino>")
                msg = manejador(lines, *[g.strip() for g in m.groups()])
            elif op == "TRANSICION":
                m = re.fullmatch(r"(\w+)\s+on\s+(\S+)\s*->\s*(.+)", resto)
                if not m: raise Error("forma: TRANSICION <Contexto> on <acción> -> <destino>")
                msg = transicion(lines, *[g.strip() for g in m.groups()])
            elif op == "QUITAR_MANEJADOR":
                m = re.fullmatch(r"(\w+)\s+(\w+)\s+(\S+)", resto)
                if not m: raise Error("forma: QUITAR_MANEJADOR <Contexto> <rol> <evento>")
                msg = quitar(lines, m.group(1), m.group(2), m.group(3), True)
            else:
                m = re.fullmatch(r"(\w+)\s+(\S+)", resto)
                if not m: raise Error("forma: QUITAR_TRANSICION <Contexto> <acción>")
                msg = quitar(lines, m.group(1), "transitions:", m.group(2), False)
            informe.append(f"Operación {n} ({op}): {msg}.")
        except Error as e:
            informe.append(f"Operación {n} ({op}): NO aplicada: {e}.")
    nuevo = "\n".join(lines)
    if not nuevo.endswith("\n"):
        nuevo += "\n"
    return nuevo, len(ops), informe
