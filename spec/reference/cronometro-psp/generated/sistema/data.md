# Tipos de datos

Solo estructura. Cero comportamiento.
Fuente: [`data.trz`](../../examples/cronometro-psp/trenza/data.trz)

---

## Dominio

| Tipo | Campos | Descripción |
|------|--------|-------------|
| TipoTarea | tipoId, nombre, icono, usos7d, actividades_permitidas | Categoría de tarea (ej: "Desarrollo") |
| Tarea | tareaId, tipoId, actividadId, nombre, icono | Combinación tipo + actividad |
| Actividad | id, nombre, color, permanente | Actividad asignable a tareas |
| Sesion | sesionId, tareaId, inicio, notas? | Sesión de cronometraje en curso o finalizada |

## Elementos de UI

| Tipo | Campos | Uso como rol en |
|------|--------|-----------------|
| CampoTexto | id, valor, valido | ModalComentario, ModalCrearTarea, ModalEditarTarea, ModalReset |
| CampoNumerico | id, valor, minimo | ModalComentario (retroactivo) |
| Boton | id, etiqueta, habilitado | Todos los contextos |
| Checkbox | id, etiqueta, marcado | SesionActiva, ModalCrearTarea, ModalReset |
| SelectorColor | id, opciones, seleccionado | ModalCrearActividad, ModalEditarActividad |
| SelectorIcono | id, iconos, seleccionado, filtro | ModalCrearTarea, ModalEditarTarea |
| Pestaña | id, etiqueta | ModoNormal, ModoEdicion |
| ItemMenu | id, etiqueta | MenuConfiguracion |

## Historial

| Tipo | Campos |
|------|--------|
| DiaHistorial | timestamp, total, por_actividad |
| ResumenActividad | actividadId, actividadNombre, actividadColor, tiempo |

## Reset

| Tipo | Campos |
|------|--------|
| OpcionActividad | id, nombre, color, permanente, conservar |

---

↑ [CronometroPSP](index.md)
