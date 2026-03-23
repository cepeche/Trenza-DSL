# ModalSeleccionActividad

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| boton_actividad | Actividad | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalSeleccionActividad --> ModalComentario : elegirActividad
    ModalSeleccionActividad --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| elegirActividad | [ModalComentario](../overlays/ModalComentario.md) |
| cancelar | [cerrar_overlay] |