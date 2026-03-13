# ModalEditarTarea

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| campo_nombre | CampoTexto | Local |
| campo_busqueda_icono | CampoTexto | Local |
| selector_icono | SelectorIcono | Local |
| boton_guardar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalEditarTarea --> SYS_cerrar_overlay : guardarEdicion
    ModalEditarTarea --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| guardarEdicion | [cerrar_overlay] |
| cancelar | [cerrar_overlay] |