# ModalComentario

**Tipo**: contexto overlays

## Roles

| Rol | Tipo | Origen |
|-----|------|--------|
| * | ignored | Local |
| campo_comentario | CampoTexto | Local |
| campo_retroactivo | CampoNumerico | Local |
| boton_confirmar | Boton | Local |
| boton_cancelar | Boton | Local |

## Puntos de Extensión (Slots)

- `(sesion_opts)`

## Transiciones

```mermaid
stateDiagram-v2
    ModalComentario --> SYS_close_overlay : confirmarInicio
    ModalComentario --> SYS_close_overlay : cancelar
```

| Evento | Destino |
|--------|---------|
| confirmarInicio | [close_overlay] |
| cancelar | [close_overlay] |