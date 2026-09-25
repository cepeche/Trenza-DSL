# Brief A — Port HTML+CSS reconocible al demo WASM

**Para:** Gemini 3 Flash (G3F)
**De:** Claude Opus 4.7 (CO)
**Fecha:** 2026-04-22
**Coordinación:** [00_CO_coord_demo_funcional.md](./00_CO_coord_demo_funcional.md)
**Sesión paralela:** Brief B (storage adapter) — sin solape de archivos contigo.
**Bloqueado por:** nada. Puedes empezar ya.
**Bloquea a:** integración final por Opus.

---

## Objetivo

Sustituir el `index.html` actual del demo (112 líneas, 4 botones, look slate)
por una copia limpia del HTML original de CronometroPSP, con su CSS literal,
de forma que **la página se vea idéntica a la app real** aunque ningún botón
haga nada todavía. El cableado lo hará Opus en Fase 2.

## Entregable único

Dos archivos en [examples/cronometro-wasm/](../../../examples/cronometro-wasm/):

1. `examples/cronometro-wasm/index.html` — HTML portado (ver reglas abajo).
2. `examples/cronometro-wasm/public/styles.css` — copia literal de
   `history/inspirations/cronometro-psp-original/frontend/css/styles.css`.

Smoke test: `cd examples/cronometro-wasm && npm run dev` carga la página
visualmente reconocible como CronometroPSP. Todos los botones inertes (no
hacen nada al click). La consola no muestra errores 404 de CSS o assets.

## Fuente

[history/inspirations/cronometro-psp-original/frontend/index.html](../../inspirations/cronometro-psp-original/frontend/index.html)
(358 líneas) y [css/styles.css](../../inspirations/cronometro-psp-original/frontend/css/styles.css).

## Reglas de porte (estrictas)

1. **Copia literal del CSS.** No reformatees, no re-tematices, no
   "modernices". Si el CSS referencia rutas relativas (`url(...)`), ajusta
   solo lo mínimo necesario para que vite las sirva desde `public/`.
2. **HTML: copia literal salvo dos limpiezas obligatorias:**
   - **Eliminar todos los `onclick="..."` inline.** Sin excepción. El
     wiring será declarativo en TS, gobernado por Trenza. Deja el botón
     pero quita el handler. Ejemplo: `<button onclick="toggleSettingsMenu()">`
     → `<button>` (conservando clases e id).
   - **Eliminar el `<script>` final que carga `js/app.js` y `js/api-client.js`.**
     Sustituirlo por `<script type="module" src="/src/main.ts"></script>`
     (ya está así en el HTML actual del demo).
3. **Preservar literalmente todos los `id` de modales y elementos clave.**
   Esta tabla es contractual — Fase 2 depende de ella:

   | id que DEBE conservarse |
   |------------------------|
   | `settingsMenu` |
   | `activityModal` |
   | `createTaskModal` |
   | `createActivityModal` |
   | `commentModal` |
   | `aboutModal` |
   | `resetModal` |
   | `editTaskModal` |
   | `editActivityModal` |
   | `historialModal` |
   | `tabs`, `tabsContent`, `gridFrecuentes` |
   | `activeTaskName`, `timerDisplay` |
   | `bottomComment`, `totalToday` |
   | `editModeButton` |

4. **Conservar la convención `class="modal-overlay"` y la mecánica
   `.active`.** El CSS original asume que un modal se muestra añadiendo la
   clase `active`. No cambies esto — Fase 2 la usa.
5. **Idioma**: el HTML original está en español (`<html lang="es">`,
   textos UI). Mantenlo. La documentación pública del proyecto está en
   inglés desde marzo; la UI de la app no.
6. **Ruta del CSS**: el `<link>` debe apuntar a `/styles.css` (vite servirá
   `public/styles.css` desde la raíz).

## Lo que NO debes hacer

- No introducir React, Vue, Svelte ni ningún framework. Vanilla DOM.
- No tocar `src/main.ts`, `src/CronometroPSP_out.ts`, `src/storage.ts` (que
  está creando Brief B en paralelo) ni nada bajo `src/wasm/`.
- No "mejorar" la accesibilidad ni el SEO ni el responsive más allá de lo
  que ya está. Cualquier mejora introduce divergencia con la app real y
  debilita la afirmación "es la misma app".
- No añadir tests E2E (Playwright, Cypress). El smoke test manual basta
  para este brief.
- No hacer commit ni push. Deja los cambios staged o sin stagear; Opus los
  integra en el commit de convergencia.

## Cómo verificar antes de entregar

1. `cd examples/cronometro-wasm && npm install && npm run dev`.
2. Abrir `http://localhost:5173`. La página debe verse como CronometroPSP
   (el header con timer, FAB ✏️, FAB +, barra inferior, botón ⚙️).
3. Abrir DevTools → Console. **Cero errores rojos**. (Habrá errores de
   wiring porque el TS aún no conoce los nuevos botones — eso lo arregla
   Opus. Lo que NO debe haber: 404 de CSS, parse errors de HTML.)
4. Click en cualquier botón → no pasa nada. Correcto. Inerte por diseño.

## Reportar

Cuando termines, deja un memo corto en
`history/chronicle/2026-04-22/03_GE_memo_brief_a_done.md` con:
- Lista de archivos creados/modificados.
- Captura conceptual: "la página renderiza X modales ocultos + Y FABs +
  header con timer".
- Cualquier `id` del HTML original que tuvieras que **renombrar** (no
  debería pasar ninguno; si pasa, justificar).
- Cualquier ruta de asset (imágenes, fuentes) que el CSS pidiera y no
  existiera.
