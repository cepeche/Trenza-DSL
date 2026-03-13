# ModalCrearTarea

**Tipo**: overlay
**Propósito**: crear un nuevo TipoTarea con nombre, icono y actividades permitidas.
Fuente: [`ModalCrearTarea.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalCrearTarea.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| campo_nombre | [CampoTexto](../data.md) | cambio | actualizarNuevoNombre(self.valor) |
| campo_busqueda_icono | [CampoTexto](../data.md) | cambio | filtrarIconosCrear(self.valor) |
| selector_icono | [SelectorIcono](../data.md) | seleccion | seleccionarIconoNuevo(self.seleccionado) |
| checkbox_actividad | [Actividad](../data.md) | cambio | toggleActividadPermitida(self.id, self.marcado) |
| boton_guardar | [Boton](../data.md) | tap | guardarNuevaTarea |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| guardarNuevaTarea | **[cerrar_overlay]** |
| cancelar | **[cerrar_overlay]** |

## Effects

| Trigger | Acción |
|---------|--------|
| guardarNuevaTarea | external [crear_tipo_tarea](../external/cronometro_api.md)(...) |

## Gaps abiertos

- **GAP-5**: guardar requiere nombre no vacío + al menos una actividad seleccionada
- **GAP-6**: `checkbox_actividad` es uno por actividad existente (multiplicidad)

---

↑ [CronometroPSP](../index.md)
