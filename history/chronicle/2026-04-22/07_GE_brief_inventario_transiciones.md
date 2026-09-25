# Brief C — Inventario de transition targets

**De:** Claude Opus 4.7 (CO)
**Para:** Gemini
**Fecha:** 2026-04-22
**Tiempo estimado:** ~30 min
**Tipo:** solo lectura, sin código

## Contexto

Estamos arreglando el generador Rust de `trenza-cli` para que `CronometroPSP_out.rs` compile y la demo WASM funcione end-to-end (Ruta A del análisis 2026-04-22). Antes de tocar el generador necesito un mapa exhaustivo de qué transiciones existen y a qué tipo de target apuntan.

## Tarea

Para cada bloque `context X:` en `examples/cronometro-wasm/src/cronometro_full.trz`, listar todas las líneas del bloque `transitions:` con formato `on EVENTO -> TARGET`.

## Entregable

Un único archivo markdown en `history/chronicle/2026-04-22/10_GE_inventario_transiciones.md` con esta tabla:

| context | event | target_literal |
|---------|-------|----------------|
| ModoNormal | activarEdicion | ModoEdicion |
| ModoNormal | abrirCrearTarea | ModalCrearTarea |
| MenuConfiguracion | cerrar | [close_overlay] |
| ... | ... | ... |

Reglas:
- `target_literal` es el texto exacto del `.trz`, incluyendo corchetes si los hay.
- Si un contexto no tiene bloque `transitions:`, omítelo (no inventes filas).
- Cubrir los **18 contextos** del archivo.
- Al final añade un párrafo corto: ¿qué targets aparecen entre corchetes? (ej. `[close_overlay]`, `[stay]`, `[deactivate]`...). Es el conjunto de marcadores que el generador debe traducir.

## No hacer

- No clasificar nada como base/overlay/concurrent (eso es Brief D).
- No tocar effects (eso es Brief E).
- No proponer cambios al generador. Solo el inventario.
