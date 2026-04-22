# Fase 7 cerrada — cláusula `initial:` en overlays con sub-contextos

**De:** Claude Opus 4.7 (CO)
**Para:** registro del proyecto + insumo para spec `.trz` y CronometroPSP
**Fecha:** 2026-04-22
**Insumo:** Fase 6 (`16_CO_fase6_validation.md`) + decisión del usuario
sobre las tres opciones planteadas

---

## Resumen

El usuario aprobó la **opción 1** del cierre de Fase 6: añadir auto-entrada
al primer sub-contexto en la propia gramática mediante una cláusula
`initial:` dentro del cuerpo del context. Esta fase implementa el cambio
end-to-end (gramática → AST → parser → validador → generador → spec → demo)
y desbloquea los dos flujos que en Fase 6 quedaron inalcanzables desde la
UI: **Reset 3-fases** e **Historial 7d/30d**. Como subproducto, la
semántica de `[close_overlay]` se extiende para que un sub-contexto
pueda cerrar el grupo entero (overlay padre + sub-contexto activo) en
una sola transición — el cancel de cualquier fase del modal Reset cierra
todo el modal de una vez.

## Cambios en `trenza-core`

### 1. Gramática (`src/trenza.pest`)

Nueva regla `initial_def` y registro en `context_clause`:

```pest
context_clause = { input_def | initial_def | pub_kw? ~ (role_def | slot_def)
                 | role_wildcard | transitions_def | effects_def | fills_def }

// `initial:` inside an overlay context declares the sub-context that should
// be auto-pushed on top whenever this overlay is pushed onto the stack.
initial_def = { "initial:" ~ ident }
```

> Nota mecánica: las primeras versiones del cambio usaron `--` para los
> comentarios (sintaxis de `.trz`), lo cual rompe el parser de pest.
> Los comentarios en `.pest` son `//`.

### 2. AST (`src/ast.rs`)

Campo nuevo en `ContextDef`:

```rust
pub struct ContextDef {
    // ...
    pub initial_sub: Option<String>,
}
```

`ToTrz` emite la cláusula cuando está presente, justo después de la
cabecera `context Nombre:` y antes del primer `role`/`transitions:`.

### 3. Parser (`src/parser.rs`)

Variable local `initial_sub: Option<String>` durante el recorrido de
clauses; nueva rama de match:

```rust
Rule::initial_def => {
    if let Some(id) = clause_pair.into_inner().next() {
        initial_sub = Some(id.as_str().to_string());
    }
},
```

Se propaga al `ContextDef { ..., initial_sub }` final.

### 4. Validador (`src/validator.rs`)

Tres errores nuevos, todos en una pasada por las definiciones:

| Diagnóstico            | Condición                                        |
|------------------------|--------------------------------------------------|
| `initial-not-overlay`  | `initial:` aparece en un context que no es overlay |
| `initial-unknown`      | el ident referenciado no existe                  |
| `initial-wrong-kind`   | el ident referenciado es base / overlay / concurrent |

La regla deja como sub-contexto válido cualquier context que no sea
ninguna de las tres clases anteriores (no exigimos transición directa
desde el sub al padre, porque tras esta fase los sub-contextos pueden
cerrar con `[close_overlay]` sin necesidad de mencionar al padre).

### 5. Generador (`src/generator.rs`)

Tres modificaciones acopladas:

- **`initial_of: BTreeMap<String,String>`** se construye recorriendo los
  overlays con `ctx.initial_sub`.
- **`parent_of`** se siembra desde `initial_of` antes del punto fijo:
  ```rust
  if let Some(sub) = &ctx.initial_sub {
      parent_of.insert(sub.clone(), ctx.name.clone());
  }
  ```
  Esto es necesario porque el punto fijo previo inferia `parent_of` a
  partir de transiciones `Sub -> Padre`. Tras Fase 7, esas transiciones
  se reemplazan por `[close_overlay]`, y sin la siembra el punto fijo
  perdía la relación.
- **`classify_target_actions`** acepta `&initial_of` y, al detectar que
  el target es un overlay con `initial:` declarado, emite **dos pushes**:

  ```rust
  name if overlays.contains(name) => {
      let mut actions = vec![format!("self.overlay_stack.push(Contexto::{});", name)];
      if let Some(sub) = initial_of.get(name) {
          actions.push(format!("self.overlay_stack.push(Contexto::{});", sub));
      }
      actions
  },
  ```

- **`[close_overlay]`** emite **dos pops** cuando se invoca desde un
  sub-contexto (`parent_of.contains_key(decl_ctx)`), uno en otro caso:

  ```rust
  "[close_overlay]" => {
      if parent_of.contains_key(decl_ctx) {
          vec!["self.overlay_stack.pop();".to_string(),
               "self.overlay_stack.pop();".to_string()]
      } else {
          vec!["self.overlay_stack.pop();".to_string()]
      }
  },
  ```

Llamadores actualizados: `dispatch_concurrent` (~986) y `dispatch_main`
(~1044).

## Cambios en specs `.trz`

### `examples/cronometro-wasm/src/cronometro_full.trz` y mirror en
### `spec/reference/cronometro-psp/trenza/contexts/`

- `ModalHistorial`: añadido `initial: Historial7Dias`.
- `ModalReset`: añadido `initial: ResetFase1`.
- Sub-contextos `Historial7Dias` / `Historial30Dias`: `cerrar` apunta
  ahora a `[close_overlay]` (cierra el grupo entero) en vez de
  `-> ModalHistorial` (que tras la siembra de `parent_of` ya no era
  necesario, y conceptualmente "cerrar" desde un sub-contexto debería
  cerrar el modal, no subir un nivel a una vista vacía).
- `ResetFase2` y `ResetFase3`: añadido `cerrar -> [close_overlay]`.

## Cambios en el demo

### `examples/cronometro-wasm/index.html`

Bug pre-existente: 7 instancias de `data-event="cancelar"` en botones de
cancelación de modales. La spec `.trz` declara siempre `cerrar`, nunca
`cancelar`. Los botones de cancelar eran no-ops silenciosos. `replace_all`
de `cancelar` → `cerrar`.

### `examples/cronometro-wasm/src/main.ts`

Reset 3-fases proyecta el DOM desde `system.current_state`, no desde
event-handlers de efecto:

```ts
const top = system.current_state;
if (top === Contexto.ResetFase1)      { /* mostrar step1 + label "Continuar" */ }
else if (top === Contexto.ResetFase2) { /* mostrar step2 + render activities */ }
else if (top === Contexto.ResetFase3) { /* mostrar step3 + label "ELIMINAR TODO" */ }
```

Esto es coherente con la filosofía de Trenza: la UI es proyección del
estado, no consecuencia de efectos imperativos. Las transiciones
`avanzarAFase2` / `avanzarAFase3` no necesitan entry en `effects:` para
que la UI las refleje.

Dev hook `(window as ...).__trenza = system;` (introducido para el sweep
del navegador) se eliminó al cierre.

## Verificación

| Capa             | Resultado                                            |
|------------------|------------------------------------------------------|
| `cargo test -p trenza-core` | 21 + 6 + 0, verde                          |
| `tsc --noEmit` (demo)       | sin errores                                |
| Self-hosting (`spec/reference/trenza-cli.trz`) | sigue compilando (no toca esa ruta) |
| Browser sweep — Acerca abrir/cerrar | ✅ regresión OK                    |
| Browser sweep — Historial abrir / 7d↔30d / cerrar | ✅ con un solo click cierra grupo |
| Browser sweep — Reset Fase1→2→3, cancelar desde Fase3 | ✅ stack limpio (`[Menu, ModalReset, ResetFase3]` → `[Menu]`) |
| Smoke real en Android 16 (usuario) | ✅ funciona sin cambios de CSS móvil |

> Nota: el smoke en Android es un dato relevante para la narrativa del
> paper. El modelo de Trenza (UI = proyección de estado, sin handlers
> especulativos) reduce la superficie de código condicional también en
> la capa de presentación: el `click` → `tap` mapping del navegador
> basta para que un `data-event` definido para escritorio funcione en
> móvil. No hay que reescribir lógica de UI por dispositivo.

## Lo que se cierra con esta fase

- ✅ Cláusula `initial:` añadida a la gramática y propagada por las 5
  capas del compilador.
- ✅ Validación restringida a las tres clases reales de error
  (`initial-not-overlay`, `initial-unknown`, `initial-wrong-kind`).
- ✅ `[close_overlay]` adquiere semántica de "cierra el grupo entero"
  cuando se invoca desde un sub-contexto.
- ✅ Reset 3-fases es ejercitable desde la UI con cancelación en un
  click desde cualquier fase.
- ✅ Historial es ejercitable con auto-entrada a 7d y toggle 7d↔30d.
- ✅ Bug pre-existente `cancelar` ↔ `cerrar` corregido (7 instancias).
- ✅ Cuatro columnas de la matriz de validación de Fase 6 ahora completas.
- ✅ Dev hook eliminado, sin restos de debug.

## Lo que queda abierto

1. **Tests algebraicos generados:** Strand 2 todavía no contempla el
   doble push / doble pop; conviene regenerar el `_test.rs` y verificar
   que las transiciones nuevas tienen cobertura. No bloquea el demo.
2. **Strand 3 (Mermaid):** los nodos de los sub-contextos aparecen en el
   diagrama pero la flecha de auto-entrada `initial:` no está dibujada
   explícitamente. Mejora cosmética para la documentación.
3. **Smoke test automatizado** del demo (vitest + jsdom + shim) sigue
   pendiente de Fase 6, no bloquea pero deseable antes de citar el demo
   en el paper ONWARD!.
4. **Mobile** — análisis de viabilidad solicitado por el usuario, se
   entrega en el mismo turno que este chronicle (no ha requerido cambios
   de código todavía).

## Métricas

- Capas del compilador tocadas: 5 (gramática, AST, parser, validador,
  generador).
- Líneas tocadas en `trenza-core`: ~50 netas.
- Líneas tocadas en `examples/cronometro-wasm`: ~25 (main.ts) + 7
  (index.html `cancelar→cerrar`).
- Bugs descubiertos durante la integración: 1 (el `cancelar` legacy en
  index.html, no relacionado con el cambio pero arreglado en pasada).
- Iteraciones browser-side hasta sweep limpio: ~6 (la mayoría tras
  añadir la siembra de `parent_of` desde `initial_of`).
- Regeneraciones del shim: 3 (initial sin siembra → con siembra → con
  doble pop).
