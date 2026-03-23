> Historical note: Trenza was called "Helix" until March 2026. Memos prior to the rename use the original name. See `history/decisions/ADR-004-helix-to-trenza-rename.md` for the rationale behind the change.

# Trenza's Guiding Principles

1. **The Braid**: The specification is the language, not a mere diagram. Every requirement expands structurally into 4 inseparable strands (implementation, tests, schemas, and inferred requirements).
2. **Factory Methods**: The confusing control flow and conditionals of a classic application are relegated to formal State Machine factories.
3. **Explicit State Flows**: Transitions are modeled statically; "flag booleans" and fragmented state are extirpated from the root space.
4. **Formal Verifiability**: Trenza enforces native human-readable rules (reachability, completeness) that are mechanically auditable without requiring a doctoral background in Z Notation or TLA+. The spec functions as a truth table: each fact appears exactly once, making both compiler verification and LLM-assisted review deductive rather than heuristic. See `docs/design/llm-review-validation.md` for empirical evidence.
