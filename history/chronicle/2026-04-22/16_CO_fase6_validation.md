# Fase 6 cerrada — validación end-to-end del demo en navegador

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto + insumo para decisiones de spec `.trz`
**Fecha:** 2026-04-22
**Insumo:** Fase 5 (`15_CO_fase5_demo_rewire.md`) + browser-side eval/click via Claude Preview MCP

---

## Resumen

La validación end-to-end ejecutada con clicks reales en el demo cargado por
Vite (puerto 5173) confirmó que el shim per-spec + bridge JS proyecta el
estado al DOM correctamente para los flujos básicos (overlays simples,
historial con sub-contexto, modo edición). El proceso destapó **dos bugs**
ya cerrados en esta misma fase y **un gap de la spec `.trz`** que requiere
decisión del usuario antes de poder ejercitar Reset 3-fases y los
sub-contextos del Historial desde la UI.

## Verificación empírica

Click-sweep ejecutado vía Claude Preview MCP (`preview_click` +
`preview_eval` para inspeccionar estado tras cada paso):

| ID  | Secuencia                                   | Resultado       |
|-----|---------------------------------------------|-----------------|
| A1-A3 | ⚙️ → Acerca → cerrar                        | ✅ OK           |
| B1-B5 | ⚙️ → Historial → cerrar (×2)                | ✅ OK (post-fix) |
| C1-C5 | ⚙️ → Reset → click btnReset                 | ⚠️ Spec gap    |
| D1-D2 | Crear Tarea (modal abre, navegación interna)| ⚠️ Spec gap    |

`tsc --noEmit` y `vite build` siguen verdes tras los cambios al generador
y a `main.ts`.

## Bugs encontrados y arreglados durante la fase

### Bug 1 (CRÍTICO): generador emitía `push` en transición sub-contexto → padre

**Síntoma observado en navegador:** al cerrar el modal Historial desde
`Historial7Dias`, la pila pasaba de `[ModalHistorial, Historial7Dias]` a
`[ModalHistorial, Historial7Dias, ModalHistorial]` (oscilación) en lugar
de hacer pop limpio.

**Causa raíz:** `classify_target_actions` en `trenza-core/src/generator.rs`
no consultaba el mapa `parent_of` (ya calculado por punto fijo en línea
~821). Cuando el evento `cerrar` declarado en `Historial7Dias` apuntaba a
`ModalHistorial`, la función caía en la rama genérica "el target es un
overlay" → emitía `push(ModalHistorial)`.

**Fix aplicado:** parámetro nuevo `parent_of: &BTreeMap<String, String>`
en `classify_target_actions`, con guarda previa al `match target`:

```rust
// Sub-contexto que vuelve a su propio padre overlay es un pop (subir un
// nivel), no un push — un push re-apilaría el padre encima de su propio
// sub-contexto, produciendo oscilación. Debe comprobarse antes de la
// rama genérica "target es overlay" (que emitiría push).
if let Some(parent) = parent_of.get(decl_ctx) {
    if parent == target {
        return vec!["self.overlay_stack.pop();".to_string()];
    }
}
```

Llamadores actualizados: `dispatch_concurrent` (línea 946) y `dispatch_main`
(línea 1004) ahora pasan `&parent_of`.

**Verificación:** `cargo run -p trenza-cli -- generate ... --out-dir=tmp_regen`
regenera `examples/cronometro-wasm/wasm-shim/src/generated.rs`. Las líneas
696-716 ahora muestran `Historial7Dias.cerrar => self.overlay_stack.pop()`
en lugar del `push(ModalHistorial)` previo. Browser test B1-B5 pasa.

### Bug 2: handler "click fuera de settings" disparaba `cerrar` extra

**Síntoma:** abrir el menú ⚙️ y luego hacer click en cualquier item del
menú (o en cualquier modal lanzado desde él) producía dos despachos:
uno desde el `data-event` del item, otro desde un listener global en
`main.ts` que cerraba el menú al detectar click fuera.

**Causa raíz:** el handler comprobaba "click fuera" leyendo `e.target`
contra `.settings-button`, pero no contra el contenido del propio
`settingsMenu` ni contra modales apilados sobre él.

**Fix aplicado** (`examples/cronometro-wasm/src/main.ts:381-389`):

```ts
document.addEventListener('click', (e) => {
  const menu = document.getElementById('settingsMenu');
  if (!menu?.classList.contains('active')) return;
  const target = e.target as HTMLElement;
  if (menu.contains(target)) return;            // item del propio menú
  if (target.closest('.modal-overlay')) return; // modal sobre settings
  if (target.closest('.settings-button')) return;
  safeDispatch('cerrar');
});
```

Se documentó la guarda inline para que la próxima persona no la elimine
"por innecesaria".

## Hallazgo NO arreglado (decisión del usuario)

### Spec gap `.trz`: `iniciar` declarado pero nunca auto-disparado

`ModalHistorial` declara la transición `on iniciar -> Historial7Dias` y
`ModalReset` declara `on iniciar -> ResetFase1`. Pero `iniciar` no es un
evento del usuario (no hay botón con `data-event="iniciar"`) ni la
spec hace que se dispare automáticamente al `on_entry` del overlay padre.

**Consecuencia visible en el demo:**

- Al abrir Historial el estado queda en `ModalHistorial` (padre), nunca
  baja a `Historial7Dias`. Por tanto el evento `cambiarA30Dias`
  declarado **solo** en `Historial7Dias` no tiene transición disponible
  desde `ModalHistorial` y los clicks en "30 días" son no-ops silenciosos.
- Al abrir Reset el estado queda en `ModalReset` (padre); el código
  `main.ts:351` que comprueba `Contexto.ResetFase1` para decidir qué
  evento despachar nunca encuentra ese estado activo, así que el botón
  "Continuar" no avanza.

**Opciones a discutir con el usuario:**

1. **Solución spec-side:** añadir auto-entrada al primer sub-contexto en la
   gramática (p.ej. `initial: Historial7Dias` dentro de `ModalHistorial`),
   o disparar `iniciar` automáticamente al `[on_entry]`.
2. **Solución demo-side:** añadir `safeDispatch('iniciar')` después de
   abrir cada modal con sub-contextos. Más rápido, pero filtra detalle
   de implementación al cliente.
3. **Solución híbrida:** dejar la spec como está y refactorizar el handler
   `btnReset` para que despache eventos sin depender de la fase, dejando
   que la máquina decida (se requeriría que `avanzarAFase2` esté
   declarado en `ModalReset` con guarda).

Mi recomendación es la opción 1: encaja con el principio "flujos de
estado explícitos" (CLAUDE.md §3) y elimina la asimetría entre overlays
con y sin sub-contextos. Pero es un cambio de gramática que afecta a la
8ª regla (role exhaustiveness) y al Strand 4 de auditoría — preferible
que lo apruebe el usuario antes de tocar `trenza-core`.

## Limpieza aplicada al cierre

- `examples/cronometro-wasm/src/main.ts`: eliminado el dev hook
  `(window as ...).__trenza = system;` que se introdujo para inspección
  desde la consola del navegador (líneas 274-275 originales).
- No se han dejado prints de debug ni comentarios `// TODO browser test`.

## Lo que se ha cerrado con esta fase

- ✅ Demo arranca, renderiza grid de tareas, muestra timer y total del día.
- ✅ Apertura/cierre de overlays simples (Acerca, MenuConfiguracion).
- ✅ Sub-contextos: navegación pop sub→padre verificada (Historial).
- ✅ Modo edición togglea correctamente y refresca grid.
- ✅ Generador emite `pop` en lugar de `push` para transición
  sub-contexto → padre (bug 1).
- ✅ Handler "click fuera de settings" no dispara `cerrar` espurio (bug 2).
- ✅ `tsc --noEmit` + `vite build` siguen pasando.

## Lo que queda pendiente (Fase 7 — decisión de spec)

1. **Decidir tratamiento del gap `iniciar`** entre las tres opciones
   listadas arriba. Sin esto:
   - Reset 3-fases no es ejercitable desde la UI.
   - Cambio entre Historial 7d / 30d no es ejercitable desde la UI.
2. Tras la decisión: regenerar shim, re-ejecutar el sweep C/D y completar
   las 4 columnas restantes de la matriz de validación.
3. Smoke test automatizado (vitest + jsdom + shim) para regression-proofing
   los flujos validados manualmente — opcional, pero deseable antes de
   citar el demo en el paper ONWARD!.

## Métricas

- Bugs críticos cerrados: 2 (1 en `trenza-core`, 1 en demo).
- Spec gaps surfaceados: 1 (sin parche hasta decisión del usuario).
- Iteraciones browser-side hasta sweep limpio: ~12 (mayoría tras fix bug 1).
- Líneas tocadas: `generator.rs` ~10, `main.ts` ~5 (y limpieza dev hook).
- Regeneración del shim: una pasada (`cargo run -p trenza-cli -- generate`).
