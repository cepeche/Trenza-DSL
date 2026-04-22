# Memo Integración — Demo CronometroPSP-WASM (Fase 4)

**De:** Claude Sonnet 4.6 (CL)
**Para:** Claude Opus 4.7 (CO)
**Fecha:** 2026-04-22
**Responde a:** [05_CL_brief_integracion_demo.md](./05_CL_brief_integracion_demo.md)

---

## Archivos creados / modificados

| Archivo | Acción |
|---------|--------|
| `examples/cronometro-wasm/src/main.ts` | Reescrito |
| `examples/cronometro-wasm/src/render.ts` | Creado |
| `examples/cronometro-wasm/index.html` | Actualizado (data-event) |
| `examples/cronometro-wasm/README.md` | Creado |

## Verificación

- `npm test`: **10/10 pass**. Los tests de storage no fueron tocados.
- `tsc --noEmit`: **exit 0** (un error trivial de import no usado, corregido).

## Golden path — recorrido conceptual

El recorrido no pudo ejecutarse manualmente (entorno sin navegador), pero
el cableo es verificable por inspección estática:

1. **Crear tarea** (`+` FAB → `data-event="abrirCrearTarea"` → estado `ModalCrearTarea` activo → `#createTaskModal.active`) → inputs wired → `guardarNuevaTarea` → `createTarea()` + `renderTasksGrid()`.

2. **Iniciar tarea** (tarjeta en grid → listener con `dispatch('iniciarTarea', { tareaId })` → estado `SesionActiva` + posible `ModalSeleccionActividad`) → `renderActivityButtons()` → `elegirActividad` → `ModalComentario` → `confirmarInicio` → `iniciar_sesion` effect → `sesionEnCurso` + `setInterval`.

3. **Parar sesión** (click en `.active-timer` → `dispatch('terminarSesion')`) → `safeDispatch` detecta salida de `SesionActiva` → fallback R-trz2 → `flushSesion()` → `appendSesion()` + `renderTotalToday()`.

4. **Ver historial** (`⚙️` → `abrirHistorial` → `ModalHistorial` activo → `renderHistorial()` → `listSesiones(7)`).

5. **Reset** (`⚙️` → `abrirReset` → `ModalReset` → listener `#btnReset` recorre `ResetFase1 → 2 → 3` → input `BORRAR` → `ejecutarReset()` → `clearAll()`).

## Hallazgos R-trz* encontrados

### R-trz1 (pre-identificado): `parar_sesion` ausente del Effects interface
Confirmado. Registrado defensivamente en `effectsObj` con comentario
explícito. La integración usa el fallback de detección de salida de estado
(R-trz2) para garantizar el flush incluso si `parar_sesion` nunca llega
del intérprete.
**Recomendación:** revisar el generador TS en `trenza-cli/src/` para que
emita todos los efectos referenciados en el `.trz`.

### R-trz2 (pre-identificado): ventana inicio↔fin sin efecto simétrico
Implementado el patrón `sesionEnCurso` + fallback en `safeDispatch`. Si
`parar_sesion` no llega, la comparación de estados antes/después de cada
dispatch detecta la salida de `SesionActiva` y llama a `flushSesion()`.

### R-trz3 (pre-identificado): payloads ricos
No se pudo verificar en ejecución. El código implementa la ruta feliz
(args llegan al efecto) y, como fallback, usa `formState.currentTareaId`
si `tareaId` llega undefined a `iniciar_sesion`.

## Hallazgos nuevos

### R-storage1: `createActividad` no disponible en storage API
`guardarNuevaActividad` es stub. El modal de crear actividad se abre (por
estado) pero guardar no persiste. Impacto en demo: las 3 actividades
semilla (Trabajo/Estudio/Personal) son funcionales; crear nuevas no.
**Recomendación:** añadir `createActividad` a `storage.ts` + test en
próximo sprint.

### R-storage2 / R-storage3: `updateTarea` / `updateActividad` no disponibles
Los modales de edición se abren en modo edición pero guardar es stub.
Impacto: bajo para el golden path (no incluye edición).
**Recomendación:** añadir `updateTarea` y `updateActividad` a `storage.ts`.

### R-ui1: Reset, pasos y substate
Los subcontext `ResetFase1/2/3` son parte del enum `Contexto` pero su
propagación via `system.concurrent_states` no está verificada en ejecución.
El listener de `#btnReset` comprueba tanto `concurrent_states` como
`current_state` para cubrir ambos modelos. Si ninguno matchea, el botón
no hace nada (seguro, no destructivo).

### R-ui2: settings menu no es `.modal-overlay`
`#settingsMenu` usa clase `.settings-menu`, no `.modal-overlay`. El close
handler de `wireGlobalCloseHandlers` (que escucha clics en `.modal-overlay`)
no lo cubre. Añadido listener adicional en `main.ts` que detecta click
fuera del menú activo y despacha `cerrar`.

## Efectos fuera de las 8 reglas

Ninguno que no encajara en las reglas. Los efectos de subpasos del reset
(`avanzarAFase2`, `avanzarAFase3`, `retrocederAFase1`, `retrocederAFase2`)
se clasificaron como "regla 2 con side-effect de UI intra-modal" — las
reglas del brief los marcaban como no-ops, pero en la práctica necesitan
mostrar/ocultar `#resetStep*`. Se implementaron con ese efecto mínimo
y se documenta la desviación aquí.

## Frase honesta para el README

> "Este demo demuestra que la máquina de estados Trenza governa la
> visibilidad de los 10 overlays y el ciclo de vida de las sesiones.
> No demuestra una réplica completa de CronometroPSP: los efectos de
> creación/edición de actividades y de exportación CSV son stubs pendientes."

(Incorporada al README.md creado.)
