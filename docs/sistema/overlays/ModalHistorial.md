# ModalHistorial

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| boton_cerrar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalHistorial --> SYS_cerrar_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| cerrar | [cerrar_overlay] |