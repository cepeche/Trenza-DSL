# Réplica con ATLAS: cómo ejecutarla

El diseño está en `PREREGISTRO.md`. Todo corre en el portátil y habla
directamente con Ollama en ATLAS, sin pasar por Claude ni gastar crédito.

Requisitos: `git`, `cargo`, `node` y `npm`, Python 3.10 o posterior, y
ATLAS encendido. Desde la raíz del repositorio:

```bash
python experiments/2026-09-atlas/ejecutar.py --preparar        # worktree congelado en 2689bcb, trenza-cli y jsdom
python experiments/2026-09-atlas/ejecutar.py --solo T1-B-1     # piloto B
python experiments/2026-09-atlas/ejecutar.py --solo T1-A-1     # piloto A
python experiments/2026-09-atlas/ejecutar.py                   # las 40 (se salta las ya hechas)
python experiments/2026-09-atlas/ejecutar.py --resumen         # tabla en RESUMEN.md
```

Variables: `ATLAS_URL` (por defecto `http://ATLAS-A9:11434`) y `ATLAS_MODEL`
(por defecto `gpt-oss-120b:latest`). Cada réplica deja en
`resultados/<id>/`:
- `conversacion.json`;
- `meta.json`, con turnos, tokens, tiempos y salida de cada comprobación;
- `diff.patch`;
- `evaluacion.json`, con el veredicto del oráculo;
- `work/`, el material modificado.

Se puede interrumpir y volver a lanzar: las réplicas con `evaluacion.json`
se saltan.

## B′ (material limpio)

Ver `PREREGISTRO-BPRIMA.md`. Antes de empezar, anota en ese archivo el
digest del modelo (`ollama show gpt-oss-120b:latest` en ATLAS, o
`GET /api/tags`). Después:

```bash
python experiments/2026-09-atlas/ejecutar.py --bprima          # 20 réplicas B′
python experiments/2026-09-atlas/ejecutar.py --bprima --resumen  # RESUMEN-BPRIMA.md
```
