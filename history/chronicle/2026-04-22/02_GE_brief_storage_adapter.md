# Brief B — Adaptador de persistencia `localStorage` para el demo WASM

**Para:** Gemini 3 Flash (G3F)
**De:** Claude Opus 4.7 (CO)
**Fecha:** 2026-04-22
**Coordinación:** [00_CO_coord_demo_funcional.md](./00_CO_coord_demo_funcional.md)
**Sesión paralela:** Brief A (port HTML+CSS) — sin solape de archivos contigo.
**Bloqueado por:** nada. Puedes empezar ya.
**Bloquea a:** integración final por Opus.

---

## Objetivo

Crear la capa de persistencia que usará el demo CronometroPSP-WASM. Sin DOM,
sin Trenza, sin red. Solo `localStorage` con una API estricta y tipada.
Opus la cableará desde efectos Trenza en la integración final.

## Entregable único

Tres archivos en [examples/cronometro-wasm/src/](../../../examples/cronometro-wasm/src/):

1. `src/storage.types.ts` — tipos de dominio.
2. `src/storage.ts` — implementación con la firma exacta de abajo.
3. `src/storage.test.ts` — tests vitest cubriendo los 6 métodos públicos.

Y la configuración mínima para correr los tests:

4. Añadir `vitest` a `devDependencies` en
   `examples/cronometro-wasm/package.json` y un script `"test": "vitest run"`.
5. `examples/cronometro-wasm/vitest.config.ts` con `environment: 'jsdom'`
   (para tener `localStorage` disponible) y `jsdom` también en
   `devDependencies`.

Smoke test: `cd examples/cronometro-wasm && npm test` pasa los tests en verde.

## Tipos de dominio (firma cerrada)

`src/storage.types.ts` debe exportar **exactamente** estos tipos. No añadas
campos "por si acaso", no infieras desde el dominio PSP — Opus extenderá si
hace falta.

```ts
export interface Tarea {
  id: string;          // uuid v4 generado por createTarea
  nombre: string;
  icono: string;       // emoji
  actividadIds: string[];
  creadaEn: number;    // ms epoch
}

export interface Actividad {
  id: string;          // uuid v4 generado en seed o por crearActividad (no parte de este brief)
  nombre: string;
  color: string;       // hex
}

export interface Sesion {
  id: string;          // uuid v4 generado por appendSesion
  tareaId: string;
  actividadId: string;
  inicio: number;      // ms epoch
  fin: number;         // ms epoch
  comentario: string | null;
}
```

## API de `storage.ts` (firma cerrada)

Exporta **exactamente** estas seis funciones. Mismos nombres, mismas firmas,
mismos tipos de retorno. No añadas helpers públicos, no exportes tipos
distintos a los de `storage.types.ts`.

```ts
export function listTareas(): Tarea[];
export function createTarea(input: { nombre: string; icono: string; actividadIds: string[] }): Tarea;
export function listActividades(): Actividad[];
export function appendSesion(input: { tareaId: string; actividadId: string; inicio: number; fin: number; comentario: string | null }): Sesion;
export function listSesiones(dias: number): Sesion[];   // últimas `dias` jornadas, ordenadas por inicio desc
export function clearAll(): void;
```

## Reglas de implementación

1. **Namespacing**: todas las claves bajo el prefijo `cronometro-psp:v1:`.
   Concretamente: `cronometro-psp:v1:tareas`, `cronometro-psp:v1:actividades`,
   `cronometro-psp:v1:sesiones`. Cada clave guarda un JSON array.
2. **Seed inicial**: si `cronometro-psp:v1:actividades` no existe al
   primer `listActividades()`, sembrar 3 actividades por defecto:
   - `{ id: <uuid>, nombre: 'Trabajo',   color: '#0284c7' }`
   - `{ id: <uuid>, nombre: 'Estudio',   color: '#16a34a' }`
   - `{ id: <uuid>, nombre: 'Personal',  color: '#a855f7' }`
   No sembrar tareas ni sesiones.
3. **UUIDs**: usar `crypto.randomUUID()` (disponible en jsdom y navegadores
   modernos; no añadas dependencia `uuid`).
4. **`listSesiones(dias)`**: devuelve sesiones cuyo `inicio >=`
   `Date.now() - dias * 86_400_000`, ordenadas por `inicio` descendente.
   Si `dias <= 0`, devolver `[]`.
5. **`clearAll()`**: borra solo claves bajo el prefijo `cronometro-psp:v1:`.
   No usar `localStorage.clear()` — podría borrar datos de otras apps en
   dev.
6. **Errores**: si `localStorage.getItem` devuelve JSON inválido, **lanzar**
   `Error('storage corrupted: <key>')`. No tragarse silenciosamente.
7. **Sin caché en memoria**. Cada llamada lee de `localStorage`. Es un
   demo; la simplicidad gana.

## Tests requeridos (`src/storage.test.ts`)

Cobertura mínima, un `describe` por método:

- `listTareas` con storage vacío devuelve `[]`.
- `createTarea` añade y se ve en `listTareas`; `id` es uuid v4 válido;
  `creadaEn` está cerca de `Date.now()`.
- `listActividades` siembra las 3 por defecto en primera llamada;
  segunda llamada no duplica.
- `appendSesion` añade y se ve en `listSesiones(7)`.
- `listSesiones(dias)` filtra correctamente sesiones fuera del rango;
  ordena descendente; `listSesiones(0)` devuelve `[]`.
- `clearAll` deja los tres arrays vacíos pero **no toca** una clave
  externa de prueba (`localStorage.setItem('otra-app:cosa', 'x')`).
- Test de robustez: si se inyecta `'no-json'` en
  `cronometro-psp:v1:tareas`, `listTareas` lanza el error esperado.

`beforeEach` debe limpiar las tres claves del namespace para aislar tests.

## Lo que NO debes hacer

- No tocar `index.html` (lo hace Brief A en paralelo), ni `main.ts`, ni
  `CronometroPSP_out.ts`, ni nada bajo `src/wasm/`.
- No añadir IndexedDB, ni `idb`, ni Dexie. `localStorage` es deliberado:
  síncrono, observable en DevTools, suficiente para el demo.
- No exportar el prefijo `cronometro-psp:v1:` como constante pública
  importable. Es detalle interno; los tests pueden hardcodearlo.
- No añadir migraciones, versionado dinámico, ni hooks de upgrade. El
  `:v1:` en la clave es la versión; si algún día sube, será `:v2:` y
  punto.
- No envolver en una clase `StorageService`. Funciones libres exportadas.
- No hacer commit ni push. Deja los cambios staged o sin stagear; Opus
  los integra en el commit de convergencia.

## Cómo verificar antes de entregar

1. `cd examples/cronometro-wasm && npm install && npm test`.
2. Todos los tests en verde.
3. `npx tsc --noEmit` desde `examples/cronometro-wasm/` no produce errores
   de tipos en los archivos nuevos.

## Reportar

Cuando termines, deja un memo corto en
`history/chronicle/2026-04-22/04_GE_memo_brief_b_done.md` con:
- Lista de archivos creados/modificados.
- Salida de `npm test` (resumen: N tests, todos pass).
- Cualquier desviación de la firma propuesta y justificación (no debería
  haber ninguna).
