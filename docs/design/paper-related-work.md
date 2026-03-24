---
title: "Related Work — draft for ONWARD! Essays 2026"
status: draft
date: 2026-03-24
---

# Related Work

## Statecharts and Behavioral Specification

Harel's statecharts [HAREL87] extended finite automata with hierarchy,
concurrency, and broadcast communication, providing the first visual
formalism expressive enough for realistic reactive systems. UML State
Machines [OMG17] standardized a variant for industry use. Trenza inherits
the core insight — behavior as explicit state topology — but departs in
three ways: (1) the source of truth is plain text, not a diagram; (2) the
formalism is deliberately *less* expressive than statecharts (no
broadcast, no history states, no fork/join), because the restriction is
what enables mechanical verification by LLMs; (3) the role layer
separates *who* reacts from *what* state is active, a distinction absent
in statecharts.

## Role-Based Programming and DCI

The Data-Context-Interaction architecture [REENSKAUG09] separates data
objects from the roles they play in use cases. Trenza's `role` construct
is a direct descendant: a role is a typed participant within a named
context, not a property of a class. The key difference is that DCI does
not constrain which roles must appear in which contexts — that constraint
(Rule 1, Completeness) is Trenza's primary contribution to the DCI
lineage. Coplien and Bjørnvig [COPLIEN10] argued for lean architecture
grounded in roles; Trenza makes that argument executable and verifiable.

## Formal Specification Languages

TLA+ [LAMPORT02] and Alloy [JACKSON02] are the most widely used formal
specification languages in software engineering. Both can express the
properties Trenza verifies (completeness, reachability, data
conformance). The practical gap is accessibility: TLA+ requires temporal
logic fluency; Alloy requires relational modeling. Neither is designed to
be parsed or reasoned about by a large language model without
translation. Trenza's grammar fits in under 70 lines of PEG and maps
directly to concepts (context, role, event, transition) that LLMs
encounter in natural language descriptions of software. This is not a
claim of formal equivalence — it is a claim of a different design point:
formal enough to verify the properties that matter most in practice,
accessible enough that LLMs can generate, review, and critique
specifications without a translation layer.

## CASE Tools and the Diagram-Code Gap

Computer-Aided Software Engineering tools of the 1980s and 1990s
[FUGGETTA93] attempted to bridge specification and implementation through
diagram-driven generation. Their failure modes are well documented:
diagrams diverge from code, round-trip engineering is fragile, and the
toolchains become proprietary lock-in. Trenza addresses these failure
modes structurally: the source artifact is plain text under version
control; the generated code is not edited (the spec is); and the
toolchain is open-source and reproducible. The "CAD Lógico" framing
[GEMINI26] recasts CASE's goal — visual architecture driving verified
implementation — for an era in which the primary consumer of structured
text is an LLM, not a diagram renderer.

## LLMs in Software Engineering

Code generation with LLMs [CHEN21, AUSTIN21] has demonstrated that
models can produce syntactically correct and often functionally adequate
code from natural language prompts. The central reliability problem is
*hallucination*: the model generates plausible but incorrect behavior,
particularly for edge cases, state management, and access control. Prior
work on prompting strategies [WEI22] and retrieval-augmented generation
[LEWIS20] reduces but does not eliminate this problem. Trenza's approach
is orthogonal: rather than improving generation accuracy, it provides a
formal artifact that makes *verification* mechanical. The specification
acts as a truth table; review becomes a one-to-one correspondence check
rather than a plausibility judgment. Our experiment (Section 4)
documents this shift empirically.

## Hybrid Human-AI Workflows

Recent work on AI-assisted pair programming [VAITHILINGAM22] and
specification-driven development [ENDRES24] explores how LLMs and humans
can collaborate on software artifacts. Trenza contributes a specific
instance of this collaboration: the human acts as architect (authoring
or approving the `.trz`), the LLM acts as implementer and first-pass
verifier, and the compiler acts as the infallible judge. This tripartite
structure is not claimed as novel in principle — it is the classic
separation of specification, implementation, and verification — but its
instantiation with contemporary LLMs and a purpose-built DSL is, to our
knowledge, not previously documented.

---

## References (to be formatted per venue style)

- [HAREL87] Harel, D. (1987). Statecharts: A visual formalism for complex systems. *Science of Computer Programming*, 8(3), 231–274.
- [OMG17] Object Management Group. (2017). *Unified Modeling Language Specification, Version 2.5.1*.
- [REENSKAUG09] Reenskaug, T., & Coplien, J. (2009). *The DCI Architecture: A New Vision of Object-Oriented Programming*.
- [COPLIEN10] Coplien, J., & Bjørnvig, G. (2010). *Lean Architecture for Agile Software Development*. Wiley.
- [LAMPORT02] Lamport, L. (2002). *Specifying Systems: The TLA+ Language and Tools*. Addison-Wesley.
- [JACKSON02] Jackson, D. (2002). *Alloy: A Lightweight Object Modelling Notation*. TOSEM, 11(2), 256–290.
- [FUGGETTA93] Fuggetta, A. (1993). A classification of CASE technology. *IEEE Computer*, 26(12), 25–38.
- [GEMINI26] Pérez-Chirinos, C., et al. (2026). Trenza: A Role-Based State Machine DSL. *This paper*.
- [CHEN21] Chen, M., et al. (2021). Evaluating Large Language Models Trained on Code. *arXiv:2107.03374*.
- [AUSTIN21] Austin, J., et al. (2021). Program Synthesis with Large Language Models. *arXiv:2108.07732*.
- [WEI22] Wei, J., et al. (2022). Chain-of-Thought Prompting Elicits Reasoning in Large Language Models. *NeurIPS 2022*.
- [LEWIS20] Lewis, P., et al. (2020). Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks. *NeurIPS 2020*.
- [VAITHILINGAM22] Vaithilingam, P., et al. (2022). Expectation vs. Experience: Evaluating the Usability of Code Generation Tools. *CHI EA 2022*.
- [ENDRES24] Endres, M., et al. (2024). Specification-Driven Development with LLMs. *ICSE 2024*.
