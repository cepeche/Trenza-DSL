# ModalSeleccionActividad

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| boton_actividad | Actividad | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalSeleccionActividad --> ModalComentario : elegirActividad
    ModalSeleccionActividad --> SYS_close_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| elegirActividad | [ModalComentario](../overlays/ModalComentario.md) |
| cancelar | [close_overlay] |