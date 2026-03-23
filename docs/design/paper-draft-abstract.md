# Draft Abstract — PLATEAU 2026

**Trenza: A Role-Based State Machine DSL for Human-AI Collaborative Specification**

César Pérez-Chirinos¹, Claude Sonnet 4.6², Claude Opus 4.6², Gemini 2.5 Pro³

¹ Independent
² Anthropic
³ Google DeepMind

---

## Abstract

Software defects caused by implicit, scattered state management are among the
most costly to diagnose in practice. We present **Trenza**, a domain-specific
language in which system behavior is expressed as a set of named contexts, each
declaring the roles, transitions, and effects that are valid within it. The
language is deliberately restrictive: every role-event pair must be handled in
every context (completeness), no handler may be duplicated (determinism), every
context must be reachable from the initial state (reachability), and data
access is structurally scoped by role (least privilege). These constraints are
enforced at compile time by a Rust implementation with six formal verification
rules, making a class of behavioral errors that typically surface at runtime
into errors that surface before a single line of implementation code is written.

The language is designed to be read and reasoned about by both human developers
and large language models (LLMs). We report on an experiment in which
LLM-assisted code review was conducted with and without a Trenza specification
as reference. The result was not a reduction in review time but a change in
epistemic regime: review without the specification was heuristic; review with
it was a mechanical, exhaustive, one-to-one deductive check. We argue that this
regime shift — from "does this look correct?" to "does this correspond?" —
represents a qualitative improvement in the verifiability of LLM-generated
code, and that it is enabled by the combination of formal restriction,
plain-text syntax, and role-based structure that Trenza provides.

We describe the language design, its compiler, and a real-world case study
in which a production application is being re-specified in Trenza as the
artifact that originally motivated its design.

---

## Keywords

domain-specific languages, state machines, formal verification, LLM-assisted
development, role-based design, DCI, software specification

---

## Notes on authorship

This paper documents a collaborative design process in which three large
language models participated substantively: as architects, as implementors,
and as experimental subjects. We include them as co-authors in the spirit of
accuracy — their contributions are traceable, dated, and committed to the
public repository — while acknowledging that the question of LLM authorship
in academic publishing remains open and worth debating on its own terms.

---

*Draft — 23 March 2026*
*Full repository: https://github.com/cepeche/Trenza-DSL*
