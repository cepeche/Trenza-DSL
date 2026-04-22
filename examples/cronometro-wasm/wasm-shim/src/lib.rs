//! CronometroPSP WASM shim.
//!
//! Thin wasm-bindgen wrapper over the Rust module emitted by `trenza-cli`
//! (see `src/generated.rs`). Exposes a single class, `SystemWasm`, with three
//! methods consumed by the browser demo:
//!
//! - `new()`            — instantiate with the spec's initial base context.
//! - `dispatch(event, payload_json)` — route a UI event with optional JSON
//!   payload; runs the deterministic transition + effects per the .trz spec
//!   and returns a rich snapshot string.
//! - `snapshot()`       — return the runtime state (base, overlay stack,
//!   concurrent set, derived current) as a JSON string for projection by JS.
//!
//! Effect plumbing: the shim uses the `RecordingEffects` impl emitted by the
//! generator. After each dispatch, the recorded calls are drained, attached
//! to the snapshot under `triggered_effects`, and the host JS re-routes each
//! one to its real implementation. This keeps the deterministic engine pure
//! while letting the browser handle DOM-y side effects.

#![allow(non_snake_case, unused_variables, dead_code)]

use wasm_bindgen::prelude::*;

mod generated;
pub use generated::{Contexto, RecordingEffects, Snapshot, System};

#[wasm_bindgen]
pub struct SystemWasm {
    // Heap-allocated, leaked into 'static. There is exactly one SystemWasm per
    // page load, so the leak is bounded and acceptable. Box::leak avoids a
    // self-referential struct (System holds &'a dyn Effects).
    effects: &'static RecordingEffects,
    inner: System<'static>,
}

#[wasm_bindgen]
impl SystemWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SystemWasm {
        let effects: &'static RecordingEffects = Box::leak(Box::new(RecordingEffects::new()));
        let inner = System::new(Contexto::ModoNormal, effects);
        SystemWasm { effects, inner }
    }

    /// Route an event with a JSON payload (any shape; `null`/`{}` are fine).
    /// Returns a JSON string with the post-dispatch snapshot plus the list of
    /// recorded effect calls (`triggered_effects`).
    pub fn dispatch(&mut self, event: &str, payload_json: &str) -> String {
        // Drain anything left from the previous dispatch (defensive).
        self.effects.calls.borrow_mut().clear();
        let payload: serde_json::Value =
            serde_json::from_str(payload_json).unwrap_or(serde_json::Value::Null);
        self.inner.dispatch(event, &payload);
        self.snapshot_with_effects()
    }

    /// Pure snapshot of `base`, `overlay_stack`, `concurrent`, `current`.
    pub fn snapshot(&self) -> String {
        serde_json::to_string(&self.inner.snapshot()).unwrap_or_else(|_| "{}".to_string())
    }
}

impl SystemWasm {
    fn snapshot_with_effects(&self) -> String {
        let snap = self.inner.snapshot();
        let calls: Vec<String> = self.effects.calls.borrow().clone();
        let rich = serde_json::json!({
            "base": snap.base,
            "overlay_stack": snap.overlay_stack,
            "concurrent": snap.concurrent,
            "current": snap.current,
            "triggered_effects": calls,
        });
        rich.to_string()
    }
}

impl Default for SystemWasm {
    fn default() -> Self {
        Self::new()
    }
}
