# Análisis POST HOC (no pre-registrado): prototipo de una regla de coherencia
# entre hermanos. Si una acción que produce un contexto base dispara una
# transición en otro contexto base hermano que también la produce, pero no
# en éste, se marca. Uso: python posthoc_r11.py <archivo.trz>...
import re,sys,collections
def parse(t):
    L=t.splitlines(); ctx={}; cur=None; mode=None; base=[]; insys=False; lst=None
    for l in L:
        if re.match(r"system\s+\w+:",l): insys=True; continue
        if re.match(r"context\s+(\w+):",l):
            insys=False; cur=re.match(r"context\s+(\w+):",l).group(1); ctx[cur]={"prod":set(),"trans":set()}; mode=None; continue
        if l[:1] not in (" ","\t","") : insys=False; cur=None; continue
        s=l.strip().split("--")[0].strip()
        if insys:
            if s.endswith(":"): lst=s
            elif s and lst=="contexts:": base.append(s)
            continue
        if not cur or not s: continue
        if s=="transitions:": mode="t"; continue
        if re.match(r"(role|pub role|fills|effects:)",s): mode="r" if s.startswith(("role","pub")) else "x"; continue
        m=re.match(r"on\s+\S+\s*->\s*(\w+)",s)
        if m and mode=="r" and m.group(1) not in ("ignored","forbidden","pending"): ctx[cur]["prod"].add(m.group(1))
        if m and mode=="t": ctx[cur]["trans"].add(re.match(r"on\s+(\S+)",s).group(1))
    return ctx, base
def r11(t):
    ctx,base=parse(t); out=[]
    grp=[c for c in base if c in ctx]
    for c in grp:
        for a in ctx[c]["prod"]-ctx[c]["trans"]:
            otros=[o for o in grp if o!=c and a in ctx[o]["prod"] and a in ctx[o]["trans"]]
            if otros: out.append(f"{c}: produce {a} sin transición (sí la tienen {', '.join(otros)})")
    return out
for f in sys.argv[1:]:
    print(f.split('/')[-3] if 'work' in f else f, r11(open(f).read()))
