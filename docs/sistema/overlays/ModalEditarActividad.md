# ModalEditarActividad

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
    ModalEditarActividad --> SYS_cerrar_overlay : guardarEdicionActividad
    ModalEditarActividad --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarEdicionActividad | [cerrar_overlay] |
| cancelar | [cerrar_overlay] |