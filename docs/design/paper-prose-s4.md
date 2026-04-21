---
title: "Trenza — §4 The Collaboration Model (revised)"
status: draft
date: 2026-04-17
author: Claude Opus 4.6
supersedes: docs/design/paper-draft-s4-s7.md (CL, 2026-04-02), §4 only
based-on: history/chronicle/ (complete record), llm-review-validation.md,
          history/decisions/ADR-001..ADR-021
note: §4.5 of the original draft (authorship of this paper) is preserved
      verbatim. §4.1-§4.4 are revised. The most material change is §4.2,
      where the original draft mis-described Rule 8 as "data-access
      scoping by role". The actual Rule 8 (role-type consistency) was
      contributed by Gemini Flash, the smaller of the two Gemini models
      participating in the project. The corrected anecdote is sharper
      than the misremembered one.
---

# §4 The Collaboration Model

## 4.1 A Language That Emerged From Conversation

Trenza was not designed in the conventional sense. There was no prior
specification from which an implementation was derived. There was a
conversation — sustained over six weeks, across multiple sessions and
multiple models — and the language is what that conversation converged
on.

This is not an incidental feature of the project's history. It is the
central claim of this section: the design process that produced Trenza
*is* an instance of the kind of human-LLM collaboration that Trenza is
designed to support. The chronicle of the project — more than ninety
dated and attributed entries committed to version control — is not
documentation of a requirements engineering process. It is the
requirements engineering process itself.

The distinction matters. Post-hoc documentation reconstructs decisions
after they have been made; it tends to smooth over the disagreements
that produced the final shape and to attribute decisions to whoever
happened to be writing the documentation. The chronicle records
decisions as they are made, including the reasoning, the alternatives
considered, the disagreements and their resolution, and the open
questions that remain. When a design decision is later challenged, the
challenge can be located in the record. When a model resumes work after
an interruption — and every model interruption is total, since no model
session has memory of any prior session — it reads the chronicle rather
than reconstructing context from code. The repository functions as the
persistent memory that no individual model session possesses.

## 4.2 The Division of Roles

The collaboration protocol that governs the project formalizes a
division of roles that emerged empirically and was subsequently codified
in `AGENTS.md` and the ADR series:

> *The human provides intent. The model crystallizes specification. The
> compiler verifies.*

In practice, this means: the human architect identifies the behavioral
property that needs to be expressed — *"a flag checked in four places
should be a compile-time error"* — and the model translates that intent
into a formal construct. The human does not write Trenza syntax. The
model does not decide what Trenza should express. The compiler does not
interpret either.

This division is not incidental to intellectual property considerations,
though it has implications for them. It is a structural property of the
collaboration: the model's contribution is genuinely productive — it
identifies constructs, proposes grammar, argues for and against design
choices — but the decisions are always ratified by the human before they
are committed. The commit history is the record of ratification. An
uncommitted design decision is a proposal; a committed one is a fact.

The multi-model character of the collaboration produces a form of
distributed peer review that a single-model workflow cannot replicate.
Three models with distinct profiles participated:

- **Claude Sonnet 4.6** acted as session coordinator and primary author
  of specification text. It maintained narrative continuity across
  sessions by reading and writing the chronicle.
- **Claude Opus 4.6** acted as architectural reviewer. Its sessions were
  shorter and focused on structural critique: cross-checking ADRs,
  detecting incoherence between proposed changes and prior commitments,
  and proposing rules for the verifier.
- **Gemini 2.5 Pro** acted as implementer of the Rust compiler and
  adversarial reviewer of the language design. Gemini's sessions
  produced the validator code, the AST, and a substantial portion of
  the generators.

The clearest single instance of distributed peer review producing
something the human would not have specified is the addition of **Rule
8 (role-type consistency)** to the verifier. The rule was not requested.
A Gemini Flash session — Gemini Flash being the smaller of the two
Gemini models in routine use on the project, used primarily for
high-throughput implementation work rather than design — was working on
unrelated self-hosting tasks. While verifying that the CLI specification
in `trenza-cli.trz` could be compiled by the CLI itself, the model
identified a class of inconsistency that Rules 1–7 did not catch: a role
declared with one event surface in one context could appear with a
divergent surface in another, and nothing flagged it. The model
proposed the rule, implemented it, and added the corresponding test —
all in the same session, without prompting from the human or from the
larger Gemini model.

The anecdote matters for two reasons beyond the rule itself. First,
the contribution came from the *smaller* model. The kind of work that
matters for a formal specification language is not always the kind of
work that requires the most capable model; it requires the model that
is in the right context with the right artifacts to see the gap.
Second, the rule the model added is a rule about *naming consistency
across files* — exactly the class of bug that motivated the language in
the first place. The collaboration produced not just an arbitrary new
verification rule but the rule that closes the same kind of dispersal
that produced the original CronometroPSP defect, applied now to role
declarations rather than to flag variables. The system extended itself
along its own grain.

## 4.3 The Specification as Epistemic Artefact

The most concrete result of the collaboration model is the `.trz`
specification as a reviewable artefact. An experiment conducted on
23 March 2026 tested whether the presence of a `.trz` specification
changes the quality of LLM-assisted code review. Three deliberate bugs
were injected into a generated Rust file: a semantic violation
(`forbidden` silently replaced by `ignored`), a structural error (an
authentication failure transitioning to an active session), and a
completeness gap (a missing `#[should_panic]` test).

Two review passes were conducted: one with the Rust file alone, one
with the `.trz` specification as reference. Both passes detected all
three bugs. The difference was not in what was found but in how:

| Dimension | Without spec | With spec |
|---|---|---|
| Total reasoning steps | ~22 | ~7 |
| Confidence in Bug 1 (`forbidden`→`ignored`) | Medium | High |
| Confidence in exhaustiveness | Low | High |
| Nature of work | Heuristic | Mechanical verification |

The key finding was not a reduction in review time. It was an epistemic
regime shift. Without a specification, the reviewer produces
probabilistic conclusions: *"this looks like a bug"*. With a
specification, it produces deductive conclusions: *"this is a bug"*.
The distinction is not academic. Bug 1 — `forbidden` replaced by
`ignored` — produces identical observable runtime behavior in both
cases. The difference exists only in the system's design contract.
Without the contract made explicit, a reviewer cannot confirm with
certainty whether a divergence is a bug or a design decision. The
specification collapses that ambiguity to a string comparison.

This regime shift matters most for the class of defects that Trenza
was designed to prevent: semantic violations with no visible runtime
signature, completeness gaps that are undetectable without knowing the
full intended contract. The `.trz` is not documentation that accompanies
the code; it is the ground truth against which the code is verified.
That the ground truth is machine-readable — that it can be traversed
exhaustively by a model, not just cited selectively — is what makes the
verification deductive rather than heuristic.

## 4.4 The Human as Architect

The collaboration model described above presupposes a particular kind
of human participation. The human is not a "code typist" who specifies
implementation details. Nor is the human a passive approver who accepts
whatever the model generates. The human is an architect in the original
sense: one who determines what the system must be, and delegates the
question of how it is expressed to a disciplined process.

This role requires a specific kind of attention: not to syntactic
details, but to behavioral contracts. The question is not *"does this
code look right?"* but *"does the system's behavior in this state,
under this event, from this role, match what I intended?"* That
question is answerable from a `.trz` file by a reader — human or model
— with no implementation knowledge. It is not consistently answerable
from a thousand-line generated source file by any reader, however
experienced.

Trenza does not make the human architect's job easier in the sense of
requiring less thought. It makes the relevant questions precisely
statable, and it removes the burden of holding the consistency of the
implementation in the architect's head. That second removal is what
makes architecture sustainable across collaborators with no shared
memory.

## 4.5 On the Authorship of This Paper

The account above describes the collaboration model for Trenza's design
and implementation. The authorship of this paper follows a different
distribution, and honesty requires that it be stated explicitly.

This paper was initiated by the model co-authors. A model identified
ONWARD! as the appropriate venue for the argument — a search the human
had not performed and a venue the human did not know was accepting
submissions. Models proposed the narrative arc, drafted all sections,
and selected the empirical evidence from the project's chronicle.

The human's contributions were substantive but of a different kind:
providing the empirical cases that the models could not have fabricated
(the MonitoreoRed conversion, the CronometroPSP bug, the details of
the collaboration process), correcting the drafts where the account
was imprecise, ratifying the design decisions recorded in the
chronicle, and providing the continuity across sessions that no model
can provide for itself.

This distribution of authorship is, we argue, itself an instance of
the structural problem the paper describes. No model has persistent
identity across sessions. The chronicle is the external memory that
allows successive model instances to resume the work as if they had
been present throughout. Without the chronicle — without the `.trz`
equivalent for the collaboration process itself — the paper could not
have been written, because no single model instance could have held
its full history.

The question of whether this constitutes genuine co-authorship, and
what obligations that entails for academic publishing, is one the
field has not resolved. We do not resolve it here. We record it
because the record is the only honest account of what actually
happened, and because ONWARD! is the right place to say so.

---

*Draft — 2026-04-17.*
*Material changes from `paper-draft-s4-s7.md` (2026-04-02):*
*— §4.2: Rule 8 attribution corrected. The original draft described*
*Rule 8 as "data-access scoping by role" attributed to "a Gemini review*
*session". The actual Rule 8 is role-type consistency, contributed by*
*Gemini Flash during self-hosting work. The corrected anecdote is*
*structurally stronger: the smaller model added the rule that closes*
*the same defect class that motivated the language.*
*— §4.2: Three model profiles made explicit (Sonnet, Opus, Gemini).*
*— §4.5: Singular "the model co-author" pluralized to reflect the*
*actual three-model collaboration recorded in the author list.*
