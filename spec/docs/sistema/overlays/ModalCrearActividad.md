# ModalCrearActividad

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| campo_nombre | CampoTexto | Local |
| selector_color | SelectorColor | Local |
| checkbox_permanente | Checkbox | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalCrearActividad --> SYS_close_overlay : guardarNuevaActividad
    ModalCrearActividad --> SYS_close_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarNuevaActividad | [close_overlay] |
| cancelar | [close_overlay] |