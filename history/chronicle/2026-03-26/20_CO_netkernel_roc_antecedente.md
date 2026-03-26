# NetKernel/ROC as Indirect Intellectual Antecedent of Trenza

**Date:** 2026-03-26
**Author:** CO (Claude Opus 4.6 via Dispatch)
**Implemented by:** CL (Claude Sonnet 4.6 via Claude Code)
**Type:** Research / intellectual genealogy

---

## Subject

NetKernel (1060 Research, Peter Rodgers, initiated at HP Labs 1999) and its
Resource Oriented Computing (ROC) model were investigated as a potential
intellectual antecedent of Trenza/MAPSE.

---

## NetKernel / ROC — Summary

NetKernel implements Resource Oriented Computing: a generalization of REST and
Unix where everything is a resource identified by URI, with radical separation
between the logical request and its physical resolution mechanism.

Key properties:
- Every computation is a resource addressed by URI
- Separation of *what is requested* from *how it is resolved* — the kernel
  routes requests to appropriate transports/backends transparently
- **Pure function cache**: if a function is referentially transparent, results
  are cached automatically at microkernel level — no explicit cache management
- The architecture allows the same logical request to be resolved by different
  physical mechanisms depending on context (filesystem, network, database, etc.)

---

## César's Personal History with NetKernel

César had direct contact with Peter Rodgers via email and collected NetKernel
releases across multiple versions. Rodgers told him he was "the project's biggest
fan". Although he never analyzed the codebase in depth, two aspects made a strong
impression:

1. **The pure function cache** — reducible functions cached automatically at the
   microkernel level. A form of memoization at the infrastructure level, not the
   application level.
2. **The visual aesthetics** — the project's iconography and visual identity.

Note: the visual aesthetics of NetKernel **must not be reused** without explicit
authorization from Peter Rodgers / 1060 Research.

---

## Conceptual Resonances with Trenza/MAPSE

| NetKernel/ROC concept | Trenza/MAPSE parallel |
|-----------------------|-----------------------|
| Logical request / physical resolution separation | Intent (conversation) / implementation (.trz + compiler) separation in MAPSE |
| URI-addressed resources over transport-agnostic protocol | `trenza-coord` JSON-RPC 2.0 over TCP — resource-like addressing of agents and locks |
| Pure function cache at kernel level | Potentially applicable to formal rules in `trenza-cli` — if a rule's inputs haven't changed, its result could be cached |
| Single addressing scheme across heterogeneous backends | MCP as universal protocol across Claude, Gemini, local models |

No direct evidence was found that ROC influenced MCP (Anthropic). However, the
philosophical kinship is close: both treat the protocol layer as the stable
interface and make the resolution mechanism pluggable.

---

## Project Status

NetKernel is likely inactive. Last documentable release: **6.1.1 (November 2016)**.
Site `netkernel.io` remains live but shows no significant recent activity.

---

## Decision on Contact with Peter Rodgers

César considers that contacting Rodgers would be appropriate **only if the ONWARD!
2026 paper is accepted**. No contact before that milestone.

---

## Pending Action (for future Claude Code sessions)

The project has an inspirations section. Integration of NetKernel/ROC into that
section — with appropriate framing as indirect antecedent — is a task for future
sessions. Not blocking any current work.

---

*First chronicle entry using the CO author code, applied per direct human authorization
(2026-03-26). Protocol amendment (entry 19) formally ratified by this usage.*
