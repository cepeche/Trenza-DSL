# SesionActiva

**Tipo**: contexto concurrente (ortogonal)
**Propósito**: coexiste con el contexto base. Añade timer y lógica de sesión
sin reemplazar ModoNormal ni ModoEdicion.
Fuente: [`SesionActiva.trz`](../../../examples/cronometro-psp/trenza/contexts/SesionActiva.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| display_timer | [Boton](../data.md) | tap | **ignorar** (solo lectura) |
| checkbox_sustituir | [Checkbox](../data.md) | cambio | marcarSustituir(self.marcado) |

> **GAP-4**: `checkbox_sustituir` solo debería existir cuando
> [ModalComentario](../overlays/ModalComentario.md) está abierto.
> El DSL no tiene sintaxis para roles condicionales entre contextos.

## Transiciones

| Evento | Destino |
|--------|---------|
| sesionFinalizada | **[desactivar]** (sale del concurrente) |

## Effects

| Trigger | Acción |
|---------|--------|
| actualizarTimer | external [calcular_tiempo_transcurrido](../external/cronometro_api.md)(...) |

---

↑ [CronometroPSP](../index.md)
