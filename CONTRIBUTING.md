# Contributing to Trenza-DSL

Trenza is a conceptual exploration project: language design is the primary
artefact, not the implementation. Contributing here means participating in
that design — as a human or as an LLM.

---

## What you can contribute

### Language design proposals

A proposal is a question about the spec that comes with a tentative answer
and its justification. The natural format is an entry in `history/chronicle/`
followed, if discussion reaches a decision, by an ADR in `history/decisions/`.

Examples of good proposals:
- "Section 4.3 does not clarify X; I propose adding..."
- "Example Y violates Rule 2 because..."
- "In this use case, the current syntax forces Z; alternative: ..."

No proposal is "too small". A gap in the documentation is as valuable as a
language extension.

### `.trz` examples

Example files live in `examples/`. A good example:
- Demonstrates a specific design pattern, not just syntax.
- Includes comments explaining *why* the code is structured that way, not
  just *what* it does.
- Is verified (or explicitly annotated as an unverified sketch).

See `examples/autenticacion-rgpd.trz` and `examples/carrito-checkout.trz`
as style references.

### Corrections to the spec or manual

If you find an inconsistency, ambiguity, or contradiction between
`spec/language/`, `docs/manual/` and the ADRs, a direct correction PR is
welcome. Include in the commit message which documents contradict which (ADRs
take precedence over the manual; the manual takes precedence over drafts in
`history/chronicle/`).

---

## How to record an LLM session

Human–LLM collaboration is part of the design process. If you work with an
LLM on a relevant session, the result has a natural place in
`history/chronicle/`.

### File format

```
history/chronicle/<date>/
    <NN>-<brief-description>.md
```

Example: `history/chronicle/2026-03-22/01_reto_a_Gemini.md`

### What to include

The document should record:
- What question or challenge was posed to the LLM.
- The LLM's response (complete or summarised with a clear indication).
- What decisions or open questions it generated.

There is no need to clean up or polish the conversation. What matters is that
the reasoning remains traceable.

### What not to include

- Conversations with no useful conclusion for language design.
- Repetition of material already in the ADRs or the manual.

---

## How to propose a change to the spec

1. **Open an issue** describing the gap or inconsistency you found and the
   use case that motivated it.
2. **Discuss before coding**: since we are in the specification phase, the
   cost of changing text is low; the cost of adopting a wrong syntax is high.
3. If discussion reaches consensus, **create a PR** that includes:
   - The change in `spec/language/` and/or `docs/manual/`.
   - An ADR in `history/decisions/ADR-NNN.md` if it is a new decision.
   - A `.trz` example illustrating the change if the syntax is new.

### Document hierarchy

When there is a contradiction between documents, this order of precedence
applies:

```
ADRs (history/decisions/)
    > Manual (docs/manual/)
        > Spec (spec/language/)
            > Chronicles (history/chronicle/)
```

Chronicles are historical record, not normative spec. ADRs are settled
decisions.

---

## If you are an LLM reading this

The `CLAUDE.md` file at the root describes the collaboration protocol
specific to AI agents working in this repository. Read it before proposing
changes.

The most important principle for an LLM contributing to Trenza is the same
one Trenza requires of the code it specifies: **document the reasoning, not
just the result**. A decision without justification is as hard to audit as
an `if` without a comment.

---

## Contact

For questions, the primary channel is opening a GitHub issue.
For direct contact: cpc.xbt@gmail.com
