---
date: 2026-03-23
session: Status review after Gemini's second iteration on the Rust CLI
participants: developer, Claude Sonnet 4.6
---

# Trenza CLI — Second Review (Post-Gemini Iteration 2)

## Overall Assessment

Gemini has resolved every architectural issue identified in Opus's first review.
The implementation has advanced from a structural prototype to a functional
3-strand compiler for one of the two canonical examples.

## What Works

- `autenticacion-rgpd.trz` parses, validates, and generates all three strands
- `ActionTarget` is a proper enum: `Call(ActionCall) | Ignored | Forbidden`
- `effects:` fully preserved in AST and generated in Strand 1
- `SystemDef` preserves full topology (`contexts:`, `concurrent:`, `overlays:`, `events:`)
- Grammar covers: `external`, `input:/mutable`, `bind:`, `[on_entry]/[on_exit]`,
  `[stay]`, `with`, `@audit()` decorators
- Validator implements Rules 1 (completeness), 2 (determinism), 3 (reachability)
- `--profile=pre/pro` infrastructure in place

## Remaining Gaps

| Gap | Blocker? | Notes |
|-----|----------|-------|
| Parametrized effect triggers `[on E.ok(r)]` | ✅ Yes | `carrito-checkout.trz` fails at this syntax |
| Rule 4: Concurrent synthesis | No | `concurrent:` preserved in AST, not yet validated |
| Rule 5: Post-action state routing | No | `.ok/.error` textual, not semantic |
| Rule 6: Data classification enforcement | No | Grammar ready, validator not enforcing |

## Next Milestone

**Get `carrito-checkout.trz` to parse and validate.**

The blocking construct is the parametrized effect trigger on line 91:

```
[on agregar_item.ok(r)] -> resumen.asignar(r)
```

Proposed grammar extension:

```pest
effect_trigger = {
    lifecycle_hook
    | "[" ~ "on" ~ ident ~ ("(" ~ ident ~ ")")? ~ "]"
}
```

Parser needs to capture the argument binding `r` as an `Option<String>` on `EffectTrigger`.

Once `carrito-checkout.trz` is unblocked, Rule 4 (concurrent synthesis) is the
next natural target — both canonical examples have `concurrent:` sections that
are architecturally significant and currently unvalidated.
