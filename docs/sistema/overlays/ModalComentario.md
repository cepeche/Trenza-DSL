# ModalComentario

**Tipo**: overlay
**Propósito**: el usuario escribe un comentario y configura minutos retroactivos
antes de iniciar una sesión de cronometraje.
Fuente: [`ModalComentario.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalComentario.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| campo_comentario | [CampoTexto](../data.md) | cambio | actualizarComentario(self.valor) |
| campo_retroactivo | [CampoNumerico](../data.md) | cambio | actualizarRetroactivo(self.valor) |
| boton_confirmar | [Boton](../data.md) | tap | confirmarInicio |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| confirmarInicio | **[cerrar_overlay]** |
| cancelar | **[cerrar_overlay]** |

## Effects

| Trigger | Acción |
|---------|--------|
| confirmarInicio | external [iniciar_sesion](../external/cronometro_api.md)(...) |

## Gaps abiertos

- **GAP-1**: necesita recibir `tipoTareaId` (y opcionalmente `actividadId`)
  como parámetros de entrada
- **GAP-4**: cuando [SesionActiva](../concurrent/SesionActiva.md) está activo,
  aparece `checkbox_sustituir` — un rol que no existe en este contexto sino en
  el concurrente
- **GAP-5**: `confirmarInicio` solo debería proceder si el estado del modal
  es válido

---

↑ [CronometroPSP](../index.md)
