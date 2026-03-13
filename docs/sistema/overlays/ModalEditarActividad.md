# ModalEditarActividad

**Tipo**: overlay
**Propósito**: editar nombre, color y flag de permanencia de una Actividad.
Fuente: [`ModalEditarActividad.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalEditarActividad.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| campo_nombre | [CampoTexto](../data.md) | cambio | actualizarNombreActividad(self.valor) |
| selector_color | [SelectorColor](../data.md) | seleccion | seleccionarColor(self.seleccionado) |
| checkbox_permanente | [Checkbox](../data.md) | cambio | marcarPermanente(self.marcado) |
| boton_guardar | [Boton](../data.md) | tap | guardarEdicionActividad |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| guardarEdicionActividad | **[cerrar_overlay]** |
| cancelar | **[cerrar_overlay]** |

## Effects

| Trigger | Acción |
|---------|--------|
| guardarEdicionActividad | external [actualizar_actividad](../external/cronometro_api.md)(...) |

## Gaps abiertos

- **GAP-1**: necesita recibir `actividadId` para cargar datos actuales
- **GAP-5**: guardar requiere nombre no vacío

---

↑ [CronometroPSP](../index.md) · ← abierto desde [ModoEdicion](../base/ModoEdicion.md)
