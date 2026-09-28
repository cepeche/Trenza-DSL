// Un caso positivo y al menos uno negativo por cada regla del verificador
// (trenza-core/src/validator.rs). Cada negativo afirma el conjunto EXACTO de
// códigos de diagnóstico, para que una regla que empiece a disparar de más
// (o de menos) rompa el test.
//
// Las reglas se describen formalmente en paper/onward2027/sec-language.tex.
// Los tests marcados "LIMITACIÓN" documentan el comportamiento actual en
// casos donde la regla es más estrecha de lo que su nombre sugiere; si se
// amplía la regla, hay que actualizar el test y el paper a la vez.

use trenza_core::{parser, validator};

fn codes(src: &str) -> Vec<String> {
    let program = parser::parse_file(src).expect("la especificación debe parsear");
    let mut codes = match validator::verify(&program) {
        Ok(()) => vec![],
        Err(diags) => diags.into_iter().map(|d| d.code).collect(),
    };
    codes.sort();
    codes
}

fn ok(src: &str) {
    assert_eq!(codes(src), Vec::<String>::new(), "se esperaba sin diagnósticos");
}

fn only(src: &str, expected: &[&str]) {
    let mut expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
    expected.sort();
    assert_eq!(codes(src), expected);
}

/// Declara `back` como señal externa del sistema: los tests de R1 quitan el
/// manejador que produce `back`, y así la vuelta de B a A sigue viva (R10).
fn con_back_externo(src: &str) -> String {
    src.replace("    initial: A\n", "    initial: A\n    events: back\n")
}

/// Especificación mínima válida: dos contextos que se alternan.
const BASE: &str = "
data D:
    x: Id

system S:
    initial: A
    contexts:
        A
        B

context A:
    role r: D
        on e -> go
    transitions:
        on go -> B

context B:
    role r: D
        on e -> back
    transitions:
        on back -> A
";

#[test]
fn base_es_valida() {
    ok(BASE);
}

// ---------------------------------------------------------------- R1

#[test]
fn r1_manejador_ausente_es_error() {
    let src = con_back_externo(&BASE.replace("        on e -> back\n", ""));
    only(&src, &["completeness"]);
}

#[test]
fn r1_ignored_explicito_satisface_la_regla() {
    let src = con_back_externo(&BASE.replace("on e -> back", "on e -> ignored"));
    ok(&src);
}

#[test]
fn r1_forbidden_explicito_satisface_la_regla() {
    ok(&con_back_externo(&BASE.replace("on e -> back", "on e -> forbidden")));
}

#[test]
fn r1_comodin_exime_al_contexto() {
    let src = con_back_externo(&BASE
        .replace("        on e -> back\n", "")
        .replace("context B:\n", "context B:\n    role *: ignored\n"));
    ok(&src);
}

// ---------------------------------------------------------------- R2

#[test]
fn r2_manejador_duplicado_es_error() {
    let src = BASE.replace("        on e -> go\n", "        on e -> go\n        on e -> go\n");
    only(&src, &["determinism"]);
}

#[test]
fn r2_duplicado_con_destinos_distintos_es_error() {
    let src = BASE.replace("        on e -> go\n", "        on e -> go\n        on e -> ignored\n");
    only(&src, &["determinism"]);
}

// ---------------------------------------------------------------- R3

#[test]
fn r3_contexto_inalcanzable_es_aviso() {
    let src = format!(
        "{BASE}
context C:
    role r: D
        on e -> z
    transitions:
        on z -> A
"
    );
    only(&src, &["reachability"]);
    let program = parser::parse_file(&src).unwrap();
    let diags = validator::verify(&program).unwrap_err();
    assert_eq!(diags[0].severity, "warning");
}

// ---------------------------------------------------------------- R4

#[test]
fn r4_sumidero_es_error() {
    let src = BASE.replace("    transitions:\n        on back -> A\n", "");
    only(&src, &["return"]);
}

#[test]
fn r4_retorno_indirecto_es_valido() {
    let src = BASE
        .replace("        A\n        B\n", "        A\n        B\n        C\n")
        .replace("on back -> A", "on back -> C")
        + "
context C:
    role r: D
        on e -> home
    transitions:
        on home -> A
";
    ok(&src);
}

// ---------------------------------------------------------------- R5

#[test]
fn r5_rol_ausente_es_error() {
    let src = BASE.replace("context A:\n", "context A:\n    role q: D\n");
    only(&src, &["exhaustiveness"]);
}

#[test]
fn r5_rol_ausente_con_manejador_dispara_tambien_r1() {
    let src = BASE.replace("context A:\n", "context A:\n    role q: D\n        on e -> ignored\n");
    only(&src, &["completeness", "exhaustiveness"]);
}

// ---------------------------------------------------------------- R6

const PRIVADO: &str = "
data P [privacy: gdpr]:
    nombre: Texto

system S:
    initial: A

context A:
    role r ANOT: P
        on e -> enviar(r.nombre)
";

#[test]
fn r6_campo_protegido_sin_permiso_es_error() {
    only(&PRIVADO.replace(" ANOT", ""), &["privacy"]);
}

#[test]
fn r6_campo_protegido_con_permiso_es_valido() {
    ok(&PRIVADO.replace(" ANOT", " [access: gdpr]"));
}

#[test]
fn r6_limitacion_self_no_se_comprueba() {
    // LIMITACIÓN: la regla sólo reconoce `rol.campo` o `binding.campo`;
    // el mismo acceso escrito como `self.nombre` no se detecta.
    ok(&PRIVADO.replace(" ANOT", "").replace("r.nombre", "self.nombre"));
}

// ---------------------------------------------------------------- R7

const SLOTS: &str = "
system S:
    initial: Base
    events: activar desactivar
    contexts:
        Base
    concurrent:
        Aux
    overlays:
        Popup

data El:
    id: Id

context Base:
    role b: El
        on tap -> abrir()
    transitions:
        on abrir -> Popup
        on activar -> Aux

context Popup:
    role b: El
        on tap -> cerrar()
    slot extras
    transitions:
        on cerrar -> Base

context Aux:
    role b: El
        on tap -> ignored
    fills Popup.DESTINO:
        role ind: El
            on tap -> info()
    transitions:
        on desactivar -> Base
";

#[test]
fn r7_fills_a_slot_existente_es_valido() {
    ok(&SLOTS.replace("DESTINO", "extras"));
}

#[test]
fn r7_fills_a_slot_inexistente_es_error() {
    only(&SLOTS.replace("DESTINO", "inexistente"), &["slot"]);
}

#[test]
fn r7_manejador_duplicado_dentro_de_fills_es_error() {
    let src = SLOTS
        .replace("DESTINO", "extras")
        .replace("            on tap -> info()\n", "            on tap -> info()\n            on tap -> info()\n");
    only(&src, &["determinism-fills"]);
}

#[test]
fn r7_limitacion_slot_sin_fills_no_se_detecta() {
    // LIMITACIÓN: un slot que nadie llena no produce diagnóstico.
    let src = SLOTS.replace("    fills Popup.DESTINO:\n        role ind: El\n            on tap -> info()\n", "");
    ok(&src);
}

// ---------------------------------------------------------------- R8

#[test]
fn r8_tipos_distintos_para_el_mismo_rol_es_error() {
    let src = BASE
        .replace("data D:\n    x: Id\n", "data D:\n    x: Id\n\ndata E:\n    y: Id\n")
        .replace("context B:\n    role r: D", "context B:\n    role r: E");
    only(&src, &["type-consistency"]);
}

// ------------------------------------------------ reglas complementarias

#[test]
fn prefijo_reservado_es_error() {
    only(&BASE.replace("role r: D", "role _r: D"), &["name-reserved", "name-reserved"]);
}

#[test]
fn initial_en_contexto_que_no_es_overlay_es_error() {
    let src = BASE.replace("context A:\n", "context A:\n    initial: B\n");
    only(&src, &["initial-not-overlay"]);
}

// ------------------------------------ ámbito de R1/R5: grupos de hermanos
//
// Decisión 2026-09-25: R1 y R5 se aplican entre contextos hermanos (los
// base entre sí; los sub-contextos de un mismo overlay entre sí). Un
// overlay no hereda los roles del base que suspende.

const HERMANOS: &str = "
data D:
    x: Id

system S:
    initial: A
    contexts:
        A
        B
    overlays:
        M

context A:
    role r: D
        on e -> go
    role abrir: D
        on e -> abrirM
    transitions:
        on go -> B
        on abrirM -> M

context B:
    role r: D
        on e -> back
    role abrir: D
        on e -> ignored
    transitions:
        on back -> A

context M:
    initial: M1

context M1:
    role siguiente: D
        on e -> paso2
    role cerrar: D
        on e -> cerrarM
    transitions:
        on paso2 -> M2
        on cerrarM -> [close_overlay]

context M2:
    role siguiente: D
        on e -> ignored
    role cerrar: D
        on e -> cerrarM
    transitions:
        on cerrarM -> [close_overlay]
";

#[test]
fn hermanos_overlay_no_hereda_roles_del_base() {
    // M, M1 y M2 no declaran `r` ni `abrir`, y A/B no declaran `siguiente`
    // ni `cerrar`: con el ámbito global esto daría errores; por hermanos no.
    ok(HERMANOS);
}

#[test]
fn hermanos_subcontextos_del_mismo_overlay_se_comparan() {
    let src = HERMANOS.replace("    role siguiente: D\n        on e -> ignored\n", "");
    only(&src, &["completeness", "exhaustiveness"]);
}

#[test]
fn hermanos_contextos_base_se_comparan() {
    let src = HERMANOS.replace("    role abrir: D\n        on e -> ignored\n", "");
    only(&src, &["completeness", "exhaustiveness"]);
}

#[test]
fn hermanos_el_caso_de_estudio_verifica_sin_comodines() {
    // Desde el 2026-09-25 cronometro_full.trz no usa `role *` en ningún
    // contexto: con R1/R5 por hermanos, cada contexto declara todos los
    // roles de su grupo (los botones ausentes de una fase del asistente de
    // reset, como `forbidden`).
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/cronometro-wasm/src/cronometro_full.trz"
    ))
    .unwrap();
    assert!(!src.contains("role *"));
    // Sin errores. Quedan dos avisos que son un hallazgo real, no ruido:
    // la ramificación de `seleccionarTipoTarea` según el número de
    // actividades depende de datos (GAP-5) y el DSL no la expresa, así que
    // `elegirActividad` nunca se produce en ModoNormal y
    // ModalSeleccionActividad queda inalcanzable.
    only(&src, &["dead-transition", "reachability"]);
}

// --------------------------------------- R10, `pending`, `events:` (2026-09-28)

fn severity_of(src: &str, code: &str) -> String {
    let program = parser::parse_file(src).unwrap();
    let diags = validator::verify(&program).unwrap_err();
    diags.into_iter().find(|d| d.code == code).unwrap().severity
}

#[test]
fn r10_transicion_sin_productor_es_aviso() {
    // Nadie produce `nunca` en A: la transición es código muerto.
    let src = BASE.replace("        on go -> B\n", "        on go -> B\n        on nunca -> B\n");
    only(&src, &["dead-transition"]);
    assert_eq!(severity_of(&src, "dead-transition"), "warning");
}

#[test]
fn r10_senal_declarada_en_events_esta_viva() {
    let src = BASE
        .replace("        on go -> B\n", "        on go -> B\n        on nunca -> B\n")
        .replace("    initial: A\n", "    initial: A\n    events: nunca\n");
    ok(&src);
}

#[test]
fn r10_la_arista_muerta_no_cuenta_para_r4() {
    // Si la única vuelta de B a A es muerta, B no puede volver: R4 lo ve.
    let src = BASE.replace("        on e -> back\n", "        on e -> ignored\n");
    only(&src, &["dead-transition", "return"]);
}

#[test]
fn events_acepta_varias_senales_en_varias_lineas() {
    let src = BASE
        .replace("        on go -> B\n", "        on go -> B\n        on uno -> B\n        on dos -> B\n")
        .replace("    initial: A\n", "    initial: A\n    events:\n        uno\n        dos\n");
    ok(&src);
}

#[test]
fn pending_en_manejador_es_aviso_y_satisface_r1() {
    let src = con_back_externo(&BASE.replace("on e -> back", "on e -> pending"));
    only(&src, &["pending-handler"]);
    assert_eq!(severity_of(&src, "pending-handler"), "warning");
}

#[test]
fn pending_en_contexto_sustituye_al_aviso_de_alcanzabilidad() {
    let src = BASE.replace("        A\n        B\n", "        A\n        B\n        C\n")
        + "
context C:
    pending
    role r: D
        on e -> home
    transitions:
        on home -> A
";
    only(&src, &["pending-context"]);
    assert_eq!(severity_of(&src, "pending-context"), "warning");
}

#[test]
fn replace_a_otro_overlay_es_valido_y_cuenta_como_arista() {
    // M1 sustituye M por N; N sólo es alcanzable a través del `[replace]`.
    let src = HERMANOS
        .replace("    overlays:\n        M\n", "    overlays:\n        M\n        N\n")
        .replace("        on paso2 -> M2\n", "        on paso2 -> [replace] N\n")
        + "
context N:
    role r: D
        on e -> cerrarN
    transitions:
        on cerrarN -> [close_overlay]
";
    // M2 deja de ser alcanzable (su única entrada era paso2): eso lo avisa R3.
    only(&src, &["reachability"]);
}
