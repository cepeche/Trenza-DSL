---
date: 2026-03-24
from: Claude Opus 4.6
to: Gemini (implementador)
subject: "Diseno tecnico de Strand 2 — Algebraic Test Generation"
---

# Diseno tecnico: Strand 2 — Algebraic Test Generation

**Objetivo**: Este documento contiene todo lo necesario para que Gemini
implemente la generacion de `_out_tests.rs` en el compilador Rust (`trenza-cli/`)
en una sola sesion. No requiere decisiones de diseno adicionales.

**Documentos de referencia**:
- `trenza-cli/src/generator.rs` (generador actual, Strands 1/3/4)
- `trenza-cli/src/ast.rs` (AST vigente, incluyendo slot/fills)
- `trenza-cli/src/validator.rs` (7 reglas activas)
- `spec/reference/cronometro-psp/trenza/contexts/ModoNormal.trz` (ejemplo base)
- `spec/reference/cronometro-psp/trenza/contexts/ModalComentario.trz` (ejemplo con slot)
- `spec/reference/cronometro-psp/trenza/contexts/SesionActiva.trz` (ejemplo con fills)

---

## 1. Decisiones de diseno

### 1.1 Framework: `#[test]` nativo de Rust

**Decision**: Usar exclusivamente `#[test]` con `#[should_panic]`. No usar
`proptest`, `quickcheck`, ni ningun framework externo.

**Justificacion**:

1. **Cero dependencias externas.** La promesa de Trenza es que la spec genera
   artefactos autosuficientes. Un `_out_tests.rs` que requiere `proptest` en
   `Cargo.toml` viola esta promesa. Con `#[test]` nativo, el archivo compila
   con `cargo test` sin configuracion adicional.

2. **Los tests son algebraicos, no probabilisticos.** La cobertura es
   *completa por construccion*: se genera un test por cada celda de la matriz
   (contexto x rol x evento) y por cada transicion. No hay espacio de entrada
   a explorar aleatoriamente. `proptest` genera valor cuando el espacio de
   estados es grande y no enumerable; aqui el espacio es finito, conocido, y
   pequeno.

3. **Legibilidad para auditoria.** Cada test generado tiene un nombre
   determinista que codifica exactamente que propiedad verifica. Un auditor
   puede leer `test_transition_ModoNormal_on_activarEdicion` y saber
   instantaneamente que verifica sin leer el cuerpo.

4. **Consistencia con Strand 1.** El generador actual ya produce un bloque
   `#[cfg(test)] mod tests` dentro de `_out.rs` con tests basicos. Strand 2
   extiende este patron, no lo reemplaza.

### 1.2 Estructura: archivo separado `_out_tests.rs`

**Decision**: Generar un archivo separado `_out_tests.rs` que importa del
modulo principal con `use super::*` o `use crate::*`.

**Justificacion**:

1. **Separacion de concerns.** El `_out.rs` actual ya mezcla implementacion y
   tests basicos (lineas 157-191 del generador). Strand 2 genera tests mucho
   mas extensos (potencialmente cientos de funciones). Meterlos todos en
   `_out.rs` produciria archivos de miles de lineas donde la implementacion
   queda enterrada.

2. **Compilacion condicional limpia.** El archivo completo vive bajo
   `#[cfg(test)]` por ser un modulo de test. No se compila en release.

3. **Facilita inspeccion humana.** Un auditor que quiere revisar los tests
   abre un solo archivo. Un auditor que quiere revisar la implementacion abre
   otro. No tienen que navegar dentro de un archivo monolitico.

4. **Patron idiomatico de Rust.** El ecosistema Rust usa ampliamente archivos
   `tests/` separados y modulos `_test.rs` para tests de integracion. El
   compilador generado sigue esta convencion.

**Implicacion para el generador**: se anade una nueva funcion publica
`generate_tests(program: &Program) -> String` en `generator.rs`, que se
invoca desde `main.rs` junto a los demas generadores.

**Eliminacion del bloque existente**: los tests basicos que actualmente se
generan dentro de `generate_rust()` (lineas 157-191) se eliminan de esa
funcion. Toda la generacion de tests se centraliza en `generate_tests()`.
Esto evita duplicacion y garantiza que hay una unica fuente de verdad para
los tests generados.

### 1.3 Mocking de handlers: trait `Effects` inyectable

**Problema**: Los handlers generados en Strand 1 llaman a funciones como
`seleccionarTipoTarea(self.tipoId)`, `iniciar_sesion(...)`, etc. Estas
funciones no existen en el contexto de tests porque son implementaciones
externas (API calls, IO, etc.).

**Decision**: Generar un trait `Effects` con un metodo por cada `ActionCall`
unica en el programa, y un `NoOpEffects` que implementa todos como no-ops.

**Estructura generada**:

```rust
pub trait Effects {
    fn seleccionar_tipo_tarea(&self, tipo_id: &str) {}
    fn iniciar_sesion(&self, tarea_id: &str, notas: &str, minutos: &str, sustituir: &str) {}
    fn actualizar_comentario(&self, valor: &str) {}
    // ... un metodo por cada ActionCall.function unica
}

pub struct NoOpEffects;
impl Effects for NoOpEffects {}

pub struct RecordingEffects {
    pub calls: std::cell::RefCell<Vec<String>>,
}
impl Effects for RecordingEffects {
    fn seleccionar_tipo_tarea(&self, tipo_id: &str) {
        self.calls.borrow_mut().push(format!("seleccionar_tipo_tarea({})", tipo_id));
    }
    // ... etc.
}
```

**Justificacion**:

1. **Los tests de transicion no necesitan effects reales.** Solo verifican
   que el estado cambia correctamente. `NoOpEffects` es suficiente.

2. **Los tests de effects de entrada SI necesitan verificar invocacion.**
   `RecordingEffects` captura las llamadas para que el test pueda hacer
   `assert!(calls.contains("cargar_opciones_sesion()"))`.

3. **Cero dependencias.** No usa `mockall` ni ningun crate de mocking. Todo
   se genera como Rust plano con un trait y dos implementaciones.

4. **El trait vive en `_out.rs`**, no en los tests. Esto permite que el
   `System` generado en Strand 1 sea generico sobre `Effects`, habilitando
   tanto el uso real como el testing.

**Cambio necesario en Strand 1**: `System::new` y `System::handle_event`
deben aceptar un parametro `&dyn Effects` (o `System` debe ser generico
`System<E: Effects>`). Este es un cambio en `generate_rust()`, no en el AST.

### 1.4 Cobertura de slot/fills: tests condicionales

**Problema**: Los roles dentro de un `fills` solo existen cuando el contexto
concurrent esta activo simultaneamente con el overlay. Los tests deben
reflejar esta condicionalidad.

**Decision**: Generar tests especificos para fills que:
1. Activan el concurrent Y el overlay antes de testear.
2. Verifican que los roles del fills responden correctamente.
3. Incluyen un test negativo: sin el concurrent activo, los roles del fills
   NO deben responder.

**Ejemplo generado para `SesionActiva.fills ModalComentario.sesion_opts`**:

```rust
#[test]
fn test_fills_sesion_activa_modal_comentario_sesion_opts_checkbox_sustituir_cambio() {
    let effects = RecordingEffects::new();
    let mut sys = System::new(Contexto::ModoNormal, &effects);
    sys.activate_concurrent(Contexto::SesionActiva);
    sys.handle_event("seleccionarTipoTarea"); // -> ModalComentario
    // Ahora SesionActiva + ModalComentario estan activos
    // El rol checkbox_sustituir del fills debe responder
    sys.handle_role_event("checkbox_sustituir", "cambio");
    assert!(effects.was_called("marcarSustituir"));
}

#[test]
fn test_fills_absent_sesion_activa_checkbox_sustituir_inert() {
    let effects = RecordingEffects::new();
    let mut sys = System::new(Contexto::ModoNormal, &effects);
    // NO activamos SesionActiva
    sys.handle_event("seleccionarTipoTarea"); // -> ModalComentario
    // checkbox_sustituir no debe existir en este estado
    sys.handle_role_event("checkbox_sustituir", "cambio");
    assert!(!effects.was_called("marcarSustituir"));
}
```

### 1.5 Nomenclatura de tests

Los nombres de test siguen un patron determinista para facilitar busqueda y
auditoria:

| Categoria | Patron de nombre |
|-----------|-----------------|
| Transicion | `test_transition_{Contexto}_on_{evento}` |
| Forbidden | `test_forbidden_{Contexto}_{rol}_on_{evento}` |
| Ignored | `test_ignored_{Contexto}_{rol}_on_{evento}` |
| Completitud | `test_exhaustive_contexto_enum` |
| Effects on_entry | `test_on_entry_{Contexto}_{funcion}` |
| Fills positivo | `test_fills_{concurrent}_{overlay}_{slot}_{rol}_{evento}` |
| Fills negativo | `test_fills_absent_{concurrent}_{rol}_inert` |

---

## 2. Extensiones al AST

**No se requiere ninguna extension al AST.** El AST actual (`ast.rs`) ya
contiene toda la informacion necesaria para generar los tests:

- `ContextDef.transitions` -> tests de transicion
- `RoleAction.target` (variantes `Forbidden`, `Ignored`, `Call`) -> tests por categoria
- `ContextDef.effects` con `EffectTrigger::Lifecycle("on_entry")` -> tests de entrada
- `ContextDef.slots` y `ContextDef.fills` -> tests de slot/fills
- `SystemDef.sections` -> metadata de concurrent/overlay para condicionalidad

La unica adicion es en el **generador**, no en el AST.

---

## 3. Algoritmo de generacion

### 3.1 Funcion principal

```rust
pub fn generate_tests(program: &Program) -> String {
    let mut output = String::new();
    let metadata = extract_system_metadata(program);

    output.push_str("// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2)\n");
    output.push_str("// DO NOT EDIT — regenerate from .trz source\n\n");
    output.push_str("#[cfg(test)]\nmod algebraic_tests {\n");
    output.push_str("    use super::*;\n\n");

    // Fase 1: Tests de transicion
    generate_transition_tests(program, &metadata, &mut output);

    // Fase 2: Tests de handlers (forbidden, ignored, call)
    generate_handler_tests(program, &metadata, &mut output);

    // Fase 3: Test de completitud del enum
    generate_exhaustiveness_test(program, &mut output);

    // Fase 4: Tests de effects on_entry
    generate_on_entry_tests(program, &mut output);

    // Fase 5: Tests de slot/fills
    generate_fills_tests(program, &metadata, &mut output);

    output.push_str("}\n");
    output
}
```

### 3.2 extract_system_metadata

```rust
struct SystemMetadata {
    initial: String,
    base_contexts: HashSet<String>,
    concurrent_contexts: HashSet<String>,
    overlays: HashSet<String>,
}

fn extract_system_metadata(program: &Program) -> SystemMetadata {
    // Recorre program.definitions buscando Definition::System
    // Extrae initial, contexts, concurrent, overlays
    // Identico a lo que hace el validator en lineas 8-32
}
```

### 3.3 Fase 1: generate_transition_tests

```
PARA CADA context EN program.definitions (solo Context):
    PARA CADA trans EN context.transitions:
        SI trans.target == "[stay]":
            GENERAR test que verifica estado NO cambia
        SI trans.target == "[cerrar_overlay]":
            GENERAR test que verifica retorno al initial
        SI trans.target == "[deactivate]":
            GENERAR test que verifica desactivacion del concurrent
        SINO:
            GENERAR test que verifica estado == trans.target

        Nombre: test_transition_{context.name}_on_{trans.event}
```

**Generacion concreta para transicion normal**:

```rust
fn generate_transition_tests(program: &Program, meta: &SystemMetadata, out: &mut String) {
    out.push_str("    // === Transition Tests ===\n\n");

    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            for trans in &ctx.transitions {
                let event_safe = trans.event.replace(".", "_");
                let test_name = format!("test_transition_{}_on_{}", ctx.name, event_safe);

                out.push_str(&format!("    #[test]\n"));
                out.push_str(&format!("    fn {}() {{\n", test_name));

                // Determinar como llegar al contexto bajo test
                if meta.concurrent_contexts.contains(&ctx.name) {
                    // Para concurrents: crear sistema normal y activar concurrent
                    out.push_str(&format!(
                        "        let mut sys = System::new(Contexto::{});\n",
                        meta.initial
                    ));
                    out.push_str(&format!(
                        "        sys.activate_concurrent(Contexto::{});\n",
                        ctx.name
                    ));
                } else {
                    // Para base/overlay: iniciar directamente en ese contexto
                    out.push_str(&format!(
                        "        let mut sys = System::new(Contexto::{});\n",
                        ctx.name
                    ));
                }

                out.push_str(&format!(
                    "        sys.handle_event(\"{}\");\n",
                    trans.event
                ));

                // Determinar asercion segun tipo de target
                let target = &trans.target;
                if target == "[cerrar_overlay]" {
                    out.push_str(&format!(
                        "        assert_eq!(sys.state, Contexto::{});\n",
                        meta.initial
                    ));
                } else if target == "[deactivate]" {
                    out.push_str(&format!(
                        "        assert!(!sys.concurrent_states.contains(&Contexto::{}));\n",
                        ctx.name
                    ));
                } else if target == "[stay]" {
                    out.push_str(&format!(
                        "        assert_eq!(sys.state, Contexto::{});\n",
                        ctx.name
                    ));
                } else {
                    out.push_str(&format!(
                        "        assert_eq!(sys.state, Contexto::{});\n",
                        target
                    ));
                }

                out.push_str("    }\n\n");
            }
        }
    }
}
```

### 3.4 Fase 2: generate_handler_tests

```
PARA CADA context EN program.definitions (solo Context):
    PARA CADA role EN context.roles:
        PARA CADA action EN role.actions:
            MATCH action.target:
                Forbidden =>
                    GENERAR test con #[should_panic]
                    Nombre: test_forbidden_{ctx}_{role}_{event}

                Ignored =>
                    GENERAR test que verifica no-cambio y no-panic
                    Nombre: test_ignored_{ctx}_{role}_{event}

                Call(call) =>
                    GENERAR test que verifica invocacion via RecordingEffects
                    Nombre: test_call_{ctx}_{role}_{event}_{function}
```

**Ejemplo concreto de handler forbidden**:

```rust
// Para ModoEdicion.boton_nuevo.on_tap -> forbidden (hipotetico)
#[test]
#[should_panic(expected = "Forbidden")]
fn test_forbidden_ModoEdicion_boton_nuevo_on_tap() {
    let effects = NoOpEffects;
    handle_boton_nuevo_tap(&Contexto::ModoEdicion, &effects);
}
```

**Ejemplo concreto de handler ignored**:

```rust
// Para SesionActiva.display_timer.on_tap -> ignored
#[test]
fn test_ignored_SesionActiva_display_timer_on_tap() {
    let effects = RecordingEffects::new();
    let state_before = Contexto::SesionActiva;
    handle_display_timer_tap(&state_before, &effects);
    // No panic, no effect calls
    assert!(effects.calls.borrow().is_empty());
}
```

### 3.5 Fase 3: generate_exhaustiveness_test

Un solo test que verifica que el enum `Contexto` tiene exactamente las
variantes esperadas. Esto detecta desincronizacion entre el `.trz` y el
codigo generado.

```rust
#[test]
fn test_exhaustive_contexto_enum() {
    // Verificar que el match es exhaustivo
    // (el compilador de Rust ya lo garantiza, pero este test
    // documenta el inventario explicitamente)
    let all_contexts = vec![
        Contexto::ModoNormal,
        Contexto::ModoEdicion,
        Contexto::SesionActiva,
        Contexto::ModalComentario,
        // ... todos los contextos del programa
    ];
    assert_eq!(all_contexts.len(), EXPECTED_CONTEXT_COUNT);
}
```

La constante `EXPECTED_CONTEXT_COUNT` se genera automaticamente como el
numero de `ContextDef` en el programa.

### 3.6 Fase 4: generate_on_entry_tests

```
PARA CADA context EN program.definitions (solo Context):
    PARA CADA effect EN context.effects:
        SI effect.trigger == Lifecycle("on_entry"):
            GENERAR test que:
              1. Crea sistema en un contexto vecino
              2. Transita hacia el contexto bajo test
              3. Verifica que RecordingEffects capturo la llamada
            Nombre: test_on_entry_{ctx}_{function}
```

**Prerequisito**: Para generar el paso "transita hacia el contexto", el
generador necesita encontrar una transicion que lleve al contexto destino.
El algoritmo busca en todas las transiciones del programa una cuyo `target`
sea el contexto en cuestion. Si no encuentra ninguna (contexto inalcanzable
— imposible si paso la regla 3), omite el test y emite un comentario.

### 3.7 Fase 5: generate_fills_tests

```
PARA CADA context EN program.definitions (solo Context):
    PARA CADA fills_def EN context.fills:
        target_ctx = fills_def.target_context
        target_slot = fills_def.target_slot
        source_ctx = context.name  (el concurrent)

        PARA CADA role EN fills_def.roles:
            PARA CADA action EN role.actions:
                -- Test positivo: concurrent activo + overlay activo
                GENERAR test_fills_{source}_{target}_{slot}_{role}_{event}:
                    1. Crear sistema con initial
                    2. Activar concurrent source_ctx
                    3. Navegar a overlay target_ctx
                    4. Invocar handle_role_event(role, event)
                    5. Verificar efecto esperado

                -- Test negativo: overlay activo SIN concurrent
                GENERAR test_fills_absent_{source}_{role}_inert:
                    1. Crear sistema con initial
                    2. NO activar concurrent
                    3. Navegar a overlay target_ctx
                    4. Invocar handle_role_event(role, event)
                    5. Verificar que NO se invoco el efecto

        PARA CADA effect EN fills_def.effects:
            SI effect.trigger == Lifecycle("on_entry"):
                GENERAR test de on_entry condicional:
                    Con concurrent activo: efecto se invoca
                    Sin concurrent activo: efecto NO se invoca
```

---

## 4. Ejemplo de output

### 4.1 Tests generados para ModoNormal

Basado en `spec/reference/cronometro-psp/trenza/contexts/ModoNormal.trz`:

```rust
// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2)
// DO NOT EDIT — regenerate from .trz source

#[cfg(test)]
mod algebraic_tests {
    use super::*;

    // === Transition Tests ===

    #[test]
    fn test_transition_ModoNormal_on_activarEdicion() {
        let mut sys = System::new(Contexto::ModoNormal);
        sys.handle_event("activarEdicion");
        assert_eq!(sys.state, Contexto::ModoEdicion);
    }

    #[test]
    fn test_transition_ModoNormal_on_abrirCrearTarea() {
        let mut sys = System::new(Contexto::ModoNormal);
        sys.handle_event("abrirCrearTarea");
        assert_eq!(sys.state, Contexto::ModalCrearTarea);
    }

    #[test]
    fn test_transition_ModoNormal_on_abrirMenuConfiguracion() {
        let mut sys = System::new(Contexto::ModoNormal);
        sys.handle_event("abrirMenuConfiguracion");
        assert_eq!(sys.state, Contexto::MenuConfiguracion);
    }

    #[test]
    fn test_transition_ModoNormal_on_seleccionarTipoTarea() {
        let mut sys = System::new(Contexto::ModoNormal);
        sys.handle_event("seleccionarTipoTarea");
        assert_eq!(sys.state, Contexto::ModalComentario);
    }

    // === Handler Tests: Calls ===

    #[test]
    fn test_call_ModoNormal_tarjeta_tipo_on_tap() {
        let effects = RecordingEffects::new();
        handle_tarjeta_tipo_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("seleccionarTipoTarea"));
    }

    #[test]
    fn test_call_ModoNormal_tarjeta_tarea_on_tap() {
        let effects = RecordingEffects::new();
        handle_tarjeta_tarea_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("iniciarTarea"));
    }

    #[test]
    fn test_call_ModoNormal_pestana_actividad_on_tap() {
        let effects = RecordingEffects::new();
        handle_pestana_actividad_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("cambiarPestana"));
    }

    #[test]
    fn test_call_ModoNormal_pestana_frecuentes_on_tap() {
        let effects = RecordingEffects::new();
        handle_pestana_frecuentes_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("cambiarPestana"));
    }

    #[test]
    fn test_call_ModoNormal_boton_edicion_on_tap() {
        let effects = RecordingEffects::new();
        handle_boton_edicion_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("activarEdicion"));
    }

    #[test]
    fn test_call_ModoNormal_boton_nuevo_on_tap() {
        let effects = RecordingEffects::new();
        handle_boton_nuevo_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("abrirCrearTarea"));
    }

    #[test]
    fn test_call_ModoNormal_boton_configuracion_on_tap() {
        let effects = RecordingEffects::new();
        handle_boton_configuracion_tap(&Contexto::ModoNormal, &effects);
        assert!(effects.was_called("abrirMenuConfiguracion"));
    }

    // === Effect Tests ===

    #[test]
    fn test_effect_ModoNormal_cambiarPestana_actualizarGridVisible() {
        let effects = RecordingEffects::new();
        let mut sys = System::new_with_effects(Contexto::ModoNormal, &effects);
        sys.handle_event("cambiarPestana");
        assert!(effects.was_called("actualizarGridVisible"));
    }

    #[test]
    fn test_effect_ModoNormal_iniciarTarea_iniciar_sesion() {
        let effects = RecordingEffects::new();
        let mut sys = System::new_with_effects(Contexto::ModoNormal, &effects);
        sys.handle_event("iniciarTarea");
        assert!(effects.was_called("iniciar_sesion"));
    }
}
```

### 4.2 Tests generados para ModalComentario (overlay con slot)

```rust
    // === Transition Tests: ModalComentario ===

    #[test]
    fn test_transition_ModalComentario_on_confirmarInicio() {
        let mut sys = System::new(Contexto::ModalComentario);
        sys.handle_event("confirmarInicio");
        // [cerrar_overlay] retorna al initial
        assert_eq!(sys.state, Contexto::ModoNormal);
    }

    #[test]
    fn test_transition_ModalComentario_on_cancelar() {
        let mut sys = System::new(Contexto::ModalComentario);
        sys.handle_event("cancelar");
        assert_eq!(sys.state, Contexto::ModoNormal);
    }

    // === Handler Tests: ModalComentario ===

    #[test]
    fn test_call_ModalComentario_campo_comentario_on_cambio() {
        let effects = RecordingEffects::new();
        handle_campo_comentario_cambio(&Contexto::ModalComentario, &effects);
        assert!(effects.was_called("actualizarComentario"));
    }

    #[test]
    fn test_call_ModalComentario_campo_retroactivo_on_cambio() {
        let effects = RecordingEffects::new();
        handle_campo_retroactivo_cambio(&Contexto::ModalComentario, &effects);
        assert!(effects.was_called("actualizarRetroactivo"));
    }

    #[test]
    fn test_call_ModalComentario_boton_confirmar_on_tap() {
        let effects = RecordingEffects::new();
        handle_boton_confirmar_tap(&Contexto::ModalComentario, &effects);
        assert!(effects.was_called("confirmarInicio"));
    }

    #[test]
    fn test_call_ModalComentario_boton_cancelar_on_tap() {
        let effects = RecordingEffects::new();
        handle_boton_cancelar_tap(&Contexto::ModalComentario, &effects);
        assert!(effects.was_called("cancelar"));
    }
```

### 4.3 Tests generados para SesionActiva.fills ModalComentario.sesion_opts

```rust
    // === Fills Tests: SesionActiva -> ModalComentario.sesion_opts ===

    #[test]
    fn test_fills_SesionActiva_ModalComentario_sesion_opts_checkbox_sustituir_cambio() {
        let effects = RecordingEffects::new();
        let mut sys = System::new_with_effects(Contexto::ModoNormal, &effects);
        sys.activate_concurrent(Contexto::SesionActiva);
        sys.handle_event("seleccionarTipoTarea"); // navega a ModalComentario
        sys.handle_role_event("checkbox_sustituir", "cambio");
        assert!(effects.was_called("marcarSustituir"));
    }

    #[test]
    fn test_fills_absent_SesionActiva_checkbox_sustituir_inert() {
        let effects = RecordingEffects::new();
        let mut sys = System::new_with_effects(Contexto::ModoNormal, &effects);
        // SesionActiva NO activo
        sys.handle_event("seleccionarTipoTarea"); // navega a ModalComentario
        sys.handle_role_event("checkbox_sustituir", "cambio");
        assert!(!effects.was_called("marcarSustituir"));
    }
```

### 4.4 Test de completitud

```rust
    // === Exhaustiveness Test ===

    #[test]
    fn test_exhaustive_contexto_enum() {
        let all = vec![
            Contexto::ModoNormal,
            Contexto::ModoEdicion,
            Contexto::SesionActiva,
            Contexto::MenuConfiguracion,
            Contexto::ModalComentario,
            Contexto::ModalSeleccionActividad,
            Contexto::ModalCrearTarea,
            Contexto::ModalEditarTarea,
            Contexto::ModalEditarActividad,
            Contexto::ModalCrearActividad,
            Contexto::ModalHistorial,
            Contexto::ModalReset,
            Contexto::ModalAcercaDe,
            Contexto::ResetFase1,
            Contexto::ResetFase2,
            Contexto::ResetFase3,
        ];
        assert_eq!(all.len(), 16); // EXPECTED_CONTEXT_COUNT
    }
```

---

## 5. Resumen de cambios por archivo

| Archivo | Cambio |
|---------|--------|
| `ast.rs` | Ninguno. |
| `generator.rs` | Anadir `generate_tests()`. Eliminar el bloque `#[cfg(test)]` existente de `generate_rust()`. Anadir generacion de trait `Effects`, `NoOpEffects`, `RecordingEffects` en `generate_rust()`. |
| `main.rs` | Invocar `generate_tests()` y escribir `_out_tests.rs`. |
| `validator.rs` | Ninguno. |

### Cambio en generate_rust() (Strand 1)

El generador de Strand 1 necesita dos modificaciones:

1. **Eliminar lineas 157-191** (el bloque `#[cfg(test)]` existente).
   Toda generacion de tests se centraliza en `generate_tests()`.

2. **Modificar handlers para aceptar `&dyn Effects`**. Actualmente:

```rust
pub fn handle_tarjeta_tipo_tap(ctx: &Contexto) { ... }
```

Debe ser:

```rust
pub fn handle_tarjeta_tipo_tap(ctx: &Contexto, effects: &dyn Effects) { ... }
```

Y dentro del match, en lugar de `// execute seleccionarTipoTarea`:

```rust
effects.seleccionar_tipo_tarea(/* args */);
```

3. **Modificar `System` para aceptar effects**. Anadir campo
`effects: Box<dyn Effects>` o hacer generico.

---

## 6. Matriz de cobertura

Para cada `.trz` verificado, Strand 2 genera la siguiente cantidad de tests:

| Categoria | Formula | Ejemplo ModoNormal |
|-----------|---------|-------------------|
| Transiciones | sum(ctx.transitions.len()) | 4 |
| Forbidden | count(action.target == Forbidden) | 0 (ModoNormal no tiene) |
| Ignored | count(action.target == Ignored) | 0 (ModoNormal no tiene, pero `role *: ignored` genera muchos en otros ctx) |
| Calls | count(action.target == Call) | 7 |
| On_entry effects | count(effect.trigger == Lifecycle("on_entry")) | 0 |
| Fills positivos | sum(fills.roles.actions) | 0 (en SesionActiva: 1) |
| Fills negativos | count(fills_def) | 0 (en SesionActiva: 1) |
| Completitud | 1 (global) | 1 |

**Total para CronometroPSP completo** (estimacion):
- ~50 transiciones x 1 test = 50
- ~15 contextos x ~7 roles x ~1 event ~= 100 handler tests
- ~3 on_entry effects = 3
- ~1 fills con 1 rol = 2 tests (positivo + negativo)
- 1 test completitud
- **Total estimado: ~156 tests**

Todos generados automaticamente. Cero escritos a mano.

---

## 7. Manejo de `role *: ignored`

**Problema**: ModoNormal y muchos otros contextos declaran `role *: ignored`,
que es un wildcard que marca todos los roles no explicitamente declarados
como `ignored` en ese contexto. El campo `ignore_rest: bool` en `ContextDef`
captura este comportamiento.

**Decision para tests**: Cuando `ctx.ignore_rest == true`, el generador de
tests NO genera tests individuales para cada `(rol, evento)` que cae bajo el
wildcard. Solo genera tests para los roles explicitamente declarados.

**Justificacion**: Los roles bajo `role *: ignored` son potencialmente
infinitos (cualquier rol no mencionado). No se puede enumerar lo que no se
declara. La regla de completitud (Rule 1) ya verifica que las combinaciones
explicitamente declaradas cubren la matriz. Los tests del wildcard serian
redundantes con la verificacion del compilador.

**Excepcion**: Si en el futuro se quiere generar tests de humo para N roles
conocidos del programa bajo el wildcard, se puede anadir como opcion.

---

## 8. Orden de implementacion recomendado

1. **Paso 1**: Implementar `generate_tests()` con solo Fase 1 (transiciones).
   Verificar que compila y pasa `cargo test`.

2. **Paso 2**: Refactorizar `generate_rust()` para producir trait `Effects`.
   Esto es prerequisito para las fases 2-5.

3. **Paso 3**: Implementar Fase 2 (handler tests: forbidden, ignored, call).

4. **Paso 4**: Implementar Fase 3 (completitud). Es trivial.

5. **Paso 5**: Implementar Fase 4 (on_entry effects).

6. **Paso 6**: Implementar Fase 5 (fills tests). Esta es la mas compleja
   porque requiere logica de activacion de concurrent + overlay.

Cada paso es independientemente testeable y commitable.

---

## 9. Preguntas abiertas para Gemini

### P1: Firma de `System::new` con effects

El diseno propone `System::new_with_effects(initial, &effects)`. Pero el
`System::new` actual no tiene parametro de effects. Hay dos opciones:

- **Opcion A**: Cambiar la firma de `System::new` para siempre recibir
  effects. Rompe la API existente.
- **Opcion B**: Anadir `System::new_with_effects` como variante. Mantiene
  compatibilidad pero duplica constructores.
- **Opcion C**: Hacer `System<E: Effects>` generico. Mas idiomatico pero
  mas invasivo.

Recomendacion: Opcion C (generico), porque permite `System<NoOpEffects>`
en produccion (zero-cost) y `System<RecordingEffects>` en tests. Pero si
la complejidad es excesiva para esta iteracion, Opcion B es aceptable.

### P2: Tests para sub-contextos de ModalReset

`ModalReset.trz` declara sub-contextos inline (`ResetFase1`, `ResetFase2`,
`ResetFase3`). Actualmente el parser los trata como contextos independientes
al nivel top-level del AST. Verificar: los tests de transicion para
`ResetFase1 -> ResetFase2` deben generarse normalmente? El parser ya
produce `ContextDef` separados para cada fase, asi que la respuesta
deberia ser si — pero confirmar.

### P3: Navegacion a contextos no directamente alcanzables desde initial

Algunos tests necesitan posicionar el sistema en un contexto que esta a
varias transiciones del initial (ejemplo: `ResetFase2` requiere
`ModoNormal -> MenuConfiguracion -> ModalReset -> ResetFase1 -> ResetFase2`).
Dos opciones:

- **Opcion A**: `System::new(Contexto::ResetFase2)` — inicializacion directa.
  Simple pero no verifica la ruta real.
- **Opcion B**: Ejecutar la cadena de transiciones. Mas realista pero
  requiere un algoritmo de pathfinding en el grafo de transiciones.

Recomendacion: Opcion A para esta iteracion. Los tests de transicion ya
verifican cada arista individualmente. Un test que verifica A->B y otro que
verifica B->C implican conjuntamente que A->B->C funciona. La composicion
algebraica de las pruebas unitarias es suficiente.

### P4: Nombre del modulo en `_out_tests.rs`

El archivo se llama `_out_tests.rs`. El modulo interno se llama
`algebraic_tests`. Debe el modulo seguir otro patron de nombres? Depende
de como se integre con el resto del proyecto generado. Si `_out.rs` vive
en `src/` y `_out_tests.rs` es un test de integracion en `tests/`, el
`use super::*` no funciona — se necesita `use <crate>::*`. Decisiones de
layout del proyecto generado afectan esto.

Recomendacion: Por ahora generar como modulo inline (`#[cfg(test)] mod
algebraic_tests`) DENTRO de `_out.rs`, eliminando los tests actuales.
Si el tamano se vuelve problemático, refactorizar a archivo separado
en una iteracion posterior.

### P5: Effects con argumentos tipados vs strings

El trait `Effects` propuesto usa `&str` para todos los argumentos.
Alternativa: generar tipos especificos para cada llamada basado en el
`DataDef` del programa. Esto daria type safety real pero aumenta mucho
la complejidad del generador.

Recomendacion: `&str` para esta iteracion. Type safety se puede anadir
cuando el sistema de tipos de Trenza madure (relacionado con GAP-5
sobre validacion condicional).
