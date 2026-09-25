---
title: "Related Work — audit and gap analysis for ONWARD! 2026 submission"
status: audit
date: 2026-04-17
author: Claude Opus 4.6
audits: docs/design/related-work-research.md (2026-03-25, 16 entries)
target: docs/design/paper-structure-onward.md (§6, ~1.5 pp)
---

# Summary

The current `related-work-research.md` is solid: 16 entries across six
sections (statecharts, formal specification, MDE/CASE, state-management
DSLs, AI-assisted SE, GDPR), each with citation, summary and explicit
relation to Trenza. Coverage of the formal-methods and statechart
ancestry is strong.

Three categories of gap remain. In rough order of urgency:

1. **Antecedents the paper itself cites** but related-work does not.
2. **Venue fit** — Felienne Hermans (chair) and the ONWARD! community.
3. **Adversarial / contemporary** — claude-flow, MCP, agent protocols.

A successful ONWARD! submission needs gap (1) closed, gap (2) addressed
substantively, and gap (3) handled at minimum as a footnote in §2 or §6.

---

# Gap 1 — Antecedents the paper cites that are missing here

The paper structure (`paper-structure-onward.md`) and §3 prose
(`paper-draft-s3-s5.md`) cite three influences that are not in
related-work. These need entries before camera-ready, and ideally
before submission, because reviewers will look for them.

## 1.1 DCI — Data, Context, Interaction (Reenskaug & Coplien)

**Why required:** §3.1 of the current draft opens with *"the central
design decision in Trenza is the identification of the minimum unit of
specification. The answer, borrowed from Reenskaug's DCI architecture,
is the *context*"*. A direct citation is non-negotiable; otherwise §3
makes an attribution it does not back up.

**Suggested citation:** Reenskaug, T. and Coplien, J. O. (2009). *The
DCI Architecture: A New Vision of Object-Oriented Programming*.
Artima Developer.
https://www.artima.com/articles/dci_vision.html

Book-length treatment: Coplien, J. O. and Bjørnvig, G. (2010).
*Lean Architecture for Agile Software Development*. Wiley. ISBN
978-0-470-68420-7.

**Suggested relation paragraph:** DCI introduces the *context* as the
locus where roles play out their interactions for the duration of a use
case. Trenza adopts the term and the conceptual unit, and adds three
constraints DCI does not impose: the context must be *named in the
specification*, its set of role/event handlers must be *exhaustive*,
and the transitions between contexts must be *closed under reachability
and return*. DCI is a programming-style proposal; Trenza is the formal
specification language that makes the DCI discipline machine-checkable.

## 1.2 Naked Objects (Pawson)

**Why required:** Listed in `paper-structure-onward.md` §6 as a planned
related work entry. Not in related-work-research.md.

**Suggested citation:** Pawson, R. (2004). *Naked Objects*. PhD thesis,
Trinity College Dublin.
https://www.cs.tcd.ie/publications/tech-reports/reports.04/TCD-CS-2004-22.pdf

Book version: Pawson, R. and Matthews, R. (2002). *Naked Objects*.
John Wiley & Sons. ISBN 0-470-84420-0.

**Suggested relation paragraph:** Naked Objects argues that domain
behavior should be exposed directly through the user interface,
without intermediate presentation layers. Trenza shares the underlying
commitment to making behavior visible at the level where decisions are
taken: in Naked Objects, the UI exposes what the domain object does;
in Trenza, the `.trz` exposes what every (role, event) pair does in
every context. The two proposals attack the same defect — behavior
hidden behind a layer of indirection — at different points in the
stack.

## 1.3 NetKernel / Resource-Oriented Computing (Rodgers)

**Why required:** Listed in `paper-structure-onward.md` §6 as
*"personal intellectual antecedent — indirect connection, honestly
declared"*. The honesty matters more than the strength of the
connection: declaring the influence with the reservation already in
place is more credible than omitting it.

**Suggested citation:** Rodgers, P. (2010). *Resource-Oriented
Computing with NetKernel: Taming the Complexity of System
Development*. O'Reilly Media. ISBN 978-1-4493-9408-6.

Foundational architecture paper: Rodgers, P. and Sissel, S. (2004).
*Resource-Oriented Computing*. 1060 Research / HP Labs.

**Suggested relation paragraph:** ROC and Trenza share an architectural
intuition that has not been mainstream: that computation is best
described as a sequence of named, addressable, side-effect-bounded
transformations, and that caching and reasoning become tractable when
the addressing is part of the language rather than added on top. The
direct lineage is weak — Trenza did not derive its design from ROC —
but the convergence on *named contexts as the addressable unit* is
worth noting honestly.

---

# Gap 2 — Venue fit: Felienne Hermans and accessible language design

**Why this matters:** Felienne Hermans (VU Amsterdam) chairs ONWARD!
Papers 2026. Her published work centers on *making programming
languages accessible to non-expert users* — exactly the framing Trenza
adopts when it argues that its eight rules are "readable by an engineer
who does not know what a temporal operator is" (§3.3). Citing her work
is not a rhetorical move; it is the right citation for the claim the
paper makes.

## 2.1 The Programmer's Brain

**Suggested citation:** Hermans, F. (2021). *The Programmer's Brain:
What every programmer needs to know about cognition*. Manning. ISBN
978-1-61729-877-7.

**Suggested relation paragraph:** Hermans frames the design of
programming languages as a problem of working memory and chunking:
languages that minimize the number of unrelated concepts a reader
must hold in mind simultaneously are easier to learn and easier to
review. Trenza's eight rules are deliberately phrased so that each
rule corresponds to a single chunk a reader can hold (*"every role
handles every event in every context"*); the verifier's diagnostics
report violations in the same vocabulary. The design pressure that
produced this — *the model must be able to read the rule and reason
about it without unpacking a definition* — is the same pressure
Hermans documents for human readers.

## 2.2 Hedy

**Suggested citation:** Hermans, F. (2020). Hedy: A Gradual Language
for Classroom Programming. *Proceedings of the 2020 ACM SIGPLAN
International Symposium on New Ideas, New Paradigms, and Reflections
on Programming and Software (Onward! 2020)*, 1–12.
https://doi.org/10.1145/3426428.3426917

**Suggested relation paragraph:** Hedy demonstrates that a programming
language's *syntactic surface* can be designed to grow with the
learner: each level of the language is itself a complete language, and
the transition between levels is small enough that the learner is
never confronted with too much new structure at once. Trenza's design
takes a related stance for an LLM audience: the surface is intentionally
small and self-describing, so that a model with a `.trz` file in
context can reason about modifications without consulting external
documentation. Hedy's grading argument applies, with adjustments, to
the LLM-readability argument.

**Bonus:** Hedy was published at Onward! 2020 — the same paper venue
Trenza is targeting. Citing a chair's prior Onward! paper is the
clearest possible signal that the submission understands the venue.

---

# Gap 3 — Contemporary / adversarial systems

These are not academic citations and may be relegated to footnotes,
but their absence would be conspicuous to any reviewer who has
followed the agent-coordination space.

## 3.1 claude-flow

**Why required:** §2 of the paper structure cites claude-flow as *"the
opposite approach: complexity as response to indiscipline"*. The paper
makes an argument *against* a real, named system; the system needs a
citation, even if only to its documentation.

**Suggested citation:** Anthropic Community Tools (2024–present).
*claude-flow: Multi-agent orchestration for Claude Code* [software].
GitHub: https://github.com/ruvnet/claude-flow

**Note:** This is a tool, not a paper. The honest framing is: *"We
contrast Trenza's structural approach with claude-flow [cite], a
representative example of the alternative approach: when state is
dispersed across an agent system, add coordination machinery (sub-
agents, orchestrators, message brokers) rather than constrain the
state model."* No academic credit issue arises because no academic
claim is being attributed.

## 3.2 MCP — Model Context Protocol

**Why required:** §6 of the paper structure proposes MCP as a related
work entry. MCP is the dominant standard for agent-tool coordination;
not citing it would suggest unawareness.

**Suggested citation:** Anthropic (2024). *Model Context Protocol
specification*. https://modelcontextprotocol.io/specification

**Suggested relation paragraph:** MCP standardizes the protocol layer
by which an LLM session communicates with external tools and
resources. Trenza occupies a different layer: it specifies *what the
system does*, not *how the model invokes external capabilities*. The
two compose naturally — a `.trz` file can declare `external:` modules
whose runtime binding is provided by MCP servers — but Trenza adds no
constraints on the protocol itself.

---

# Recommended action items for Sonnet (pre-submission)

1. **Promote DCI to a top-level entry** in related-work-research.md
   Section A or as a new Section G ("Conceptual Antecedents"). Without
   this, §3.1 has an unbacked attribution.
2. **Add Hermans entries** (Programmer's Brain + Hedy) as a new
   Section H ("Language Design for Non-Expert Readers"). This is the
   single highest-leverage change for venue fit.
3. **Add Naked Objects + NetKernel** as the rest of Section G
   (Conceptual Antecedents). Honest declaration of indirect lineage is
   more credible than omission.
4. **Add claude-flow + MCP** as a footnote in §2 or as a short
   "Contemporary Systems" subsection in §6. Both can cite documentation
   rather than papers.

The current 16 entries become 22 with these additions. That is still
within the typical Onward! related-work footprint (1.5 pages) if the
relations are written tightly.

---

*Audit — 2026-04-17.*
*This document is a planning artifact for Sonnet's related-work*
*revision. It does not modify `related-work-research.md` directly.*
