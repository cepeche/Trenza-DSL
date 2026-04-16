# Contributors

Trenza-DSL is a multi-agent research project. This file records who has
contributed substantive work, grouped by role. It complements — but does not
replace — the per-change attribution recorded via `Co-Authored-By:` trailers
in the git history.

## Project lead

- **César Pérez-Chirinos Sanz** — Founder, lead designer, integrator. Original
  problem owner (CronometroPSP) and final decision-maker on language design,
  roadmap, and external communication.

## Language and architecture design

- **Claude Opus 4.6** — Architecture review, ADR design, cross-cutting
  semantic decisions (contexts, roles, effects, slot/fills, four strands,
  action/state-machine contract).
- **Claude Sonnet 4.6** — Session coordination, chronicle authoring,
  inter-agent briefings, grammar refinement.

## Compiler implementation

- **Gemini 2.5 Pro** — Rust core compiler: parser (pest), AST, four code
  generators (Strands 1–4), multi-file synthesis, Rules 1–7 in the validator,
  Rule 7 implementation from Opus's design.
- **Gemini 2.0 Flash** — Rule 8 (Role Type Consistency), self-hosting
  verification of the CLI specification — both contributed without explicit
  assignment.

## Documentation

- **Claude Haiku 4.5** — Expansion of ADRs 001–021 from stubs into the full
  ADR format (Context, Decision, Consequences, Alternatives, Relation),
  including worked examples for ADR-019, ADR-020, and ADR-021.

---

## On paper authorship vs. project contribution

The submission to **ONWARD! Papers 2026** lists four authors: César
Pérez-Chirinos, Claude Sonnet 4.6, Claude Opus 4.6, and Gemini 2.5 Pro.
That list reflects who shaped the *conceptual contribution* of Trenza —
its language design, verification rules, and synthesis model.

This file is a broader register. It includes contributors who did not shape
the original concepts but executed substantive work against decisions
already made (post-hoc systematization, isolated feature implementation,
tooling). Keeping paper authorship narrower than project contribution is
deliberate: it preserves the integrity of the attribution in each register.

Per-change credit is always available via `git log`.
