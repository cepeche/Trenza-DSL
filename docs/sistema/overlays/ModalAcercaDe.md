# ModalAcercaDe

**Tipo**: overlay
**Propósito**: información de la aplicación y estado de conexión. Solo lectura.
Fuente: [`ModalAcercaDe.trz`](../../../examples/cronometro-psp/trenza/contexts/ModalAcercaDe.trz)

---

## Roles

| Rol | Tipo | Evento | Acción |
|-----|------|--------|--------|
| boton_cerrar | [Boton](../data.md) | tap | cerrar |

## Transiciones

| Evento | Destino |
|--------|---------|
| cerrar | **[cerrar_overlay]** |

## Effects (GAP-7)

| Trigger | Acción |
|---------|--------|
| [al_entrar] | external [verificar_conexion](../external/cronometro_api.md)() |
| [al_entrar] | external [cargar_tiempo_acumulado](../external/cronometro_api.md)() |

---

↑ [CronometroPSP](../index.md) · ← abierto desde [MenuConfiguracion](MenuConfiguracion.md)
