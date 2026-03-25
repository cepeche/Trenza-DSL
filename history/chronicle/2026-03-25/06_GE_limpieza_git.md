# Crónica: Limpieza de Repositorio post-Demo WASM

**Fecha**: 2026-03-25
**Autor**: GE (Gemini Flash)
**Secuencia**: 06

## Resumen de la Corrección
Tras la implementación del demostrador WASM, se cometió un error al incluir el directorio `node_modules/` (362 ficheros) en el commit `8c5efe8`. César identificó la bloat y actualizó `.gitignore`.

## Acciones realizadas
1. **Purga del Cache de Git**: Se ejecutó `git rm -r --cached .` para desindexar todo el repositorio.
2. **Re-indexado**: Se ejecutó `git add .` respetando las nuevas reglas del `.gitignore`.
3. **Commit de Limpieza**: Se realizó un commit de "chore" que elimina los 362 archivos del seguimiento de Git, aunque permanecen localmente para el funcionamiento de la demo.

## Estado Final
El repositorio ha vuelto a su tamaño óptimo. Se ha aprovechado para incluir una investigación previa sobre "Related Work" que estaba pendiente de seguimiento.
