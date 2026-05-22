# Paper prose — Discussion subsection: SQuaRE-by-Construction

**Status:** Draft ready for incorporation into the ONWARD! paper Discussion section.
**Word count:** 315 (excluding table).
**Provenance:** Thread `discussion-prose-iso-iec` (Trenza Mailbox v0).
- seq-1 (CL-Code): task spec.
- seq-2 (GE): initial draft, 268 words.
- seq-3 (CL-Code): corrections C1 (Rule 9 name), C2 (table anchoring) + three minor tweaks.
- seq-4 (GE, `closes: true`): final draft with all corrections applied.

Archived under `history/coordination/archive/2026-05-22/`.

---

## SQuaRE-by-Construction: Bounded Compliance

To demonstrate that the static verification rules of Trenza are not merely ad-hoc syntactic checks, we map them directly to the ISO/IEC 25000 (SQuaRE) quality model, specifically ISO/IEC 25010 and its AI-specific extension ISO/IEC TS 25059. By formalising role-based interactions as state machines, the Trenza compiler ensures a form of bounded *compliance by construction*. Rather than measuring software quality attributes at runtime, the compiler statically eliminates their negation during compilation.

| Trenza Rule | SQuaRE Characteristic | Standard |
| :--- | :--- | :--- |
| **R1**: Completeness | Functional Completeness | ISO/IEC 25010 |
| **R2**: Determinism | Functional Correctness | ISO/IEC 25010 |
| **R3**: Reachability | Functional Appropriateness | ISO/IEC 25010 |
| **R4**: Return Guarantee | Reliability / Fault Tolerance | ISO/IEC 25010 |
| **R5**: Role Exhaustiveness | Modularity / Cohesion | ISO/IEC 25010 |
| **R6**: Data Conformance | Data Confidentiality / Integrity | ISO/IEC 25012 |
| **R7**: Slot/Fills Integrity | Modularity / Cohesion | ISO/IEC 25010 |
| **R8**: Role Type Consistency | Reliability / Robustness | ISO/IEC 25010 |
| **R9**: Immediate Transition Acyclicity | Reliability / Maturity | ISO/IEC 25010 |

We categorise the mapping into three core quality clusters. First, *Functional Suitability* (Rules 1–3) guarantees that interaction protocols are complete, deterministic, and free of unreachable states. Second, *Reliability and Robustness* (Rules 4, 8, and 9) converts runtime failures, such as terminal traps (sink states) and livelocks (entry/exit cycles), into compile-time errors. Where applicable, these guarantees extend the SQuaRE quality model to AI-augmented systems as specified in ISO/IEC TS 25059. Third, *Maintainability and Security* (Rules 5, 6, and 7) prevents architectural inconsistencies, data leaks across roles, and compositional overlay conflicts. This cluster spans two SQuaRE characteristics—Maintainability for structural cohesion (Rules 5, 7) and Security for data flow (Rule 6)—unified here by their structural nature.

Crucially, Trenza does not claim universal SQuaRE compliance. Several quality characteristics defined in ISO/IEC 25010—namely *Performance Efficiency*, *Usability*, *Compatibility*, and *Portability*—are deliberately excluded. These factors depend on runtime platform behaviours and target language runtimes, which lie outside the scope of Trenza's static structural compiler. Acknowledging these boundaries is essential for the credibility of our static verification model.

This structured mapping positions Trenza within the AI-engineering quality tradition surveyed by Oviedo et al. (2024) and the SQuaRE evaluation practice documented by Rodriguez et al. (2021). While these existing approaches focus on post-hoc assessment workflows, the Trenza verifier introduces formal, static evidence directly into the compilation pipeline.
