# SesionActiva

**Tipo**: contexto concurrent

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| display_timer | Boton | Local |
| checkbox_sustituir | Checkbox | Local |

## Contribuciones (Fills)

- Llenando **ModalComentario.sesion_opts** con:

## Transiciones

```mermaid
stateDiagram-v2
    SesionActiva --> SYS_deactivate : sesionFinalizada
```

| Evento | Destino |
|--------|---------|
| sesionFinalizada | [deactivate] |