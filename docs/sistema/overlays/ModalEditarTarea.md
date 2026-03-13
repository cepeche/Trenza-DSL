# ModalEditarTarea

**Tipo**: overlay
**Propósito**: editar nombre e icono de un TipoTarea existente.
Fuente: [`ModalEditarTarea.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalEditarTarea.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| campo_nombre | [CampoTexto](../data.md) | cambio | actualizarNombre(self.valor) |
| campo_busqueda_icono | [CampoTexto](../data.md) | cambio | filtrarIconos(self.valor) |
| selector_icono | [SelectorIcono](../data.md) | seleccion | seleccionarIcono(self.seleccionado) |
| boton_guardar | [Boton](../data.md) | tap | guardarEdicion |
| boton_cancelar | [Boton](../data.md) | tap | cancelar |

## Transiciones

| Evento | Destino |
|--------|---------|
| guardarEdicion | **[cerrar_overlay]** |
| cancelar | **[cerrar_overlay]** |

## Effects

| Trigger | Acción |
|---------|--------|
| guardarEdicion | external [editar_tipo_tarea](../external/cronometro_api.md)(...) |

## Gaps abiertos

- **GAP-1**: necesita recibir `tipoTareaId` para cargar datos actuales
- **GAP-5**: guardar requiere nombre no vacío

---

↑ [CronometroPSP](../index.md) · ← abierto desde [ModoEdicion](../base/ModoEdicion.md)
