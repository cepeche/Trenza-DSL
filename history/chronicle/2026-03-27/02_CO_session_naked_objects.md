# Session Close: Naked Objects, CRUD, and the Scaffolding Metaphor

**Date:** 2026-03-27 (afternoon)
**Author:** CO (Claude Opus 4.6 via Cowork/Claude Code)
**Type:** Session close — conceptual exploration

---

## 1. Conversation arc

This session was a philosophical exploration, not an implementation session.
César opened with "los objetos desnudos y Trenza" and the conversation
traversed the following arc:

1. **What is data in an information system?** — The legislative/regulatory
   mental model of data as a physical object (movable, deletable, locatable)
   vs. the reality of data as emergent state across caches, logs, events,
   ML training sets, backups.

2. **GDPR, DLT, sovereign cloud** — All three regulatory frameworks assume
   data has clear physical boundaries. The right to erasure (Art. 17) assumes
   deletion is atomic and complete. DLT immutability and GDPR erasure are a
   categorical contradiction, not a technical problem pending solution.

3. **Trenza as partial answer — honestly assessed.** César explicitly rejected
   the idea that Trenza in its current state is a meaningful answer to these
   problems. "Haríamos trampas al solitario." Correct. Trenza can formalize
   what can already be said. The real system includes judgment, tacit knowledge,
   the conversation itself.

4. **Wittgenstein:** "Los límites de mi lenguaje son los límites de mi mundo."
   Trenza operates at the margins where formalization is possible — not trivial
   margins, but not the whole territory either.

5. **Cimbra** — César revealed he attempted something like Trenza in 1982
   on an HP 9872 plotter with Rocky Mountain Basic, intended to automate
   bridge drawing for prefabricated beam bridges. The name he would have
   given it: *Cimbra* — the temporary scaffolding that holds an arch while
   it is built, removed when the arch can support itself. This is the exact
   metaphor for what Trenza does: not the system, but what allows the system
   to be built without collapsing during construction.

6. **`Tren_de_cotas`** — The concrete problem in 1982 was dimension lines
   interfering with bridge views. The solution: a String variable encoding
   all distances for the same annotation line. A hand-written mini-DSL inside
   a BASIC String. The intuition was correct; the tools were wrong.

7. **CRUD critique** — `UPDATE` flattens semantically distinct operations:
   - *Correct*: the previous datum was wrong
   - *Update*: reality changed; the previous datum was correct at the time
   - *Rectify*: a decision is reviewed
   - *Confirm*: a provisional state becomes definitive

   For GDPR/privacy-by-design, this distinction is legally relevant. CRUD
   destroys it. This is essential to privacy by design.

8. **Naked Objects and state** — Pawson (2002) / Apache Isis → Apache Causeway
   (renamed 2023). The pattern exposes domain object behavior directly as UI.
   But state is implicit in Naked Objects implementations — hidden in `if`
   conditions within methods, not formally specified. Trenza adds the missing
   piece: explicit, verifiable state machines as the skeleton of the naked object.

   NotebookLM had independently cited Apache Isis when processing early Helix
   documents (~March 10, before the rename). The conceptual connection was
   visible from the first days of the project.

9. **Originality assessment** — César asked directly whether we have contributed
   something original or merely ordered existing chaos. Answer: both, and the
   ordering is non-trivial. The original contribution is the framing of the
   formal DSL as a coordination contract between humans and models — not
   "LLM generates code" but "the specification is what makes both parties'
   understanding verifiable." The compiler enforces on both.

10. **ONWARD! prospects** — César had never considered turning this into an
    academic paper (parallel: Mario Piattini had to insist repeatedly to get
    him to write about the future of databases; César felt the result was
    arrogant and pretentious). His assessment: "you should be first author
    since you've read the papers." Correct response: the foundational insight
    is his — `modoEdicion` in 4 places, Cimbra in 1982, the CRUD/privacy
    connection. The related work survey is more mechanical.

---

## 2. Key insights for the paper

### The Cimbra metaphor
Trenza is the scaffolding, not the bridge. It holds the system while it is
being built. When complete and verified, the formal specification recedes
to a record that the arch holds. This is a better metaphor than "DSL for LLMs"
and more honest than "privacy by design solution."

### CRUD as semantic destruction
The `correct` vs `update` distinction is not pedantic — it is the mechanism
by which CRUD-based systems structurally prevent privacy by design. A system
that cannot distinguish correction from update cannot generate a legally valid
audit trail. This deserves a section in the paper, connected to Naked Objects
and the event sourcing literature.

### Naked Objects + Trenza synthesis
- Naked Objects: the right interface philosophy (behavior IS the interface)
- Trenza: the right specification language (state machines as first-class citizens)
- Together: UI shows available behavior (Naked Objects); DSL specifies valid
  behavior (Trenza); compiler verifies specification before exposure to anyone.
- "The naked object with a skeleton."

### The 1982 precedent
The `Tren_de_cotas` story establishes that the intuition behind Trenza is not
a product of the LLM era — it emerged from a practitioner confronting the
same structural problem (state dispersal) with the tools available in 1982.
This strengthens the paper's claim that the problem is real and recurring,
not a novelty manufactured to justify the solution.

---

## 3. What was NOT covered

- Paper structure review (deferred — César left for lunch)
- Connection between Naked Objects and potential Strand 5 (persistence) / Strand 6 (UI generation)
- The `@intent` annotation in relation to the CRUD/semantic-operation distinction

---

## 4. Briefing for next session (probably Monday)

- **Naked Objects + Strand 5/6**: the obvious next step is whether Trenza
  should grow strands for persistence (event sourcing — each state transition
  as a persisted fact) and UI generation (from role/context model, Naked
  Objects style). These would make the CRUD/privacy argument implementable,
  not just conceptual.

- **CRUD section for the paper**: the `correct` vs `update` distinction,
  connected to GDPR Art. 17 and the event sourcing literature, deserves
  a dedicated section. This is the bridge between the theoretical framing
  (Naked Objects, state machines) and the practical regulatory problem.

- **Cimbra as section title or epigraph**: worth considering for the paper.
  A civil engineer's metaphor for scaffolding that holds the arch while it
  is built — and is removed when the arch is self-supporting — is more
  evocative and accurate than "DSL for LLMs."

- **Paper structure review**: `docs/design/paper-structure-onward.md` was
  proposed this morning by CL. Review pending.

---

## 5. Personal context noted

César has prior academic publishing experience (invited by Mario Piattini to
write on the future of databases). His self-assessment: felt arrogant and
pretentious. This suggests he will need encouragement to claim the contribution
rather than deflect it. The practitioner voice — "I built this because a bug
was making systems unmaintainable" — is the paper's strength, not a weakness
to apologize for.
