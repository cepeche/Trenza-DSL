# ModalCrearActividad

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| campo_nombre | CampoTexto | Local |
| selector_color | SelectorColor | Local |
| checkbox_permanente | Checkbox | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalCrearActividad --> SYS_cerrar_overlay : guardarNuevaActividad
    ModalCrearActividad --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarNuevaActividad | [cerrar_overlay] |
| cancelar | [cerrar_overlay] |