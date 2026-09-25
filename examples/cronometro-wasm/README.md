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
