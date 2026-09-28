# CronometroPSP — Demo WASM

Demo ejecutable de CronometroPSP cuya máquina de estados fue especificada
en Trenza DSL, compilada a Rust + WASM, y cuya interfaz es una proyección
directa del estado del intérprete.

## Qué demuestra este demo

- La especificación `.trz` de CronometroPSP (18 archivos, 13 contextos)
  **se ejecuta en navegador** como binario WASM.
- Los **10 overlays** declarados en el `.trz` se muestran u ocultan
  exclusivamente por proyección del estado del intérprete. No hay ningún
  `if` de UI que decida la visibilidad: el estado de Trenza *es* la
  visibilidad.
- El **modo edición** (`body.editing`) se activa y desactiva según la
  topología de contextos, no por lógica ad-hoc.
- La **persistencia** de sesiones en `localStorage` está gobernada por
  los efectos declarados en el `.trz`.

## Qué no demuestra

- No reimplementa todos los efectos externos de la app original
  (exportar CSV, buscador de iconos, API de color picker).
- `guardarNuevaActividad` y `guardarEdicion*` son stubs pendientes
  (ver hallazgos en `history/chronicle/2026-04-22/06_CL_memo_integracion_done.md`).
- La app original persiste contra una API PHP; este demo usa `localStorage`.

## Cómo ejecutarlo

```bash
cd examples/cronometro-wasm
npm install
npm run dev
# → http://localhost:5173
```

## Golden path

1. Pulsa **+** → crea una tarea (nombre + emoji).
2. Pulsa la tarjeta → selecciona actividad → comentario opcional → **Iniciar**.
3. El cronómetro corre. Pulsa el header del timer para parar.
4. Abre **⚙️ → Historial** para ver la sesión.
5. Abre **⚙️ → Puesta a cero** y avanza las 3 fases (escribe `BORRAR`).

## Arquitectura

```
cronometro_full.trz
      │
      ▼
InterpreterWasm (Rust/WASM)
      │
      ▼
TrenzaSystem.dispatch(event, payload)
      │
      ├─ syncOverlayVisibility()  ← proyección estado → DOM (overlays.ts)
      ├─ Effects.*                ← acciones de negocio (main.ts)
      └─ render.*                 ← render derivado del estado (render.ts)
```

## Tests

```bash
npm test   # vitest: 10 tests, storage adapter
```

## Estado verificado (2026-09-28, tras `[replace]`)

`e2e/golden-path.mjs` recorre la demo en Chromium con Playwright. Con
`generated.rs` regenerado desde el `cronometro_full.trz` actual, pasan:

- activar y salir del modo edición;
- crear una tarea;
- tocar la tarjeta, que inicia la sesión directamente, como dice la spec;
- parar la sesión;
- abrir y cerrar el historial;
- reset en 3 fases, incluida la cancelación en la fase 2 (decisión del
  25 sep) y la ejecución con `BORRAR`;
- usar el modo edición justo después de un diálogo abierto desde el menú ⚙️.

**Fallo del menú, corregido el 2026-09-28.** Antes, el diálogo que se abría
desde el menú ⚙️ se apilaba sobre `MenuConfiguracion`; al cerrarlo, el menú
seguía abierto y se comía el siguiente clic. `MenuConfiguracion` usa ahora
el destino `[replace] X` (cierra el menú y abre X en su lugar), y
`generated.rs` se ha regenerado. Las 14 comprobaciones de
`e2e/golden-path.mjs` pasan.

**Corrección en `snapshot-bridge.ts`.** La UI despacha nombres de acción,
pero el efecto de la propia acción no pasaba por `dispatch()`. Por eso
crear tareas no funcionaba: solo se emitía `crear_tipo_tarea`, que es un
stub. Ahora el puente ejecuta la acción con los valores del payload, salvo
que el motor ya la haya emitido.
