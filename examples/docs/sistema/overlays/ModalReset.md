# ModalReset

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalReset --> SYS_cerrar_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| cerrar | [cerrar_overlay] |