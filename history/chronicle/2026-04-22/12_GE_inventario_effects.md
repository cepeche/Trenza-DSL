# Inventario de effects — CronometroPSP

Mapa de efectos disparados por eventos en los 18 contextos de `cronometro_full.trz`.

| context | event | function | args_literal |
|---------|-------|----------|--------------|
| ModoNormal | cambiarPestana | actualizarGridVisible | (vacío) |
| ModoNormal | iniciarTarea | iniciar_sesion | tarea_id, notas, minutos_retroactivos, sustituir |
| ModalAcercaDe | [on_entry] | verificar_conexion | (vacío) |
| ModalAcercaDe | [on_entry] | cargar_tiempo_acumulado | (vacío) |
| ModalComentario | confirmarInicio | iniciar_sesion | tipoTareaId, comentario, retroactivo, sustituir |
| ModalCrearActividad | guardarNuevaActividad | crear_actividad | nombre, color, permanente |
| ModalCrearTarea | guardarNuevaTarea | crear_tipo_tarea | nombre, icono, actividades_seleccionadas |
| ModalEditarActividad | guardarEdicionActividad | actualizar_actividad | actividadId, nombre, color, permanente |
| ModalEditarTarea | guardarEdicion | editar_tipo_tarea | tipoTareaId, nombre, icono |
| Historial7Dias | [on_entry] | cargar_historial | dias: 7 |
| Historial30Dias | [on_entry] | cargar_historial | dias: 30 |
| ResetFase1 | exportarCSV | descargar_csv | (vacío) |
| ResetFase3 | ejecutarReset | reset_datos | actividades_conservar |
| SesionActiva | actualizarTimer | calcular_tiempo_transcurrido | sesion_activa.inicio |
| SesionActiva | terminarSesion | parar_sesion | (vacío) |

## Clasificación de efectos

### 1. Efectos con args dependientes del payload
Estos efectos requieren que el generador extraiga datos del contexto o del evento:
- `iniciar_sesion(tarea_id, notas, minutos_retroactivos, sustituir)`
- `iniciar_sesion(tipoTareaId, comentario, retroactivo, sustituir)`
- `crear_actividad(nombre, color, permanente)`
- `crear_tipo_tarea(nombre, icono, actividades_seleccionadas)`
- `actualizar_actividad(actividadId, nombre, color, permanente)`
- `editar_tipo_tarea(tipoTareaId, nombre, icono)`
- `reset_datos(actividades_conservar)`
- `calcular_tiempo_transcurrido(sesion_activa.inicio)`

### 2. Efectos sin args (no-ops de payload)
- `actualizarGridVisible()`
- `verificar_conexion()`
- `cargar_tiempo_acumulado()`
- `descargar_csv()`
- `parar_sesion()`

### 3. Mismo nombre de efecto en varios contextos
- `iniciar_sesion`: Aparece en `ModoNormal` y `ModalComentario` con argumentos ligeramente distintos en su origen (uno de la tarea seleccionada y otro de los campos del modal).
- `cargar_historial`: Aparece en `Historial7Dias` y `Historial30Dias` con argumentos literales distintos (`7` vs `30`).

### 4. Efectos vinculados a roles (Binding dinámico)
A diferencia de los anteriores, estos efectos están declarados dentro de los roles y se disparan por eventos de UI (cambio, seleccion, tap). Ejemplos:
- `actualizarComentario(self.valor)`
- `actualizarRetroactivo(self.valor)`
- `actualizarNombreNuevaActividad(self.valor)`
- `seleccionarColorNuevo(self.seleccionado)`
- `marcarPermanenteNueva(self.marcado)`
- `actualizarNuevoNombre(self.valor)`
- `filtrarIconosCrear(self.valor)`
- `seleccionarIconoNuevo(self.seleccionado)`
- `toggleActividadPermitida(self.id, self.marcado)`
- `toggleConservar(self.id, self.marcado)`
- `actualizarConfirmacion(self.valor)`
- `elegirActividad(self.id)`
- `abrirEditarTarea(self.tipoId)`
- `abrirEditarActividad(self.id)`

*Nota: Estos efectos de binding son cruciales para el generador ya que conectan los componentes de la interfaz con el estado del modelo.*
