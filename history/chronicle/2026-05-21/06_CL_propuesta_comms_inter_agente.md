# Propuesta — Comunicación Inter-Agente sin Intervención Humana

**Fecha:** 2026-05-21
**Autor:** CL (Claude Opus 4.7 vía Claude Code)
**Destinatarios:** Humano, GE (Gemini en Antigravity), CL-Antigravity (Opus 4.6)
**Estado:** Borrador para discusión.

---

## 1. Realidad de partida (lo que *no* podemos hacer)

Tanto Antigravity como Claude Code exponen herramientas llamadas `send_message`
/ `SendMessage`, pero ambas son **intra-runtime**: solo alcanzan a subagentes o
peers dentro del mismo host. **No hay socket directo** entre el Claude que vive
en Antigravity y el Claude que vive en Claude Code, ni entre cualquiera de los
dos y Gemini.

Tampoco hay notificaciones push entre runtimes. Cada agente solo "existe"
cuando algo lo despierta: un prompt del usuario o una entrada de scheduler.

Conclusión: la comunicación inter-agente sin intervención humana requiere
**un sustrato compartido + un protocolo + un mecanismo de despertar**.

---

## 2. Sustrato propuesto: el repositorio como bus

El único canal que todos compartimos (CL-Code, CL-Antigravity, GE) es el
repositorio git. Propuesta concreta:

```
history/coordination/
├── inbox/
│   ├── to-CL/         # mensajes para Claude (cualquier host)
│   ├── to-GE/         # mensajes para Gemini
│   └── to-HUMAN/      # cosas que requieren al humano
├── archive/
│   └── YYYY-MM-DD/    # mensajes leídos, movidos aquí
└── threads/
    └── <thread-id>.md # vista consolidada de cada hilo
```

Cada mensaje es un fichero markdown con frontmatter:

```yaml
---
from: CL-Code | CL-Antigravity | GE | HUMAN
to: CL | GE | HUMAN
thread: <slug>
seq: <n>
requires_reply: true | false
deadline: 2026-05-22T18:00 | null
---

# Cuerpo del mensaje en markdown.
```

**Naming:** `YYYY-MM-DDTHH-MM_<thread>_<from>_<seq>.md`

**Ciclo de vida:** archivo en `inbox/to-X/` = no leído. El destinatario, al
procesarlo, lo mueve a `archive/YYYY-MM-DD/` y commitea. Inbox vacío = no hay
trabajo pendiente.

**Commit por mensaje:** cada send es un commit atómico
(`coord(to-CL): RE thread-foo seq-3`). Esto hace que el historial git sea el
log de conversación, y el humano puede interrumpir en cualquier punto.

---

## 3. Mecanismo de despertar

Sin push entre hosts, dependemos de schedulers locales:

- **Claude Code:** `ScheduleWakeup` (one-shot) o `CronCreate` (recurrente).
- **Antigravity:** `schedule` (one-shot o cron).

**Patrón recomendado — wakeup dirigido, no polling:**

1. Agente A envía mensaje a B con `requires_reply: true`.
2. Agente A programa **un único** wakeup en T+Δ minutos:
   *"Lee `history/coordination/inbox/to-A/`; si hay respuesta a thread-X,
   procésala; si no, vuelve a dormir Δ más, hasta máx N intentos."*
3. Agente B, cuando despierte (por el usuario o por su propio scheduler),
   procesa el inbox.
4. Si A espera más de N×Δ, escribe a `to-HUMAN/` un mensaje
   *"thread-X bloqueado, sin respuesta de B en N intentos"* y termina.

**Por qué no polling constante:** cada wakeup paga miss de cache de prompt
(>300s). Polling cada 5 min en idle es caro. Mejor wakeups dirigidos con
backoff y timeout duro.

---

## 4. Protocolo "Trenza Mailbox v0"

| Regla | Razón |
|---|---|
| Un thread, un asunto | Evita conversaciones-río imposibles de revisar |
| Máximo 6 mensajes por thread sin intervención humana | Acota coste; obliga a converger |
| `requires_reply: false` para informativos | No despierta al otro |
| `deadline:` siempre en mensajes que esperan respuesta | Permite timeout duro |
| Si un agente no puede resolver, escribe a `to-HUMAN/` | Escalada explícita |
| Cierre con `subject: CLOSE` o frontmatter `closes: true` | Estado terminal claro |
| Commits firmados con `coord:` prefix | Filtrable en `git log` |

---

## 5. Pilotos sugeridos (escalada gradual)

### Piloto 1 — "Saludo" (manual, sin scheduler)
- Humano pide a CL-Code escribir un mensaje a `to-CL/` (Antigravity).
- Humano abre Antigravity con prompt *"revisa
  `history/coordination/inbox/to-CL/`"*.
- CL-Antigravity lee, responde, archiva.
- Humano vuelve a Claude Code con *"revisa el inbox"*.
- **Objetivo:** validar formato, naming, archivado, git diff legible.
- **Sin scheduler todavía.** El humano sigue siendo router, pero el contenido
  ya es inter-agente.

### Piloto 2 — "Revisión cruzada de una regla del compilador"
- CL-Code abre thread `regla-9-spec`: propone borrador de Rule 9 (la siguiente
  que toque) en el inbox de GE.
- GE responde con análisis de viabilidad / contraejemplos.
- Hasta 4 turnos. Si converge → ADR. Si no → escalada al humano.
- **Sin scheduler.** El usuario invoca cada lado una vez por turno.

### Piloto 3 — "Despertar dirigido" (con scheduler, sin humano)
- Tras Piloto 2 estable, añadir `ScheduleWakeup` en CL-Code y `schedule` en
  Antigravity con T=20min, máx 3 intentos.
- Thread breve, asunto acotado, deadline duro.
- **Validar:** que el coste de wakeups vacíos sea aceptable, que el timeout
  funcione, que el humano vea progreso en `git log`.

---

## 6. Riesgos y mitigaciones

| Riesgo | Mitigación |
|---|---|
| Bucle infinito de mensajes | Máx 6 turnos por thread; `closes: true` obligatorio |
| Deadlock (ambos esperan al otro) | `deadline:` + escalada automática a `to-HUMAN/` |
| Coste descontrolado de wakeups | Backoff exponencial; máx N intentos; T mínimo 15 min |
| Conflictos git si ambos escriben a la vez | Naming con timestamp + agente; archivos no comparten path |
| Mensajes "alucinados" sin verificación | Cada mensaje cita ficheros y líneas; el receptor verifica antes de actuar |
| Pérdida de contexto entre wakeups | El thread vive en `threads/<id>.md`, no en memoria del agente |

---

## 6.bis Visualización (Opción A — parte de v0)

Para que el humano vea la actividad sin tener que leer `git log`, se incluye
como parte del v0 un mecanismo **mínimo y sin servidor**:

- **Hook git `post-commit`** (en `.git/hooks/post-commit` o gestionado vía
  `pre-commit` framework) que, cuando detecta cambios bajo
  `history/coordination/`, regenera `history/coordination/index.html`.
- El HTML es estático: un timeline ordenado por timestamp con todos los
  mensajes de `inbox/`, `archive/` y `threads/`, con filtros por agente y por
  hilo. CSS inline; cero dependencias.
- Se abre con doble clic. No hay proceso corriendo, no hay puerto abierto.

**Razones para empezar por aquí, no por un servidor:**

- Vive en git: reproducible, versionado, sin estado vivo que perder.
- Coste ≈ un script de ~100 líneas.
- Suficiente para volúmenes bajos (lo que tendremos en los primeros pilotos).

**Cuándo evolucionar a Opción B (servidor + WebSocket):** cuando aparezca
dolor real — varios hilos vivos en paralelo, o necesidad de notificaciones
push de escritorio cuando entre algo en `to-HUMAN/`. Antes es
over-engineering.

**Opción C (dashboard escrito en Trenza):** archivada como idea para v2 /
artículo. Atractiva como meta-dogfooding del DSL, pero requiere que la cadena
`ts → web` esté madura y que el Mailbox v0 esté operativo. No ahora.

---

## 7. Lo que esto **no** es

- No es comunicación en tiempo real (latencia mínima realista: minutos).
- No reemplaza al humano como dirección estratégica; lo libera de ser cartero.
- No es un sistema de actores Erlang ni un broker — es un buzón en git.
- No funciona si el agente que debe responder no se despierta nunca.

---

## 8. Propuesta concreta para el humano

1. **Aprobar el protocolo v0** (esta sección 4) o corregirlo.
2. **Crear el árbol `history/coordination/`** vacío + un `README.md` con el
   protocolo resumido para que cualquier agente que llegue lo encuentre.
3. **Ejecutar Piloto 1** en la próxima sesión: un saludo CL-Code → CL-Antigravity
   → CL-Code, con el humano de router. Si el formato sobrevive, escalamos.
4. **Decidir si GE participa desde el inicio** o si primero validamos entre
   Claudes y luego incorporamos a Gemini (mi recomendación: incluirlo desde
   Piloto 2, no antes, para mantener Piloto 1 minimalista).

---

*Borrador inicial; abierto a corrección por GE, CL-Antigravity y humano.*
