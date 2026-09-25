# Modelo runtime para el generador Rust — Fase 2 Ruta A

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto + insumo de Fase 3
**Fecha:** 2026-04-22
**Insumo:** auditoría 10_CO + inventarios Gemini 10/11/12_GE.

---

## 1. Objetivo

Definir la representación interna y el algoritmo de `dispatch` que el generador Rust debe emitir, de modo que el código generado:

1. **Compile** sin warnings de tipos.
2. **Ejecute** la semántica de overlays apilables, contextos concurrentes y sub-contextos jerárquicos.
3. **Reciba payload runtime** y enrute sus campos a los args declarados en `effects:`.
4. **Sea consumible por wasm-bindgen** sin envoltorio adicional (Fase 4).

No se modifica el lenguaje. Se modifica solo el generador.

---

## 2. Representación

```rust
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: Contexto,
    pub overlay_stack: Vec<Contexto>,
    pub concurrent: Vec<Contexto>,    // Vec para que el JSON tenga orden estable
    pub current: Contexto,            // = overlay_stack.last() ?? base
}

pub struct System<'a> {
    pub base: Contexto,
    pub overlay_stack: Vec<Contexto>,
    pub concurrent: HashSet<Contexto>,
    pub effects: &'a dyn Effects,
}
```

**Invariantes:**
- `base` es siempre uno de los contextos declarados en `system.contexts:`. Para CronometroPSP: `ModoNormal | ModoEdicion`.
- `overlay_stack` puede contener overlays Y sub-contextos. Cuando es no-vacío, `current_state == top of stack`. Cuando es vacío, `current_state == base`.
- `concurrent` solo contiene contextos declarados en `system.concurrent:`.

Justificación de meter sub-contextos en la pila: un sub-contexto es semánticamente un "estado interno del overlay padre". Cuando el usuario está en `Historial7Dias`, conceptualmente sigue dentro de `ModalHistorial`. Para la proyección de UI necesitamos saber qué overlay es visualmente activo — eso lo resuelve la tabla `parent_overlay_of` (sección 4).

---

## 3. Clasificación de targets en compile-time

El generador, al emitir cada arm de `match event` para un contexto, clasifica `transition.target` en una de seis categorías. Para hacerlo, primero recopila tres conjuntos a partir del `SystemDef`:

```rust
let bases:       HashSet<&str> = system.contexts.iter().collect();
let overlays:    HashSet<&str> = system.overlays.iter().collect();
let concurrents: HashSet<&str> = system.concurrent.iter().collect();
let all_named:   HashSet<&str> = bases ∪ overlays ∪ concurrents;
let sub_contexts: HashSet<&str> = all_context_defs - all_named;
```

Después, por cada `target`:

| Caso | Target literal | Acción emitida |
|------|----------------|----------------|
| 1 | `[stay]` | (no-op; el match arm no toca state) |
| 2 | `[close_overlay]` | `self.overlay_stack.pop();` |
| 3 | `[deactivate]` | `self.concurrent.remove(&Contexto::SELF);` (donde SELF es el contexto cuyo bloque `transitions` lo declara) |
| 4 | name ∈ bases | `self.base = Contexto::name; self.overlay_stack.clear();` |
| 5 | name ∈ overlays | `self.overlay_stack.push(Contexto::name);` |
| 6 | name ∈ concurrents | `self.concurrent.insert(Contexto::name);` |
| 7 | name ∈ sub_contexts | `replace_top_or_push(Contexto::name);` (helper, ver sección 4) |

**Caso especial** — el target es un base pero estamos dentro de un overlay (overlay_stack no vacío): la transición debería conceptualmente cerrar todos los overlays. La acción del caso 4 (`overlay_stack.clear()`) ya lo hace. Esto cubre transiciones como `SesionActiva.terminarSesion -> ModoNormal` (que fuerza el base a ModoNormal y limpia overlays).

**Caso especial** — el target es el mismo contexto declarante (auto-loop): cae en cualquier categoría según el tipo. No hay fallback `[stay]` implícito.

---

## 4. Sub-contextos: `parent_overlay_of` y `replace_top_or_push`

Como confirma Brief D (Gemini), el `.trz` no declara explícitamente la paternidad de los sub-contextos. El generador la deriva en compile-time así:

```rust
// Heurística: un sub-contexto S tiene parent P si:
// - S no está en system.contexts/overlays/concurrent
// - S declara un transición `on cerrar -> P` donde P sí es overlay
fn derive_parent(sub_ctx: &ContextDef, overlays: &HashSet<&str>) -> Option<String> {
    sub_ctx.transitions.iter()
        .find(|t| t.event == "cerrar" && overlays.contains(t.target.as_str()))
        .map(|t| t.target.clone())
}
```

Para CronometroPSP esta heurística produce:
- `Historial7Dias` → `ModalHistorial` (`on cerrar -> ModalHistorial`)
- `Historial30Dias` → `ModalHistorial` (idem)
- `ResetFase1` → `ModalReset` (`on cerrar -> ModalReset`)
- `ResetFase2` → ? (no tiene `on cerrar`; se infiere por sibling: `retrocederAFase1 -> ResetFase1`, así que su parent = parent de ResetFase1 = ModalReset)
- `ResetFase3` → idem ResetFase2

**Heurística refinada**: si un sub-contexto no tiene `cerrar -> Overlay`, su parent es el parent de cualquier sub-contexto al que transite. Implementación: BFS/iteración hasta punto fijo.

**Para MVP, alternativa más simple:** añadir un parámetro `--parent-map=...` al CLI o un archivo `.trz.parents` opcional. **Pero más simple aún**: que el generador emita la tabla derivada y, si un sub-contexto no tiene parent inferible, lo trate como overlay normal y emita un warning. Para CronometroPSP las cinco asignaciones funcionan.

El generador emite:

```rust
fn parent_overlay_of(c: Contexto) -> Option<Contexto> {
    match c {
        Contexto::Historial7Dias  => Some(Contexto::ModalHistorial),
        Contexto::Historial30Dias => Some(Contexto::ModalHistorial),
        Contexto::ResetFase1      => Some(Contexto::ModalReset),
        Contexto::ResetFase2      => Some(Contexto::ModalReset),
        Contexto::ResetFase3      => Some(Contexto::ModalReset),
        _ => None,
    }
}
```

Y el helper `replace_top_or_push`:

```rust
fn replace_top_or_push(&mut self, sub: Contexto) {
    let parent = parent_overlay_of(sub);
    match (self.overlay_stack.last(), parent) {
        (Some(top), Some(p)) if *top == p || parent_overlay_of(*top) == parent => {
            // Reemplazar top por el sub (estamos dentro del mismo overlay parent)
            *self.overlay_stack.last_mut().unwrap() = sub;
        },
        _ => {
            self.overlay_stack.push(sub);
        }
    }
}
```

El host TS proyecta visibilidad usando una versión expandida de `OVERLAY_DOM_IDS`:

```ts
const OVERLAY_DOM_IDS = {
  ...originales,
  Historial7Dias:  'historialModal',
  Historial30Dias: 'historialModal',
  ResetFase1:      'resetModal',
  ResetFase2:      'resetModal',
  ResetFase3:      'resetModal',
};
```

Alternativamente, el bridge TS expone `parent_overlay_of()` y la lógica de proyección walks-up — más limpio pero requiere round-trip al WASM por cada sync. Optamos por la tabla expandida (Fase 5).

---

## 5. Concurrent dispatch

Cada contexto en `concurrent` necesita su propio `match event` que se ejecuta antes que el del current_state. El generador emite, dentro de `dispatch`:

```rust
// 1. Procesar contextos concurrentes (cada uno consume su evento si lo conoce)
for cctx in [Contexto::SesionActiva /* etc */] {
    if !self.concurrent.contains(&cctx) { continue; }
    match (cctx, event) {
        (Contexto::SesionActiva, "sesionFinalizada") => {
            self.concurrent.remove(&Contexto::SesionActiva);
        },
        (Contexto::SesionActiva, "terminarSesion") => {
            self.base = Contexto::ModoNormal;
            self.overlay_stack.clear();
            self.concurrent.remove(&Contexto::SesionActiva);
        },
        // event-effects:
        (Contexto::SesionActiva, "actualizarTimer") => {
            self.effects.calcular_tiempo_transcurrido(payload_arg("sesion_activa.inicio"));
        },
        (Contexto::SesionActiva, "terminarSesion") => {
            self.effects.parar_sesion();
        },
        _ => {}
    }
}
```

Detalle: cuando una transición concurrente apunta a un contexto base (caso poco frecuente, pero `terminarSesion -> ModoNormal` lo hace), también se aplica la acción del caso 4. **Adicionalmente, deactivamos el concurrent que disparó la transición** para evitar que SesionActiva quede activo sin sesión. El generador detecta esto: si la transición sale de un contexto concurrente y apunta a un base, emitir tanto el cambio de base como `concurrent.remove(&Contexto::SELF)`.

---

## 6. Algoritmo de `dispatch`

```rust
pub fn dispatch(&mut self, event: &str, payload: &serde_json::Value) {
    // ── 1. Concurrent contexts procesan primero ──
    self.dispatch_concurrent(event, payload);

    // ── 2. Current state (top of overlay_stack si no vacía, sino base) ──
    let current = self.current_state();

    // ── 3. Aplicar transición + event-effects ──
    self.dispatch_main(current, event, payload);

    // ── 4. on_entry effects del nuevo current_state si cambió ──
    let new_current = self.current_state();
    if new_current != current {
        self.run_on_entry(new_current, payload);
    }
}

pub fn current_state(&self) -> Contexto {
    self.overlay_stack.last().copied().unwrap_or(self.base)
}
```

Cada uno de `dispatch_concurrent`, `dispatch_main`, `run_on_entry` es un `match` exhaustivo emitido por el generador.

---

## 7. Payload routing y resolución de args

### Tipos de args (categorización del Brief E + observación adicional)

| Tipo | Ejemplo en .trz | Resolución |
|------|-----------------|------------|
| `self.X` | `self.tareaId` | Solo aplica a role-handlers; lee del campo del role |
| literal numérico/string | `7`, `'frecuentes'` | Embeber como literal Rust |
| identificador plano | `tarea_id`, `nombre`, `comentario` | Buscar `payload[ident]` |
| `clave: valor` | `dias: 7` | Embeber la cadena completa como literal `"dias: 7"` |
| `external.field` | `sesion_activa.inicio` | Como ident plano: `payload["sesion_activa.inicio"]` (host pasa el valor pre-resuelto) |

### Firma del trait `Effects`

Para evitar el bug B (mismatched types `&str` vs `&bool`), resolvemos tipo por arg si es posible:

```rust
fn resolve_effect_arg_type(
    arg: &str,
    role_datatype: Option<&str>,
    ctx_inputs: &BTreeMap<&str, &str>,
    data_fields: &BTreeMap<&str, BTreeMap<&str, &str>>,
) -> &'static str {
    if arg.starts_with("self.") && role_datatype.is_some() {
        let field = &arg[5..];
        return data_fields.get(role_datatype.unwrap())
            .and_then(|f| f.get(field))
            .copied()
            .unwrap_or("&str");
    }
    if let Some(ty) = ctx_inputs.get(arg) { return ty; }
    if arg.starts_with('\'') || arg.starts_with('"') { return "&str"; }
    if arg.chars().next().map_or(false, |c| c.is_ascii_digit()) { return "i64"; }
    "&str"  // por defecto
}
```

**Conflicto de firmas:** una misma función puede aparecer en varios contextos con args distintos (Brief E señala `iniciar_sesion` y `cargar_historial`). Política:

- Para `iniciar_sesion(tarea_id, notas, ...)` vs `iniciar_sesion(tipoTareaId, comentario, ...)` — los **tipos** de los args son los mismos (todo `&str` o `Option<&str>`). El generador unifica por posición usando el tipo más laxo (Optional union).
- Para `cargar_historial(dias: 7)` vs `cargar_historial(dias: 30)` — un solo arg `&str`. Sin conflicto.

Si dos firmas tienen tipos verdaderamente incompatibles, emitir warning y usar `&str` como fallback. Para CronometroPSP no se da el caso.

### Cómo se invoca un effect con payload

Por cada `EffectRule` que el generador emita:

```rust
self.effects.iniciar_sesion(
    payload_str(payload, "tarea_id"),
    payload_str(payload, "notas"),
    payload_str(payload, "minutos_retroactivos"),
    payload_str(payload, "sustituir"),
);
```

donde `payload_str` es un helper local:

```rust
fn payload_str<'a>(p: &'a serde_json::Value, key: &str) -> &'a str {
    p.get(key).and_then(|v| v.as_str()).unwrap_or("")
}
```

Para args con `:` (`dias: 7`) se embebe la cadena completa como literal:
```rust
self.effects.cargar_historial("dias: 7");
```

Para args `self.X`, ya hay tratamiento especial en role-handlers (no entran en event-effects).

---

## 8. Snapshot para wasm-bindgen

El bridge JS necesita leer estado tras cada dispatch. En vez de `get_state()` y `get_concurrent_states()` separados, exponer:

```rust
#[wasm_bindgen]
impl SystemWasm {
    pub fn snapshot(&self) -> String {
        serde_json::to_string(&Snapshot {
            base: self.inner.base,
            overlay_stack: self.inner.overlay_stack.clone(),
            concurrent: self.inner.concurrent.iter().copied().collect(),
            current: self.inner.current_state(),
        }).unwrap_or_else(|_| "{}".to_string())
    }
}
```

El TS bridge lo deserializa una vez por dispatch.

---

## 9. Plan de fixes para Fase 3, en orden

1. **Bug-A** (sintaxis `format!`): cambiar plantilla en línea 653. 5 min.
2. **Bug-C** (test `[cerrar_overlay]` → `[close_overlay]`): cambiar match en línea 1184. 1 min.
3. **Bug-B** (tipos de trait): añadir `resolve_effect_arg_type` + reusarla en `unique_functions`. 30 min.
4. **Refactor de `System` struct**: cambiar a `base + overlay_stack + concurrent` + `current_state()` getter. 30 min.
5. **Bug-G** (overlay stack): emitir push/pop en lugar de `state = initial`. 30 min (depende del 4).
6. **Bug-D + parte de Bug-E** (concurrent dispatch): emitir `dispatch_concurrent` con match exhaustivo. 1 h.
7. **Bug-E** (event-effects): emitir llamadas `self.effects.X(...)` después de la transición en `dispatch_main`. 30 min.
8. **Bug-F** (payload routing): cambiar firma `dispatch(event)` → `dispatch(event, payload)`, emitir `payload_str(...)` helpers en effect calls. 1 h.
9. **Sub-contexto support**: derivar `parent_overlay_of` table + `replace_top_or_push`. 45 min.
10. **Snapshot serialization**: añadir `Snapshot` struct + emitir método. 15 min.
11. **Regresión**: re-correr `tests/interpreter_smoke.rs` actualizado a la nueva semántica + cualquier test del proyecto. 30 min.

**Total estimado:** 6-7 horas. Coincide con la estimación de Fase 1.

---

## 10. Lo que queda fuera de este modelo (post-MVP)

- **Bug-H** (role-events re-rutados a transiciones de contexto). El demo lo evita usando `data-event` directo. El generador podría emitir `dispatch_role_event(role, event, role_data, payload)` que combine effect-call + re-entry, pero no es necesario para Cronómetro.
- **Slot/fills semántica completa**. Los `fills` aparecen en SesionActiva → ModalComentario.sesion_opts. Para MVP el host añade los roles del fill manualmente; el generador ya emite los handler functions.
- **Guardas `when`**. La rama 5 del modelo (sub-contextos por sintaxis) podría usar guardas para distinguir `seleccionarTipoTarea` con 1 actividad (→ ModalComentario) vs >1 (→ ModalSeleccionActividad). Hoy el .trz tiene ambas transiciones sin guarda y la primera gana siempre. Documentar como limitación conocida.
- **Decoradores `@audit`**. Ya respetados en lifecycle effects; falta verificar en transiciones.

---

## 11. Compatibilidad hacia atrás

El generador actual emite `System::new(initial: Contexto, effects)` y `handle_event(event)`. La nueva firma rompe ambas. Estrategia:

- Mantener `handle_event(event)` como wrapper deprecado que llama `dispatch(event, &serde_json::Value::Null)`. Permite que tests viejos sigan compilando.
- `System::new(initial)` se mantiene; internamente inicializa `base = initial`, `overlay_stack` vacía, `concurrent` con todos los declarados (igual que ahora). Esto es semánticamente discutible — los concurrent activan al *entrar*, no por defecto — pero conserva el comportamiento previo. Para CronometroPSP, SesionActiva debería empezar **inactivo**; lo arreglamos en este patch: `concurrent = HashSet::new()` por defecto, y la transición `iniciarTarea -> SesionActiva` lo activa.

---

## 12. Riesgos identificados

1. **Heurística de parent inference**: si un sub-contexto futuro no tiene `on cerrar -> Overlay` ni transita a otro sub con parent, falla silenciosamente. Mitigación: emitir warning del compilador.
2. **Sub-contextos como overlays apilables**: si el usuario navega `Historial7Dias → ModalAcercaDe → close`, ¿debe volver a Historial7Dias o a ModalHistorial? Decisión: vuelve a Historial7Dias (la pila lo conserva). Documentado.
3. **Conflicto de firmas en trait `Effects`**: si dos contextos declaran un effect con args incompatibles. Para CronometroPSP no aplica. Mitigación: warning + fallback a `&str`.
4. **`terminarSesion -> ModoNormal` desde SesionActiva**: rompe ModoEdicion si el usuario estaba editando. Es un bug del `.trz`, no del generador. Lo respetamos hoy; documentar para reportar.

---

## Próximo paso

Pasar a Fase 3: aplicar los 11 fixes en `generator.rs` y validar con `cargo check` sobre regeneración limpia + `cargo test` sobre `interpreter_smoke.rs` actualizado.
