# ModalEditarTarea

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| campo_nombre | CampoTexto | Local |
| campo_busqueda_icono | CampoTexto | Local |
| selector_icono | SelectorIcono | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalEditarTarea --> SYS_close_overlay : guardarEdicion
    ModalEditarTarea --> SYS_close_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarEdicion | [close_overlay] |
| cancelar | [close_overlay] |