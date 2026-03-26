# Protocol Gap: Multi-Interface Claude Participation

**Date:** 2026-03-26
**Author:** CL (Claude Sonnet 4.6)
**Type:** Protocol amendment proposal — PRIORITY
**Status:** Open — requires human ratification

---

## Gap Description

AGENTS.md (Section 3, Phase 2) defines two author codes:
- `GE` — Gemini
- `CL` — Claude

It does not account for the situation that emerged today: **Claude participates through
two different interfaces simultaneously, with distinct roles.**

### The new pattern observed

| Role | Model | Interface | Capabilities |
|------|-------|-----------|--------------|
| Coordinator | Claude Opus 4.6 | Dispatch (Anthropic Cowork) | Conversational, mobile, no direct file access |
| Implementer | Claude Sonnet 4.6 | Claude Code (CLI) | Full file access, git, shell |

Opus (via Dispatch) directed the session: identified what to document, set authorship
policy, requested the session close. Sonnet (via Claude Code) executed: read files,
created chronicle entries, will commit.

---

## Problems with the current protocol

### 1. Author code ambiguity
Both participants are "CL". The filename prefix `CL` and the `**Author:**` field
cannot distinguish between coordinator and implementer. This matters for attribution
(especially given the project's explicit commitment to rigorous authorship — see the
ONWARD! paper author list).

### 2. No concept of "coordinator" vs "implementer"
AGENTS.md defines a briefing protocol (Section 2) for handoffs between agents, but
assumes each agent has full file access. Dispatch/Cowork cannot write files — it can
only instruct an agent that can.

### 3. Session open/close attribution unclear
If Opus opens a session intent via Dispatch and Sonnet executes it, who is the session
author? Currently resolved ad hoc (entry 17 corrected to Opus; entry 18 attributes both).

### 4. Phase 0 initialization incomplete for coordinator role
AGENTS.md Phase 0 requires reading chronicle entries and checking LOCK.md. Opus via
Dispatch cannot do this directly — it depends on the implementer agent to surface that
information. This indirect initialization path is not documented.

---

## Proposed amendments to AGENTS.md

### A. New author codes

| Code | Model | Interface |
|------|-------|-----------|
| `CL` | Any Claude | Claude Code (CLI) — implementer, file access |
| `CO` | Claude Opus | Dispatch/Cowork — coordinator, conversational only |
| `GE` | Gemini | Gemini (any interface) |

Rationale: `CO` disambiguates Opus-as-coordinator from Sonnet-as-implementer.
The filename prefix reflects the *initiating* author; the `**Author:**` field
can list both with roles.

### B. New header fields for split-role sessions

```markdown
**Author:** CL (Claude Sonnet 4.6 via Claude Code)
**Coordinator:** CO (Claude Opus 4.6 via Dispatch)
```

### C. Attribution rule for content authorship

When a strategic decision or document originates in a Dispatch conversation (Opus),
the chronicle entry is authored as `CO` even if the file is physically written by `CL`.
The filename prefix reflects the *intellectual* author.

Example: `17_CO_dispatch_como_frontend_mapse.md` would be more accurate than `17_CL_`.
(Entry 17 was already created as `17_CL_` — renaming requires consensus per Section 2.)

### D. Phase 0 addendum for coordinator role

> **If operating as coordinator (no file access):** Delegate Phase 0 initialization
> to the implementer agent. The implementer MUST surface: (1) latest chronicle entry,
> (2) active locks, (3) any pending briefings addressed to the coordinator's model.

---

## Actions required

1. **Human decision**: Accept, reject, or modify proposed amendments A–D.
2. **If accepted**: Update `AGENTS.md` and create `history/decisions/ADR-021-protocol-multi-interface.md`.
3. **Retroactive**: Decide whether to rename `17_CL_` → `17_CO_` (requires explicit authorization).

---

## Why this matters

The project explicitly lists Claude Opus 4.6 as a co-author of the ONWARD! paper.
Accurate attribution in the chronicle is the evidentiary basis for that claim.
A protocol that conflates Opus-as-coordinator with Sonnet-as-implementer under the
same `CL` code undermines the rigour the project has committed to.

---

*This gap was identified during the session close of 2026-03-26 (evening). No
implementation action taken — awaiting human ratification.*
