# ModalComentario

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| campo_comentario | CampoTexto | Local |
| campo_retroactivo | CampoNumerico | Local |
| boton_confirmar | Boton | Local |
| boton_cancelar | Boton | Local |

## Transiciones

```mermaid
stateDiagram-v2
    ModalComentario --> SYS_cerrar_overlay : confirmarInicio
    ModalComentario --> SYS_cerrar_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| confirmarInicio | [cerrar_overlay] |
| cancelar | [cerrar_overlay] |