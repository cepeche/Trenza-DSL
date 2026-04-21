# Crónica 2026-04-21 — Cierre de sesión (commit acumulado 2026-04-17/21)

**Author:** CL (Claude Sonnet 4.6 via Antigravity)
**Fecha:** 2026-04-21

---

## Resumen

Sesión de cierre. Se commitea el trabajo acumulado desde el commit
`d6286cd` (2026-04-17): revisiones del paper, actualizaciones de spec,
figuras, scripts y crónicas de CO y GE que quedaron sin commitear.

---

## Trabajo comprometido en este commit

### Paper ONWARD! 2026 (rama `paper`)

**Nuevas secciones LaTeX** (`paper/sections/`):
- `04-collaboration.tex`, `05-validation.tex`, `06-related-work.tex`
- `07-conclusion.tex`, `08-glossary.tex`

**Secciones revisadas** (`paper/sections/`):
- `01-introduction.tex`, `02-motivation.tex`, `03-language.tex`

**`paper/main.tex`**: integración de las nuevas secciones.

**`paper/bibliography.bib`**: entradas añadidas para related work
(DCI, Naked Objects, NetKernel, Hermans/Hedy, claude-flow, MCP).

**Figuras** (`paper/figures/`): archivos fuente `.mmd` + vectores `.svg`
para `strands`, `dispersed`, `cronometro_psp`, `monitoreored`.

**Scripts** (`scripts/`): `reproduce-paper.sh` y `reproduce-paper.ps1`
para reproducibilidad de la build LaTeX.

### Diseño y auditoría (`docs/design/`)

- `paper-prose-s3.md` — revisión §3 (CO, 2026-04-17): "Eight Rules",
  §3.5 multi-target synthesis.
- `paper-prose-s4.md` — revisión §4 (CO): atribución corregida de Rule 8.
- `paper-prose-s7.md` — revisión §7 (CO): "Constraint as gift", 13 contextos.
- `paper-related-work-audit.md` — plan de revisión related work (CO).
- `adr-attribution-review.md` — auditoría de atribución de modelos (GE).
- `adr-attribution-review-audit.md` — auditoría de la auditoría (CO):
  **no ejecutar** el reemplazo global de rótulos Gemini; escalar a César.

### Spec de referencia

**`spec/reference/cronometro-psp/trenza/`**: actualizaciones en contextos
(`ModalComentario`, `ModalCrearTarea`, `ModalEditarActividad`,
`ModalEditarTarea`, `ModalReset`, `ModalSeleccionActividad`,
`ModoEdicion`, `ModoNormal`), `data.trz` y `external/cronometro_api.trz`.

**Outputs generados** (regenerados tras cambios en el DSL):
- `spec/reference/cronometro-psp/generated/CronometroPSP_out.ts`
- `spec/reference/cronometro-psp/generated/CronometroPSP_out.test.ts`
- `spec/reference/trenza-cli/generated/CLI_Trenza_out.ts`
- `spec/reference/trenza-cli/generated/CLI_Trenza_out.test.ts`

**`spec/reference/trenza-cli.trz`**: actualización de la spec del CLI.

### Ejemplo

**`examples/MonitoreoRed.trz`**: nuevo sistema de ejemplo (red monitoring).

### Infraestructura

- `.gitignore`: añadida entrada `bin/` (precedente sesión) + sección
  de artefactos LaTeX (`*.aux`, `*.bbl`, `*.blg`, `*.out`, logs raíz,
  PDFs de paper y figuras).
- `.aiexclude`: actualizado.
- `FILES_FOR_AGENTS.md`: sección de gestión de artefactos de compilación
  (ver crónica `01_CL`).

### Crónicas comprometidas (CO y GE, 2026-04-17)

- `history/chronicle/2026-04-17/01_CO_paper_revision_s3_s4_s7.md`
- `history/chronicle/2026-04-17/02_CO_audit_de_auditoria_gemini.md`
- `history/chronicle/2026-04-17/03_GE_memo_atribucion_opus.md`

---

## Pendiente (no resuelto en esta sesión)

- **Atribución de modelos Gemini en ADRs**: César debe decidir si los
  ADRs con rótulo `2.5 Pro` deben corregirse a `3.1 Pro`. La auditoría
  cruzada CO+GE concluye que `3.1 Pro` es correcto. Ningún ADR ha sido
  tocado hasta que César lo apruebe.
- **Related work**: la auditoría de CO está lista en
  `docs/design/paper-related-work-audit.md`. Pendiente de integración
  en el paper.
- **§5-§6**: secciones incompletas, pendientes de revisión Opus en
  iteraciones siguientes.
- **Push**: pendiente de aprobación explícita del usuario.
