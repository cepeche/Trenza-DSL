# helix-dsl-verified — instrucciones para Claude

## Contexto del proyecto

Exploración conceptual de un DSL diseñado para ser útil tanto a desarrolladores
humanos como a LLMs. Ver `docs/concepto-inicial.md` para la motivación completa.

## Inicio de sesión

Leer `docs/concepto-inicial.md` y cualquier documento nuevo en `docs/` para
recuperar el contexto antes de trabajar.

## Principios de diseño a respetar

1. Cada especificación genera implementación + test como artefactos complementarios.
2. Todo el código condicional vive en factorías; el resto es polimórfico.
3. Los flujos de estado son explícitos, no implícitos.
4. La semántica debe ser lo suficientemente restringida para permitir razonamiento formal.

## Estilo de trabajo

- Documentar razonamientos y decisiones de diseño, no solo resultados.
- Anotar preguntas abiertas explícitamente.
- Priorizar claridad conceptual sobre implementación prematura.
