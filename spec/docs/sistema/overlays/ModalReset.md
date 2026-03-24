# ModalReset

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalReset --> ResetFase1 : iniciar
    ModalReset --> SYS_close_overlay : cerrar
```

| Evento | Destino |
|--------|---------|
| iniciar | [ResetFase1](../ResetFase1.md) |
| cerrar | [close_overlay] |