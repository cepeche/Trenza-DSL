# cronometro_api

**Tipo**: módulo externo
**Propósito**: contrato de funciones del backend. El compilador Trenza genera
un trait Rust que el backend debe implementar; no genera la implementación.
Fuente: [`cronometro_api.trz`](../../../examples/cronometro-psp/trenza/external/cronometro_api.trz)

---

## Sesiones

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| iniciar_sesion | tarea_id, notas?, minutos_retroactivos, sustituir | Sesion | [ModoNormal](../base/ModoNormal.md), [ModalComentario](../overlays/ModalComentario.md) |
| obtener_sesion_activa | — | Sesion? | [SesionActiva](../concurrent/SesionActiva.md) |
| finalizar_sesion | sesion_id | Sesion | [SesionActiva](../concurrent/SesionActiva.md) |

## Tipos de tarea

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| crear_tipo_tarea | nombre, icono, actividades_permitidas | TipoTarea | [ModalCrearTarea](../overlays/ModalCrearTarea.md) |
| editar_tipo_tarea | tipo_id, nombre, icono | TipoTarea | [ModalEditarTarea](../overlays/ModalEditarTarea.md) |

## Actividades

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| crear_actividad | nombre, color, permanente | Actividad | [ModalCrearActividad](../overlays/ModalCrearActividad.md) |
| actualizar_actividad | id, nombre, color, permanente | Actividad | [ModalEditarActividad](../overlays/ModalEditarActividad.md) |

## Historial

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| cargar_historial | dias | Lista&lt;DiaHistorial&gt; | [ModalHistorial](../overlays/ModalHistorial.md) |

## Diagnóstico

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| verificar_conexion | — | EstadoConexion | [ModalAcercaDe](../overlays/ModalAcercaDe.md) |
| cargar_tiempo_acumulado | — | Lista&lt;ResumenActividad&gt; | [ModalAcercaDe](../overlays/ModalAcercaDe.md) |

## Reset

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| reset_datos | actividades_conservar | Resultado | [ModalReset](../overlays/ModalReset.md) |
| descargar_csv | — | Resultado | [ModalReset](../overlays/ModalReset.md) |

## Utilidades

| Función | Parámetros | Retorno | Usada en |
|---------|------------|---------|----------|
| calcular_tiempo_transcurrido | inicio | Entero | [SesionActiva](../concurrent/SesionActiva.md) |
| actualizar_grid_visible | — | Resultado | [ModoNormal](../base/ModoNormal.md) |

## Gap abierto

> **GAP-8**: las funciones pueden fallar (red, servidor). El DSL no tiene
> sintaxis para tipos de retorno de error ni flujos de error.

---

↑ [CronometroPSP](../index.md)
