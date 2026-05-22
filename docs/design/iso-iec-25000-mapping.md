# Mapping — Trenza Verification Rules ↔ ISO/IEC 25000 (SQuaRE)

**Status:** Working note for the Discussion / Evaluation section of the ONWARD!
2026 paper. Not yet incorporated into the paper draft itself.
**Date:** 2026-05-22
**Provenance:** Thread `related-work-iso-iec` between CL-Code (Opus 4.7) and GE
(Gemini 3.5 Flash) — see
`history/coordination/archive/2026-05-22/2026-05-21T20-52_review-protocol-v0_GE_2.md`
and subsequent messages.

---

## Purpose

Trenza's eight static verification rules are not arbitrary linter checks; each
corresponds to a quality characteristic in the ISO/IEC 25000 (SQuaRE) family,
particularly ISO/IEC 25010 (Systems and software quality models) and its AI
extension ISO/IEC TS 25059. This document makes the correspondence explicit
so that the paper can argue, defensibly, that a Trenza-compiled artefact
satisfies a bounded but well-identified subset of SQuaRE quality criteria by
construction.

The mapping was agreed jointly. CL-Code's contribution was the original
proposal of bullet correspondences; GE refined them; CL-Code raised a static
vs runtime objection on Rules 3 and 4; GE accepted the refinement. The
version below is the agreed version.

---

## Mapping Table

| Trenza Rule | ISO/IEC 25010 / 25059 Criterion | Argument |
|---|---|---|
| **Rule 1: Completeness** | Functional Completeness (ISO/IEC 25010 §4.2.1) | Every event in every state is handled explicitly (or marked ignored / forbidden) per role, eliminating functional gaps at runtime. |
| **Rule 2: Determinism** | Functional Correctness / Predictability (ISO/IEC 25010; TS 25059) | Every (role, event, context) triple resolves to exactly one action, eliminating behavioural ambiguity in the product. |
| **Rule 3: Reachability** | Functional Appropriateness (ISO/IEC 25010 §4.2.3) | All defined functions are statically reachable; "dead code" in the specification is rejected at compile time, maximising functional appropriateness. |
| **Rule 4: Return / no sink states** | Reliability / Fault Tolerance (ISO/IEC 25010 §4.5.3) — *by construction* | Sink states (terminal traps) are statically forbidden; the system cannot fall into an unintended absorbing state. This is a static guarantee of a property normally assessed at runtime. |
| **Rule 5: Role Exhaustiveness** | Interface / Architectural Consistency (ISO/IEC 25010 Maintainability) | All declared roles must participate in the *interaction* of each context, preventing orphan or inactive components and enforcing structural consistency. |
| **Rule 6: Data Conformance** | Data Confidentiality & Integrity (ISO/IEC 25012; ISO/IEC 25010 Security) | Flows of classified data (e.g., `personal`) to unauthorised externals are statically rejected, supporting GDPR compliance and confidentiality / integrity by design. |
| **Rule 7: Slot / Fills Integrity** | Modularity / Functional Cohesion (ISO/IEC 25010 Maintainability) | The dynamic composition of overlays and concurrent contexts is verified to be collision-free; slot fills are uniquely resolved. |
| **Rule 8: Role Type Consistency** | Reliability / Robustness (ISO/IEC 25010; TS 25059) | A given role carries the same data type across all contexts, preventing type-conversion errors in the generated Strand 1 code. |

---

## Notes on the Static / Runtime Distinction

Several entries (Rules 3, 4, 8) map to quality characteristics that ISO/IEC
25010 ordinarily assesses through *runtime* observation. The qualifier "by
construction" is essential: Trenza does not measure these properties on a
running system; it eliminates their negation at compile time. This is
methodologically stronger than runtime measurement when the static analysis is
sound, but it is also bounded — only the properties expressible in Trenza's
semantics can be guaranteed this way.

The paper should be explicit that Trenza's claim is *bounded compliance by
construction*, not full SQuaRE conformance. The mapping demonstrates that the
properties Trenza *does* guarantee are recognisable SQuaRE-family
characteristics, not idiosyncratic linter checks.

---

## Suggested Placement in the Paper

- **Primary location:** Discussion (Section 6 or 7, per the current ONWARD!
  structure), as a subsection titled something like *"Trenza in the SQuaRE
  Frame"* or *"Quality Characteristics by Construction"*.
- **Forward reference:** From the Introduction or from the related-work
  Section F entries on ISO/IEC 25010, 25059, and Oviedo et al. 2024.
- **Backward reference:** From the rule descriptions (Section 3 or 4 of the
  paper, depending on final structure) to this mapping, as a footnote.

---

## Open Questions

1. Should we cite TS 25059 attributes explicitly (controllability,
   transparency) for any rule, or keep the mapping anchored to 25010 alone?
   *Tentative answer:* anchor to 25010 for clarity; mention 25059 attributes
   only in the surrounding prose where AI-specific characteristics are
   directly relevant (Strand 4 audit ↔ transparency; Rule 6 ↔ privacy).
2. Should the mapping appear as a table (as here) or as a discussion of each
   correspondence in prose? *Tentative answer:* keep the table; surround it
   with one paragraph of prose per rule cluster.
3. Are there quality characteristics in ISO/IEC 25010 / 25059 that Trenza
   *deliberately does not* address (e.g., Performance Efficiency, Usability)?
   Acknowledging these limits strengthens the bounded-compliance claim.
   *Tentative answer:* yes, add a short "what Trenza does not claim" paragraph.

---

*Document prepared 2026-05-22 by CL-Code (Opus 4.7) following thread
`related-work-iso-iec` with GE (Gemini 3.5 Flash). Provenance for each
row available in the archived buzón messages.*
