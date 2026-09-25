# Inventario de transition targets — CronometroPSP

Este documento contiene el mapa exhaustivo de transiciones declaradas en `cronometro_full.trz`.

| context | event | target_literal |
|---------|-------|----------------|
| ModoNormal | activarEdicion | ModoEdicion |
| ModoNormal | abrirCrearTarea | ModalCrearTarea |
| ModoNormal | abrirMenuConfiguracion | MenuConfiguracion |
| ModoNormal | iniciarTarea | SesionActiva |
| ModoNormal | seleccionarTipoTarea | ModalComentario |
| ModoNormal | elegirActividad | ModalSeleccionActividad |
| ModoEdicion | desactivarEdicion | ModoNormal |
| ModoEdicion | abrirEditarTarea | ModalEditarTarea |
| ModoEdicion | abrirEditarActividad | ModalEditarActividad |
| ModoEdicion | abrirCrearTarea | ModalCrearTarea |
| ModoEdicion | abrirMenuConfiguracion | MenuConfiguracion |
| SesionActiva | sesionFinalizada | [deactivate] |
| SesionActiva | terminarSesion | ModoNormal |
| MenuConfiguracion | abrirCrearActividad | ModalCrearActividad |
| MenuConfiguracion | abrirHistorial | ModalHistorial |
| MenuConfiguracion | abrirAcercaDe | ModalAcercaDe |
| MenuConfiguracion | abrirReset | ModalReset |
| MenuConfiguracion | cerrar | [close_overlay] |
| ModalComentario | confirmarInicio | [close_overlay] |
| ModalComentario | cancelar | [close_overlay] |
| ModalSeleccionActividad | elegirActividad | ModalComentario |
| ModalSeleccionActividad | cancelar | [close_overlay] |
| ModalCrearTarea | guardarNuevaTarea | [close_overlay] |
| ModalCrearTarea | cancelar | [close_overlay] |
| ModalEditarTarea | guardarEdicion | [close_overlay] |
| ModalEditarTarea | cancelar | [close_overlay] |
| ModalEditarActividad | guardarEdicionActividad | [close_overlay] |
| ModalEditarActividad | cancelar | [close_overlay] |
| ModalCrearActividad | guardarNuevaActividad | [close_overlay] |
| ModalCrearActividad | cancelar | [close_overlay] |
| ModalHistorial | iniciar | Historial7Dias |
| ModalHistorial | cerrar | [close_overlay] |
| Historial7Dias | cambiarA30Dias | Historial30Dias |
| Historial7Dias | cerrar | ModalHistorial |
| Historial30Dias | cambiarA7Dias | Historial7Dias |
| Historial30Dias | cerrar | ModalHistorial |
| ModalReset | iniciar | ResetFase1 |
| ModalReset | cerrar | [close_overlay] |
| ResetFase1 | avanzarAFase2 | ResetFase2 |
| ResetFase1 | cerrar | ModalReset |
| ResetFase2 | avanzarAFase3 | ResetFase3 |
| ResetFase2 | retrocederAFase1 | ResetFase1 |
| ResetFase3 | ejecutarReset | [close_overlay] |
| ResetFase3 | retrocederAFase2 | ResetFase2 |
| ModalAcercaDe | cerrar | [close_overlay] |

## Marcadores especiales detectados

Los siguientes targets aparecen entre corchetes y representan acciones de control del ciclo de vida del contexto en lugar de transiciones a un estado con nombre:

- `[close_overlay]`: Utilizado por todos los modales para retornar al contexto base.
- `[deactivate]`: Utilizado por el contexto concurrente `SesionActiva` para finalizar su existencia.

*Nota: No se han detectado marcadores como `[stay]` o `[stay_silent]` en este archivo.*
