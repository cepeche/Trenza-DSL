# Auditoría del generador Rust — Fase 1 Ruta A

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto
**Fecha:** 2026-04-22
**Insumo:** `trenza-core/src/generator.rs` (1448 líneas) + regeneración limpia de `CronometroPSP_out.rs` desde `examples/cronometro-wasm/src/cronometro_full.trz`.

## Resumen ejecutivo

El generador actual **emite Rust que casi compila** y se acerca más de lo que parecía a una solución viable. La rotura `Contexto::[cerrar_overlay]` que vimos no procede del generador actual: era un artefacto stale generado de un `.trz` viejo en español. El `.trz` actual usa `[close_overlay]` (inglés) y el generador lo traduce a `Contexto::ModoNormal`.

Sin embargo, la regeneración fresca produce **102 errores de compilación** (cargo check) por dos bugs concretos del generador, más **5 limitaciones semánticas** que afectan la corrección runtime aunque no impidan la compilación.

| ID | Tipo | Bloqueante para compilar | Bloqueante para demo |
|----|------|--------------------------|----------------------|
| Bug-A | Sintaxis: `format!("...".to_string(), ...)` | **SÍ** (32 errores) | n/a |
| Bug-B | Tipos: trait `&str` vs args `&bool`/`&i32` | **SÍ** (6 errores) | sí |
| Bug-C | Tests: `[close_overlay]` no traducido | sí (16 errores en tests) | no |
| Bug-D | Concurrent contexts: placeholder vacío | no | **SÍ** |
| Bug-E | Effects de eventos no se invocan en `handle_event` | no | **SÍ** |
| Bug-F | Args literales del AST en lugar de payload runtime | no | **SÍ** |
| Bug-G | `[close_overlay]` siempre vuelve a `initial_state` | no | parcial |
| Bug-H | Role-events no se rutean a transiciones de contexto | no | **SÍ** (parcheable en TS) |

---

## Bug-A — `format!(...".to_string(), ...)` mal formado

**Ubicación:** `generator.rs` línea 653.

```rust
output.push_str(&format!(
    "        self.calls.borrow_mut().push(format!(\"{}({})\".to_string(), {}));\n",
    func, format_args, format_values
));
```

Emite código como:
```rust
self.calls.borrow_mut().push(format!("abrirEditarActividad({})".to_string(), arg0));
```

`format!` espera un literal de format-string como primer arg, no `String`. Genera 32 errores E0425 + 32 errores `expected ',', found '.'`.

**Fix:** quitar `.to_string()` del macro y mover fuera:
```rust
"        self.calls.borrow_mut().push(format!(\"{}({})\", {}));\n"
```

---

## Bug-B — `Effects` trait declara todos los args como `&str`

**Ubicación:** `generator.rs` líneas 617, 627, 645.

```rust
let arg_list = args.iter().enumerate()
    .map(|(i, _)| format!("arg{}: &str", i))
    .collect::<Vec<_>>().join(", ");
```

El trait declara, p. ej., `fn marcarPermanente(&self, arg0: &str)`. Pero el role handler (línea 825 onward) emite:
```rust
effects.marcarPermanente(&checkbox_permanente.marcado);
```
y `marcado: bool`. Resultado: 6 errores E0308 (tipo `&bool` vs esperado `&str`, también `&i32`).

**Fix:** resolver el tipo real del arg consultando `data` definitions. Función `resolve_arg_type` ya existe (línea 8) pero no se usa en `generate_rust`. Aplicarla:
```rust
// Por cada (función, args), reconstruir tipos resolviendo
//   - "self.X" → tipo del campo X en el datatype del role
//   - ident plano → tipo del input del contexto
//   - literal → string/number
// Y emitir `arg_X: &TipoReal` en vez de `&str`.
```

Necesita conocer el role-datatype donde se declaró cada función — varios contextos pueden declarar la misma función con args diferentes. Conservadora: si hay ambigüedad, fall-back a `&str` y aceptar coerción explícita.

---

## Bug-C — Tests siguen filtrando `[close_overlay]` literal

**Ubicación:** `generator.rs` línea 1184.

```rust
} else if target == "[cerrar_overlay]" {  // ← "cerrar" en español
    out.push_str(&format!("        assert_eq!(sys.state, Contexto::{});\n", meta.initial));
}
```

El `.trz` usa `[close_overlay]` (inglés), no `[cerrar_overlay]`. El branch nunca matchea y cae al `else` final que escribe `Contexto::[close_overlay]` literal. 16 errores en tests.

**Fix:** sustituir `"[cerrar_overlay]"` por `"[close_overlay]"`. Una línea.

---

## Bug-D — Concurrent contexts: placeholder sin lógica

**Ubicación:** `generator.rs` líneas 711-716.

```rust
output.push_str("        // Evaluate concurrent states sequentially (Composite Mode)\n");
for cctx in &concurrent_contexts {
    output.push_str(&format!("        if self.concurrent_states.contains(&Contexto::{}) {{\n", cctx));
    output.push_str(&format!("            // In a full composite synthesis, we evaluate {} logic here\n", cctx));
    output.push_str("        }\n");
}
```

Resultado en código generado: un `if` con un comentario. **`SesionActiva` nunca procesa eventos de su propio bloque `transitions:`**. La línea 1410 del .trz dice `on sesionFinalizada -> [deactivate]`, pero como SesionActiva no es el `current_state` (es concurrente), su match no se evalúa. El generador emite un match para SesionActiva (líneas 628-634 del generado), pero solo se ejecuta si `state == SesionActiva` literalmente — y eso solo pasa cuando ModoNormal se reemplaza por SesionActiva (Bug-H, no concurrencia real).

**Fix:** dentro del bloque `for cctx in &concurrent_contexts` emitir un `match event` real con las transiciones del contexto concurrente:
```rust
if self.concurrent_states.contains(&Contexto::SesionActiva) {
    match event {
        "sesionFinalizada" => { self.concurrent_states.remove(&Contexto::SesionActiva); },
        // ...
        _ => {}
    }
}
```

**Pero antes de eso:** la transición `iniciarTarea -> SesionActiva` desde ModoNormal debe **insertar** SesionActiva en `concurrent_states`, no reemplazar el state base. Eso requiere clasificar el target (Bug-G).

---

## Bug-E — `effects:` por evento no se invocan en `handle_event`

**Síntoma:** ModoNormal declara:
```
transitions:
    on iniciarTarea -> SesionActiva
effects:
    iniciarTarea -> iniciar_sesion(tarea_id, notas, ...)
```

El generado solo emite la transición. Nunca llama `self.effects.iniciar_sesion(...)`. Hoy solo se invocan los effects con trigger `Lifecycle("on_entry")` (líneas 752-776 del generador).

**Fix:** dentro del `match event` de cada contexto, además de aplicar la transición, recorrer `ctx.effects` con `EffectTrigger::Event(e) if e == event` y emitir la llamada `self.effects.X(args...)`. Patrón ya existe en role-handlers (línea 832-844).

---

## Bug-F — Args literales del AST, no del payload runtime

**Ubicación:** `generator.rs` líneas 765-767 (lifecycle effects), 829-844 (role handlers — parcial), y *cualquier futura emisión* de Bug-E.

```rust
for arg in &effect.call.args {
    args.push(format!("\"{}\"", arg));   // ← solo lifecycle: arg AST entre comillas
}
```

Resultado en código generado:
```rust
self.effects.cargar_historial("dias: 7");   // OK porque es literal en .trz
```
Pero si la línea fuera `cargar_historial(filtro)` con `filtro` como input, se emitiría `self.effects.cargar_historial("filtro");` — el nombre del input, no su valor.

Para los role-handlers (línea 838 en adelante), `self.X` se traduce bien (`&role_name.X`), `ident plano` se trata como literal — incorrecto: debería leerse del input del contexto.

**Fix:**
1. Cambiar `handle_event(event: &str)` a `handle_event(event: &str, payload: &serde_json::Value)`.
2. Por cada arg en cualquier effect:
   - `self.X` no aplica fuera de role handlers (`self` es el role).
   - `ident plano` → `payload.get("ident").and_then(|v| v.as_str()).unwrap_or_default()`.
   - Literales → emitir como literal Rust.
3. La firma del trait debe aceptar los tipos correctos (relacionado con Bug-B).

Para CronometroPSP, los effects que toman valores reales del payload son: `iniciar_sesion`, `crear_actividad`, `crear_tipo_tarea`, `editar_tipo_tarea`, `actualizar_actividad`, `cargar_historial`, `reset_datos`. Brief E (Gemini) confirmará la lista.

---

## Bug-G — `[close_overlay]` siempre devuelve a `initial_state`

**Ubicación:** `generator.rs` línea 730-732.

```rust
"[close_overlay]" => {
    output.push_str(&format!(
        "                    \"{}\" => self.state = Contexto::{},\n",
        trans.event, initial_state
    ));
},
```

Para CronometroPSP `initial_state == "ModoNormal"`. Funciona en ~todos los casos del .trz porque los modales se abren desde ModoNormal. **Pero rompe** el caso "abrir ModalEditarTarea desde ModoEdicion": al cerrar, vuelves a ModoNormal en vez de ModoEdicion. El usuario pierde el modo edición sin saber por qué.

**Fix correcto:** mantener una pila de overlays e implementar push/pop. La estructura `System` necesita un campo nuevo:
```rust
pub struct System<'a> {
    pub base: Contexto,                      // ModoNormal | ModoEdicion
    pub overlay_stack: Vec<Contexto>,        // pila
    pub concurrent_states: StdHashSet<Contexto>,
    pub effects: &'a dyn Effects,
}
```
Y `state` pasa a ser un getter computado: `top of overlay_stack` si la pila no está vacía, sino `base`.

**Fix mínimo (workaround):** reemplazar `initial_state` por una heurística: detectar si la pila debería volver a ModoEdicion (si el usuario estaba en ModoEdicion al abrir) o ModoNormal. Sin estructura de datos, esto requiere un campo `previous_base: Contexto`. Más simple que la pila completa pero menos correcto si hay overlays apilados.

Diseño detallado en Fase 2.

---

## Bug-H — Role-events no se rutean a transiciones del contexto

**Síntoma:** la .trz declara, p. ej.:
```
context MenuConfiguracion:
    role item_historial: ItemMenu
        on tap -> abrirHistorial
    transitions:
        on abrirHistorial -> ModalHistorial
```

El generador emite `handle_item_historial_tap(ctx, role, effects)` que llama `effects.abrirHistorial()`. Pero esto es una llamada al *trait Effects* (que el host implementa como callback), **no** un evento que vuelva a entrar en `handle_event`. Es decir: el role-event no causa la transición a `ModalHistorial` salvo que el host re-despache `abrirHistorial` manualmente.

**En la demo actual (TS):** el HTML usa `data-event="abrirHistorial"` directamente y el listener despacha al sistema. El intermedio role no se usa, así que **este bug no es bloqueante para nuestra demo**.

**Para una arquitectura limpia:** el generador debería emitir un `dispatch_role_event` que combine: invocar el effect del role (para side-effects de UI/log) **y** re-entrar en `handle_event` con el nombre del evento emitido. Es deseable pero post-MVP.

---

## Estado de las hebras Strand 1 (logic) y Strand 2 (tests)

- **Strand 1:** 38 errores de compilación. Todos de Bug-A y Bug-B. Si solo se arreglan esos dos, el código compila — aunque siga siendo semánticamente incompleto (Bugs D, E, F, G, H).
- **Strand 2:** 16 errores adicionales de Bug-C. Trivial de arreglar.

## Lo que SÍ funciona del generador actual

- Enum `Contexto` correctamente exhaustivo.
- Structs `Data` correctamente derivadas con serde.
- Trait `Effects` con todas las funciones agrupadas.
- `NoOpEffects` y `RecordingEffects` (con la sintaxis de Bug-A corregida).
- `match self.state` exhaustivo sobre transiciones.
- on_entry effects sí se invocan (aunque con args literales, Bug-F).
- Role handlers individuales (`handle_X_tap`) compilables (módulo Bug-B).
- Mermaid topology y audit doc.

## Recomendación para Fase 2

El "modelo runtime nuevo" del plan original sigue siendo necesario, pero el alcance es menor de lo temido. Los fixes mínimos para que la demo funcione end-to-end son:

1. **Críticos para compilar (Strand 1):** Bug-A + Bug-B. ~30 min de trabajo.
2. **Críticos para correr la demo:**
   - Bug-D (concurrent dispatch real): ~2 h.
   - Bug-E (event-effects en handle_event): ~1 h.
   - Bug-G (overlay stack o previous_base): ~2 h para la versión simple.
3. **Importantes pero diferibles:**
   - Bug-F (payload routing): ~2-3 h. Sin esto, los effects parametrizados reciben strings AST en lugar de valores reales. La demo puede sobrevivir si el host ignora los args y relee del DOM/formState (como hace ahora).
   - Bug-C (tests): ~5 min, pero no bloquea la demo.
   - Bug-H (role-event re-dispatch): post-MVP. La demo lo evita usando `data-event` directo.

**Total estimado para una demo Ruta A funcional:** 5-7 horas de trabajo en el generador. Más Fase 4 (wasm-shim per-spec) y Fase 5 (rewire demo). Ya no es "días" — es "una sesión larga".

## Archivos relevantes

- Generador a parchear: `trenza-core/src/generator.rs`.
- Test smoke pre-existente: `trenza-core/tests/interpreter_smoke.rs`.
- Carpeta de auditoría: `tmp_audit/` (regenerado limpio + crate de prueba).
- Tres briefs Gemini paralelos: `07_GE_*`, `08_GE_*`, `09_GE_*`.
