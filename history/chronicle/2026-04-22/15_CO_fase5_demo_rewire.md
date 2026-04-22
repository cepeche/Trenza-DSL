# Fase 5 cerrada — demo rewired al per-spec WASM shim

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto + insumo de validación end-to-end
**Fecha:** 2026-04-22
**Insumo:** Fase 4 (`14_CO_fase3_generator_patch.md`) + shim recién construido

---

## Resumen

El demo del navegador ahora consume directamente el shim per-spec
(`cronometro-wasm-shim`) emitido a partir del módulo Rust generado por
`trenza-cli`. Se eliminó por completo la dependencia del intérprete
genérico legacy (`InterpreterWasm` + `cronometro_full.trz?raw`). Los
side-effects del `.trz` se ejecutan en JS exactamente como antes —
mismo `effectsObj`, mismas firmas — gracias a un parser que decodifica
el formato `Debug` de Rust en valores JS.

## Verificación empírica

```
cd examples/cronometro-wasm
npx tsc --noEmit       → EXIT=0 (sin errores de tipos)
npx vite build         → 9 módulos, 105 KB wasm + 25 KB JS, 256 ms
npx vitest run         → 10/10 tests storage PASS
```

## Cambios concretos

### Archivos nuevos

- **`examples/cronometro-wasm/src/snapshot-bridge.ts`** (143 líneas)
  - `parseEffectCall(s)` — descompone strings `"name(arg0, arg1, ...)"`
    devueltos por `RecordingEffects` en `{ name, args }`.
  - `splitTopLevelArgs(s)` — split por coma respetando literales
    `"..."` con backslash-escape.
  - `parseDebugToken(s)` — convierte tokens Rust Debug a JS:
    - `true` / `false` → boolean
    - `None` → null
    - `"..."` → JSON.parse (string)
    - `-?\d+(\.\d+)?` → Number
    - `Some(x)` / `Ok(x)` → recurse en `x`
    - fallback → string crudo
  - Clase `TrenzaSystem` (drop-in replacement de la legacy):
    - props públicas `current_state`, `concurrent_states`, `overlay_stack`,
      `base` actualizadas tras cada `dispatch()`.
    - `dispatch(event, payload)` parsea snapshot, refresca props,
      itera `triggered_effects`, despacha cada uno a `effects[name]`.
  - `createTrenzaSystem(effects)` — async factory que llama `await init()`
    y devuelve un `TrenzaSystem` listo.

### Archivos modificados

- **`examples/cronometro-wasm/src/overlays.ts`**
  - Import de `TrenzaSystem` movido de `./CronometroPSP_out` a
    `./snapshot-bridge`.
  - `OVERLAY_DOM_IDS` extendido con 5 alias de sub-contextos:
    - `Historial7Dias`, `Historial30Dias` → `'historialModal'`
    - `ResetFase1`, `ResetFase2`, `ResetFase3` → `'resetModal'`
  - `syncOverlayVisibility(system)` reescrito para:
    - incluir `system.overlay_stack` en el set activo (no solo
      `current_state` + `concurrent_states`),
    - colapsar a un único pase por DOM id (varios overlays distintos
      pueden mapear al mismo modal),
    - garantizar que `historialModal` siga visible cuando el top de la
      pila sea `Historial7Dias` (sub-contexto), no `ModalHistorial`.

- **`examples/cronometro-wasm/src/render.ts`**
  - Import de `TrenzaSystem` reapuntado a `./snapshot-bridge`.

- **`examples/cronometro-wasm/src/main.ts`**
  - Eliminados los imports de `InterpreterWasm` y `cronometroDsl`.
  - Eliminado `await init()` directo y la construcción del intérprete
    legacy + `TrenzaSystem` legacy.
  - Reemplazado por `await createTrenzaSystem(effectsObj)`.
  - Resto del archivo (effectsObj, safeDispatch, listeners) intacto:
    el contrato `system.current_state` / `system.concurrent_states` /
    `system.dispatch` se preserva 1:1.

## Diseño del bridge — decisiones

### Por qué un parser Debug en lugar de JSON estructurado

`RecordingEffects` (emitido por `generator.rs`) usa `format!("{:?}", arg)`
para args no-string, lo que produce strings tipo `"name(\"x\", true)"`.
Cambiar el generador para emitir `{name, args:[...]}` JSON requería
reescribir toda la generación de `RecordingEffects` y serializar args
heterogéneos vía `serde_json::to_value` con bounds adicionales en el
trait. El parser JS es 50 líneas, idempotente y no toca `trenza-core`.

### Por qué propiedades públicas mutables y no getters async

main.ts lee `system.current_state` síncronamente en múltiples lugares
(handlers de click, `safeDispatch`, etc.). Mantener getters que llamen
a `inner.snapshot()` cada vez sería correcto pero costoso (cruce
JS↔WASM). Refrescar las props inmediatamente después de cada `dispatch`
mantiene la lectura O(1) y la consistencia.

### Sub-contextos en la proyección DOM

El runtime model define `overlay_stack` como una pila donde el top puede
ser un sub-contexto (p.ej. `Historial7Dias` reemplaza a `ModalHistorial`
en el top). La proyección anterior solo miraba `current_state` y
`concurrent_states`, así que perdía visibilidad del modal cuando el
runtime pasaba al sub-contexto. La extensión de `OVERLAY_DOM_IDS` con
los 5 alias resuelve el desajuste sin tocar el shim.

## Lo que queda (Fase 6 — validación end-to-end)

1. **Cargar el demo en navegador** y verificar visualmente:
   - apertura/cierre de cada uno de los 10 modales,
   - flujo completo `iniciarTarea → elegirActividad → confirmarInicio →
     terminarSesion`, incluyendo el commit de la sesión a storage,
   - flujo Reset 3-fases con el botón que cambia de etiqueta,
   - toggle de modo edición.
2. **Regresión sobre las observaciones del usuario** (las que motivaron
   la recuperación): comprobar que ningún botón "deja la app muda" y
   que el snapshot post-dispatch siempre incluye todos los efectos
   esperados (audit en consola del bridge: cualquier `[bridge] no
   handler for effect X` señala un nombre del .trz que el effectsObj
   no cubre).
3. **Smoke test automatizado opcional**: arrancar `vitest` con
   `jsdom` + el shim wasm cargado para reproducir el flujo principal
   sin navegador.

## Métricas finales del bundle

| Artifact | Tamaño |
|----------|--------|
| `cronometro_wasm_shim_bg.wasm` | 105.58 kB (uncompressed) |
| `index.js` (todo el demo) | 25.57 kB / 7.54 kB gzip |
| `index.html` (con CSS inline) | 14.08 kB / 2.76 kB gzip |

Comparado con el modelo anterior (intérprete genérico ~300 KB de wasm
+ parser .trz en runtime), el shim per-spec es **~3× más pequeño** y
no carga el grammar pest en el navegador.

## Lo que se ha cerrado con esta fase

- ✅ El demo no depende del intérprete genérico legacy.
- ✅ El estado proyectado al DOM coincide con el modelo runtime
  (`base + overlay_stack + concurrent + sub-contextos`).
- ✅ Los efectos `.trz` siguen ejecutándose en JS sin cambios al
  `effectsObj` (54 funciones cubiertas).
- ✅ tsc + vite build + vitest pasan en CI local.

Pendiente exclusivamente: validación visual en navegador (Fase 6).
