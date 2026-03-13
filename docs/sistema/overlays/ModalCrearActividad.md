# ModalCrearActividad

**Tipo**: overlay
**Propósito**: crear una nueva Actividad con nombre, color y flag de permanencia.
Fuente: [`ModalCrearActividad.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalCrearActividad.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| campo_nombre | [CampoTexto](../data.md) | cambio | actualizarNombreNuevaActividad(self.valor) |
| selector_color | [SelectorColor](../data.md) | seleccion | seleccionarColorNuevo(self.seleccionado) |
| checkbox_permanente | [Checkbox](../data.md) | cambio | marcarPermanenteNueva(self.marcado) |
| boton_guardar | [Boton](../data.md) | tap | guardarNuevaActividad |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| guardarNuevaActividad | **[cerrar_overlay]** |
| cancelar | **[cerrar_overlay]** |

## Effects

| Trigger | Acción |
|---------|--------|
| guardarNuevaActividad | external [crear_actividad](../external/cronometro_api.md)(...) |

## Gaps abiertos

- **GAP-5**: guardar requiere nombre no vacío

---

↑ [CronometroPSP](../index.md) · ← abierto desde [MenuConfiguracion](MenuConfiguracion.md)
