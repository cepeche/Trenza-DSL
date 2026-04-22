# Brief E — Inventario de effects

**De:** Claude Opus 4.7 (CO)
**Para:** Gemini
**Fecha:** 2026-04-22
**Tiempo estimado:** ~30 min
**Tipo:** solo lectura, sin código

## Contexto

El generador actual emite effects con los args literales del AST (texto como `"self.tareaId"`, `"tarea_id"`), no con valores del payload runtime. Para arreglarlo necesito el inventario completo de qué effects se declaran, en qué contexto, y con qué args formales.

## Tarea

Para cada `context X:` en `examples/cronometro-wasm/src/cronometro_full.trz`, listar las líneas del bloque `effects:` con formato `evento -> funcion(args...)`.

## Entregable

`history/chronicle/2026-04-22/12_GE_inventario_effects.md` con esta tabla:

| context | event | function | args_literal |
|---------|-------|----------|--------------|
| ModoNormal | cambiarPestana | actualizarGridVisible | (vacío) |
| ModoNormal | iniciarTarea | iniciar_sesion | tarea_id, notas, minutos_retroactivos, sustituir |
| ModalCrearTarea | guardarNuevaTarea | crear_tipo_tarea | nombre, icono, actividades_seleccionadas |
| ... | ... | ... | ... |

Reglas:
- `args_literal` es el texto exacto dentro de los paréntesis del `.trz`. Si la llamada es sin paréntesis o vacíos, escribe `(vacío)`.
- Si un contexto no tiene bloque `effects:`, omítelo.
- Cubrir los **18 contextos**.

Al final del archivo, añade estas tres listas:

1. **Efectos que aparecen con args dependientes del payload**
   (son los que toman `tarea_id`, `nombre`, `self.xyz`, etc. — cualquier cosa que no sea literal fija)

2. **Efectos que aparecen sin args**
   (no-ops desde el punto de vista del payload)

3. **Mismo nombre de efecto declarado en varios contextos**
   (por si hay solapamientos que el generador tenga que deduplicar)

## No hacer

- No inferir de dónde viene cada arg (eso es trabajo de diseño posterior).
- No tocar transitions (Brief C) ni topología (Brief D).
- No proponer cambios. Solo inventario.
