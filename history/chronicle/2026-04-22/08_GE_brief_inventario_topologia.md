# Brief D — Inventario de topología de contextos

**De:** Claude Opus 4.7 (CO)
**Para:** Gemini
**Fecha:** 2026-04-22
**Tiempo estimado:** ~20 min
**Tipo:** solo lectura, sin código

## Contexto

Para arreglar el generador necesito clasificar cada contexto del `.trz` por su rol topológico (base / overlay / concurrent / sub-contexto). El generador necesita esta clasificación para emitir la operación correcta cuando una transición apunta a un contexto: reemplazar base, push overlay, activar concurrent, etc.

## Tarea

Leer el bloque `system CronometroPSP:` (líneas 245-292 de `examples/cronometro-wasm/src/cronometro_full.trz`) y producir una tabla con los 18 contextos del archivo, marcando cuáles aparecen en cada sección (`contexts:`, `concurrent:`, `overlays:`) y cuáles no aparecen en ninguna.

## Entregable

`history/chronicle/2026-04-22/11_GE_inventario_topologia.md` con esta tabla:

| context_name | tipo |
|--------------|------|
| ModoNormal | base |
| ModoEdicion | base |
| SesionActiva | concurrent |
| MenuConfiguracion | overlay |
| ModalAcercaDe | overlay |
| ... | ... |
| Historial7Dias | sub-contexto (¿de quién?) |
| ResetFase1 | sub-contexto (¿de quién?) |

Reglas:
- `tipo ∈ {base, overlay, concurrent, sub-contexto}`.
- `base` = aparece en `contexts:` del system block.
- `overlay` = aparece en `overlays:`.
- `concurrent` = aparece en `concurrent:`.
- `sub-contexto` = no aparece en ninguno de los tres. En la columna añade entre paréntesis cuál crees que es su contexto padre, basándote en su nombre (`ResetFase*` → `ModalReset`, `Historial*Dias` → `ModalHistorial`). Si no está claro, escribe "?".

Al final añade un párrafo corto:
- ¿Cuántos contextos hay de cada tipo?
- ¿Hay sub-contextos? Si sí, ¿están declarados en alguna sintaxis dentro del `.trz` o solo se infieren del nombre? Busca en el archivo `nested:`, `substates:`, `parent:` o sintaxis similar.

## No hacer

- No tocar transitions (eso es Brief C).
- No tocar effects (eso es Brief E).
- No proponer cambios al `.trz` ni al generador. Solo el inventario.
