# Brief — Integración del demo CronometroPSP-WASM (Fase 4)

**Para:** Claude Sonnet 4.6 (CL)
**De:** Claude Opus 4.7 (CO)
**Fecha:** 2026-04-22
**Coordinación:** [00_CO_coord_demo_funcional.md](./00_CO_coord_demo_funcional.md)
**Bloqueado por:** nada. Brief A (port HTML+CSS) y Brief B (storage adapter)
ya están en `paper` (commits [d633dba](https://github.com/cepeche/Trenza-DSL/commit/d633dba)
y [853aaba](https://github.com/cepeche/Trenza-DSL/commit/853aaba)). Fase 2
(capa de overlays) en [faf85b6](https://github.com/cepeche/Trenza-DSL/commit/faf85b6).
**Bloquea a:** nada. Es la integración final.

---

## Estado actual del demo

Tres ladrillos listos, sin cablear entre sí:

1. **HTML real de CronometroPSP** en `examples/cronometro-wasm/index.html`
   con todos los `id` de modal preservados, sin handlers `onclick`.
2. **Storage `localStorage`** en `src/storage.ts` (firma cerrada de 6
   funciones, namespaced bajo `cronometro-psp:v1:`).
3. **Capa de overlays** en `src/overlays.ts` que proyecta el estado Trenza
   al DOM (visibilidad de los 10 modales sin `if`s de UI).

`src/main.ts` actual aún cablea solo los 4 botones del demo viejo
(`btn-iniciar`, `btn-parar`, `btn-config`, `btn-reset`) que **ya no existen**
en el HTML portado. Resultado: la página renderiza la app pero los botones
no hacen nada.

## Tu objetivo

Convertir el demo en réplica funcional del *golden path PSP*: el usuario
crea/elige una tarea, arranca cronómetro, lo para, comentario opcional, ve
el historial, puede resetear. Sin un solo `if` de UI: cableo declarativo
y proyección de estado desde Trenza.

## Entregable

Tres archivos:

1. **`examples/cronometro-wasm/src/main.ts`** — reescrito para usar el
   nuevo HTML, cableo declarativo y storage real.
2. **`examples/cronometro-wasm/src/render.ts`** (nuevo) — funciones de
   render derivadas del estado: `renderTasksGrid()`, `renderHistorial()`,
   `renderTotalToday()`, `renderActiveTimer()`. Se invocan desde efectos
   `cargar_*` / `actualizar*` o tras cualquier dispatch que toque storage.
3. **`examples/cronometro-wasm/README.md`** (nuevo) — explica qué demuestra
   el demo, qué overlays están funcionales vs stub, y cómo ejecutarlo.

Y un memo de cierre:

4. **`history/chronicle/2026-04-22/06_CL_memo_integracion_done.md`** con
   los hallazgos y el recorrido manual del golden path (qué funcionó, qué
   no, capturas conceptuales).

## Convención de cableo declarativo (decisión cerrada)

El HTML portado tiene botones sin `onclick`. Cableo por delegación:

- Añade `data-event="<nombreEvento>"` a cada botón/elemento clicable que
  deba disparar un dispatch Trenza.
- Si el evento lleva payload, añade `data-payload-<key>="<valor>"` (uno o
  varios). El handler lee todos los `data-payload-*` y construye el objeto.
- Para inputs (campos de texto, selects), añade `data-input-event="<actualizarX>"`.
  Un único delegado en `input` lee el `value` y dispatcha.

Patrón en `main.ts` (ilustrativo, no copies literal sin pensar):

```ts
document.addEventListener('click', (e) => {
  const el = (e.target as HTMLElement).closest<HTMLElement>('[data-event]');
  if (!el) return;
  const event = el.dataset.event!;
  const payload = extractPayload(el); // lee data-payload-*
  dispatch(event, payload);
});

document.addEventListener('input', (e) => {
  const el = e.target as HTMLInputElement;
  const event = el.dataset.inputEvent;
  if (!event) return;
  dispatch(event, { valor: el.value });
});
```

Esto significa que **modificarás el `index.html`** para añadir los
`data-event` correspondientes. Eso es OK — el contrato con Brief A era
preservar los `id` y eliminar los `onclick`; añadir atributos `data-*` es
ortogonal.

## Clasificación de los efectos (firma cerrada)

`CronometroPSP_out.ts` exporta `interface Effects` con ~50 entradas. Para
no perderte, aplica estas **reglas en orden**:

1. **¿Está en `makeOverlayEffectStubs()` de `overlays.ts`?** → ya cubierto.
   No reimplementes. Importa el spread como ya hace `main.ts`.
2. **¿Es una transición pura (`avanzarA*`, `retrocederA*`, `cambiarA7Dias`,
   `cambiarA30Dias`)?** → no-op. La proyección de estado actualiza la UI.
3. **¿Es input binding (`actualizar*`, `seleccionar*`, `filtrar*`,
   `marcar*`, `toggle*`, `cambiarPestana`)?** → guarda en un objeto
   `formState` en memoria de `main.ts`. No persiste. Lee de ahí en
   `guardar*` para escribir a storage.
4. **¿Es "guardar" (`guardarNuevaTarea`, `guardarNuevaActividad`,
   `guardarEdicion`, `guardarEdicionActividad`)?** → llama al método
   correspondiente de `storage.ts`. Después invoca `renderTasksGrid()` o
   el render que aplique.
5. **¿Es "cargar" (`cargar_historial`, `cargar_tiempo_acumulado`)?** →
   llama a `storage.listSesiones(...)` y luego al render correspondiente.
6. **¿Es `ejecutarReset` o `reset_datos`?** → `storage.clearAll()` +
   re-render del grid y total.
7. **¿Es externo no implementado (`descargar_csv`, `exportarCSV`,
   `verificar_conexion`)?** → no-op con `log()`. Documenta en el README
   que están fuera de alcance.
8. **¿Es `iniciar_sesion` / `calcular_tiempo_transcurrido`?** → ver sección
   "Hallazgo R-trz1" abajo.

Si encuentras un efecto que no encaja en ninguna de las 8 reglas, **flagéalo
en el memo de cierre** y sigue. No improvises.

## Render derivado (decisión cerrada)

`render.ts` exporta cuatro funciones puras: leen `storage.*`, escriben
en el DOM. Sin estado propio.

- `renderTasksGrid()` — pinta `#gridFrecuentes` desde `storage.listTareas()`
  y `storage.listActividades()`. Cada tarjeta lleva
  `data-event="iniciarTarea"` `data-payload-tareaId="<id>"`.
- `renderHistorial()` — pinta el contenido de `#historialModal` desde
  `storage.listSesiones(7)` o `(30)` según el estado activo
  (`Historial7Dias` vs `Historial30Dias` — léelo desde `system.current_state`).
- `renderTotalToday()` — suma duraciones de hoy y escribe en `#totalToday`.
- `renderActiveTimer()` — escribe `#activeTaskName` y `#timerDisplay`
  durante `SesionActiva`.

Convocatoria: estas funciones se llaman desde los efectos correspondientes
**y también** una vez al cargar (en `run()` después de
`syncOverlayVisibility`).

## Hallazgos arquitectónicos a respetar (NO RESOLVER, FLAGEAR)

### R-trz1 — `parar_sesion()` no está en el `Effects` interface

El `.trz` declara en línea 1420 de `cronometro_full.trz`:
```
terminarSesion  -> parar_sesion()
```

Pero `CronometroPSP_out.ts` (interface `Effects`) **no incluye**
`parar_sesion`. Probable bug del compilador (Strand 1 / generador TS) o
diferencia entre la consolidación `cronometro_full.trz` y los `.trz`
fuente. **No lo arregles desde TS.** En el demo, registra el efecto en el
objeto `effects` con un comentario explícito:

```ts
// FLAG R-trz1: parar_sesion declarado en .trz pero ausente del Effects
// interface. Lo registramos defensivamente; si el compilador acaba
// emitiendolo, esta entrada queda compatible.
parar_sesion: () => { /* persistir sesion: ver Hallazgo R-trz1 en memo */ },
```

Y en el memo de cierre, anota como hallazgo formal: "el contrato
`Effects` no es exhaustivo respecto al `.trz`. Recomendación: revisar el
generador TS (`trenza-cli/src/`) para emitir todos los efectos
referenciados". **Esto es para el siguiente sprint, no para este demo.**

### R-trz2 — Persistencia de sesión: ventana inicio↔fin

Una `Sesion` requiere `inicio` y `fin` (ver `storage.types.ts`). Pero
`iniciar_sesion` se dispara al ENTRAR en `SesionActiva` y no hay un
`finalizar_sesion` simétrico que pase `fin`. El demo debe:

- Mantener `let sesionEnCurso: { tareaId, actividadId, inicio } | null = null`
  en memoria de `main.ts`.
- En `iniciar_sesion(...)`: capturar la tupla, **no** persistir aún.
- En `parar_sesion()` (cuando se dispare, ver R-trz1): construir la
  `Sesion` completa con `fin: Date.now()`, llamar a
  `storage.appendSesion(...)`, vaciar `sesionEnCurso`, re-render historial
  y total.
- Si `parar_sesion` nunca llega (por R-trz1), añade un fallback: tras
  cada `dispatch`, si el estado anterior incluía `SesionActiva` y el
  nuevo no, ejecutar el flush. Documenta este fallback en el memo.

### R-trz3 — Payloads ricos en eventos

Si encuentras que `system.dispatch(event, { foo: 'bar' })` no propaga
correctamente el payload a los argumentos de un efecto (p. ej.
`iniciarTarea(arg0)` recibe `undefined`), **flagéalo y degrada el demo**:
fija un `tareaId` por defecto (la primera tarea creada). No toques
`InterpreterWasm`. Esto es para Opus en una sesión posterior.

## Lo que NO debes hacer

- **No modifiques `overlays.ts`** — Fase 2 está commiteada y verificada.
  Si algo no funciona, es candidato a hallazgo, no a refactor.
- **No modifiques `storage.ts` ni los tests.** Si necesitas un método
  nuevo, flagéalo en el memo y degrada.
- **No introduzcas framework alguno** (React, Lit, Alpine, etc.). DOM
  vanilla con delegación.
- **No "mejores" el `.trz`.** Si está incompleto, flag.
- **No reemplaces el demo de 4 botones por algo "mejor" sin haber
  pasado por la convención `data-event`**. La convención es la decisión
  arquitectónica del brief.
- **No modifiques `tsconfig.json`**.
- **No hagas commit ni push del paper/.** El usuario ha dicho
  explícitamente: "Olvida el artículo hasta nueva orden por mi parte".
  Tu commit debe limitarse a `examples/cronometro-wasm/` +
  `history/chronicle/2026-04-22/`.

## Verificación antes de entregar

1. `cd examples/cronometro-wasm && npm test` → 10/10 pass (los tests de
   storage no deben tocarse, deben seguir pasando).
2. `./node_modules/.bin/tsc --noEmit` → exit 0.
3. `npm run dev` y recorrido manual del **golden path mínimo**:
   - Crear una tarea desde el FAB `+`.
   - Pulsar la tarea → modal de selección de actividad → elegir →
     comentario opcional → confirmar → cronómetro arranca.
   - Esperar unos segundos, parar.
   - Abrir historial → ver la sesión recién creada.
   - Abrir reset → confirmar las 3 fases → grid vacío.
4. Capturar lo que funciona y lo que no en el memo, con honestidad.
   Si el golden path falla en algún punto por R-trz*, **eso también es
   entregable válido**: documenta dónde se rompe y por qué.

## Reportar

`history/chronicle/2026-04-22/06_CL_memo_integracion_done.md` con:

- Lista de archivos creados/modificados.
- Resultado de `npm test` y `tsc --noEmit`.
- Recorrido del golden path: paso a paso, qué funciona y qué no.
- Lista explícita de hallazgos R-trz* tropezados (R-trz1 ya está
  identificado, puede que aparezcan más).
- Cualquier efecto que no encajara en las 8 reglas de clasificación.
- Una frase honesta para el README del demo: "Este demo demuestra X.
  No demuestra Y."

## Contexto que necesitas leer antes de empezar

- `examples/cronometro-wasm/src/overlays.ts` — la capa que ya está
  proyectando estado a DOM. Tu cableo se enchufa AQUÍ.
- `examples/cronometro-wasm/src/CronometroPSP_out.ts` — el `Effects`
  interface. Tu objeto `effects` debe satisfacerlo (con stubs +
  implementaciones reales).
- `examples/cronometro-wasm/src/storage.ts` — la API de persistencia.
- `examples/cronometro-wasm/index.html` — el DOM real, dónde pondrás los
  `data-event`.
- `examples/cronometro-wasm/src/cronometro_full.trz` — la spec. Solo si
  necesitas resolver una duda concreta sobre qué evento triggerea qué
  estado.
- Este brief y `00_CO_coord_demo_funcional.md`.

No leas el paper. No toques `paper/`.
