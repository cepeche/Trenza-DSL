---
date: 2026-03-23
session: Opus architectural review of Trenza CLI (Rust) — updated after Gemini's second iteration
participants: developer, Claude Sonnet 4.6, Claude Opus 4.6, Gemini
---

# Opus Review of Trenza CLI — Updated Status

Opus reviewed the first Rust CLI implementation. By the time the review was
delivered, Gemini had already advanced significantly. This document reflects
the **current state** after Gemini's second iteration.

---

## What Gemini Has Already Fixed

### ✅ Critical: Generator architecture corrected

The original generator produced one function per context with a `_ => unimplemented!()`
catch-all — architecturally inverted relative to the spec.

The current `generator.rs` now uses a `BTreeMap` to group actions by
`(role_name, event_name)` and generates **one function per role+event** with
match arms for each context that handles that combination. This is the correct
pattern from the spec: Rust's compiler can now enforce completeness as the
context enum grows.

### ✅ Bonus: Mermaid schematic generator

`generate_mermaid()` added unprompted — produces `stateDiagram-v2` output
from the parsed AST. This is the first sketch of Strand 3 (schematic).

---

## Still Open (from Opus's review)

### 1. `_ => unimplemented!()` — completeness guarantee still broken

`generator.rs` line 63 still has a catch-all arm. This means adding a new
context variant to the `Contexto` enum still compiles without error and panics
at runtime. The spec requires that the match be exhaustive with no catch-all,
so Rust enforces completeness at compile time.

The fix: remove the `_ =>` arm. If a role+event combination is undefined in a
given context, that context simply should not appear in the match at all — and
the compiler will correctly warn if an unhandled variant exists.

### 2. AST: effects silently dropped

`parser.rs` line 88: `Rule::effects_def => {}` — effects are parsed by pest,
then discarded. `ContextDef` has no `effects` field. Effects are a first-class
citizen of the spec (one of the six components of a context). They must be
stored and eventually drive Strand 1 generation.

### 3. AST: `RoleAction.call` should be `Option<ActionCall>`

The grammar allows `on event` with no `-> action_call`. Currently, missing
action calls produce `ActionCall { function: "", args: [] }`, which is
indistinguishable from a parse error.

More importantly: `forbidden` and `ignored` are reserved keywords with
verification consequences (Rule 1, structural least privilege). They must be
represented as distinct AST variants, not as function names. Proposed:

```rust
pub enum ActionTarget {
    Call(ActionCall),
    Ignored,
    Forbidden,
}
```

### 4. AST: `SystemDef` drops topology

`parse_system` discards `system_sections` (the `contexts:`, `concurrent:`,
`overlays:`, and `events:` lists). `SystemDef` only stores `name` and
`initial`. The topology is required for verification rules 4 and 5
(context reachability and concurrent composition).

### 5. Grammar: missing constructs from canonical examples

None of the non-trivial `.trz` examples parse with the current grammar.
Priority order:

| Construct | Example | Notes |
|-----------|---------|-------|
| `external` blocks | `autenticacion-rgpd.trz` | Boundary with conventional code; required for Rule 6 |
| `input:` / `mutable` | both canonical examples | Data flow between contexts |
| `[on_entry]` / `[on_exit]` | both canonical examples | Lifecycle hooks in `effects:` |
| `[stay]` as transition target | `carrito-checkout.trz` | Grammar expects `ident`, not `[stay]` |
| `forbidden` / `ignored` as action targets | both canonical examples | Currently parsed as function names |
| `with` clause in transitions | both canonical examples | Typed data channel between contexts |
| `when` guards | `carrito-checkout.trz` | GAP-3 resolved in spec, missing in grammar |
| `bind:` in role declaration | `autenticacion-rgpd.trz` | Parenthesized binding syntax |

### 6. Minor (low priority but worth a pass)

- `Contexto` enum name should be `Context` (ADR-005: keywords in English;
  generated code follows spec naming)
- CLI error messages are in Spanish (`"Uso: ..."`, `"Debe especificar..."`)
- Output filename `file.trz_out.rs` is not a valid Rust module name; strip
  `.trz` before appending `_out.rs`

---

## Suggested Next Milestone

**Target: parse `autenticacion-rgpd.trz` successfully.**

This requires addressing item 5 above (grammar extensions). It is a concrete,
verifiable milestone — either the file parses or it doesn't. Once it parses,
the AST gaps (items 2–4) become the natural next focus, and the generator can
start producing meaningful output for a real example.
