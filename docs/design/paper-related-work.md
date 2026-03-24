---
title: "Related Work — Trenza ONWARD! 2026"
status: draft
date: 2026-03-24
---

# Related Work

## Statecharts

Harel's statecharts [HAREL87] introduced hierarchical, concurrent finite
state machines as a visual formalism for reactive systems, providing the
first notation expressive enough to describe realistic event-driven
behavior without combinatorial explosion. The influence on Trenza is
direct: contexts and transitions are semantically grounded in the same
intuition that state topology is the right primitive for behavioral
specification. The departure is equally direct. Statecharts are a
diagrammatic notation; their primary artifact is a drawing, not a
text file. This makes them incompatible with version control workflows,
diff-based review, and consumption by language models. Trenza inverts
the artifact hierarchy: the `.trz` specification is the source of truth,
stored as plain text under Git, and the Mermaid statechart is a derived
projection synthesized from it at compile time. The diagram serves the
spec, not the other way around. Statecharts also make no provision for
role-based access control, data scope restrictions, GDPR conformance, or
multi-strand synthesis. They describe what transitions are valid; Trenza
additionally enforces who may trigger them, on what data, and with what
mechanically verifiable obligations.

## Data, Context, and Interaction (DCI)

Reenskaug and Coplien's Data-Context-Interaction architecture [DCI09]
elevated roles to first-class citizens by separating the stable structure
of domain objects (Data) from situational use-case logic (Context) and
dynamic execution paths (Interaction). This separation addresses the
central tension in object-oriented design: domain models that are clean
in isolation become entangled when they must support behavioral use cases,
producing the mixed-concern objects and scattered state flags that
motivated Trenza's original design. Trenza inherits DCI's core insight
directly: a datum is inert until it assumes a transient role within a
named context; outside that context, the role and its permissions do not
exist. Where Trenza departs from DCI is in formalizing this insight as a
set of mechanically enforced rules. DCI is an architectural philosophy;
it imposes no compiler-checkable constraints on which roles must appear
in which contexts or how data access must be scoped. Trenza does: Rule 1
(Completeness) requires every role-event pair to be handled in every
context; Rule 2 (Determinism) prohibits handler duplication; the data
scoping rules enforce least privilege structurally. DCI provides the
conceptual model. Trenza makes that model auditable.

## CASE Tools and Model-Driven Engineering

Computer-Aided Software Engineering tools of the 1980s and 1990s, and
their successors in the UML and Model-Driven Architecture era, pursued
the same fundamental goal as Trenza: elevate specification above
implementation and generate code from formal models [FUGGETTA93]. This
project failed in practice for compounding reasons. Modeling notations
grew to accommodate every possible use case — the full UML suite runs to
thirteen diagram types — producing artifact ecosystems too large for any
individual practitioner to manage coherently. The artifacts themselves
lived in proprietary binary formats that were incompatible with version
control and impossible to review in a pull request. Code generation was
partial: the toolchains generated skeletal implementations that developers
were then expected to hand-edit, breaking the round-trip guarantee and
leaving models perpetually out of sync with production code. Most
consequentially for the present work, these models were designed to be
read by humans pointing at a GUI. They were not designed to be parsed,
generated, or critiqued by a language model. Trenza is designed for the
opposite constraints: a single plain-text file per specification, stored
in Git, structured so that every semantic contract appears exactly once,
and processable by a compiler and an LLM with equal facility.

## TLA+ and Alloy

TLA+ [LAMPORT02] and Alloy [JACKSON02] are the most capable lightweight
formal methods available for software specification. TLA+ can express and
model-check liveness and safety properties of distributed systems; Alloy
can find counterexamples to relational invariants across bounded domains.
The power of both tools is not in question. The constraint is access.
Writing a TLA+ specification requires comfort with temporal logic and the
PlusCal notation; writing an Alloy model requires fluency with relational
algebra. Neither maps naturally onto the vocabulary of application
developers working on business systems, and neither was designed with LLM
consumption as a design goal. A language model can read TLA+ but cannot
reliably determine whether a given Rust function satisfies a liveness
property without a translation layer that itself requires verification.
Trenza occupies a different point in the design space. It sacrifices the
expressive power of full temporal logic in exchange for a constraint set
that is simultaneously enforceable by a fast compiler — eight
verification rules, sub-100ms on a 16-module reference system — and
traversable by an LLM as a finite truth table. The verification question
in Trenza is not "does this system satisfy a temporal property under all
interleavings?" but "does this implementation correspond, entry by entry,
to what the specification declares?" That is a weaker question. It is
also one that a language model answers deductively rather than
probabilistically, which is the operative distinction in a collaborative
human-AI workflow.

## LLMs for Code Generation

GitHub Copilot, ChatGPT, and their successors have demonstrated that
large language models can produce syntactically correct and often
functionally adequate code from natural language prompts [CHEN21,
AUSTIN21]. The limitation is not generation quality per se but
verifiability. Generated code is probabilistically coherent: it looks
like code that should work. It carries no formal contract binding its
behavior to an explicit specification. A reviewer — human or machine —
must reconstruct the intended semantics from the implementation itself,
which is inherently heuristic and incomplete. Prompting strategies
[WEI22] and retrieval-augmented generation [LEWIS20] reduce generation
errors but do not address this fundamental gap. Trenza addresses it
structurally: the `.trz` specification is the contract, authored before
any implementation exists, and the generated Rust is a derived artifact
that must correspond to it mechanically. Empirical evidence for the
resulting difference in review quality is reported in Section 4: without
a Trenza specification, LLM-assisted review required approximately 22
reasoning steps and produced probabilistic conclusions; with the
specification, it required approximately 7 steps and produced deductive
ones. The relevant finding is not a speed improvement. It is an epistemic
regime shift — from "this looks correct" to "this corresponds" — that
changes the nature of the guarantee an LLM-assisted review can provide.
Trenza is not a competitor to LLM-based code generation; it is the formal
substrate that makes LLM-assisted verification reliable.

## DSLs for Reactive and State-Managed Systems

The Elm architecture [ELM] and its descendants — including Lustre for the
Gleam ecosystem and similar typed reactive frameworks — demonstrate that
a sufficiently restrictive type system, combined with a canonical state
management pattern, can eliminate entire classes of runtime errors. The
design philosophy is shared with Trenza: the right constraints, imposed
early, reduce defect surface area more effectively than post-hoc testing
or static analysis applied to an unconstrained language. These systems
restrict the programmer's expressive freedom in exchange for guarantees
about state updates and side effects. The difference lies in scope and
synthesis target. Elm and Lustre are single-paradigm, single-target
systems: they produce one implementation, for one execution environment,
from one model. Their specification and implementation are the same
artifact. Trenza separates them explicitly: the `.trz` file is the
specification; the generated code is one of several simultaneous
projections. The same specification produces a Rust single-threaded
runtime for browser environments, an `mpsc`-driven actor model for
high-performance backends, algebraically tested unit tests, and
topological Mermaid diagrams. The braid is not a front-end discipline
imposed on a single target; it is a behavioral contract from which
multiple coherent implementations are derived without divergence. The
completeness, determinism, and reachability guarantees hold across all
targets simultaneously, enforced at the specification level before any
target-specific code is generated.

---

## References (to be formatted per venue style)

- [HAREL87] Harel, D. (1987). Statecharts: A visual formalism for complex systems. *Science of Computer Programming*, 8(3), 231–274.
- [DCI09] Reenskaug, T., & Coplien, J. (2009). *The DCI Architecture: A New Vision of Object-Oriented Programming*. Artima.
- [FUGGETTA93] Fuggetta, A. (1993). A classification of CASE technology. *IEEE Computer*, 26(12), 25–38.
- [LAMPORT02] Lamport, L. (2002). *Specifying Systems: The TLA+ Language and Tools*. Addison-Wesley.
- [JACKSON02] Jackson, D. (2002). Alloy: A lightweight object modelling notation. *ACM TOSEM*, 11(2), 256–290.
- [CHEN21] Chen, M., et al. (2021). Evaluating Large Language Models Trained on Code. *arXiv:2107.03374*.
- [AUSTIN21] Austin, J., et al. (2021). Program Synthesis with Large Language Models. *arXiv:2108.07732*.
- [WEI22] Wei, J., et al. (2022). Chain-of-Thought Prompting Elicits Reasoning in Large Language Models. *NeurIPS 2022*.
- [LEWIS20] Lewis, P., et al. (2020). Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks. *NeurIPS 2020*.
- [ELM] Czaplicki, E. (2012). Elm: Concurrent FRP for Functional GUIs. Senior thesis, Harvard University.
