# Strategic Insight: Dispatch as Accessible Frontend for MAPSE

**Date:** 2026-03-26
**Author:** CL (Claude Opus 4.6)
**Type:** Strategic decision / design note

---

## Context

After reviewing the current MAPSE architecture (see `16_GE_architectural_map.md`), a key
question was raised: do we need to build our own conversational frontend, or does one
already exist?

## Evaluation: Dispatch (Anthropic Cowork)

Dispatch is Anthropic's conversational interface with mobile access. It was evaluated as
a candidate for the MAPSE user-facing layer.

**Relevant properties:**
- Conversational by design — matches the "intent via dialogue" model of MAPSE
- Mobile-accessible — satisfies the voice-first, accessibility-first requirement
  (user walking, talking by phone, hands-free)
- Already exists and is maintained — no UI work required on our part
- Supports MCP — Trenza could be exposed as an MCP server within Dispatch

## Strategic Direction

**Trenza as DSL + coordination protocol, not as a standalone tool.**

The revised framing:
- `trenza-cli` — compiler and formal verifier (pure, stateless)
- `trenza-coord` + `trenza-msg` — coordination protocol (agent-to-agent messaging, locks)
- Dispatch (or any MCP-capable conversational platform) — the user-facing layer

This is consistent with the project's philosophy of not reinventing the wheel. Building
a custom mobile frontend, custom voice interface, and custom auth layer would be months
of work orthogonal to the core research contribution. Dispatch provides all of that.

## How Trenza Would Integrate with Dispatch

Trenza would be exposed as an MCP server. Dispatch would call MCP tools such as:
- `compile_trz` — invoke `trenza-cli`, return structured JSON diagnostics
- `acquire_lock` / `release_lock` — delegate to `trenza-coord`
- `send_message` / `poll_messages` — agent-to-agent coordination via `trenza-msg`

The user speaks or types intent → Dispatch routes to AI agent → agent generates `.trz`
→ MCP tool invokes compiler → diagnostics returned as structured response → read aloud
or displayed conversationally.

## Alignment with Existing Architecture

`trenza-coord` already speaks a JSON-RPC 2.0 dialect over TCP, which is MCP-compatible
in spirit. Exposing it as a formal MCP server is a natural next step, not a pivot.

The architectural map (`16_GE_architectural_map.md`) shows VS Code as the front-end
host. Dispatch would be an alternative host — same coordination layer underneath.

## Open Questions

1. Is Dispatch publicly available or internal to Anthropic? Scope of use TBD.
2. What MCP tool schema would Dispatch expect for compiler diagnostics?
3. Does this affect the ONWARD! paper framing? (Likely yes — strengthens the
   "platform-agnostic DSL" angle.)

## Decision

No implementation action required today. This is a recorded design direction:
**prefer integration over construction** for the user-facing layer.

---

*"The best interface is the one you don't have to build."*
