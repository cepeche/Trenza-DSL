---
title: "Trenza — §3 The Language (revision delta against paper-draft-s3-s5.md)"
status: draft
date: 2026-04-17
author: Claude Opus 4.6
based-on: docs/design/paper-draft-s3-s5.md (CL, 2026-04-02)
note: This document supersedes §3.3 of the original draft and adds §3.5.
      §3.1, §3.2 and §3.4 of the original draft are kept verbatim.
---

# Audit summary against the 2026-04-02 draft

The original §3 was written when the compiler had six active rules. Two
additional rules have since been formalized and are active in
`trenza-core/src/validator.rs`. The §3.3 text below replaces the original
"Six Rules" section. A new §3.5 documents the multi-target synthesis
(Rust + TypeScript) that the original draft did not yet describe.

Other §3 corrections:

- §3.2 ("Strand 1 — Implementation") should note that the same generator
  pipeline now emits a TypeScript projection alongside Rust. The TypeScript
  output is a *projection of Strand 1*, not a fifth strand: the four-strand
  model remains the conceptual unit.
- §3.4 reference to the `.tzp` package format is accurate; no change.

---

# §3.3 The Eight Rules (replaces "The Six Rules")

The behavioral properties that Trenza enforces are expressed as eight
readable rules, checked statically in milliseconds. Each rule closes a
specific class of defect identified in the motivation cases of §2.

**Rule 1 — Completeness.** Every role that handles an event in any context
must handle that same event in all contexts, even if only with `ignored`
or `forbidden`. This rule makes the original CronometroPSP bug — a handler
absent in one context — a compile-time error. The `ignored` keyword is
not syntactic sugar; it is a declaration of intent. An absent handler and
an `ignored` handler are structurally different: the first is an error;
the second is a design decision.

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

**Rule 7 — Slot/fills integrity.** Concurrent contexts may declare typed
slots that other contexts fill at runtime. Rule 7 verifies, at compile
time, that every declared slot has at least one filling context, that
every `fills` declaration targets an actual slot, and that the type of
the filling context matches the slot's declared role-set. Without this
rule, the overlay composition primitive — which is what makes complex
modal systems like CronometroPSP expressible — could produce structurally
incoherent compositions that would fail only at runtime.

**Rule 8 — Role-type consistency.** A role name carries a type signature
across the entire specification. If `EditorTarea` is declared with a
particular event surface in one context, the same name in any other
context must declare the same surface. This prevents a subtle class of
defect in which the same conceptual actor acquires divergent capabilities
depending on which file the reader happens to be looking at — exactly the
kind of dispersal that motivated the language.

Rules 1–5 verify behavioral correctness. Rule 6 verifies regulatory
compliance. Rules 7 and 8 verify compositional and onomastic consistency
across contexts. All eight are first-class citizens in the verifier.

The eight rules are formally equivalent to properties that would be
expressed as temporal logic invariants in TLA+ or as constraints in
Alloy. The difference is that a software engineer can read and discuss
them without knowing what a temporal operator is. The rigor comes from
the structure, not the notation.

---

# §3.5 Multi-Target Synthesis

The four strands described in §3.2 are the conceptual artifacts every
specification produces. Strand 1 — the implementation — is itself
parameterized by a target platform. The current compiler emits two
implementation projections from the same `.trz` source: a Rust state
machine for native and WASM execution, and a TypeScript module for
browser environments. Both projections share the same exhaustiveness
guarantees: the Rust output is verified by `rustc`'s `match`
exhaustiveness; the TypeScript output is verified by the compiler itself
before emission, since the TypeScript type system does not enforce
exhaustive discriminated-union handling by default.

Multi-target synthesis is not a generality claim about platforms. It is
a structural claim about the specification: a `.trz` file describes
behavior at a level of abstraction that admits more than one concrete
realization without recompilation of the source. The same CronometroPSP
specification produces a Rust binary that runs in WASM and a TypeScript
module that runs in a React frontend; both implementations are derived
from the same role-event tables and cannot diverge from each other
without diverging from the source.

This property has direct relevance to the LLM collaboration argument of
§4. A model asked to make a behavioral change does not modify two
implementations and hope they remain consistent. It modifies the
specification, and the consistency between targets is preserved by
construction.

---

*Draft — 2026-04-17.*
*This document is a delta against `paper-draft-s3-s5.md`. Sonnet should*
*integrate §3.3 (replacing the "Six Rules" section) and §3.5 (new) when*
*porting to LaTeX. §3.1, §3.2 and §3.4 of the original draft are*
*unchanged.*
