# ModalHistorial

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| boton_cerrar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalHistorial --> Historial7Dias : iniciar
    ModalHistorial --> SYS_close_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| iniciar | [Historial7Dias](../Historial7Dias.md) |
| cerrar | [close_overlay] |