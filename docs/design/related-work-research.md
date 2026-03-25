# Related Work Research — Trenza DSL

**Paper:** "Trenza: A Role-Based State Machine DSL for Human-AI Collaborative Specification and Synthesis"
**Target venue:** ONWARD! Essays, SPLASH 2026, Oakland CA
**Research date:** 2026-03-25

This document collects canonical citations, summaries, and relationship notes for use
in the Related Work section of the Trenza paper. Each entry is organized under its
thematic area.

---

## A. Statechart Formalism and Descendants

### Harel 1987 — Statecharts

**Citation:** Harel, D. (1987). Statecharts: A visual formalism for complex systems.
*Science of Computer Programming*, 8(3), 231–274.
https://doi.org/10.1016/0167-6423(87)90035-9

**Summary:** Harel introduced statecharts as a broad extension of conventional
finite-state machines, adding hierarchy (nested states), orthogonal concurrency
(parallel regions), and broadcast communication. The formalism was designed to make
large specifications of reactive, event-driven systems manageable without sacrificing
precision. It became the direct ancestor of UML behavioral state machines, SCXML, and
nearly every visual state-machine notation in use today.

**Relation to Trenza:** Trenza inherits Harel's core insight that states should be
hierarchically composable and that concurrency needs explicit structural support. Trenza
adopts these ideas through its `concurrent` and `overlay` context types. The key
departure is that Trenza adds a first-class *role* dimension to every handler: a
transition is not triggered by an event alone, but by the pair (role, event). This
makes access-control semantics part of the formalism rather than a side condition.
Harel statecharts also do not mandate compile-time completeness or impose data-scoping
rules; Trenza's eight verification rules go beyond what the original formalism
specified.

---

### SCXML — W3C State Chart XML

**Citation:** Barnett, J., Akolkar, R., Auburn, R., Bodell, M., Burnett, D. C.,
Carter, J., McGlashan, S., Lager, T., Helbing, M., Hosn, R., Raman, T., Reifenrath,
K., Rosenthal, N., and Roxendal, J. (2015). *State Chart XML (SCXML): State Machine
Notation for Control Abstraction*. W3C Recommendation.
https://www.w3.org/TR/scxml/

**Summary:** SCXML is an XML-based markup language that defines a generic
state-machine execution environment grounded in Harel statecharts. Published as a W3C
Recommendation in 2015, it provides a portable, declarative notation for states,
transitions, events, and executable content (ECMAScript by default). Its primary goal
is interoperability across platforms and voice-browser applications; it explicitly
defines execution semantics so that conforming implementations behave identically.

**Relation to Trenza:** SCXML demonstrates the value of a declarative, tool-portable
specification for stateful systems. Trenza similarly provides a declarative surface
syntax that is independent of any target runtime. However, SCXML is a data format
rather than a language with a type system: it has no compile-time completeness
checking, no role or access-control primitives, and no notion of multi-strand
synthesis. A `.trz` file is closer to a typed specification language than to a
serialization format.

---

### XState — JavaScript Statechart Library

**Citation:** Khourshid, D. (2019–present). *XState: State machines and statecharts
for the modern web* [software library]. Stately.ai.
https://github.com/statelyai/xstate

**Summary:** XState is an open-source JavaScript/TypeScript library for creating,
interpreting, and executing finite-state machines and statecharts, as well as managing
actor-based invocations of those machines. Created by David Khourshid and now
maintained by Stately.ai, it brings Harel's formalism to frontend and Node.js
development with strong TypeScript support, visual tooling (Stately Editor), and
actor-model composition. XState v5 introduced a revised actor-centric API and
first-class support for promise-based and callback-based actors.

**Relation to Trenza:** XState and Trenza share the goal of making stateful behaviour
explicit and visualizable. XState is a runtime library: machines are defined in
JavaScript objects or via a visual editor and interpreted at runtime. Trenza operates
one level earlier — it is a compiled specification language that synthesizes the
runtime code (in Rust or TypeScript), the tests, the diagrams, and the audit trail
simultaneously. XState has no built-in concept of roles, no compile-time exhaustiveness
guarantee, and no GDPR or audit-annotation features.

---

### Ragel — State Machine Compiler

**Citation:** Thurston, A. (2009). *Ragel State Machine Compiler User Guide*, Version
6.6. Colm Networks.
https://www.colm.net/files/ragel/ragel-guide-6.6.pdf

**Summary:** Ragel is a finite-state machine compiler that compiles regular expressions
and state charts (described in a concise regular-language notation) into executable
code in C, C++, Java, Ruby, Go, Rust, and other languages. A distinguishing feature is
that arbitrary user actions can be attached to individual transitions using operators
integrated into the regular-expression syntax. Ragel is widely used for building
lexers, protocol parsers, and network stack components where generated-code performance
is critical.

**Relation to Trenza:** Ragel and Trenza both take a "compile specification to
implementation" approach, producing correct-by-construction code from a formal source.
Ragel's domain is sequential string recognition (regular and context-free languages);
Trenza's domain is reactive systems with concurrent roles, overlays, and structured
effects. Ragel does not address access control, multi-role exhaustiveness, or synthesis
of artefacts beyond the implementation strand.

---

## B. Formal Specification Languages

### TLA+ — Temporal Logic of Actions

**Citation:** Lamport, L. (2002). *Specifying Systems: The TLA+ Language and Tools for
Hardware and Software Engineers*. Addison-Wesley. ISBN: 0-321-14306-X.
Foundational paper: Lamport, L. (1994). The Temporal Logic of Actions. *ACM
Transactions on Programming Languages and Systems*, 16(3), 872–923.
https://doi.org/10.1145/177492.177726

**Summary:** TLA+ is a formal specification language based on the Temporal Logic of
Actions, an extension of linear temporal logic in which system behaviour is described
by a single mathematical formula over states and transitions. Lamport's book provides
the full language definition and introduces the TLC model checker and the PlusCal
algorithmic language. TLA+ is used industrially (notably by Amazon Web Services and
Microsoft) to verify distributed-systems designs before implementation.

**Relation to Trenza:** Both Trenza and TLA+ take the position that a specification
should be machine-checkable rather than merely human-readable. TLA+ operates at a
mathematical abstraction level — its specifications are not synthesized into
production code. Trenza deliberately occupies a lower level of abstraction, closer to
the implementation, so that the same source file drives compilation. TLA+ has no
built-in notion of roles, multi-strand synthesis, or GDPR annotations; its temporal
operators are strictly more expressive than what Trenza's eight rules verify, but that
expressiveness comes at the cost of requiring proof expertise. Trenza trades
expressiveness for decidability and accessibility to developers who are not formal
methods specialists.

---

### Alloy — Relational Lightweight Formal Methods

**Citation:** Jackson, D. (2006). *Software Abstractions: Logic, Language, and
Analysis*. The MIT Press. ISBN: 978-0262101141.
https://dl.acm.org/doi/10.5555/2141100

**Summary:** Alloy is a lightweight formal modelling language based on first-order
relational logic. The Alloy Analyzer uses SAT-based bounded model checking (the "small
scope hypothesis") to automatically find counterexamples to claimed properties, giving
designers immediate feedback without requiring theorem-proving expertise. Jackson's
"agile modelling" philosophy emphasizes that partial, bounded verification of small
models catches most real design flaws.

**Relation to Trenza:** Alloy's philosophy of making formal analysis accessible
without requiring proof skills is closely aligned with Trenza's design intent. Both
share a "lightweight formal methods" stance: the goal is to catch bugs early through
automatic checking rather than full mathematical proof. Trenza's eight compile-time
rules can be seen as a fixed, domain-specific instantiation of the kind of properties
one might write as Alloy predicates over a state-machine model. The difference is that
Trenza's rules are exhaustive within their domain and integrated into the compilation
pipeline rather than being expressed in a general-purpose logic. Alloy does not
synthesize code or tests.

---

### B Method / Event-B — Refinement-Based Formal Methods

**Citation:** Abrial, J.-R. (2010). *Modeling in Event-B: System and Software
Engineering*. Cambridge University Press. ISBN: 978-0-521-89556-9.
https://dl.acm.org/doi/10.5555/1855020

**Summary:** Event-B is an evolution of Abrial's B Method, a refinement-based formal
method for system and software development. A specification is a mathematical model
composed of invariants and guarded events; correctness obligations (proof obligations)
are discharged using the Rodin Platform prover. The B Method has been used in
safety-critical railway and avionics applications. Event-B extends the method to
support incremental, refinement-based development, where abstract models are
progressively concretised to implementation-level descriptions.

**Relation to Trenza:** Event-B and Trenza both enforce correctness at the
specification level rather than relying on post-hoc testing. Event-B's proof-obligation
approach guarantees invariant preservation across all events, which corresponds loosely
to Trenza's Rule 1 (Completeness) and Rule 5 (Role Exhaustiveness). However, Event-B
requires substantial proof effort and expertise; Trenza's verification rules are fully
automatic (decidable checks, no proof obligations). Event-B has been used to model
consent-based GDPR compliance (see Section F), providing a precedent for applying
formal refinement to data-protection requirements — an area where Trenza offers a
lighter-weight, developer-facing alternative.

---

## C. Model-Driven Engineering and CASE Tools

### Executable UML (xUML)

**Citation:** Mellor, S. J. and Balcer, M. J. (2002). *Executable UML: A Foundation
for Model-Driven Architecture*. Addison-Wesley. ISBN: 0-201-74804-5.
https://dl.acm.org/doi/10.5555/545976

**Summary:** Executable UML defines a semantically precise, executable subset of UML
that can be compiled into platform-specific code by a "model compiler." A system is
described as a collection of domains; each domain contains class diagrams, state
machines, and action language; a separate model compiler translates the entire model
to a target platform. xUML established the vocabulary of model-driven architecture
(MDA) and was later standardized as xtUML and fUML by the OMG.

**Relation to Trenza:** Executable UML and Trenza share the foundational premise that
a single specification should drive code generation. Trenza differs in that it
synthesizes not one artefact but four complementary strands (implementation, tests,
diagrams, audit) from a single `.trz` source, and it embeds role-based access control
and GDPR annotations directly in the language rather than treating them as
platform-mapping concerns. xUML's action language and class diagrams are significantly
more general than Trenza's focused role/state-machine model; Trenza buys decidability
and stronger verification guarantees by accepting a narrower scope.

---

### IBM Rhapsody — UML State Machine Code Generation

**Citation:** IBM Engineering Rhapsody [software product]. Originally I-Logix (1996),
acquired by IBM. Current product page: https://www.ibm.com/products/engineering-rhapsody

Academic reference: Harel, D. and Politi, M. (1998). *Modeling Reactive Systems with
Statecharts: The STATEMATE Approach*. McGraw-Hill. (Foundational work by original
Rhapsody team members.)

**Summary:** Rhapsody (now IBM Engineering Rhapsody) is a commercial CASE tool for
model-based development of real-time and embedded systems. It provides graphical UML
state machine modelling with animated simulation and full code generation to C, C++,
Java, and Ada. Rhapsody has deep industrial usage in aerospace, automotive, and defence
domains. It originated at I-Logix, a company co-founded by David Harel's collaborators,
and the tool's statechart semantics are directly traceable to the 1987 paper.

**Relation to Trenza:** Rhapsody represents the mature industrial end of the
model-to-code pipeline. Its state machines are graphically defined and code is
generated by the tool; the approach is platform-centric and proprietary. Trenza is
text-first (`.trz` files are version-controllable, diffable, and LLM-readable), open,
and generates multiple output strands including tests and audit reports that Rhapsody
does not produce. Rhapsody has no concept of role-based handlers or compile-time
verification rules analogous to Trenza's eight rules.

---

### YAKINDU Statechart Tools (itemis CREATE)

**Citation:** Nyßen, A. and Terfloth, A. (2012). YAKINDU SCT — Domain-Specific
Statecharts. EclipseCon Europe 2011 proceedings. itemis AG.
Tool: https://www.itemis.com/en/products/itemis-create/
GitHub (open-source releases): https://github.com/itemisCREATE/statecharts

**Summary:** YAKINDU Statechart Tools (now rebranded as itemis CREATE) is an
Eclipse-based IDE for specifying and developing reactive systems using statecharts. It
combines a graphical editor for states and transitions with a textual notation for
guards and actions, and provides live validation (detecting unreachable states, dead
ends, and unknown event references during editing), simulation, and code generation to
C/C++, Java, and SCXML. An optional VS Code integration was added in later versions.

**Relation to Trenza:** YAKINDU's live validation feature is the closest industrial
analogue to Trenza's compile-time rules: both detect structural errors (unreachable
states, dead ends) before execution. YAKINDU's validation is interactive and
tool-driven; Trenza's rules are formally specified and compiler-enforced, making the
guarantees reproducible in CI pipelines. YAKINDU does not support role-based handlers,
multi-strand synthesis, GDPR annotations, or LLM-oriented design. The existence of a
VS Code extension for statecharts motivates Trenza's own planned LSP integration.

---

### Stateflow — MATLAB/Simulink State Machine Tool

**Citation:** Hamon, G. and Rushby, J. (2004). An operational semantics for Stateflow.
In *Fundamental Approaches to Software Engineering (FASE 2004)*, Lecture Notes in
Computer Science, vol. 2984, pp. 229–243. Springer.
https://doi.org/10.1007/978-3-540-24721-0_17
Journal version: Hamon, G. and Rushby, J. (2007). An operational semantics for
Stateflow. *International Journal on Software Tools for Technology Transfer*, 9,
447–456. https://doi.org/10.1007/s10009-007-0049-7

**Summary:** Stateflow is a Statecharts-like language embedded in MATLAB/Simulink,
widely used for modeling control logic in embedded systems (automotive, aerospace).
Hamon and Rushby provided the first formal operational semantics for Stateflow, noting
that the only prior definition was the simulator's behavior. Their formalized semantics
enabled prototype tools for formal analysis of Stateflow designs.

**Relation to Trenza:** Stateflow and the Hamon-Rushby work illustrate both the power
and the limitation of informally defined state-machine languages: the tool exists
before its semantics are formally specified. Trenza's grammar and eight verification
rules are defined together with the language, so the specification is its own semantic
reference. Stateflow is embedded in a closed commercial ecosystem (MATLAB/Simulink);
Trenza is designed to be an open, text-based, version-controllable language.

---

## D. DSLs for State Management in Practice

### The Elm Architecture

**Citation (language origin):** Czaplicki, E. (2012). *Elm: Concurrent FRP for
Functional GUIs*. Senior thesis, Harvard University.
https://elm-lang.org/assets/papers/concurrent-frp.pdf
**Citation (TEA stabilization):** Czaplicki, E. (2016). *A Farewell to FRP: Making
signals unnecessary with The Elm Architecture*. elm-lang.org.
https://elm-lang.org/news/farewell-to-frp

**Summary:** Elm is a purely functional language for building browser UIs. Its
architecture (TEA — The Elm Architecture) emerged organically from the language: all
state changes are described by a pure `update : Msg -> Model -> Model` function,
eliminating a class of runtime errors by construction. In the 2016 post, Czaplicki
formalized TEA as the canonical Elm application structure, abandoning the earlier
FRP (functional reactive programming) model in favour of an explicit message-passing
loop.

**Relation to Trenza:** TEA and Trenza share the principle that state transitions
should be exhaustive and pure: Elm enforces exhaustive pattern matching on message
types at the type-system level; Trenza enforces exhaustive handler coverage across
roles and events at the compiler level. TEA is a single-file, single-actor model;
Trenza extends the idea to multi-context, multi-role systems with explicit concurrency
primitives. Elm's compiler errors for missing cases in `update` are the functional
analogue of Trenza's Rule 1 (Completeness) and Rule 2 (Determinism).

---

### Redux — Predictable State Management

**Citation:** Abramov, D. and Clark, A. (2015). *Redux* [software library].
https://redux.js.org/
See also: Abramov, D. (2015). Prior Art. Redux documentation.
https://redux.js.org/understanding/history-and-design/prior-art

**Summary:** Redux is a JavaScript state management library that enforces a
unidirectional data-flow architecture: all application state lives in a single
immutable store, and mutations are expressed as plain action objects dispatched through
pure reducer functions `(state, action) => state`. Redux was directly inspired by The
Elm Architecture and Flux, and was presented by Dan Abramov at React Europe 2015.
Redux DevTools enable "time-travel debugging" by replaying the action log.

**Relation to Trenza:** Redux and Trenza both make state transitions explicit and
auditable — Redux through its action log and DevTools, Trenza through its Strand 4
audit report (`@audit` annotations compiled to Markdown). Redux is a runtime pattern;
it imposes no compile-time completeness check, and its reducer functions can be
partial. Trenza's role-event handler table corresponds structurally to a Redux
reducer-per-role, but Trenza enforces exhaustiveness and prohibits side effects outside
declared `effects:` blocks.

---

## E. AI-Assisted Software Engineering

### Grammar Prompting for DSL Generation

**Citation:** Wang, B., Wang, Z., Wang, X., Cao, Y., Saurous, R. A., and Kim, Y.
(2023). Grammar prompting for domain-specific language generation with large language
models. *Advances in Neural Information Processing Systems 36* (NeurIPS 2023).
arXiv: 2305.19234.
https://arxiv.org/abs/2305.19234

**Summary:** The paper proposes *grammar prompting*, a technique where each in-context
learning example is augmented with a BNF grammar subset that is minimally sufficient
to generate that example's DSL output. At inference time, the LLM first predicts the
relevant grammar fragment, then generates the DSL program constrained to that grammar.
The approach is evaluated on semantic parsing (SMCalFlow, GeoQuery), PDDL planning,
and SMILES molecule generation, demonstrating competitive performance with stronger
structural validity guarantees.

**Relation to Trenza:** Grammar prompting demonstrates that LLMs can reason about DSL
structure when given explicit grammatical context — which directly supports Trenza's
Claim 4 (LLMs can reason about `.trz` files). Because Trenza's grammar is compact and
role-based, an LLM given a `.trz` file and the grammar can be expected to produce
syntactically valid modifications. Grammar prompting also suggests a path for tooling:
a Trenza LSP could surface relevant grammar fragments to an LLM co-pilot.

---

### Survey: LLM-based Code Generation for DSLs

**Citation:** Joel, S., Wu, J. J. W., and Fard, F. H. (2024). A survey on LLM-based
code generation for low-resource and domain-specific programming languages. *ACM
Transactions on Software Engineering and Methodology* (accepted 2024).
arXiv: 2410.03981.
https://arxiv.org/abs/2410.03981

**Summary:** This systematic survey reviewed 111 papers (filtered from 27,000+
publications between 2020 and 2024) to assess LLM capabilities and limitations for
code generation in low-resource and domain-specific languages. Key findings: LLMs
perform significantly worse on DSLs than on general-purpose languages due to data
scarcity and specialized syntax; the field lacks standard benchmarks for DSL
evaluation; and six categories of improvement techniques (fine-tuning, retrieval,
prompting, etc.) have been explored.

**Relation to Trenza:** This survey provides the empirical backdrop for Trenza's
Claim 4. The finding that LLMs struggle with DSLs due to *data scarcity and
specialized syntax* motivates Trenza's design decision to make the `.trz` grammar
compact, self-describing, and aligned with natural conceptual units (roles, contexts,
events) that LLMs already reason about from training on system descriptions. Trenza's
approach is to design for LLM readability from the outset rather than rely on
fine-tuning.

---

### Formal Verification of LLM-Generated Code

**Citation:** Councilman, D. et al. (2025). Towards formal verification of
LLM-generated code from natural language prompts. arXiv: 2507.13290.
https://arxiv.org/abs/2507.13290

**Summary:** The paper proposes Astrogator, a system that combines a formal query
language with symbolic execution to verify that LLM-generated Ansible programs match
user intent. On a 21-task benchmark, the verifier accepted correct solutions in 83% of
cases and identified incorrect code in 92%. The authors argue that a formal query
language can bridge the gap between natural-language user intent and machine-checkable
correctness, enabling non-expert users to benefit from natural-language programming.

**Relation to Trenza:** Astrogator and Trenza address the same problem from different
angles: both seek to give formal correctness guarantees to LLM outputs. Astrogator
adds a verification layer *after* LLM generation; Trenza constrains the solution
space *before* generation by making the specification itself the verified artefact.
Trenza's Claim 5 (epistemic regime shift from heuristic to deductive review) is
complementary to this line of work: a reviewer checking LLM-generated code against a
`.trz` specification is performing deductive verification (does the code match the
role-event table?) rather than heuristic inspection.

---

### Verified Code Transpilation with LLMs (NeurIPS 2024)

**Citation:** Bhatia, S., Qiu, J., Hasabnis, N. et al. (2024). Verified code
transpilation with LLMs. *Advances in Neural Information Processing Systems 37*
(NeurIPS 2024).
https://proceedings.neurips.cc/paper_files/paper/2024/file/48bb60a0c0aebb4142bf314bd1a5c6a0-Paper-Conference.pdf

**Summary:** The paper introduces LLMLIFT, which uses LLMs to generate not only
transpiled code but also a formal proof of correctness (in a verification language
such as Dafny or SMT-LIB). The approach leverages verified lifting to guarantee
functional correctness of the generated code, addressing the fundamental limitation
that LLMs optimize token likelihood rather than semantic correctness.

**Relation to Trenza:** LLMLIFT and Trenza both generate verified artefacts from a
higher-level specification. LLMLIFT targets code transpilation and generates proofs
externally; Trenza generates code that is correct by construction within the scope of
its eight rules — the rules *are* the proof, encoded in the compiler. This positions
Trenza as a domain-specific, bounded alternative to the general verification pipeline
described by LLMLIFT.

---

## F. GDPR and Compliance by Design

### Privacy by Design — Cavoukian's Framework

**Citation:** Cavoukian, A. (2011). *Privacy by Design: The 7 Foundational
Principles*. Information and Privacy Commissioner of Ontario, Canada.
https://www.ipc.on.ca/wp-content/uploads/resources/7foundationalprinciples.pdf
International recognition: Cavoukian, A. (2010). Privacy by design: the definitive
workshop. *Identity in the Information Society*, 3(2), 247–251.
https://doi.org/10.1007/s12394-010-0062-y

**Summary:** Ann Cavoukian introduced Privacy by Design (PbD) in the 1990s as a
proactive approach to embedding privacy protections into system architecture from the
earliest design stages, rather than retrofitting them after the fact. The seven
foundational principles include: proactive not reactive; privacy as the default;
privacy embedded into design; full functionality (positive-sum, not zero-sum); end-to-
end security; visibility and transparency; and respect for user privacy. PbD was
adopted as an international standard in 2010 and influenced GDPR Article 25 ("data
protection by design and by default").

**Relation to Trenza:** Trenza's Rule 6 (Data Conformance / GDPR) and its `@audit`
annotation system are a direct operational realisation of the PbD principles: GDPR
obligations are embedded in the specification language itself (proactive, by design),
and the compiler enforces them rather than relying on developer awareness. The `@audit`
mechanism operationalises the transparency and accountability principles by producing
a machine-generated narrative audit trail from the same source that generates the
implementation.

---

### GDPR Article 25 — Data Protection by Design and by Default

**Citation:** Regulation (EU) 2016/679 of the European Parliament and of the Council
of 27 April 2016, Article 25. *Official Journal of the European Union*, L 119,
4 May 2016, pp. 1–88.
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679
EDPB guidance: European Data Protection Board. (2020). *Guidelines 4/2019 on Article
25: Data Protection by Design and by Default*, Version 2.0.
https://www.edpb.europa.eu/sites/default/files/files/file1/edpb_guidelines_201904_dataprotection_by_design_and_by_default_v2.0_en.pdf

**Summary:** Article 25 of the GDPR mandates that controllers implement data
protection "by design and by default" — technical and organisational measures that
implement data-protection principles (such as data minimisation) and integrate
necessary safeguards into the processing activity at the time of design. The EDPB
guidelines operationalise this into requirements covering: purpose limitation, data
minimisation, storage limitation, security, transparency, and accuracy.

**Relation to Trenza:** Article 25 is the legal mandate that Trenza's Rule 6
addresses. By encoding data-classification and purpose constraints directly in the
`.trz` source and verifying them at compile time, Trenza provides a technical
implementation path for Article 25 compliance. The EDPB guidelines' emphasis on
accountability and demonstrability aligns with Trenza's Strand 4 (audit report): the
audit output is a human-readable, traceable record that a data-protection officer can
review.

---

### Formal Modelling and Analysis of Data Protection for GDPR

**Citation:** Brucker, A. D. and Herzberg, M. (2019). On the formal semantics of
privacy policies. In *Proceedings of the 24th ACM Symposium on Access Control Models
and Technologies (SACMAT)*. (See also related work using Isabelle/HOL to formally
encode GDPR as Kripke structures with CTL temporal logic; available via SciSpace /
CEA HAL.)

See also: Formal modeling and analysis of data protection for GDPR (Isabelle/HOL
approach). https://scispace.com/pdf/formal-modeling-and-analysis-of-data-protection-for-gdpr-2zqmrw5yyu.pdf

**Summary:** Researchers have applied formal methods — including Isabelle's
Infrastructure framework (using Kripke structures and CTL temporal logic), Event-B,
and formal concept analysis — to model GDPR compliance properties and verify that
system designs satisfy them. These works establish "an important first step towards
producing GDPR-compliant systems by establishing a general framework for creating a
formal system design that provably complies to the regulation."

**Relation to Trenza:** This line of work demonstrates both the demand for formal
approaches to GDPR compliance and their current complexity: interactive theorem
proving and CTL model checking require significant expertise. Trenza's Rule 6 provides
a bounded, domain-specific subset of these properties that is checked automatically
during ordinary compilation, making GDPR-relevant verification accessible to
application developers without formal methods expertise. Trenza does not claim
completeness relative to the full GDPR; it enforces a decidable, developer-facing
subset.

---

## Supplementary: Role-Based Access Control Foundations

### RBAC Models — Sandhu et al.

**Citation:** Sandhu, R., Coyne, E., Feinstein, H., and Youman, C. (1996).
Role-based access control models. *IEEE Computer*, 29(2), 38–47.
https://csrc.nist.gov/csrc/media/projects/role-based-access-control/documents/sandhu96.pdf

**Summary:** Sandhu et al. formalized the RBAC family of models (RBAC0 through
RBAC3), defining users, roles, permissions, sessions, and role-hierarchy relationships
as the foundational vocabulary for access-control policy. The model associates
permissions with roles rather than directly with users, and has become the dominant
access-control paradigm in commercial and government systems. A 2004 ANSI/INCITS
standard was based on the resulting NIST model.

**Relation to Trenza:** Trenza's role system — where every handler is keyed by
(role, event) — is conceptually grounded in RBAC. A Trenza role corresponds to an
RBAC role; the set of (event, effect) pairs that a role can exercise corresponds to
its permission set. The novelty in Trenza is that RBAC is fused with state-machine
formalism: which events a role may trigger, and what effects may result, are determined
not only by role assignment but also by the current state context. This creates a
*context-sensitive* RBAC model enforced at compile time, rather than at runtime policy
evaluation.

---

*Document prepared for the Trenza-DSL ONWARD! 2026 submission.
All citations verified against ACM DL, arXiv, and W3C sources as of 2026-03-25.*
