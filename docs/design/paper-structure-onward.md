# ONWARD! 2026 — Paper Structure Proposal

**Title:** Trenza: A Role-Based State Machine DSL for Human-AI Collaborative
Specification and Synthesis
**Venue:** ONWARD! Papers @ SPLASH 2026, Oakland CA, 4-9 Oct 2026
**Deadline:** Friday 15 May 2026 (AoE)
**Format:** ACM SIGPLAN acmart, sigplan sub-format, 10pt, 13 pages (excl. refs)
**Authors:** César Pérez-Chirinos, Claude Sonnet 4.6, Claude Opus 4.6, Gemini 2.5 Pro

---

## Core Thesis

> *Trenza is not a DSL for LLMs to write better code. It is the adult in the room
> that makes indiscipline — human and model alike — a compilation error.*

LLMs are undisciplined because they are faithful to the corpus, which was written
by undisciplined humans. The problem is not capability — it is the absence of formal
constraint. Trenza imposes that constraint on both parties.

---

## Narrative Arc

| Act | Event |
|-----|-------|
| Prologue | 30+ years of distributed objects. Skeptic of "stochastic parrots". No AI code generation attempted. |
| Act I | One morning: Claude rebuilds Fing (MonitoreoRed). Conversion. The skeptic is wrong. |
| Act II | MonitoreoRed: state dispersal emerges. claude-flow's answer: more agents. Wrong direction. |
| Act III | CronometroPSP: `modoEdicion` in 4 places. *"This bug should be a compilation error."* |
| Act IV | Trenza: DSL + compiler. Four strands. Eight formal rules. Self-hosting verified. |
| Coda | The DSL is for the agents. Humans provide intent; agents crystallize .trz; compiler enforces on both. |

---

## Section Structure (~13 pages)

### §1 Introduction (~1.5 pp)
- Opening: the skeptic with 30 years in distributed objects
- The conversion moment: one morning, Fing rebuilt
- Same experiment reveals the problem: functional code, dispersed state
- Thesis declared

### §2 Motivation: The State Dispersal Problem (~2 pp)
Two real empirical cases:

**Case 1 — MonitoreoRed**
Real infrastructure: Prometheus + Grafana + Blackbox + Node Exporter + Docker.
Device state split across: dispositivos.json (snapshot), Prometheus timeseries
(metrics, not states), ping scripts (point-in-time only).
claude-flow as the wrong answer: adds coordination where design is needed.

**Case 2 — CronometroPSP**
`modoEdicion` in four places. The fracture point. Not developer frustration —
precise diagnosis of a structural pattern.

*Unified pattern:* implicit state is the technical debt LLMs cannot avoid when
the language does not forbid its creation.

### §3 The Language: Trenza (~3.5 pp)
- Four core concepts: roles, contexts, transitions, effects
- Illustrated with real CronometroPSP fragments
- Eight formal rules as direct consequences of §2
- Each rule closes a specific class of state dispersal
- Self-hosting as completeness proof

### §4 The Collaboration Model (~2 pp)
- The language was not designed — it emerged from conversation
- Human provides intent; agent crystallizes .trz; compiler verifies
- The chronicle as real requirements engineering process, not post-hoc docs
- Empirical finding (llm-review-validation.md): .trz changes review from
  heuristic to deductive 1:1 verification

### §5 Validation (~2 pp)
- trenza-cli.trz self-hosting
- CronometroPSP: 18 .trz files, 884 lines Rust generated, 5 active generators
- Multi-LLM collaboration as evidence: Gemini implements Rule 8 without
  explicit request; Opus identifies protocol gap from mobile session;
  Sonnet verifies and ratifies

### §6 Related Work (~1.5 pp)
- **Statecharts** (Harel 1987) — formal foundation; Trenza deliberately
  restricts expressiveness
- **Naked Objects** (Pawson 2002) — domain behavior as interface; Trenza
  formalizes that behavior
- **NetKernel/ROC** (Rodgers, HP Labs 1999) — personal intellectual
  antecedent: logic/physics separation, reducible function cache;
  indirect connection, honestly declared
- **MCP / Agent protocols** — Trenza as specification layer over any
  coordination protocol
- **claude-flow** — the opposite approach: complexity as response to
  indiscipline

### §7 Conclusion (~0.5 pp)
- The language that human-LLM collaboration *demanded* into existence
- Constraint as a gift — not to LLMs, but to the collaboration itself
- Close the circle: MonitoreoRed remains unsolved; the second .trz awaits

---

## Existing Materials (covers ~70% of technical content)

| Material | Location | Covers |
|----------|----------|--------|
| Language spec | `spec/language/` | §3 |
| ADRs 001-021 | `history/decisions/` | §3, §4 |
| LLM review experiment | `docs/design/llm-review-validation.md` | §4 |
| Existing abstract | `docs/design/paper-hybrid-abstract-onward.md` | §1 |
| Related work draft | `docs/design/related-work-research.md` | §6 |
| CronometroPSP .trz | `spec/reference/cronometro-psp/` | §3, §5 |
| Self-hosting | `spec/reference/trenza-cli.trz` | §5 |

## Remaining work (the narrative)
- §1: Convert the arc above to prose (the skeptic → the conversion)
- §2: Write up MonitoreoRed case formally
- §4: Collaboration model narrative (chronicle as requirements engineering)
- §7: Closing synthesis

---

*Structure proposed by CL (Claude Sonnet 4.6), session 2026-03-27 morning.*
*Core thesis coined in dialogue — attributed to the session, not to any single author.*
