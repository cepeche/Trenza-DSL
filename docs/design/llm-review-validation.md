# Validation: The `.trz` Spec as a Review Artifact for LLMs

**Date:** March 23, 2026
**Type:** Empirical validation of Principle 4 (Formal Verifiability)
**Full record:** `history/chronicle/2026-03-23/04_experimento_revision_llm.md`

---

## What was tested

Whether having a `.trz` specification changes the quality of code review
performed by an LLM on LLM-generated Rust output.

Three deliberate bugs were injected into `autenticacion-rgpd.trz_out.rs`:

| Bug | Category | Detectable without spec? |
|-----|----------|--------------------------|
| `forbidden` silently replaced by `ignored` in one handler (+ test removed) | Semantic | With uncertainty only |
| Authentication failure transitioning to active session instead of login form | Structural | Yes — visually obvious |
| Missing `#[should_panic]` test for a `forbidden` case | Completeness gap | Partially, with low confidence |

Two review passes were run: one with only the Rust file, one with the `.trz`
as a reference alongside it.

---

## What was found

Both passes detected all three bugs. The difference was not in *what* was found
but in *how*:

**Without the spec — heuristic review:**
- Bug detection relied on semantic reasoning about what the system *should* do.
- Bug 1 (`forbidden`→`ignored`) could not be confirmed with certainty: both
  `forbidden` and `ignored` produce identical observable runtime behavior. The
  distinction only exists in the system's design contract.
- Exhaustiveness could not be guaranteed: verifying test coverage required
  mentally reconstructing the full `role × context × event` matrix from the
  code itself.
- Approximate reasoning steps: ~22. Confidence in completeness: low.

**With the spec — mechanical verification:**
- Each check became a string comparison against a known truth table.
- Bug 1 was confirmed in one comparison with zero ambiguity.
- Bug 3 (test coverage gap) was verified exhaustively in 5 comparisons —
  one per `forbidden` declared in the spec.
- Approximate reasoning steps: ~7. Confidence in completeness: high.

---

## The key finding

The experiment did not reveal a speed improvement. It revealed an
**epistemic regime shift**:

> Without a spec, an LLM reviewer produces *probabilistic* conclusions
> ("this looks like a bug"). With a spec, it produces *deductive* conclusions
> ("this is a bug").

This matters most for the class of bugs that have no visible runtime signature —
semantic violations where wrong behavior and correct behavior look identical
from the outside. These are precisely the bugs that Trenza was designed to
prevent in the first place (see `history/chronicle/2026-03-04/01-concepto-inicial.md`
for the motivating case).

---

## Relation to Principle 4

Principle 4 states: *"Trenza enforces native human-readable rules that are
mechanically auditable."*

This experiment provides empirical evidence that the spec fulfills that
promise not only for the compiler's verifier, but also for LLMs acting as
reviewers. The spec functions as a **truth table with finite, enumerable
entries** — a property that maps directly onto how LLMs reason most reliably.

---

## Open question

Does this benefit scale with system complexity, or is there a threshold where
the spec itself becomes too large to traverse?

Hypothesis: the spec scales better than the code because it is declarative —
each fact appears exactly once. The generated code implements each fact N times
(handler + test + possible documentation). As complexity grows, the code's
redundancy grows faster than the spec's size.

This hypothesis is untestable until `carrito-checkout.trz` and a more complex
multi-concurrent example are fully supported by the compiler.
