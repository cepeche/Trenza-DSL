// Smoke test del intérprete WASM contra cronometro_full.trz.
//
// Objetivo: confirmar empíricamente las hipótesis sobre por qué la demo
// del navegador no funciona. Si estas afirmaciones pasan, el intérprete
// está roto para CronometroPSP en formas concretas:
//
// 1. `[close_overlay]` se traduce literalmente a un estado-cadena
//    "close_overlay" del que no se puede salir (sin contexto que lo defina).
// 2. `concurrent_states` se fija en `new()` y nunca se actualiza, por lo
//    que SesionActiva nunca se "co-activa" con ModoNormal: lo reemplaza.
// 3. El payload de `dispatch()` se ignora; los args del Effect proceden
//    del AST literal del .trz (cadenas como "self.tareaId").

use std::fs;
use trenza_core::interpreter::Interpreter;
use trenza_core::parser;

fn load() -> Interpreter {
    let src = fs::read_to_string(
        "../examples/cronometro-wasm/src/cronometro_full.trz",
    )
    .expect("cronometro_full.trz debe existir");
    let program = parser::parse_file(&src).expect("parseo OK");
    Interpreter::new(program)
}

#[test]
fn estado_inicial_es_modo_normal() {
    let interp = load();
    assert_eq!(interp.state.current_state, "ModoNormal");
}

#[test]
fn abrir_crear_tarea_transita_a_modal() {
    let mut interp = load();
    let result = interp.dispatch("abrirCrearTarea", "{}");
    assert_eq!(result.new_state, "ModalCrearTarea");
    assert_eq!(interp.state.current_state, "ModalCrearTarea");
}

/// HALLAZGO 1: tras `cancelar` desde un overlay, el estado es la
/// cadena literal "close_overlay", no un retorno al base anterior.
#[test]
fn close_overlay_deja_estado_huerfano() {
    let mut interp = load();
    interp.dispatch("abrirCrearTarea", "{}");
    let result = interp.dispatch("cancelar", "{}");

    // El intérprete strip-ea los corchetes de "[close_overlay]"
    // y deja la cadena literal como current_state.
    assert_eq!(result.new_state, "close_overlay");
    assert_eq!(interp.state.current_state, "close_overlay");

    // Y a partir de aquí, NINGÚN evento se procesa: no hay contexto
    // llamado "close_overlay", así que dispatch() no encuentra match.
    let result = interp.dispatch("abrirCrearTarea", "{}");
    assert_eq!(
        result.new_state, "close_overlay",
        "el sistema se queda muerto tras cualquier [close_overlay]"
    );
    assert!(
        result.triggered_effects.is_empty(),
        "ningún efecto se dispara desde el estado huérfano"
    );
}

/// HALLAZGO 2: SesionActiva NO es concurrente — reemplaza ModoNormal.
#[test]
fn sesion_activa_reemplaza_en_vez_de_concurrir() {
    let mut interp = load();
    let result = interp.dispatch("iniciarTarea", "{\"tareaId\":\"t1\"}");

    // Esperamos: ModoNormal sigue activo + SesionActiva concurrente.
    // Realidad: current_state pasa a SesionActiva.
    assert_eq!(result.new_state, "SesionActiva");
    // concurrent_states se inicializa una vez y no se actualiza:
    // contiene SesionActiva por declaración, no por estar realmente activo.
    assert_eq!(interp.state.concurrent_states, vec!["SesionActiva".to_string()]);
}

/// HALLAZGO 3: el payload se ignora; los args provienen del AST.
#[test]
fn payload_se_ignora_args_son_del_ast() {
    let mut interp = load();
    let result = interp.dispatch(
        "iniciarTarea",
        "{\"tareaId\":\"runtime-id-xyz\"}",
    );

    // El effect se dispara, pero sus args son las cadenas literales
    // del .trz (`tarea_id`, `notas`, etc.), no el payload runtime.
    let iniciar = result.triggered_effects.iter()
        .find(|e| e.name == "iniciar_sesion")
        .expect("iniciar_sesion debe dispararse");

    // Los args son nombres de parámetros del .trz, no valores reales:
    assert!(
        iniciar.args.iter().all(|a| !a.contains("runtime-id-xyz")),
        "el runtime payload nunca llega al effect: args={:?}",
        iniciar.args
    );
}

/// HALLAZGO 4: eventos en `role` no se procesan si no están también
/// declarados en `transitions:` del contexto actual.
#[test]
fn eventos_de_rol_sin_transition_no_se_procesan() {
    let mut interp = load();
    // En MenuConfiguracion, `role item_historial` declara `on tap -> abrirHistorial`.
    // Pero si dispatchamos `tap` directamente (que es lo que un click podría
    // emitir si confundimos eventos de rol con eventos de contexto), no pasa nada.
    interp.dispatch("abrirMenuConfiguracion", "{}");
    assert_eq!(interp.state.current_state, "MenuConfiguracion");

    let result = interp.dispatch("tap", "{}");
    assert_eq!(
        result.new_state, "MenuConfiguracion",
        "el evento `tap` (de rol) no se traduce automáticamente a abrirHistorial"
    );
}
