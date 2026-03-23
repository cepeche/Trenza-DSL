# System Visualization: CronometroPSP

## 1. High-Level Topology (Context Map)

```mermaid
stateDiagram-v2

    MenuConfiguracion --> ModalCrearActividad : abrirCrearActividad
    MenuConfiguracion --> ModalHistorial : abrirHistorial
    MenuConfiguracion --> ModalAcercaDe : abrirAcercaDe
    MenuConfiguracion --> ModalReset : abrirReset
    MenuConfiguracion --> cerrar_overlay : cerrar
    ModalAcercaDe --> cerrar_overlay : cerrar
    ModalComentario --> cerrar_overlay : confirmarInicio
    ModalComentario --> cerrar_overlay : cancelar
    ModalCrearActividad --> cerrar_overlay : guardarNuevaActividad
    ModalCrearActividad --> cerrar_overlay : cancelar
    ModalCrearTarea --> cerrar_overlay : guardarNuevaTarea
    ModalCrearTarea --> cerrar_overlay : cancelar
    ModalEditarActividad --> cerrar_overlay : guardarEdicionActividad
    ModalEditarActividad --> cerrar_overlay : cancelar
    ModalEditarTarea --> cerrar_overlay : guardarEdicion
    ModalEditarTarea --> cerrar_overlay : cancelar
    ModalHistorial --> cerrar_overlay : cerrar
    Historial7Dias --> Historial30Dias : cambiarA30Dias
    Historial7Dias --> ModalHistorial : cerrar
    Historial30Dias --> Historial7Dias : cambiarA7Dias
    Historial30Dias --> ModalHistorial : cerrar
    ModalReset --> cerrar_overlay : cerrar
    ResetFase1 --> ResetFase2 : avanzarAFase2
    ResetFase1 --> ModalReset : cerrar
    ResetFase2 --> ResetFase3 : avanzarAFase3
    ResetFase2 --> ResetFase1 : retrocederAFase1
    ResetFase3 --> cerrar_overlay : ejecutarReset
    ResetFase3 --> ResetFase2 : retrocederAFase2
    ModalSeleccionActividad --> ModalComentario : elegirActividad
    ModalSeleccionActividad --> cerrar_overlay : cancelar
    ModoEdicion --> ModoNormal : desactivarEdicion
    ModoEdicion --> ModalEditarTarea : abrirEditarTarea
    ModoEdicion --> ModalEditarActividad : abrirEditarActividad
    ModoEdicion --> ModalCrearTarea : abrirCrearTarea
    ModoEdicion --> MenuConfiguracion : abrirMenuConfiguracion
    ModoNormal --> ModoEdicion : activarEdicion
    ModoNormal --> ModalCrearTarea : abrirCrearTarea
    ModoNormal --> MenuConfiguracion : abrirMenuConfiguracion
    ModoNormal --> ModalComentario : seleccionarTipoTarea
    SesionActiva --> deactivate : sesionFinalizada
    [*] --> ModoNormal

```

## 2. Context Details

### Context: MenuConfiguracion

```mermaid
stateDiagram-v2

    state MenuConfiguracion {
        MenuConfiguracion_tap_item_nueva_actividad --> abrirCrearActividad
        MenuConfiguracion_tap_item_historial --> abrirHistorial
        MenuConfiguracion_tap_item_acerca_de --> abrirAcercaDe
        MenuConfiguracion_tap_item_reset --> abrirReset
        MenuConfiguracion_tap_overlay --> cerrar
    }

```

### Context: ModalAcercaDe

```mermaid
stateDiagram-v2

    state ModalAcercaDe {
        ModalAcercaDe_tap_boton_cerrar --> cerrar
        ModalAcercaDe_on_entry --> verificar_conexion
        ModalAcercaDe_on_entry --> cargar_tiempo_acumulado
    }

```

### Context: ModalComentario

```mermaid
stateDiagram-v2

    state ModalComentario {
        ModalComentario_cambio_campo_comentario --> actualizarComentario
        ModalComentario_cambio_campo_retroactivo --> actualizarRetroactivo
        ModalComentario_tap_boton_confirmar --> confirmarInicio
        ModalComentario_tap_boton_cancelar --> cancelar
        ModalComentario_confirmarInicio --> iniciar_sesion
    }

```

### Context: ModalCrearActividad

```mermaid
stateDiagram-v2

    state ModalCrearActividad {
        ModalCrearActividad_cambio_campo_nombre --> actualizarNombreNuevaActividad
        ModalCrearActividad_seleccion_selector_color --> seleccionarColorNuevo
        ModalCrearActividad_cambio_checkbox_permanente --> marcarPermanenteNueva
        ModalCrearActividad_tap_boton_guardar --> guardarNuevaActividad
        ModalCrearActividad_tap_boton_cancelar --> cancelar
        ModalCrearActividad_guardarNuevaActividad --> crear_actividad
    }

```

### Context: ModalCrearTarea

```mermaid
stateDiagram-v2

    state ModalCrearTarea {
        ModalCrearTarea_cambio_campo_nombre --> actualizarNuevoNombre
        ModalCrearTarea_cambio_campo_busqueda_icono --> filtrarIconosCrear
        ModalCrearTarea_seleccion_selector_icono --> seleccionarIconoNuevo
        ModalCrearTarea_cambio_checkbox_actividad --> toggleActividadPermitida
        ModalCrearTarea_tap_boton_guardar --> guardarNuevaTarea
        ModalCrearTarea_tap_boton_cancelar --> cancelar
        ModalCrearTarea_guardarNuevaTarea --> crear_tipo_tarea
    }

```

### Context: ModalEditarActividad

```mermaid
stateDiagram-v2

    state ModalEditarActividad {
        ModalEditarActividad_cambio_campo_nombre --> actualizarNombreActividad
        ModalEditarActividad_seleccion_selector_color --> seleccionarColor
        ModalEditarActividad_cambio_checkbox_permanente --> marcarPermanente
        ModalEditarActividad_tap_boton_guardar --> guardarEdicionActividad
        ModalEditarActividad_tap_boton_cancelar --> cancelar
        ModalEditarActividad_guardarEdicionActividad --> actualizar_actividad
    }

```

### Context: ModalEditarTarea

```mermaid
stateDiagram-v2

    state ModalEditarTarea {
        ModalEditarTarea_cambio_campo_nombre --> actualizarNombre
        ModalEditarTarea_cambio_campo_busqueda_icono --> filtrarIconos
        ModalEditarTarea_seleccion_selector_icono --> seleccionarIcono
        ModalEditarTarea_tap_boton_guardar --> guardarEdicion
        ModalEditarTarea_tap_boton_cancelar --> cancelar
        ModalEditarTarea_guardarEdicion --> editar_tipo_tarea
    }

```

### Context: ModalHistorial

```mermaid
stateDiagram-v2

    state ModalHistorial {
        ModalHistorial_tap_boton_cerrar --> cerrar
    }

```

### Context: Historial7Dias

```mermaid
stateDiagram-v2

    state Historial7Dias {
        Historial7Dias_tap_boton_7dias --> ignored
        Historial7Dias_tap_boton_30dias --> cambiarA30Dias
        Historial7Dias_on_entry --> cargar_historial
    }

```

### Context: Historial30Dias

```mermaid
stateDiagram-v2

    state Historial30Dias {
        Historial30Dias_tap_boton_7dias --> cambiarA7Dias
        Historial30Dias_tap_boton_30dias --> ignored
        Historial30Dias_on_entry --> cargar_historial
    }

```

### Context: ModalReset

```mermaid
stateDiagram-v2

    state ModalReset {
        ModalReset_tap_boton_cancelar --> cerrar
    }

```

### Context: ResetFase1

```mermaid
stateDiagram-v2

    state ResetFase1 {
        ResetFase1_tap_boton_cancelar --> cerrar
        ResetFase1_tap_boton_continuar --> avanzarAFase2
        ResetFase1_tap_boton_exportar_csv --> exportarCSV
        ResetFase1_exportarCSV --> descargar_csv
    }

```

### Context: ResetFase2

```mermaid
stateDiagram-v2

    state ResetFase2 {
        ResetFase2_cambio_checkbox_actividad --> toggleConservar
        ResetFase2_tap_boton_continuar --> avanzarAFase3
        ResetFase2_tap_boton_atras --> retrocederAFase1
    }

```

### Context: ResetFase3

```mermaid
stateDiagram-v2

    state ResetFase3 {
        ResetFase3_cambio_campo_confirmacion --> actualizarConfirmacion
        ResetFase3_tap_boton_ejecutar --> ejecutarReset
        ResetFase3_tap_boton_atras --> retrocederAFase2
        ResetFase3_ejecutarReset --> reset_datos
    }

```

### Context: ModalSeleccionActividad

```mermaid
stateDiagram-v2

    state ModalSeleccionActividad {
        ModalSeleccionActividad_tap_boton_actividad --> elegirActividad
        ModalSeleccionActividad_tap_boton_cancelar --> cancelar
    }

```

### Context: ModoEdicion

```mermaid
stateDiagram-v2

    state ModoEdicion {
        ModoEdicion_tap_tarjeta_tipo --> abrirEditarTarea
        ModoEdicion_tap_tarjeta_tarea --> abrirEditarTarea
        ModoEdicion_tap_pestana_actividad --> abrirEditarActividad
        ModoEdicion_tap_pestana_frecuentes --> ignored
        ModoEdicion_tap_boton_edicion --> desactivarEdicion
        ModoEdicion_tap_boton_nuevo --> abrirCrearTarea
        ModoEdicion_tap_boton_configuracion --> abrirMenuConfiguracion
    }

```

### Context: ModoNormal

```mermaid
stateDiagram-v2

    state ModoNormal {
        ModoNormal_tap_tarjeta_tipo --> seleccionarTipoTarea
        ModoNormal_tap_tarjeta_tarea --> iniciarTarea
        ModoNormal_tap_pestana_actividad --> cambiarPestana
        ModoNormal_tap_pestana_frecuentes --> cambiarPestana
        ModoNormal_tap_boton_edicion --> activarEdicion
        ModoNormal_tap_boton_nuevo --> abrirCrearTarea
        ModoNormal_tap_boton_configuracion --> abrirMenuConfiguracion
        ModoNormal_cambiarPestana --> actualizarGridVisible
        ModoNormal_iniciarTarea --> iniciar_sesion
    }

```

### Context: SesionActiva

```mermaid
stateDiagram-v2

    state SesionActiva {
        SesionActiva_tap_display_timer --> ignored
        SesionActiva_actualizarTimer --> calcular_tiempo_transcurrido
    }

```

*Generated by Trenza CLI*