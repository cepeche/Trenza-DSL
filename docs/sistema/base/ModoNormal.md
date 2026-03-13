# ModoNormal

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
    ModoNormal --> ModoEdicion : activarEdicion
    ModoNormal --> ModalCrearTarea : abrirCrearTarea
    ModoNormal --> MenuConfiguracion : abrirMenuConfiguracion
    ModoNormal --> ModalComentario : seleccionarTipoTarea
```

| Evento | Destino |
|--------|---------|
| activarEdicion | [ModoEdicion](../base/ModoEdicion.md) |
| abrirCrearTarea | [ModalCrearTarea](../overlays/ModalCrearTarea.md) |
| abrirMenuConfiguracion | [MenuConfiguracion](../overlays/MenuConfiguracion.md) |
| seleccionarTipoTarea | [ModalComentario](../overlays/ModalComentario.md) |