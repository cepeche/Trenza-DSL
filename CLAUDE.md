# Trenza-DSL — instrucciones para Claude

> Anteriormente conocido como "Helix" (ver docs anteriores a marzo 2026).

## Contexto del proyecto

Exploración conceptual de un DSL ("Trenza") diseñado para ser útil tanto a
desarrolladores humanos como a LLMs. Ver `history/chronicle/2026-03-04/01-concepto-inicial.md`
para la motivación completa. Ver `history/chronicle/2026-03-12/01-nombre-helix-vs-trenza.md`
para la justificación del cambio de nombre.

## Extensiones oficiales

- `.trz` — archivos fuente Trenza
- `.tzp` — paquetes verificables Trenza

## Inicio de sesión

Leer `history/chronicle/2026-03-04/01-concepto-inicial.md` y los documentos en
`spec/language/` y `docs/design/` para recuperar el contexto antes de trabajar.

## Principios de diseño a respetar

1. Cada especificación genera implementación + test como artefactos complementarios.
2. Todo el código condicional vive en factorías; el resto es polimórfico.
3. Los flujos de estado son explícitos, no implícitos.
4. La semántica debe ser lo suficientemente restringida para permitir razonamiento formal.

## Estilo de trabajo

- Documentar razonamientos y decisiones de diseño, no solo resultados.
- Anotar preguntas abiertas explícitamente.
- Priorizar claridad conceptual sobre implementación prematura.
