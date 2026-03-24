// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2)
// DO NOT EDIT — regenerate from .trz source

#[cfg(test)]
mod algebraic_tests {
    use super::*;

    // === Transition Tests ===

    #[test]
    fn test_transition_MenuConfiguracion_on_abrirCrearActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirCrearActividad");
        assert_eq!(sys.state, Contexto::ModalCrearActividad);
    }

    #[test]
    fn test_transition_MenuConfiguracion_on_abrirHistorial() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirHistorial");
        assert_eq!(sys.state, Contexto::ModalHistorial);
    }

    #[test]
    fn test_transition_MenuConfiguracion_on_abrirAcercaDe() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirAcercaDe");
        assert_eq!(sys.state, Contexto::ModalAcercaDe);
    }

    #[test]
    fn test_transition_MenuConfiguracion_on_abrirReset() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirReset");
        assert_eq!(sys.state, Contexto::ModalReset);
    }

    #[test]
    fn test_transition_MenuConfiguracion_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalAcercaDe_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalAcercaDe, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalComentario_on_confirmarInicio() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalComentario, &effects);
        sys.handle_event("confirmarInicio");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalComentario_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalComentario, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalCrearActividad_on_guardarNuevaActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalCrearActividad, &effects);
        sys.handle_event("guardarNuevaActividad");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalCrearActividad_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalCrearActividad, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalCrearTarea_on_guardarNuevaTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalCrearTarea, &effects);
        sys.handle_event("guardarNuevaTarea");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalCrearTarea_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalCrearTarea, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalEditarActividad_on_guardarEdicionActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalEditarActividad, &effects);
        sys.handle_event("guardarEdicionActividad");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalEditarActividad_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalEditarActividad, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalEditarTarea_on_guardarEdicion() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalEditarTarea, &effects);
        sys.handle_event("guardarEdicion");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalEditarTarea_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalEditarTarea, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModalHistorial_on_iniciar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalHistorial, &effects);
        sys.handle_event("iniciar");
        assert_eq!(sys.state, Contexto::Historial7Dias);
    }

    #[test]
    fn test_transition_ModalHistorial_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalHistorial, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_Historial7Dias_on_cambiarA30Dias() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::Historial7Dias, &effects);
        sys.handle_event("cambiarA30Dias");
        assert_eq!(sys.state, Contexto::Historial30Dias);
    }

    #[test]
    fn test_transition_Historial7Dias_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::Historial7Dias, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::ModalHistorial);
    }

    #[test]
    fn test_transition_Historial30Dias_on_cambiarA7Dias() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::Historial30Dias, &effects);
        sys.handle_event("cambiarA7Dias");
        assert_eq!(sys.state, Contexto::Historial7Dias);
    }

    #[test]
    fn test_transition_Historial30Dias_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::Historial30Dias, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::ModalHistorial);
    }

    #[test]
    fn test_transition_ModalReset_on_iniciar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalReset, &effects);
        sys.handle_event("iniciar");
        assert_eq!(sys.state, Contexto::ResetFase1);
    }

    #[test]
    fn test_transition_ModalReset_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalReset, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ResetFase1_on_avanzarAFase2() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase1, &effects);
        sys.handle_event("avanzarAFase2");
        assert_eq!(sys.state, Contexto::ResetFase2);
    }

    #[test]
    fn test_transition_ResetFase1_on_cerrar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase1, &effects);
        sys.handle_event("cerrar");
        assert_eq!(sys.state, Contexto::ModalReset);
    }

    #[test]
    fn test_transition_ResetFase2_on_avanzarAFase3() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase2, &effects);
        sys.handle_event("avanzarAFase3");
        assert_eq!(sys.state, Contexto::ResetFase3);
    }

    #[test]
    fn test_transition_ResetFase2_on_retrocederAFase1() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase2, &effects);
        sys.handle_event("retrocederAFase1");
        assert_eq!(sys.state, Contexto::ResetFase1);
    }

    #[test]
    fn test_transition_ResetFase3_on_ejecutarReset() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase3, &effects);
        sys.handle_event("ejecutarReset");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ResetFase3_on_retrocederAFase2() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ResetFase3, &effects);
        sys.handle_event("retrocederAFase2");
        assert_eq!(sys.state, Contexto::ResetFase2);
    }

    #[test]
    fn test_transition_ModalSeleccionActividad_on_elegirActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalSeleccionActividad, &effects);
        sys.handle_event("elegirActividad");
        assert_eq!(sys.state, Contexto::ModalComentario);
    }

    #[test]
    fn test_transition_ModalSeleccionActividad_on_cancelar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModalSeleccionActividad, &effects);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::[close_overlay]);
    }

    #[test]
    fn test_transition_ModoEdicion_on_desactivarEdicion() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoEdicion, &effects);
        sys.handle_event("desactivarEdicion");
        assert_eq!(sys.state, Contexto::ModoNormal);
    }

    #[test]
    fn test_transition_ModoEdicion_on_abrirEditarTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoEdicion, &effects);
        sys.handle_event("abrirEditarTarea");
        assert_eq!(sys.state, Contexto::ModalEditarTarea);
    }

    #[test]
    fn test_transition_ModoEdicion_on_abrirEditarActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoEdicion, &effects);
        sys.handle_event("abrirEditarActividad");
        assert_eq!(sys.state, Contexto::ModalEditarActividad);
    }

    #[test]
    fn test_transition_ModoEdicion_on_abrirCrearTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoEdicion, &effects);
        sys.handle_event("abrirCrearTarea");
        assert_eq!(sys.state, Contexto::ModalCrearTarea);
    }

    #[test]
    fn test_transition_ModoEdicion_on_abrirMenuConfiguracion() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoEdicion, &effects);
        sys.handle_event("abrirMenuConfiguracion");
        assert_eq!(sys.state, Contexto::MenuConfiguracion);
    }

    #[test]
    fn test_transition_ModoNormal_on_activarEdicion() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("activarEdicion");
        assert_eq!(sys.state, Contexto::ModoEdicion);
    }

    #[test]
    fn test_transition_ModoNormal_on_abrirCrearTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("abrirCrearTarea");
        assert_eq!(sys.state, Contexto::ModalCrearTarea);
    }

    #[test]
    fn test_transition_ModoNormal_on_abrirMenuConfiguracion() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("abrirMenuConfiguracion");
        assert_eq!(sys.state, Contexto::MenuConfiguracion);
    }

    #[test]
    fn test_transition_ModoNormal_on_iniciarTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("iniciarTarea");
        assert_eq!(sys.state, Contexto::SesionActiva);
    }

    #[test]
    fn test_transition_ModoNormal_on_seleccionarTipoTarea() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("seleccionarTipoTarea");
        assert_eq!(sys.state, Contexto::ModalComentario);
    }

    #[test]
    fn test_transition_ModoNormal_on_elegirActividad() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("elegirActividad");
        assert_eq!(sys.state, Contexto::ModalSeleccionActividad);
    }

    #[test]
    fn test_transition_SesionActiva_on_sesionFinalizada() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.handle_event("sesionFinalizada");
        // In threads mode this might differ, but for composite:
        assert!(!sys.concurrent_states.contains(&Contexto::SesionActiva));
    }

    // === Handler Tests ===

    #[test]
    fn test_call_MenuConfiguracion_item_nueva_actividad_on_tap_invokes_abrirCrearActividad() {
        let effects = RecordingEffects::new();
        let data = ItemMenu::default();
        handle_item_nueva_actividad_tap(&Contexto::MenuConfiguracion, &data, &effects);
        assert!(effects.was_called("abrirCrearActividad"));
    }

    #[test]
    fn test_call_MenuConfiguracion_item_historial_on_tap_invokes_abrirHistorial() {
        let effects = RecordingEffects::new();
        let data = ItemMenu::default();
        handle_item_historial_tap(&Contexto::MenuConfiguracion, &data, &effects);
        assert!(effects.was_called("abrirHistorial"));
    }

    #[test]
    fn test_call_MenuConfiguracion_item_acerca_de_on_tap_invokes_abrirAcercaDe() {
        let effects = RecordingEffects::new();
        let data = ItemMenu::default();
        handle_item_acerca_de_tap(&Contexto::MenuConfiguracion, &data, &effects);
        assert!(effects.was_called("abrirAcercaDe"));
    }

    #[test]
    fn test_call_MenuConfiguracion_item_reset_on_tap_invokes_abrirReset() {
        let effects = RecordingEffects::new();
        let data = ItemMenu::default();
        handle_item_reset_tap(&Contexto::MenuConfiguracion, &data, &effects);
        assert!(effects.was_called("abrirReset"));
    }

    #[test]
    fn test_call_MenuConfiguracion_overlay_on_tap_invokes_cerrar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_overlay_tap(&Contexto::MenuConfiguracion, &data, &effects);
        assert!(effects.was_called("cerrar"));
    }

    #[test]
    fn test_call_ModalAcercaDe_boton_cerrar_on_tap_invokes_cerrar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cerrar_tap(&Contexto::ModalAcercaDe, &data, &effects);
        assert!(effects.was_called("cerrar"));
    }

    #[test]
    fn test_call_ModalComentario_campo_comentario_on_cambio_invokes_actualizarComentario() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_comentario_cambio(&Contexto::ModalComentario, &data, &effects);
        assert!(effects.was_called("actualizarComentario"));
    }

    #[test]
    fn test_call_ModalComentario_campo_retroactivo_on_cambio_invokes_actualizarRetroactivo() {
        let effects = RecordingEffects::new();
        let data = CampoNumerico::default();
        handle_campo_retroactivo_cambio(&Contexto::ModalComentario, &data, &effects);
        assert!(effects.was_called("actualizarRetroactivo"));
    }

    #[test]
    fn test_call_ModalComentario_boton_confirmar_on_tap_invokes_confirmarInicio() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_confirmar_tap(&Contexto::ModalComentario, &data, &effects);
        assert!(effects.was_called("confirmarInicio"));
    }

    #[test]
    fn test_call_ModalComentario_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalComentario, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModalCrearActividad_campo_nombre_on_cambio_invokes_actualizarNombreNuevaActividad() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_nombre_cambio(&Contexto::ModalCrearActividad, &data, &effects);
        assert!(effects.was_called("actualizarNombreNuevaActividad"));
    }

    #[test]
    fn test_call_ModalCrearActividad_selector_color_on_seleccion_invokes_seleccionarColorNuevo() {
        let effects = RecordingEffects::new();
        let data = SelectorColor::default();
        handle_selector_color_seleccion(&Contexto::ModalCrearActividad, &data, &effects);
        assert!(effects.was_called("seleccionarColorNuevo"));
    }

    #[test]
    fn test_call_ModalCrearActividad_checkbox_permanente_on_cambio_invokes_marcarPermanenteNueva() {
        let effects = RecordingEffects::new();
        let data = Checkbox::default();
        handle_checkbox_permanente_cambio(&Contexto::ModalCrearActividad, &data, &effects);
        assert!(effects.was_called("marcarPermanenteNueva"));
    }

    #[test]
    fn test_call_ModalCrearActividad_boton_guardar_on_tap_invokes_guardarNuevaActividad() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_guardar_tap(&Contexto::ModalCrearActividad, &data, &effects);
        assert!(effects.was_called("guardarNuevaActividad"));
    }

    #[test]
    fn test_call_ModalCrearActividad_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalCrearActividad, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModalCrearTarea_campo_nombre_on_cambio_invokes_actualizarNuevoNombre() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_nombre_cambio(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("actualizarNuevoNombre"));
    }

    #[test]
    fn test_call_ModalCrearTarea_campo_busqueda_icono_on_cambio_invokes_filtrarIconosCrear() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_busqueda_icono_cambio(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("filtrarIconosCrear"));
    }

    #[test]
    fn test_call_ModalCrearTarea_selector_icono_on_seleccion_invokes_seleccionarIconoNuevo() {
        let effects = RecordingEffects::new();
        let data = SelectorIcono::default();
        handle_selector_icono_seleccion(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("seleccionarIconoNuevo"));
    }

    #[test]
    fn test_call_ModalCrearTarea_checkbox_actividad_on_cambio_invokes_toggleActividadPermitida() {
        let effects = RecordingEffects::new();
        let data = OpcionActividad::default();
        handle_checkbox_actividad_cambio(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("toggleActividadPermitida"));
    }

    #[test]
    fn test_call_ModalCrearTarea_boton_guardar_on_tap_invokes_guardarNuevaTarea() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_guardar_tap(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("guardarNuevaTarea"));
    }

    #[test]
    fn test_call_ModalCrearTarea_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalCrearTarea, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModalEditarActividad_campo_nombre_on_cambio_invokes_actualizarNombreActividad() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_nombre_cambio(&Contexto::ModalEditarActividad, &data, &effects);
        assert!(effects.was_called("actualizarNombreActividad"));
    }

    #[test]
    fn test_call_ModalEditarActividad_selector_color_on_seleccion_invokes_seleccionarColor() {
        let effects = RecordingEffects::new();
        let data = SelectorColor::default();
        handle_selector_color_seleccion(&Contexto::ModalEditarActividad, &data, &effects);
        assert!(effects.was_called("seleccionarColor"));
    }

    #[test]
    fn test_call_ModalEditarActividad_checkbox_permanente_on_cambio_invokes_marcarPermanente() {
        let effects = RecordingEffects::new();
        let data = Checkbox::default();
        handle_checkbox_permanente_cambio(&Contexto::ModalEditarActividad, &data, &effects);
        assert!(effects.was_called("marcarPermanente"));
    }

    #[test]
    fn test_call_ModalEditarActividad_boton_guardar_on_tap_invokes_guardarEdicionActividad() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_guardar_tap(&Contexto::ModalEditarActividad, &data, &effects);
        assert!(effects.was_called("guardarEdicionActividad"));
    }

    #[test]
    fn test_call_ModalEditarActividad_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalEditarActividad, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModalEditarTarea_campo_nombre_on_cambio_invokes_actualizarNombre() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_nombre_cambio(&Contexto::ModalEditarTarea, &data, &effects);
        assert!(effects.was_called("actualizarNombre"));
    }

    #[test]
    fn test_call_ModalEditarTarea_campo_busqueda_icono_on_cambio_invokes_filtrarIconos() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_busqueda_icono_cambio(&Contexto::ModalEditarTarea, &data, &effects);
        assert!(effects.was_called("filtrarIconos"));
    }

    #[test]
    fn test_call_ModalEditarTarea_selector_icono_on_seleccion_invokes_seleccionarIcono() {
        let effects = RecordingEffects::new();
        let data = SelectorIcono::default();
        handle_selector_icono_seleccion(&Contexto::ModalEditarTarea, &data, &effects);
        assert!(effects.was_called("seleccionarIcono"));
    }

    #[test]
    fn test_call_ModalEditarTarea_boton_guardar_on_tap_invokes_guardarEdicion() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_guardar_tap(&Contexto::ModalEditarTarea, &data, &effects);
        assert!(effects.was_called("guardarEdicion"));
    }

    #[test]
    fn test_call_ModalEditarTarea_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalEditarTarea, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModalHistorial_boton_cerrar_on_tap_invokes_cerrar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cerrar_tap(&Contexto::ModalHistorial, &data, &effects);
        assert!(effects.was_called("cerrar"));
    }

    #[test]
    fn test_ignored_Historial7Dias_boton_7dias_on_tap() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_7dias_tap(&Contexto::Historial7Dias, &data, &effects);
        assert!(effects.calls.borrow().is_empty());
    }

    #[test]
    fn test_call_Historial7Dias_boton_30dias_on_tap_invokes_cambiarA30Dias() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_30dias_tap(&Contexto::Historial7Dias, &data, &effects);
        assert!(effects.was_called("cambiarA30Dias"));
    }

    #[test]
    fn test_call_Historial30Dias_boton_7dias_on_tap_invokes_cambiarA7Dias() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_7dias_tap(&Contexto::Historial30Dias, &data, &effects);
        assert!(effects.was_called("cambiarA7Dias"));
    }

    #[test]
    fn test_ignored_Historial30Dias_boton_30dias_on_tap() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_30dias_tap(&Contexto::Historial30Dias, &data, &effects);
        assert!(effects.calls.borrow().is_empty());
    }

    #[test]
    fn test_call_ModalReset_boton_cancelar_on_tap_invokes_cerrar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalReset, &data, &effects);
        assert!(effects.was_called("cerrar"));
    }

    #[test]
    fn test_call_ResetFase1_boton_cancelar_on_tap_invokes_cerrar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ResetFase1, &data, &effects);
        assert!(effects.was_called("cerrar"));
    }

    #[test]
    fn test_call_ResetFase1_boton_continuar_on_tap_invokes_avanzarAFase2() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_continuar_tap(&Contexto::ResetFase1, &data, &effects);
        assert!(effects.was_called("avanzarAFase2"));
    }

    #[test]
    fn test_call_ResetFase1_boton_exportar_csv_on_tap_invokes_exportarCSV() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_exportar_csv_tap(&Contexto::ResetFase1, &data, &effects);
        assert!(effects.was_called("exportarCSV"));
    }

    #[test]
    fn test_call_ResetFase2_checkbox_actividad_on_cambio_invokes_toggleConservar() {
        let effects = RecordingEffects::new();
        let data = OpcionActividad::default();
        handle_checkbox_actividad_cambio(&Contexto::ResetFase2, &data, &effects);
        assert!(effects.was_called("toggleConservar"));
    }

    #[test]
    fn test_call_ResetFase2_boton_continuar_on_tap_invokes_avanzarAFase3() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_continuar_tap(&Contexto::ResetFase2, &data, &effects);
        assert!(effects.was_called("avanzarAFase3"));
    }

    #[test]
    fn test_call_ResetFase2_boton_atras_on_tap_invokes_retrocederAFase1() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_atras_tap(&Contexto::ResetFase2, &data, &effects);
        assert!(effects.was_called("retrocederAFase1"));
    }

    #[test]
    fn test_call_ResetFase3_campo_confirmacion_on_cambio_invokes_actualizarConfirmacion() {
        let effects = RecordingEffects::new();
        let data = CampoTexto::default();
        handle_campo_confirmacion_cambio(&Contexto::ResetFase3, &data, &effects);
        assert!(effects.was_called("actualizarConfirmacion"));
    }

    #[test]
    fn test_call_ResetFase3_boton_ejecutar_on_tap_invokes_ejecutarReset() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_ejecutar_tap(&Contexto::ResetFase3, &data, &effects);
        assert!(effects.was_called("ejecutarReset"));
    }

    #[test]
    fn test_call_ResetFase3_boton_atras_on_tap_invokes_retrocederAFase2() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_atras_tap(&Contexto::ResetFase3, &data, &effects);
        assert!(effects.was_called("retrocederAFase2"));
    }

    #[test]
    fn test_call_ModalSeleccionActividad_boton_actividad_on_tap_invokes_elegirActividad() {
        let effects = RecordingEffects::new();
        let data = Actividad::default();
        handle_boton_actividad_tap(&Contexto::ModalSeleccionActividad, &data, &effects);
        assert!(effects.was_called("elegirActividad"));
    }

    #[test]
    fn test_call_ModalSeleccionActividad_boton_cancelar_on_tap_invokes_cancelar() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_cancelar_tap(&Contexto::ModalSeleccionActividad, &data, &effects);
        assert!(effects.was_called("cancelar"));
    }

    #[test]
    fn test_call_ModoEdicion_tarjeta_tipo_on_tap_invokes_abrirEditarTarea() {
        let effects = RecordingEffects::new();
        let data = TipoTarea::default();
        handle_tarjeta_tipo_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("abrirEditarTarea"));
    }

    #[test]
    fn test_call_ModoEdicion_tarjeta_tarea_on_tap_invokes_abrirEditarTarea() {
        let effects = RecordingEffects::new();
        let data = Tarea::default();
        handle_tarjeta_tarea_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("abrirEditarTarea"));
    }

    #[test]
    fn test_call_ModoEdicion_pestana_actividad_on_tap_invokes_abrirEditarActividad() {
        let effects = RecordingEffects::new();
        let data = Actividad::default();
        handle_pestana_actividad_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("abrirEditarActividad"));
    }

    #[test]
    fn test_ignored_ModoEdicion_pestana_frecuentes_on_tap() {
        let effects = RecordingEffects::new();
        let data = Pestana::default();
        handle_pestana_frecuentes_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.calls.borrow().is_empty());
    }

    #[test]
    fn test_call_ModoEdicion_boton_edicion_on_tap_invokes_desactivarEdicion() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_edicion_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("desactivarEdicion"));
    }

    #[test]
    fn test_call_ModoEdicion_boton_nuevo_on_tap_invokes_abrirCrearTarea() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_nuevo_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("abrirCrearTarea"));
    }

    #[test]
    fn test_call_ModoEdicion_boton_configuracion_on_tap_invokes_abrirMenuConfiguracion() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_configuracion_tap(&Contexto::ModoEdicion, &data, &effects);
        assert!(effects.was_called("abrirMenuConfiguracion"));
    }

    #[test]
    fn test_call_ModoNormal_tarjeta_tipo_on_tap_invokes_seleccionarTipoTarea() {
        let effects = RecordingEffects::new();
        let data = TipoTarea::default();
        handle_tarjeta_tipo_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("seleccionarTipoTarea"));
    }

    #[test]
    fn test_call_ModoNormal_tarjeta_tarea_on_tap_invokes_iniciarTarea() {
        let effects = RecordingEffects::new();
        let data = Tarea::default();
        handle_tarjeta_tarea_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("iniciarTarea"));
    }

    #[test]
    fn test_call_ModoNormal_pestana_actividad_on_tap_invokes_cambiarPestana() {
        let effects = RecordingEffects::new();
        let data = Actividad::default();
        handle_pestana_actividad_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("cambiarPestana"));
    }

    #[test]
    fn test_call_ModoNormal_pestana_frecuentes_on_tap_invokes_cambiarPestana() {
        let effects = RecordingEffects::new();
        let data = Pestana::default();
        handle_pestana_frecuentes_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("cambiarPestana"));
    }

    #[test]
    fn test_call_ModoNormal_boton_edicion_on_tap_invokes_activarEdicion() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_edicion_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("activarEdicion"));
    }

    #[test]
    fn test_call_ModoNormal_boton_nuevo_on_tap_invokes_abrirCrearTarea() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_nuevo_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("abrirCrearTarea"));
    }

    #[test]
    fn test_call_ModoNormal_boton_configuracion_on_tap_invokes_abrirMenuConfiguracion() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_boton_configuracion_tap(&Contexto::ModoNormal, &data, &effects);
        assert!(effects.was_called("abrirMenuConfiguracion"));
    }

    #[test]
    fn test_ignored_SesionActiva_display_timer_on_tap() {
        let effects = RecordingEffects::new();
        let data = Boton::default();
        handle_display_timer_tap(&Contexto::SesionActiva, &data, &effects);
        assert!(effects.calls.borrow().is_empty());
    }

    // === Exhaustiveness Test ===

    #[test]
    fn test_exhaustive_contexto_enum() {
        let all_contexts = vec![
            Contexto::MenuConfiguracion,
            Contexto::ModalAcercaDe,
            Contexto::ModalComentario,
            Contexto::ModalCrearActividad,
            Contexto::ModalCrearTarea,
            Contexto::ModalEditarActividad,
            Contexto::ModalEditarTarea,
            Contexto::ModalHistorial,
            Contexto::Historial7Dias,
            Contexto::Historial30Dias,
            Contexto::ModalReset,
            Contexto::ResetFase1,
            Contexto::ResetFase2,
            Contexto::ResetFase3,
            Contexto::ModalSeleccionActividad,
            Contexto::ModoEdicion,
            Contexto::ModoNormal,
            Contexto::SesionActiva,
        ];
        assert_eq!(all_contexts.len(), 18);
    }

    // === On-Entry Effect Tests ===

    #[test]
    fn test_on_entry_ModalAcercaDe_verificar_conexion() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirAcercaDe");
        assert!(effects.was_called("verificar_conexion"));
    }

    #[test]
    fn test_on_entry_ModalAcercaDe_cargar_tiempo_acumulado() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::MenuConfiguracion, &effects);
        sys.handle_event("abrirAcercaDe");
        assert!(effects.was_called("cargar_tiempo_acumulado"));
    }

    #[test]
    fn test_on_entry_Historial7Dias_cargar_historial() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::ModalHistorial, &effects);
        sys.handle_event("iniciar");
        assert!(effects.was_called("cargar_historial"));
    }

    #[test]
    fn test_on_entry_Historial30Dias_cargar_historial() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::Historial7Dias, &effects);
        sys.handle_event("cambiarA30Dias");
        assert!(effects.was_called("cargar_historial"));
    }

    // === Fills Tests ===

    #[test]
    fn test_fills_SesionActiva_ModalComentario_sesion_opts_checkbox_sustituir_cambio() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::ModoNormal, &effects);
        sys.state = Contexto::ModalComentario;
        let data = Checkbox::default();
        handle_checkbox_sustituir_cambio(&sys.state, &data, &effects);
        assert!(effects.was_called("marcarSustituir"));
    }

}
