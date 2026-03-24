// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2)
// DO NOT EDIT — regenerate from .trz source

#[cfg(test)]
mod algebraic_tests {
    use super::*;

    // === Transition Tests ===

    #[test]
    fn test_transition_EsperandoComando_on_comandoGenerate() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::EsperandoComando, &effects);
        sys.handle_event("comandoGenerate");
        assert_eq!(sys.state, Contexto::ParseandoArchivo);
    }

    #[test]
    fn test_transition_EsperandoComando_on_comandoInvalido() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::EsperandoComando, &effects);
        sys.handle_event("comandoInvalido");
        assert_eq!(sys.state, Contexto::MostrarAyuda);
    }

    #[test]
    fn test_transition_ParseandoArchivo_on_parseoExitoso() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ParseandoArchivo, &effects);
        sys.handle_event("parseoExitoso");
        assert_eq!(sys.state, Contexto::VerificandoReglas);
    }

    #[test]
    fn test_transition_ParseandoArchivo_on_errorSintaxis() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ParseandoArchivo, &effects);
        sys.handle_event("errorSintaxis");
        assert_eq!(sys.state, Contexto::ErrorFatal);
    }

    #[test]
    fn test_transition_VerificandoReglas_on_validacionExitosa() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::VerificandoReglas, &effects);
        sys.handle_event("validacionExitosa");
        assert_eq!(sys.state, Contexto::GenerandoStrands);
    }

    #[test]
    fn test_transition_VerificandoReglas_on_validacionFallida() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::VerificandoReglas, &effects);
        sys.handle_event("validacionFallida");
        assert_eq!(sys.state, Contexto::ErrorFatal);
    }

    #[test]
    fn test_transition_GenerandoStrands_on_generacionCompleta() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::GenerandoStrands, &effects);
        sys.handle_event("generacionCompleta");
        assert_eq!(sys.state, Contexto::Exito);
    }

    #[test]
    fn test_transition_ErrorFatal_on_finalizar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::ErrorFatal, &effects);
        sys.handle_event("finalizar");
        assert_eq!(sys.state, Contexto::EsperandoComando);
    }

    #[test]
    fn test_transition_Exito_on_finalizar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::Exito, &effects);
        sys.handle_event("finalizar");
        assert_eq!(sys.state, Contexto::EsperandoComando);
    }

    #[test]
    fn test_transition_MostrarAyuda_on_finalizar() {
        let effects = NoOpEffects;
        let mut sys = System::new(Contexto::MostrarAyuda, &effects);
        sys.handle_event("finalizar");
        assert_eq!(sys.state, Contexto::EsperandoComando);
    }

    // === Handler Tests ===

    #[test]
    fn test_call_EsperandoComando_terminal_on_ejecutar_invokes_leerYEvaluarArgumentos() {
        let effects = RecordingEffects::new();
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::EsperandoComando, &data, &effects);
        assert!(effects.was_called("leerYEvaluarArgumentos"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_EsperandoComando_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::EsperandoComando, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_EsperandoComando_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::EsperandoComando, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_EsperandoComando_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::EsperandoComando, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_EsperandoComando_logger_on_entrar() {
        let effects = NoOpEffects;
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::EsperandoComando, &data, &effects);
    }

    #[test]
    fn test_call_ParseandoArchivo_lector_fs_on_iniciar_lectura_invokes_leerYParsear() {
        let effects = RecordingEffects::new();
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::ParseandoArchivo, &data, &effects);
        assert!(effects.was_called("leerYParsear"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ParseandoArchivo_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::ParseandoArchivo, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ParseandoArchivo_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::ParseandoArchivo, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ParseandoArchivo_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::ParseandoArchivo, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ParseandoArchivo_logger_on_entrar() {
        let effects = NoOpEffects;
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::ParseandoArchivo, &data, &effects);
    }

    #[test]
    fn test_call_VerificandoReglas_validador_on_iniciar_validacion_invokes_comprobarIntegridad() {
        let effects = RecordingEffects::new();
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::VerificandoReglas, &data, &effects);
        assert!(effects.was_called("comprobarIntegridad"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_VerificandoReglas_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::VerificandoReglas, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_VerificandoReglas_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::VerificandoReglas, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_VerificandoReglas_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::VerificandoReglas, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_VerificandoReglas_logger_on_entrar() {
        let effects = NoOpEffects;
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::VerificandoReglas, &data, &effects);
    }

    #[test]
    fn test_call_GenerandoStrands_generador_rust_on_iniciar_generacion_invokes_emitirCodigoRust() {
        let effects = RecordingEffects::new();
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::GenerandoStrands, &data, &effects);
        assert!(effects.was_called("emitirCodigoRust"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_GenerandoStrands_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::GenerandoStrands, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_GenerandoStrands_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::GenerandoStrands, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_GenerandoStrands_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::GenerandoStrands, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_GenerandoStrands_logger_on_entrar() {
        let effects = NoOpEffects;
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::GenerandoStrands, &data, &effects);
    }

    #[test]
    fn test_call_ErrorFatal_logger_on_entrar_invokes_imprimirErrorFatal() {
        let effects = RecordingEffects::new();
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::ErrorFatal, &data, &effects);
        assert!(effects.was_called("imprimirErrorFatal"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ErrorFatal_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::ErrorFatal, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ErrorFatal_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::ErrorFatal, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ErrorFatal_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::ErrorFatal, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_ErrorFatal_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::ErrorFatal, &data, &effects);
    }

    #[test]
    fn test_call_Exito_logger_on_entrar_invokes_imprimirMensajeExito() {
        let effects = RecordingEffects::new();
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::Exito, &data, &effects);
        assert!(effects.was_called("imprimirMensajeExito"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_Exito_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::Exito, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_Exito_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::Exito, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_Exito_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::Exito, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_Exito_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::Exito, &data, &effects);
    }

    #[test]
    fn test_call_MostrarAyuda_logger_on_entrar_invokes_imprimirAyudaCLI() {
        let effects = RecordingEffects::new();
        let data = ErrorCompilacion::default();
        handle_logger_entrar(&Contexto::MostrarAyuda, &data, &effects);
        assert!(effects.was_called("imprimirAyudaCLI"));
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_MostrarAyuda_terminal_on_ejecutar() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_terminal_ejecutar(&Contexto::MostrarAyuda, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_MostrarAyuda_lector_fs_on_iniciar_lectura() {
        let effects = NoOpEffects;
        let data = ArgumentosUsuario::default();
        handle_lector_fs_iniciar_lectura(&Contexto::MostrarAyuda, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_MostrarAyuda_validador_on_iniciar_validacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_validador_iniciar_validacion(&Contexto::MostrarAyuda, &data, &effects);
    }

    #[test]
    #[should_panic(expected = "Forbidden")]
    fn test_forbidden_MostrarAyuda_generador_rust_on_iniciar_generacion() {
        let effects = NoOpEffects;
        let data = AST::default();
        handle_generador_rust_iniciar_generacion(&Contexto::MostrarAyuda, &data, &effects);
    }

    // === Exhaustiveness Test ===

    #[test]
    fn test_exhaustive_contexto_enum() {
        let all_contexts = vec![
            Contexto::EsperandoComando,
            Contexto::ParseandoArchivo,
            Contexto::VerificandoReglas,
            Contexto::GenerandoStrands,
            Contexto::ErrorFatal,
            Contexto::Exito,
            Contexto::MostrarAyuda,
        ];
        assert_eq!(all_contexts.len(), 7);
    }

    // === On-Entry Effect Tests ===

    // === Fills Tests ===

}
