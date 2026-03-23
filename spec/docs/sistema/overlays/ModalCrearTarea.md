# ModalCrearTarea

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| campo_nombre | CampoTexto | Local |
| campo_busqueda_icono | CampoTexto | Local |
| selector_icono | SelectorIcono | Local |
| checkbox_actividad | Actividad | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalCrearTarea --> SYS_cerrar_overlay : guardarNuevaTarea
    ModalCrearTarea --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarNuevaTarea | [cerrar_overlay] |
| cancelar | [cerrar_overlay] |