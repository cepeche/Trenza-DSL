# ModalAcercaDe

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| boton_cerrar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalAcercaDe --> SYS_close_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| cerrar | [close_overlay] |