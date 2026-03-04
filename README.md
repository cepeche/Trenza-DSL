# helix-dsl-verified

Un DSL diseñado para ser útil tanto a desarrolladores humanos como a LLMs,
inspirado en la estructura de doble hélice del ADN: cada fragmento de
especificación genera simultáneamente su **implementación** y sus **pruebas**,
de forma que ambas se verifican mutuamente.

## Origen

Conversación entre un desarrollador con experiencia desde 1975 y Claude (Sonnet 4.6),
el 4 de marzo de 2026, a raíz de la dificultad para diagnosticar un bug de eventos
touch/click en una aplicación web. Se observó que:

- La complejidad accidental del software moderno dificulta el diagnóstico incluso
  para LLMs con acceso completo al código fuente.
- Los condicionales dispersos (`if modoEdicion`) son un vector de bugs estructural.
- Los lenguajes actuales no expresan explícitamente los flujos de estado ni las
  guardas, obligando al lector (humano o LLM) a reconstruir mentalmente la ejecución.

## Hipótesis de diseño

1. **Doble hélice**: la especificación de un requisito genera *dos artefactos complementarios*
   — implementación y test — que son el reverso algebraico el uno del otro.

2. **Condicionales en factorías**: todo el código condicional vive en métodos factoría.
   El resto del código es polimórfico y no sabe en qué estado está el sistema;
   solo envía mensajes. Esto hace imposible olvidar un caso.

3. **Flujos de estado explícitos**: las transiciones entre estados son ciudadanos
   de primera clase del lenguaje, no lógica implícita dispersa en funciones.

4. **Verificabilidad formal**: la semántica del DSL debe ser lo suficientemente
   restringida para permitir razonamiento formal, no solo ejecución.

## Estado

Exploración conceptual. Ver `docs/` para el desarrollo de las ideas.

## Documentos

- [`docs/concepto-inicial.md`](docs/concepto-inicial.md) — contexto y motivación detallados
- [`docs/diseno.md`](docs/diseno.md) — diseño del lenguaje (en desarrollo)
