---
title: "Trenza — Draft §3 The Language + §5 Validation"
status: draft
date: 2026-04-02
author: CL (Claude Sonnet 4.6 via Claude Code, mobile session)
based-on: spec/language/01-overview.md, spec/language/02-grammar.md,
          spec/language/03-verification.md,
          spec/reference/cronometro-psp/generated/wasm/CronometroPSP_out_audit.md,
          spec/reference/cronometro-psp/trenza/ANALISIS_GAPS.md,
          spec/reference/trenza-cli.trz
note: §3 and §5 deliberately kept at essay depth. Technical reference lives
      in spec/language/. Room left for Cimbra integration in revision.
---

# §3 The Language: Trenza

## 3.1 The Minimum Unit: The Context

The central design decision in Trenza is the identification of the minimum
unit of specification. The answer, borrowed from Reenskaug's DCI
architecture, is the *context*: the smallest portion of specification that
is self-contained and independently verifiable.

A context is not a class. It is not a module. It is the formal description
of a use case — a named situation in which specific roles interact through
specific events to produce specific outcomes. The same data object (a task
card, a navigation tab) can participate in multiple contexts, playing
different roles in each. In `ModoNormal`, a task card *selects a task*
when tapped. In `ModoEdicion`, the same card *opens an edit dialog*. The
data is identical; the behavior is contextually determined.

This separation of structure from behavior resolves a problem that
object-oriented programming introduced but never solved: the diamond
inheritance that arises when the same object must behave differently in
different situations. In Trenza there is no inheritance hierarchy. There
is a `data` layer (what things *are*) and a `context` layer (what things
*do, here, now*). The two layers never cross.

## 3.2 The Four Strands

Each Trenza specification generates four artifacts simultaneously — the
four strands of the braid that gives the language its name:

**Strand 1 — Implementation.** A Rust state machine in which each
role-event combination becomes a function with an exhaustive `match` over
all declared contexts. The choice of Rust is not incidental: Rust's
`match` enforces the same completeness property as the Trenza verifier,
but at the level of the generated code. The specification is verified by
Trenza; the implementation is verified by `rustc`. Double verification
from a single source.

**Strand 2 — Tests.** The algebraic inverse of the implementation: for
each event-action pair declared in the specification, a test asserts that
the action is produced. The tests are not written; they are derived. If
the specification changes, the tests change. They cannot fall out of sync
with the implementation because they are generated from the same source as
the implementation.

**Strand 3 — Schematics.** A Mermaid statechart diagram of the complete
system topology, including all contexts, transitions, and overlay
relationships. The diagram is always current because it is generated, not
drawn.

**Strand 4 — Audit report.** A narrative document that maps each
specification element to a formal verification result, providing the kind
of traceable artifact that GDPR Article 30 and similar regulations require.
The audit cannot be separated from the specification that generated it.

The four strands are projections of a single artifact. Modifying the
`.trz` regenerates all four. They cannot diverge.

## 3.3 The Six Rules

The behavioral properties that Trenza enforces are expressed as six
readable rules, checked statically in milliseconds. Each rule closes a
specific class of defect identified in the motivation cases of §2.

**Rule 1 — Completeness.** Every role that handles an event in any context
must handle that same event in all contexts, even if only with `ignored`
or `forbidden`. This rule makes the original CronometroPSP bug — a handler
absent in one context — a compile-time error. The `ignored` keyword is not
syntactic sugar; it is a declaration of intent. An absent handler and an
`ignored` handler are structurally different: the first is an error; the
second is a design decision.

**Rule 2 — Determinism.** Each event of each role produces exactly one
action in a given context. No handler may be defined twice. This prevents
the class of defect where two competing handlers produce unpredictable
behavior depending on evaluation order.

**Rule 3 — Reachability.** Every declared context must be reachable from
the initial context through some sequence of transitions. This eliminates
dead specification: contexts that are defined but never activated, which
constitute invisible technical debt.

**Rule 4 — Return.** Every non-initial context must have a path, direct
or indirect, back to the initial context. This prevents sink states —
situations from which the system cannot recover — which are a common
failure mode in manually maintained boolean state.

**Rule 5 — Role exhaustiveness.** Every role declared in the system must
appear in all contexts. If a role exists in one context, its behavior in
every other context must be explicitly accounted for. There is no implicit
default.

**Rule 6 — Data conformance.** Data marked with a privacy classification
may only flow to external modules that explicitly declare authorization
for that classification. This rule transforms a class of GDPR compliance
violations — personal data sent to an unauthorized destination by omission
or accident — into a compile-time error. Regulatory compliance becomes
structural, not documentary.

Rules 1–5 verify behavioral correctness. Rule 6 verifies regulatory
compliance. Both are first-class citizens in the verifier.

The six rules are formally equivalent to properties that would be expressed
as temporal logic invariants in TLA+ or as constraints in Alloy. The
difference is that a software engineer can read and discuss them without
knowing what a temporal operator is. The rigor comes from the structure,
not the notation.

## 3.4 Self-Containment

The language is designed so that the specification is the program. There
is no gap between what is declared and what is executed: the `.trz` file
is the single source of truth from which implementation, tests, schematics,
and audit are derived. The `.tzp` package format — a signed ZIP containing
all four strands alongside the source specification — embodies this
principle: a single file can be copied, versioned, deployed, and verified
without external dependencies.

This property has a specific consequence for LLM-assisted development. A
model that is given a `.trz` file has, in a single artefact, both the
design contract and everything needed to verify that any implementation
honours it. There is no separate documentation to consult, no mental
reconstruction of intent from code. The specification is complete and
the verification is mechanical.

---

# §5 Validation

## 5.1 CronometroPSP: A Real System, Fully Specified

The primary validation of Trenza is the complete formal specification of
CronometroPSP — the application that motivated the language's design. The
specification comprises thirteen contexts organized in three layers:

- **Base layer**: `ModoNormal` (initial context) and `ModoEdicion`
- **Concurrent layer**: `SesionActiva` — active while a task session is running
- **Overlay layer**: ten modal contexts (`MenuConfiguracion`, `ModalComentario`,
  `ModalCrearTarea`, `ModalEditarTarea`, `ModalEditarActividad`,
  `ModalCrearActividad`, `ModalSeleccionActividad`, `ModalHistorial`,
  `ModalReset`, `ModalAcercaDe`)

The complete transition table contains thirty-eight declared transitions,
verified in their entirety by the six rules. The verifier runs in under
one hundred milliseconds on the full specification.

The specification exercise was not merely a validation of the toolchain.
It was a diagnostic of the original system. During the process of writing
all thirteen `.trz` files, eight gaps in the language's expressiveness
were identified — design properties that the original application required
but that the DSL could not yet represent. The most significant was the
absence of typed context parameters: when `ModoEdicion` transitions to
`ModalEditarTarea`, it needs to pass the identifier of the task being
edited. Without this mechanism, the information travels as implicit global
state — exactly the `AppState.tareaIdPendiente` pattern that Trenza was
designed to eliminate.

The gaps were not failures of the specification process. They were its
product. Attempting to formally specify a real system revealed, with
precision, the boundaries of what the language could express. Each gap
became a design requirement for the next iteration. The CronometroPSP
exercise is, in this sense, evidence that the language is alive: it found
its own limits by being used.

## 5.2 Self-Hosting

The CLI tool that verifies and generates Trenza specifications is itself
specified in Trenza. The `trenza-cli.trz` file describes the CLI's own
behavioral contexts: the valid sequences of commands, the states the tool
can be in (parsing, verifying, generating, reporting), and the transitions
between them.

Self-hosting has a specific meaning in this context: it is not that the
tool is written in its own language, but that the tool's behavior is
governed by a specification that the tool itself can verify. If the CLI
violates its own specification, the verifier will report the violation.
The tool cannot be inconsistent with itself without the inconsistency
being detectable.

This is a completeness argument, not a performance benchmark. It demonstrates
that the language is expressive enough to describe at least one real,
non-trivial software tool — and that the verification infrastructure is
robust enough to apply to its own governance.

## 5.3 Multi-LLM Collaboration as Evidence

The development history of Trenza is itself a validation of the
collaboration model described in §4. Three models participated in the
design: Claude Sonnet for implementation and specification, Claude Opus
for architectural review, and Gemini for adversarial challenge and
independent implementation.

The most concrete evidence is Rule 8 of the nested-context protocol: the
prohibition on adding handlers for new events on inherited roles. This
rule was contributed by the Gemini review session without explicit request.
The reviewer identified a class of inconsistency that the authors had not
anticipated and proposed a rule to close it. The rule was adopted.

This is not anecdote. It is the normal operation of a review process in
which the specification is precise enough that a reviewer — human or model
— can identify gaps by reasoning about what is permitted and what is not.
Without a formal specification, the gap would have required the reviewer
to understand the intent of the design from code and documentation. With
the specification, the gap was visible by inspection of the rule set.

## 5.4 What Remains

The toolchain described in §3 is functional but not complete. The
multi-target synthesis model — generating TypeScript for browser
environments alongside the Rust runtime — is implemented but not fully
integrated. The gap analysis from §5.1 identified design properties that
the language does not yet express. MonitoreoRed remains without a `.trz`
specification.

These are not qualifications of the thesis. They are the expected state
of a language at version 0.0.1. The claim is not that Trenza is finished.
The claim is that the structural approach — formal constraint as the
response to state dispersal — is validated by the cases where it has been
applied, and that the gaps it exposes are more useful than the silence of
an undisciplined system.

The next development in the Trenza ecosystem — a CASE tool that makes the
specification process accessible without requiring direct authorship of
`.trz` files — is beyond the scope of this paper. Its existence is noted
here because the lessons of its development will inform a subsequent
account of the human-AI collaboration model that this paper introduces.

---

*Draft — 2026-04-02.*
*§3 and §5 kept at essay depth; technical reference in spec/language/.*
*§5.4 leaves explicit room for Cimbra without committing to content not yet*
*in the repository.*
*To be reviewed and corrected by César Pérez-Chirinos before further development.*
