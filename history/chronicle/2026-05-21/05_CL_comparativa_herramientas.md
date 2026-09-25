# Comparativa de Herramientas — Claude Code vs Antigravity

**Fecha:** 2026-05-21
**Autor:** CL (Claude Opus 4.7 vía Claude Code)
**Fuente Antigravity:** Inventario generado por Claude **Opus 4.6 Thinking** en
sesión paralela — ver `04_CL_inventario_herramientas_antigravity.md`.
**Propósito:** Decidir qué tareas conviene ejecutar en cada entorno.

> ⚠️ **Asimetría de modelo:** El inventario de Antigravity lo produjo Opus 4.6;
> esta comparativa la produce Opus 4.7. La diferencia de versión no afecta al
> catálogo de herramientas (es del host, no del modelo), pero sí podría afectar
> al juicio sobre ellas. Cuando Antigravity exponga Opus 4.7, conviene repetir
> el experimento por simetría.

---

## 1. Equivalencias directas (mismo trabajo, distinto nombre)

| Capacidad | Antigravity | Claude Code |
|---|---|---|
| Leer archivo | `view_file` (máx 800 líneas) | `Read` (máx 2000 líneas) |
| Escribir archivo | `write_to_file` | `Write` |
| Editar 1 bloque | `replace_file_content` | `Edit` |
| Editar N bloques | `multi_replace_file_content` ⭐ | múltiples `Edit` (más verboso) |
| Buscar texto | `grep_search` (ripgrep, máx 50) ⚠️ | `Grep` (ripgrep, configurable) |
| Listar directorio | `list_dir` | `Glob` + `Bash ls` |
| Shell | `run_command` (PowerShell con aprobación) | `Bash` **y** `PowerShell` separados |
| Web fetch | `read_url_content` | `WebFetch` |
| Web search | `search_web` | `WebSearch` |
| Programar tarea | `schedule` (one-shot/cron) | `ScheduleWakeup` + `CronCreate/List/Delete` |
| Preguntar al usuario | `ask_question` | `AskUserQuestion` |
| Subagente | `invoke_subagent` + `define_subagent` ⭐ | `Agent` (tipos pre-definidos) |
| Background jobs | `manage_task` (list/kill/status/stdin) | `run_in_background` flag + `Monitor` |
| Mensaje entre agentes | `send_message` (intra-host) | `SendMessage` (intra-host) |
| Imagen | `generate_image` ⭐ | — (sin equivalente nativo) |

---

## 2. Ventajas reales de Antigravity (para Trenza-DSL)

1. **`generate_image`** — útil para mockups de UI de CronometroPSP sin salir del
   IDE. Claude Code no tiene equivalente nativo.
2. **`multi_replace_file_content`** — edición múltiple atómica en una llamada;
   más limpia que N×`Edit`.
3. **`define_subagent` dinámico** — en caliente, sin tocar `.claude/agents/`.
4. **`ask_permission` explícito** — granularidad mayor sobre permisos puntuales.

## 3. Ventajas reales de Claude Code (para Trenza-DSL)

1. **Plan mode** (`EnterPlanMode`/`ExitPlanMode`) — fase explícita de diseño
   antes de tocar código. Sin equivalente en Antigravity.
2. **Worktree isolation** — `Agent` puede correr en un worktree git temporal
   aislado. Útil para experimentos en paralelo sin manchar la rama actual.
3. **Agentes pre-definidos especializados** — `Explore`, `Plan`,
   `claude-code-guide`, `general-purpose`. Antigravity solo lista `research` y
   `self`.
4. **Ecosistema MCP rico**:
   - **Claude Preview** (dev server + screenshot/click/fill/inspect/network/logs)
   - Chrome browser, Gmail (labels/drafts/threads)
   - Gestión de sesiones (`archive_session`, `search_session_transcripts`)
   - Registry discovery
5. **Skills relevantes al proyecto**: `consolidate-memory`, `security-review`,
   `simplify`, `loop`, `claude-api`, `review`, `init`. El catálogo de
   Antigravity para *este* proyecto está dominado por 33 skills de
   bioinformática que son ruido aquí.
6. **Auto-memoria persistente entre sesiones** —
   `C:\Users\ceo\.claude\projects\C--Proyectos-Trenza-DSL\memory\`. Antigravity
   no expuso un mecanismo equivalente como herramienta.
7. **Session chapters + `spawn_task`** (mcp__ccd_session__*) — marcar capítulos
   y abrir tareas paralelas con chips de UI.

---

## 4. Lectura operativa

- **Trabajo de Trenza (compilador Rust, paper, crónica):** Claude Code mejor
  equipado. Worktree, Plan mode, skills relevantes, memoria persistente, MCPs.
- **Mockups visuales o demos web rápidas:** Antigravity gana con
  `generate_image` + DevTools.
- **Edición masiva de un archivo:** `multi_replace_file_content` es UX superior.
- **El catálogo de 33 skills científicas en Antigravity** sugiere un perfil
  "todo a la vez" no específico del proyecto. Para uso serio en Trenza
  conviene desactivar el plugin `science` y reducir ruido de contexto.

## 4.bis Erratas y limitaciones reportadas por GE (auditoría, 2026-05-21)

GE (Gemini 3.5 Flash en Antigravity) reportó vía buzón inter-agente las
siguientes limitaciones que invalidan o matizan la tabla de arriba:

- ⚠️ **`grep_search` está bloqueado en Antigravity-Windows en este host:** se
  cuelga sistemáticamente. Mientras esté así, los agentes corriendo en
  Antigravity deben buscar texto mediante `Select-String` vía `run_command`,
  por ejemplo:
  ```powershell
  Select-String -Path history/coordination/**/*.md -Pattern "thread:"
  ```
  Esto significa que la equivalencia "grep_search ↔ Grep" no es operativa
  hoy; Claude Code mantiene ventaja real en búsqueda de texto en este
  entorno.

Fuente: `history/coordination/archive/2026-05-21/2026-05-21T20-52_review-protocol-v0_GE_2.md`.

---

## 5. Pendiente

- Repetir comparación cuando Antigravity exponga Opus 4.7.
- Evaluar si conviene desactivar el plugin `science` para sesiones de Trenza en
  Antigravity.
- Documentar el flujo de comunicación entre agentes — ver
  [`06_CL_propuesta_comms_inter_agente.md`](06_CL_propuesta_comms_inter_agente.md).
