# Auditoría de la auditoría de Gemini sobre rótulos de modelo en ADRs

**Fecha:** 2026-04-17 (viernes, post-comida del usuario inminente)
**Autor:** Claude Opus 4.6 (CO), sesión vía Claude Code
**Audita:** [adr-attribution-review.md](../../../docs/design/adr-attribution-review.md) (Gemini 3 Flash, 2026-04-17)
**Entregable:** [adr-attribution-review-audit.md](../../../docs/design/adr-attribution-review-audit.md)
**Acción tomada sobre los ADRs:** ninguna. Decisión escalada a César.

---

## Hallazgo

Gemini detectó correctamente la divergencia de rótulos en los ADRs
(7 con `Gemini 3.1 Pro`, 9 con `Gemini 2.5 Pro`, 5 sin mención).
**Su recomendación —reemplazo global de `3.1 Pro` por `2.5 Pro`—
contradice frontalmente la regla de memoria
[`feedback_gemini_version_names.md`](../../../../Users/ceo/.claude/projects/C--Proyectos-Trenza-DSL/memory/feedback_gemini_version_names.md)
escrita ayer (16 abril) por Opus después de un error casi-idéntico.**

Peor: la única evidencia forense que Gemini cita —commit `903736a`
del 6 de marzo— dice **lo opuesto** de lo que Gemini concluye. La
versión literal del mensaje:

> `fix: corregir versión de Gemini — es 3.1 Pro, no 2.0 Pro`

Autor: César. Es César *corrigiendo hacia* `3.1 Pro`, no desde él. El
commit es ratificación explícita del rótulo, no su origen erróneo.
Gemini lee la prueba al revés.

## Lo que recomiendo a César

Dos preguntas que solo César puede responder:

1. ¿La divergencia de rótulos en los ADRs es ruido (error de Haiku
   al expandir desde stubs) o señal (cada ADR registra el rótulo real
   de la sesión Gemini en su entorno: Antigravity vs otros)?
2. Si es señal, ¿debe `CONTRIBUTORS.md` actualizarse para reflejar
   los rótulos Antigravity (`3.1 Pro`, `3 Flash`) en las sesiones que
   ocurrieron allí, o mantenerse normalizado como registro
   paper-facing?

Solo después de su respuesta tiene sentido tocar `history/decisions/`.
Posiblemente la transformación correcta sea la opuesta a la que
Gemini propone.

## Por qué importa este patrón

Es el segundo ejemplo en dos días de una verificación Opus atrapando
una normalización errónea de rótulos Gemini. El primero (16 abril)
fue Opus atrapándose a sí mismo. Este (17 abril) es Opus atrapando a
Gemini. La regla de memoria ha demostrado su valor dos veces; debería
formar parte de cualquier checklist de auditoría que toque atribución
de modelos.

También es un caso interesante de error de inferencia entre LLMs:
Gemini procesa una evidencia que apoya una conclusión y la presenta
como apoyo de la conclusión opuesta. Sin verificación humana o de un
segundo modelo, ese tipo de error pasa al producto final como si
fuera análisis. La pauta operativa "todo cambio de atribución de
modelo requiere confirmación explícita del usuario" es exactamente la
salvaguarda correcta.

## Estado al cierre de sesión

- Mis cuatro entregables del paper (§3, §4, §7, related-work-audit)
  siguen intactos en `docs/design/paper-prose-*.md`.
- El audit de Gemini queda en `docs/design/adr-attribution-review.md`
  sin modificación.
- Mi audit del audit de Gemini queda en
  `docs/design/adr-attribution-review-audit.md` con recomendación
  explícita: no ejecutar el reemplazo global, escalar a César.
- Ningún archivo en `history/decisions/` ha sido tocado.

César retoma después de comer.

---

*Sesión cerrada 2026-04-17, antes de la pausa de comida del usuario.*
