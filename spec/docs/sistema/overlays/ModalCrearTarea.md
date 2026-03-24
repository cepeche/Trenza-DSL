# ModalCrearTarea

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| campo_nombre | CampoTexto | Local |
| campo_busqueda_icono | CampoTexto | Local |
| selector_icono | SelectorIcono | Local |
| checkbox_actividad | OpcionActividad | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalCrearTarea --> SYS_close_overlay : guardarNuevaTarea
    ModalCrearTarea --> SYS_close_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarNuevaTarea | [close_overlay] |
| cancelar | [close_overlay] |