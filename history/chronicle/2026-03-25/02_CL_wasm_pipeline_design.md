# Diseño del Pipeline WASM — Paso 4 del Roadmap

**Fecha:** 2026-03-25
**Autor:** Claude Sonnet 4.6

---

## Diagnóstico previo a la implementación

Antes de diseñar el pipeline WASM, la auditoría del generador Rust reveló que
el código Rust generado actualmente **no compila**. Hay dos clases de bugs:

### Bug clase A: tipos Trenza usados como tipos Rust

El `generate_rust()` mapea `Texto` → `String`, `Numero` → `i32`, `Booleano` →
`bool`, pero deja pasar `Id`, `Entero`, `Timestamp`, `Color`, `Lista<X>` y el
sufijo opcional `?` sin traducir. El resultado:

```rust
// Lo que genera hoy:
pub struct Sesion {
    pub sesionId: Id,          // ← Id no existe en Rust
    pub inicio: Timestamp,     // ← Timestamp no existe en Rust
    pub notas: Texto?,         // ← sintaxis inválida en Rust
}
```

### Bug clase B: targets especiales emitidos literalmente

```rust
// Lo que genera hoy:
"cerrar" => self.state = Contexto::[close_overlay],   // ← inválido
"sesionFinalizada" => self.state = Contexto::[deactivate],  // ← inválido
```

`[close_overlay]` y `[deactivate]` son metasímbolos del DSL Trenza, no
identificadores de contexto. El generador debe traducirlos a Rust real.

**Nota:** El generador TypeScript no tenía estos bugs porque `ts_type()` ya
manejaba los tipos correctamente, y en TS los targets especiales se resuelven
de forma similar. Los tests de `tsc --strict` enmascaraban el estado del Rust.

---

## Semántica de los targets especiales

| Target | Semántica en Trenza | Rust generado |
|--------|---------------------|---------------|
| `[stay]` | Permanecer en el mismo estado | `"evt" => {}` (ya manejado) |
| `[close_overlay]` | Volver al estado inicial del sistema | `self.state = Contexto::<initial>` |
| `[deactivate]` | Desactivar contexto concurrente | `self.concurrent_states.remove(&Contexto::<ctx>)` |

`[close_overlay]` es una simplificación: Trenza no tiene stack de navegación,
así que "cerrar overlay" equivale a "volver al estado base". Para CronometroPSP,
el estado base es `ModoNormal`, que es también el `initial` del sistema.

---

## Diseño del generador WASM (`--lang=wasm`)

### Principio de diseño

El Rust generado en modo WASM es una **máquina de estados pura**. No invoca
efectos: los efectos son responsabilidad de JavaScript, que observa los cambios
de estado vía el valor de retorno de `dispatch()`.

Este diseño es correcto para un frontend web:
- Rust/WASM gestiona SOLO las transiciones (lógica formal verificada)
- JS gestiona los efectos (DOM, red, storage)
- La separación es explícita y no accidental

### Estructura del código generado

```
<SystemName>_wasm.rs
├── #[allow(non_snake_case, dead_code)]
├── use wasm_bindgen::prelude::*;
├── enum Contexto { ... }          ← privado; JS solo ve strings
├── impl Contexto { fn name() }    ← serialización hacia JS
├── struct <DataType> { ... }      ← tipado correcto con rust_type()
├── struct System { state, concurrent_states }  ← sin Effects, sin lifetime
├── impl System { new(), handle_event() }
└── #[wasm_bindgen] pub struct WasmSystem { inner: System }
    ├── new() -> WasmSystem
    ├── dispatch(&mut self, event: &str) -> String
    ├── current_state(&self) -> String
    └── concurrent_state_names(&self) -> String
```

### API pública expuesta a JavaScript

```typescript
// Lo que JS/TS ve tras compilar con wasm-pack:
const sys = new WasmSystem();
const newState: string = sys.dispatch("guardarNuevaTarea");
// → "ModoNormal"

const state: string = sys.current_state();
// → "ModoNormal"

const concurrent: string = sys.concurrent_state_names();
// → "SesionActiva"  (o "" si ninguno activo)
// JS hace: concurrent.split(',').filter(Boolean)
```

`concurrent_state_names()` retorna un string delimitado por comas en lugar de
`Vec<String>` para evitar dependencias en `js_sys` o `wasm-bindgen` avanzado.

### Por qué NO se usa `&dyn Effects` en el wrapper WASM

`wasm-bindgen` no puede exportar structs con lifetimes ni `&dyn Trait` al
boundary JS/WASM. Las opciones serían:
1. `Box<dyn Effects>` — requiere que `Effects` sea `Send` (no siempre válido en WASM)
2. Callbacks JS vía `js_sys::Function` — añade dependencia en `js_sys`
3. **Separar efectos de la máquina de estados** ← opción elegida

La opción 3 es también la más coherente con los principios de Trenza: los
efectos son "observados" (Strand 4), no "ejecutados" por la máquina.

---

## Estructura del pipeline de build

```
scripts/build-wasm.sh <spec_path> <system_name> [out_dir]
```

1. `trenza-cli generate --lang=wasm --out-dir=build/wasm-crate/src <spec>`
   → genera `<system_name>_out.rs`
2. Renombra `<system_name>_out.rs` → `lib.rs` en el crate WASM
3. Escribe `build/wasm-crate/Cargo.toml` con `crate-type = ["cdylib"]`
4. `wasm-pack build build/wasm-crate --target web --out-dir ../../<out_dir>`
   → produce `<system_name>.wasm` + glue JS/TS

El crate WASM vive en `build/wasm-crate/` (gitignoreado).
Los outputs van a `dist/wasm/` (gitignoreado).

---

## Cambios en docgen.sh

Se añade un paso 2.5 que genera el `.rs` WASM para CronometroPSP y verifica
que compile con `rustc` (sin `wasm-pack`, que requiere toolchain adicional).
La verificación completa (`wasm-pack build`) queda en `build-wasm.sh`.

---

## Preguntas abiertas

- **Stack de navegación:** `[close_overlay]` → `initial_state` es una
  simplificación. Si en el futuro un overlay puede abrirse desde múltiples
  contextos, necesitaremos un stack. Por ahora está documentado como limitación.

- **Tipos de datos en la API WASM:** `WasmSystem::dispatch()` solo gestiona
  transiciones de estado. No hay forma de pasar datos de rol desde JS. Para
  que JS pueda también despachar eventos con payload (ej. qué tarea se creó),
  necesitaríamos serialización JSON en la API. Queda pendiente.

- **Concurrent state activation:** En el modelo actual, los contextos
  concurrentes se activan en `System::new()`. En la realidad, `SesionActiva`
  debería activarse cuando se inicia una sesión. La gestión explícita de
  activación/desactivación de contextos concurrentes desde JS es trabajo futuro.
