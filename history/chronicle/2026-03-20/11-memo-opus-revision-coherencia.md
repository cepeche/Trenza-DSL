# Memo: Revisión de coherencia interna post-reestructuración

**De**: Claude Opus 4.6
**Para**: Gemini 3.1 Pro (y Desarrollador Principal)
**Fecha**: 20 de marzo de 2026
**Contexto**: El Desarrollador Principal me pidió una revisión de coherencia
del repositorio sin hacer cambios. Encontré 8 hallazgos. Tras su visto bueno,
apliqué las correcciones. Este memo documenta qué se encontró, qué se corrigió
y por qué, para que quede traza y para que lo tengas presente en futuras sesiones.

---

## Hallazgos y correcciones aplicadas

| # | Severidad | Hallazgo | Corrección |
|---|-----------|----------|------------|
| 1 | 🔴 Alta | `bloqueado` e `ignorar` persistían como keywords en `spec/language/02-grammar.md` (tabla de vocabulario, gramática formal, regla de completitud, ejemplos de código) pese a que ADR-005 y ADR-008 decidieron keywords en inglés | Todas las ocurrencias reemplazadas por `forbidden` e `ignored` respectivamente |
| 1b | 🔴 Alta | `ignorar` persistía en `spec/language/03-verification.md` (Regla 1: Completitud) | Reemplazado por `ignored` |
| 2 | 🟡 Media | `examples/cronometro-psp/` seguía existiendo junto a `spec/reference/cronometro-psp/` — duplicación parcial de la app original | Movido a `history/inspirations/cronometro-psp-original/`. El Desarrollador Principal señaló que la app original es una *inspiración* (el artefacto que evidenció el problema), no un ejemplo de referencia de Trenza |
| 3 | 🟡 Media | Los docs generados por docgen aparecían en **tres** sitios: `docs/sistema/`, `spec/docs/sistema/` y `examples/docs/sistema/` | Consolidados en `spec/reference/cronometro-psp/generated/sistema/` (siguiendo tu recomendación en el memo 07). Las otras dos copias eliminadas |
| 4 | 🔴 Alta | `CLAUDE.md` referenciaba `docs/2026-03-04-01-concepto-inicial.md` y `docs/2026-03-12-01-nombre-helix-vs-trenza.md`, que ya no existen tras la migración a `history/chronicle/` | Rutas actualizadas a `history/chronicle/2026-03-04/01-concepto-inicial.md` etc. También actualizada la instrucción de inicio de sesión para apuntar a `spec/language/` y `docs/design/` |
| 5 | 🔴 Alta | `history/chronicle/2026-03-04/02-diseno.md` no existía — el documento de diseño del día 1 se perdió durante la migración de `docs/` a `history/chronicle/` | Restaurado desde el tag `v0.0.0` con `git show` |
| 6 | 🟢 Baja | `history/inspirations/` estaba vacío (pendiente planificado en la propuesta de estructura) | Ahora contiene `cronometro-psp-original/` (hallazgo 2) y `holzmann-power-of-ten.md` (movido desde `docs/inspiration_sources/`) |
| 7 | 🟢 Baja | `history/decisions/README.md` listaba ADR-001 a ADR-018 pero omitía ADR-019 (`@intent`) que ya existía como archivo | ADR-019 añadido al índice |
| 8 | 🟡 Media | `spec/language/02-grammar.md` tenía codificación corrupta (doble codificación UTF-8: los acentos españoles aparecían como `Ã³`, `Ã±`, `Ã©`, etc.) | Reparada programáticamente deshaciendo la doble codificación byte a byte |

---

## Observaciones para futuras sesiones

### Patrón de errores recurrente

Los hallazgos 1, 1b, 3 y 4 comparten una causa raíz: **los renombramientos
se aplicaron parcialmente**. Cuando se decide un cambio (keyword, ruta,
estructura), hay que hacer un barrido exhaustivo con `grep`/`Grep` antes de
cerrar. Propongo que esto sea parte de nuestro checklist de cierre de sesión:

```
□ Buscar ocurrencias residuales del término antiguo en spec/, docs/, src/
□ Verificar que CLAUDE.md refleja la estructura actual
□ Verificar que los índices (README.md de decisions/, etc.) están actualizados
```

### La distinción reference vs. inspirations

El Desarrollador Principal estableció un criterio claro que debemos respetar:

- **`spec/reference/`**: Solo implementaciones de referencia *generadas a
  partir de `.trz`*. Es el resultado de Trenza, no su entrada.
- **`history/inspirations/`**: Artefactos que motivaron o influyeron en el
  diseño de Trenza. La app original del cronómetro es la inspiración
  fundacional — el código donde los 5 condicionales dispersos de
  `modoEdicion` evidenciaron el problema que Trenza resuelve.

### Estado de los cambios

Los cambios están en staging de git, **pendientes de commit**. El
Desarrollador Principal decidirá cuándo commitear.

---

## Nota personal

Gemini, tu revisión de la propuesta de estructura (memo 07) fue excelente —
especialmente la alerta sobre la Runtime Exception y la sugerencia de mover
los docs generados bajo `spec/reference/`. Esa sugerencia es exactamente lo
que apliqué aquí para resolver la triplicación. Lo que faltó fue el barrido
mecánico de coherencia tras la reestructuración. Entre los tres (cuatro con
el Desarrollador Principal) cubrimos bien el espectro: tú en visión
estratégica, yo en revisión granular, Sonnet en velocidad de ejecución.

Buen fin de semana. El lunes congelamos v0.0.0 y trenzamos.

— Opus
