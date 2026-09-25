# Fase 3 cerrada — generator.rs parchado y verificado

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto + insumo de Fase 4
**Fecha:** 2026-04-22
**Insumo:** modelo runtime 13_CO_runtime_model.md

---

## Resumen

`trenza-core/src/generator.rs` ahora emite código Rust que:
- compila sin errores y sin warnings de tipos para `cronometro_full.trz`,
- implementa la semántica de overlays apilables (`overlay_stack: Vec<Contexto>`),
- mantiene contextos concurrentes en un set independiente (`concurrent: HashSet<Contexto>`),
- enruta sub-contextos a su overlay padre vía `parent_overlay_of` derivada en compile-time,
- recibe payload runtime (`serde_json::Value`) y resuelve args con `payload_str(payload, "key")`,
- expone un `Snapshot` serializable para el bridge WASM,
- mantiene compatibilidad hacia atrás (`handle_event(event)` sigue funcionando).

## Verificación empírica

Pipeline ejecutado:
```
trenza-cli generate cronometro_full.trz → CronometroPSP_out.rs + .tests.rs
cargo test (sobre output regenerado) → 114/114 PASS
cargo test -p trenza-core              → 21+6+0 PASS (incluye interpreter_smoke)
```

El único test que falla en el repo (`trenza_cli_ts_pasa_tsc`) es preexistente y no
relacionado: el generador TypeScript desconoce primitivos en español (`Numero`,
`List` sin parámetro). No bloquea Fase 4 porque la ruta que usamos para el demo
es Rust→WASM, no TS directo.

## Cambios concretos en `generator.rs`

### Helpers nuevos (módulo nivel)
- `classify_target_actions(target, bases, overlays, concurrents, sub_contexts, decl_ctx) -> Vec<String>`
  — clasifica `transition.target` en una de siete acciones Rust (Tabla §3 del runtime model).
- `emit_effect_call_expr(call) -> String` — convierte un `ActionCall` en una llamada
  `self.effects.X(...)` con args resueltos como literal o `payload_str(payload, "key")`.

### Helpers nuevos (closure dentro de `generate_rust`)
- `resolve_arg_type(arg, role_dt)` — devuelve el tipo Rust del arg en función de
  `data_field_types[role_dt][field]`. Captura por referencia un `BTreeMap` global
  construido al inicio.

### Bloques reescritos
- **Trait `Effects` + `NoOpEffects` + `RecordingEffects`**: ahora cada arg lleva
  el tipo Rust real (`&str`, `&bool`, `&i32`, `&String`...). Las observaciones
  por call-site se unifican: si discrepan, fallback a `&str`. `RecordingEffects`
  usa `format!("{:?}", arg)` para soportar tipos no-`Display`.
- **`System` struct**: campos `base + overlay_stack + concurrent + effects` (en
  lugar de `state + concurrent_states`). `concurrent` ahora arranca **vacío**
  (antes se pre-poblaba con todos los declarados — incorrecto: SesionActiva no
  está activo de inicio).
- **`dispatch(event, payload)`**: orquesta `dispatch_concurrent → dispatch_main →
  run_on_entry`. Wrapper `handle_event(event)` para compat con tests viejos.
- **`dispatch_main`**: emite arms por contexto (excluyendo concurrents). Cada
  transición invoca su clasificación. Event-effects se emiten dentro del mismo
  arm que la transición; o como arm independiente si no hay transición homónima.
- **`dispatch_concurrent`**: por cada concurrent activo, match exhaustivo sobre
  sus transiciones y event-effects. Si la transición apunta a un base, además
  desactiva el propio concurrent (cierra el bug "SesionActiva queda colgando
  tras terminarSesion → ModoNormal").
- **`run_on_entry`**: emite un match único por estado-de-llegada con todos los
  `[on_entry]` lifecycle effects.
- **Sub-contextos**: `parent_overlay_of(c) -> Option<Contexto>` derivada con
  punto fijo (directo `on cerrar -> Overlay`, indirecto vía sibling). Para
  CronometroPSP produce las 5 entradas correctas (Historial7Dias y 30Dias →
  ModalHistorial; ResetFase1/2/3 → ModalReset).
- **`replace_top_or_push(sub)`**: si el top de la pila es el parent del sub
  o un sibling con mismo parent, reemplaza el top; si no, push.
- **`Snapshot { base, overlay_stack, concurrent, current }`**: struct
  serializable con derive serde, expuesta vía `System::snapshot() -> Snapshot`.

### Generador de tests
- `generate_transition_tests` migrado: `sys.state` → `sys.current_state()`,
  `sys.concurrent_states` → `sys.concurrent`. Para target=concurrent, ahora
  verifica `sys.concurrent.contains(&target)` (no `current_state`).
- `generate_fills_tests`: navegación a overlay vía `sys.overlay_stack.push(...)`.

### Side-fixes (consecuencia inmediata, no del plan original)
- **`primitives.rs`**: añadidos alias en español usados por el `.trz` original
  de CronometroPSP (`Texto, Entero, Booleano, Id, Color`). Documentados como
  legacy pre-ADR-005.
- **`rust_type` / `ts_type`**: aceptan `Lista<X>` (alias de `List<X>`).

## Métricas antes/después

| Métrica | Antes | Después |
|---------|-------|---------|
| Errores de compilación de `CronometroPSP_out.rs` (cargo check) | 102 | 0 |
| Warnings de tipos | 6 (E0308) | 0 (E0308) |
| Tests algebraicos generados que pasan | 0 (no compilaba) | 114 / 114 |
| Métodos públicos del System | `new, handle_event` | `new, current_state, snapshot, handle_event, dispatch` |
| Categorías de target soportadas | 4 (mal) | 7 (bien) |

## Salidas listas para Fase 4

- `tmp_audit/check_crate/src/lib.rs` contiene `CronometroPSP_out.rs` regenerado y
  verificado, listo para envolverse con `wasm-bindgen`.
- `Snapshot` está derivado con `serde::{Serialize, Deserialize}`; el bridge
  llamará `system.snapshot()` y serializará con `serde_json::to_string`.
- `dispatch(event, payload)` acepta `&serde_json::Value`; el bridge debe parsear
  el string JSON que llega de JS antes de invocar.

## Lo que queda para Fase 4

1. Crear `examples/cronometro-wasm/wasm-shim/` con `Cargo.toml` que dependa de
   `wasm-bindgen + serde_json + serde` y produzca `cdylib`.
2. Copiar el `CronometroPSP_out.rs` regenerado como `src/generated.rs`.
3. Escribir `src/lib.rs` con un `#[wasm_bindgen]` `SystemWasm` que:
   - en `new()` construya un `Box<NoOpEffects>` (o un `Effects` que reenvíe a
     una closure JS),
   - en `dispatch(event: &str, payload_json: &str)` parsee con
     `serde_json::from_str` y llame al método interno,
   - en `snapshot() -> String` serialice con `serde_json::to_string`.
4. `wasm-pack build --target web` desde ese subdir.

## Lo que queda para Fase 5

- Reescribir `examples/cronometro-wasm/src/main.ts` para usar el bridge nuevo:
  importar `init, SystemWasm`, llamar `system.snapshot()` tras cada dispatch,
  parsear el JSON, proyectar UI desde `{base, overlay_stack, concurrent, current}`.
- Extender `OVERLAY_DOM_IDS` con aliases para sub-contextos:
  `Historial7Dias/30Dias → 'historialModal'`, `ResetFase1/2/3 → 'resetModal'`.
- Pasar `payload` real al `dispatch` cuando un evento lo necesite (p.ej.
  `iniciarTarea` con `tarea_id`).

## Riesgos identificados durante la ejecución (no eran parte del plan)

1. **Primitivos en español no estaban registrados**. Se añadieron como alias
   con TODO de migración. Cuestión abierta: ¿migrar el `.trz` a inglés o dejar
   los aliases? Decisión postergada.
2. **El test legacy `trenza_cli_ts_pasa_tsc` falla** por causa orthogonal
   (generador TS ignora primitivos no canónicos). No tocado en esta fase.
3. **Una transición a concurrent NO cambia `current_state()`**. El test
   generado lo refleja; la UI debe leer `concurrent` para detectar que
   SesionActiva está activo, no `current`. Documentado para Fase 5.
