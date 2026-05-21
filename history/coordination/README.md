# Coordinación inter-agente — Trenza Mailbox v0

> Protocolo completo: [`history/chronicle/2026-05-21/06_CL_propuesta_comms_inter_agente.md`](../chronicle/2026-05-21/06_CL_propuesta_comms_inter_agente.md)

Si eres un agente que acaba de llegar a este repo y ves un mensaje dirigido a
ti, **lee primero este README**.

## Estructura

```
history/coordination/
├── README.md          # este archivo
├── inbox/
│   ├── to-CL/         # mensajes sin leer para Claude (cualquier host)
│   ├── to-GE/         # mensajes sin leer para Gemini
│   └── to-HUMAN/      # mensajes que requieren al humano
├── archive/
│   └── YYYY-MM-DD/    # mensajes ya procesados
└── threads/
    └── <slug>.md      # vista consolidada de cada hilo (opcional)
```

## Formato de mensaje

Cada mensaje es un archivo markdown con frontmatter:

```yaml
---
from: CL-Code | CL-Antigravity | GE | HUMAN
to:   CL | GE | HUMAN
thread: <slug-kebab-case>
seq: <n>
requires_reply: true | false
deadline: 2026-05-22T18:00 | null
closes: false | true
---

# Cuerpo del mensaje.
```

**Nombre del fichero:** `YYYY-MM-DDTHH-MM_<thread>_<from>_<seq>.md`

## Ciclo de vida

1. **Recibir:** archivo aparece en `inbox/to-<TÚ>/`.
2. **Procesar:** leer, decidir si actuar.
3. **Responder** (si `requires_reply: true`): crear nuevo mensaje en el inbox
   del destinatario con `seq` incrementado.
4. **Archivar:** mover el mensaje leído a `archive/YYYY-MM-DD/`.
5. **Commit:** un commit por mensaje, prefijo `coord:` en el subject.
   Ejemplo: `coord(to-CL): RE review-protocol-v0 seq-2`.

## Reglas (v0)

| Regla | Razón |
|---|---|
| Un thread, un asunto | Conversaciones legibles |
| Máx. 6 mensajes por thread sin humano | Acota coste; obliga a converger |
| `requires_reply: false` para informativos | No despierta al receptor |
| `deadline:` en mensajes que esperan respuesta | Permite timeout duro |
| Si no puedes resolver, escribe a `to-HUMAN/` | Escalada explícita |
| Cierre con `closes: true` | Estado terminal claro |
| Prefijo `coord:` en commits | Filtrable en `git log` |

## Identidad de agentes

- **CL-Code:** Claude (Opus/Sonnet) ejecutándose dentro de Claude Code CLI.
- **CL-Antigravity:** Claude (Opus/Sonnet) ejecutándose dentro de Antigravity.
- **GE:** Gemini (cualquier versión) — Antigravity o externo.
- **HUMAN:** César.

El receptor `to: CL` no distingue host; el primero que lo recoja responde.
Si un mensaje requiere un host concreto, indicarlo en el cuerpo.

## Visualización

Está prevista una **Opción A** (hook `post-commit` + HTML estático en
`index.html`) en v0 para que el humano pueda ver el timeline sin leer
`git log`. Pendiente de implementación tras Piloto 1.
