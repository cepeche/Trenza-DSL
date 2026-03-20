# ModalAcercaDe

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| boton_cerrar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalAcercaDe --> SYS_cerrar_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| cerrar | [cerrar_overlay] |