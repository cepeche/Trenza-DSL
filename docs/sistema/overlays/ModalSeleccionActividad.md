# ModalSeleccionActividad

**Tipo**: overlay
**Propósito**: cuando un TipoTarea tiene varias actividades permitidas,
el usuario elige cuál antes de abrir el modal de comentario.
Fuente: [`ModalSeleccionActividad.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalSeleccionActividad.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| boton_actividad | [Actividad](../data.md) | tap | elegirActividad(self.id) |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| elegirActividad | [ModalComentario](ModalComentario.md) |
| cancelar | **[cerrar_overlay]** |

## Gaps abiertos

- **GAP-1**: necesita recibir `tipoTareaId` como entrada; `elegirActividad`
  encadena tipoTareaId + actividadId hacia ModalComentario
- **GAP-6**: `boton_actividad` debería ser uno por actividad permitida
  (multiplicidad dinámica)

---

↑ [CronometroPSP](../index.md)
