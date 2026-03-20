# Trenza DSL

Un DSL (anteriormente conocido como Helix) diseñado para ser útil tanto a desarrolladores humanos como a LLMs. Está inspirado en la estructura de una **trenza** de cuatro hebras: cada fragmento de especificación genera y consolida simultáneamente su **implementación**, sus **pruebas**, su **esquemático/documentación** y sus **requisitos**, de forma que las cuatro se verifican mutuamente y nunca se desincronizan.

## Estado actual
**Versión actual:** v0.0.1

Esta es la primera especificación completa y validada de Trenza. Todos los documentos iniciales de diseño han sido consolidados y reestructurados jerárquicamente para asentar las bases del desarrollo.

## Origen

Conversación entre un desarrollador con experiencia desde 1975 y Claude (Sonnet 4.6), el 4 de marzo de 2026, a raíz de la dificultad para diagnosticar un bug de eventos touch/click en una aplicación web. Se observó que:

- La complejidad accidental del software moderno dificulta el diagnóstico incluso para LLMs con acceso completo al código fuente.
- Los condicionales dispersos (`if modoEdicion`) son un vector de bugs estructural.
- Los lenguajes actuales no expresan explícitamente los flujos de estado ni las guardas, obligando al lector (humano o LLM) a reconstruir mentalmente la ejecución.

## Hipótesis de diseño

1. **La Trenza**: la especificación de un requisito genera *cuatro artefactos complementarios* (las "cuatro hebras") — implementación, test, esquemático y **requisitos** — que son proyecciones unificadas del mismo artefacto raíz estricto.
2. **Condicionales en factorías**: todo el código condicional vive en métodos factoría. El resto del código es polimórfico y no sabe en qué estado está el sistema; solo envía mensajes. Esto hace imposible olvidar un caso.
3. **Flujos de estado explícitos**: las transiciones entre estados y los eventos de su ciclo de vida son ciudadanos de primera clase del lenguaje, no lógica implícita dispersa en funciones.
4. **Verificabilidad formal**: la semántica del DSL debe ser lo suficientemente restringida para permitir razonamiento formal, no solo ejecución.

## Estructura del proyecto

Tras la consolidación v0.0.0, la documentación y especificación se divide en:

- `spec/language/` — Especificación formal del lenguaje (Gramática, Intenciones, etc.)
- `spec/reference/` — Implementaciones y ejemplos de referencia (ej. proyecto cronómetro)
- `docs/manual/` — Manuales de usuario y guías rápidas
- `history/chronicle/` — Registro cronológico de la evolución y diseño (incluyendo las conversaciones fundacionales)
- `history/meta/` — Reflexiones metafísicas y directrices de propiedad intelectual
- `history/decisions/` — Architectural Decision Records (ADRs)
