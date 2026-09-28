// Test de comportamiento sobre el Rust generado a partir de
// examples/replace_overlay.trz (lo añade scripts/check-generated.sh).
// `[replace] X` cierra el overlay actual y abre X en su lugar: la pila no
// crece, y al cerrar X se vuelve al contexto base, no al menú.
#[cfg(test)]
mod replace_target {
    use super::*;

    #[test]
    fn choosing_a_menu_item_replaces_the_menu() {
        let effects = RecordingEffects::new();
        let mut sys = System::new(Contexto::Home, &effects);
        let b = Button { id: "b".into() };

        sys.dispatch_menu_button_tap(&b);
        assert_eq!(sys.overlay_stack, vec![Contexto::Menu]);

        assert_eq!(sys.dispatch_about_item_tap(&b), Some("openAbout"));
        assert_eq!(sys.overlay_stack, vec![Contexto::About]);
        assert_eq!(sys.current_state(), Contexto::About);

        sys.dispatch_close_button_tap(&b);
        assert!(sys.overlay_stack.is_empty());
        assert_eq!(sys.current_state(), Contexto::Home);
    }
}
