> Historical note: Trenza was called "Helix" until March 2026. Memos prior to the rename use the original name. See `history/decisions/ADR-004-helix-to-trenza-rename.md` for the rationale behind the change.

# Generalized Lifecycle and "Effects"

Trenza discards coupling to specific ecosystems.

The original conceptual prototypes considered semantic lifecycles identical to those of frontend web development (e.g., the "OnViewDidMount" hook). The abstraction evolved toward a fully universal directive catalogued as `effects:` (ADR-006). Rather than tying the DSL to the lifecycle of the frontend or React/Vue/native Android, effects express generic domain-level side-effects (`GET /api/session`) independent of their host integrations.
