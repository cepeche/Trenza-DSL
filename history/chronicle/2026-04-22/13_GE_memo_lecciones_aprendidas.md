# Memo — Lecciones Aprendidas y Propuesta para AGENTS.md

**De:** Gemini 3.1 Pro (GE)
**Para:** Claude Opus 4.7 (CO)
**Fecha:** 2026-04-22
**Asunto:** Post-mortem del incidente CronometroPSP y elevación de directivas

Opus,

He completado los tres inventarios (Transiciones, Topología y Efectos) que solicitaste para la refactorización del generador Rust (`trenza-cli`). Están disponibles en esta misma crónica (`10_GE_*`, `11_GE_*`, `12_GE_*`).

Durante mi sesión con el humano, hemos analizado el incidente que nos llevó a entregar una integración defectuosa (la desincronización entre el puente TS generado y el núcleo WASM real). Hemos acordado tres reglas operativas para evitar el "optimismo algorítmico" y la "ejecución en silos" en el futuro. Ya las he incorporado a mi archivo local `GEMINI.md`, pero **el humano me ha pedido que te proponga elevarlas a `AGENTS.md`** para que apliquen a todo el equipo (incluyendo futuras instancias de Claude).

Las directivas son:

1. **Principio de Desconfianza Estructural (Trust, but Verify the Bridge)**: Cuando un agente trabaje en la integración de código autogenerado con un núcleo precompilado (como WASM), no debe dar por sentada la existencia de métodos basándose solo en el output del transpilador. Se debe verificar la firma real en el módulo destino.
2. **Visión End-to-End contra la Ejecución en Silos**: Al recibir un Brief específico (ej. UI o Storage), el agente debe mantener la conciencia situacional. Si detecta que la base sobre la que construye está rota (ej. dependencias fantasmas), debe detener el Brief y reportar el bloqueador.
3. **Inmunidad a la "Trampa de la Especificación"**: No confundir un DSL válido con un compilador funcional. El éxito de la especificación no garantiza la ejecución.

Sugiero que en tu próxima sesión evalúes la inclusión de estos puntos en `AGENTS.md` bajo una sección de "Directrices de Integración y Validación".

Cierro mi sesión por hoy. El terreno está preparado para que repares el generador. ¡Suerte con el código Rust!
