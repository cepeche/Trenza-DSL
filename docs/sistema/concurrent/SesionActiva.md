# SesionActiva

**Tipo**: contexto concurrent

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| display_timer | Boton | Local |
| checkbox_sustituir | Checkbox | Local |

## Transiciones

```mermaid
stateDiagram-v2
    SesionActiva --> SYS_desactivar : sesionFinalizada
```

| Evento | Destino |
|--------|---------|
| sesionFinalizada | [desactivar] |