// Test de comportamiento sobre el Rust generado a partir de
// paper/onward2027/listings/modes.trz (lo añade scripts/check-generated.sh).
// Fija la semántica decidida el 2026-09-25: la transición la dispara la
// ACCIÓN que produce el manejador, no el evento.
#[cfg(test)]
mod action_triggers_transition {
    use super::*;

    #[test]
    fn tap_on_edit_button_switches_mode_and_tap_on_card_does_not() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::NormalMode, &effects);

        let t = Task { taskId: "t1".into() };
        assert_eq!(sys.dispatch_card_tap(&t), Some("startTask"));
        assert_eq!(sys.current_state(), Contexto::NormalMode);

        let b = Tab { id: "b".into() };
        assert_eq!(sys.dispatch_edit_button_tap(&b), Some("enterEditMode"));
        assert_eq!(sys.current_state(), Contexto::EditMode);

        // `on tap -> ignored`: no hay acción ni transición.
        let f = Tab { id: "f".into() };
        assert_eq!(sys.dispatch_frequent_tab_tap(&f), None);
        assert_eq!(sys.current_state(), Contexto::EditMode);

        // El mismo tap sobre la tarjeta produce otra acción en EditMode.
        assert_eq!(sys.dispatch_card_tap(&t), Some("openTaskEditor"));
    }
}
