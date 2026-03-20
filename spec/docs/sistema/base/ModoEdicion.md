# ModoEdicion

**Tipo**: contexto base

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| tarjeta_tipo | TipoTarea | Local |
| tarjeta_tarea | Tarea | Local |
| pestana_actividad | Actividad | Local |
| pestana_frecuentes | Pestaña | Local |
| boton_edicion | Boton | Local |
| boton_nuevo | Boton | Local |
| boton_configuracion | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModoEdicion --> ModoNormal : desactivarEdicion
    ModoEdicion --> ModalEditarTarea : abrirEditarTarea
    ModoEdicion --> ModalEditarActividad : abrirEditarActividad
    ModoEdicion --> ModalCrearTarea : abrirCrearTarea
    ModoEdicion --> MenuConfiguracion : abrirMenuConfiguracion
```

| Evento | Destino |
|--------|---------|
| desactivarEdicion | [ModoNormal](../base/ModoNormal.md) |
| abrirEditarTarea | [ModalEditarTarea](../overlays/ModalEditarTarea.md) |
| abrirEditarActividad | [ModalEditarActividad](../overlays/ModalEditarActividad.md) |
| abrirCrearTarea | [ModalCrearTarea](../overlays/ModalCrearTarea.md) |
| abrirMenuConfiguracion | [MenuConfiguracion](../overlays/MenuConfiguracion.md) |