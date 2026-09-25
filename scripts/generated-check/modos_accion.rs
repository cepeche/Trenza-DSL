// Test de comportamiento sobre el Rust generado a partir de
// paper/onward2027/listings/modos.trz (lo añade scripts/check-generated.sh).
// Fija la semántica decidida el 2026-09-25: la transición la dispara la
// ACCIÓN que produce el manejador, no el evento.
#[cfg(test)]
mod accion_dispara_transicion {
    use super::*;

    #[test]
    fn tap_en_boton_edicion_cambia_de_modo_y_tap_en_tarjeta_no() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::ModoNormal, &effects);

        let t = Tarea { tareaId: "t1".into() };
        assert_eq!(sys.dispatch_tarjeta_tap(&t), Some("iniciarTarea"));
        assert_eq!(sys.current_state(), Contexto::ModoNormal);

        let b = Pestana { id: "b".into() };
        assert_eq!(sys.dispatch_boton_edicion_tap(&b), Some("activarEdicion"));
        assert_eq!(sys.current_state(), Contexto::ModoEdicion);

        // `on tap -> ignored`: no hay acción ni transición.
        let f = Pestana { id: "f".into() };
        assert_eq!(sys.dispatch_pestana_frecuentes_tap(&f), None);
        assert_eq!(sys.current_state(), Contexto::ModoEdicion);

        // El mismo tap sobre la tarjeta produce otra acción en ModoEdicion.
        assert_eq!(sys.dispatch_tarjeta_tap(&t), Some("abrirEditarTarea"));
    }
}
