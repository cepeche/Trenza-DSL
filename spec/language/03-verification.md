# Formal Verification in Trenza

Trenza verifies formal properties by expressing them as readable rules that are checked statically, without the need for symbolic notation such as TLA+.

## The 6 Main Rules

1. **Completeness**: Every role that handles an event in any context must handle it in all contexts, even if only explicitly with `ignored` or `forbidden`.
2. **Determinism**: In a given context, each event of each role produces exactly one action. No structural ambiguity exists.
3. **Reachability**: Every declared context must be reachable, directly or indirectly, from the `initial` context.
4. **Return**: Every non-initial context must have a transition that, directly or indirectly, returns to the initial context (prevents sink states).
5. **Role exhaustiveness**: Every role defined in the `system` block must be statically invoked in all contexts.
6. **Data conformance**: No data marked as `[clasificacion: personal]` may be sent to an `external` module that does not expressly declare `[autorizado_para: personal]` (Structural GDPR compliance).
