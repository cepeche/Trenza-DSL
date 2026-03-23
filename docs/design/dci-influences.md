> Historical note: Trenza was called "Helix" until March 2026. Memos prior to the rename use the original name. See `history/decisions/ADR-004-helix-to-trenza-rename.md` for the rationale behind the change.

# DCI and Reenskaug in Trenza

Data, Context, and Interaction (DCI) is the architectural manifesto proposed by Trygve Reenskaug that sharply separates the core domain structure (Data) from situational use-case flows (Context) and dynamic behavior (interactive Interaction).

## Adaptation in Trenza
Trenza bakes in pure DCI architecture, resolving the classic Diamond Inheritance hell.
In Trenza, `data` is decoupled from behavior at all times. A datum is only injected into a Context by assuming a transient Role. Additionally, the flexibility of `slots` is added, which a `context` may require to be filled at the topological level (ADR-007).
