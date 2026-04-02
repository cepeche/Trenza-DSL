---
title: "Trenza — Draft §4 Collaboration Model + §7 Conclusion"
status: draft
date: 2026-04-02
author: CL (Claude Sonnet 4.6 via Claude Code, mobile session)
based-on: paper-structure-onward.md, llm-review-validation.md,
          history/chronicle/2026-03-23/04_experimento_revision_llm.md,
          AGENTS.md, history/meta/directrices-pi-ia.md,
          history/chronicle/ (complete record)
---

# §4 The Collaboration Model

## 4.1 A Language That Emerged From Conversation

Trenza was not designed in the conventional sense. There was no prior
specification from which an implementation was derived. There was a conversation
— sustained over weeks, across multiple sessions and multiple models — and the
language is what that conversation converged on.

This is not an incidental feature of the project's history. It is the central
claim of this section: the design process that produced Trenza *is* an instance
of the kind of human-LLM collaboration that Trenza is designed to support. The
chronicle of the project — more than ninety entries spanning six weeks, each
dated, attributed, and committed to version control — is not documentation of a
requirements engineering process. It is the requirements engineering process.

The distinction matters. Post-hoc documentation reconstructs decisions after
they have been made. The chronicle records decisions as they are made, including
the reasoning, the alternatives considered, and the open questions that remain.
When a design decision is later challenged, the challenge can be located in the
record. When a model resumes work after an interruption, it reads the chronicle
rather than reconstructing context from code. The repository functions as the
persistent memory that no individual model session possesses.

## 4.2 The Division of Roles

The collaboration protocol that governs the project formalizes a division of
roles that emerged empirically and was subsequently codified:

> *The human provides intent. The model crystallizes specification. The
> compiler verifies.*

In practice, this means: the human architect identifies the behavioral property
that needs to be expressed — "a flag checked in four places should be a
compile-time error" — and the model translates that intent into a formal
construct. The human does not write Trenza. The model does not decide what
Trenza should express. The compiler does not interpret either.

This division is not incidental to intellectual property considerations, though
it has implications for them. It is a structural property of the collaboration:
the model's contribution is genuinely productive — it identifies constructs,
proposes grammar, argues for and against design choices — but the decisions are
always ratified by the human before they are committed. The commit history is
the record of ratification. An uncommitted design decision is a proposal;
a committed one is a fact.

The multi-model character of the collaboration — Claude Sonnet for
implementation, Claude Opus for architectural review, Gemini for adversarial
challenge — produces a form of distributed peer review that a single-model
workflow cannot replicate. One instance of this: the addition of Rule 8
(data-access scoping by role) emerged from a Gemini review session in which
the reviewer identified the gap without explicit prompting. The rule was not in
the original design; it was contributed by the adversarial process.

## 4.3 The Specification as Epistemic Artefact

The most concrete result of the collaboration model is the `.trz` specification
as a reviewable artefact. An experiment conducted on 23 March 2026 tested
whether the presence of a `.trz` specification changes the quality of LLM-assisted
code review. Three deliberate bugs were injected into a generated Rust file:
a semantic violation (`forbidden` silently replaced by `ignored`), a structural
error (authentication failure transitioning to an active session), and a
completeness gap (a missing `#[should_panic]` test).

Two review passes were conducted: one with the Rust file alone, one with the
`.trz` specification as reference. Both passes detected all three bugs. The
difference was not in what was found but in how:

| Dimension | Without spec | With spec |
|---|---|---|
| Total reasoning steps | ~22 | ~7 |
| Confidence in Bug 1 (`forbidden`→`ignored`) | Medium | High |
| Confidence in exhaustiveness | Low | High |
| Nature of work | Heuristic | Mechanical verification |

The key finding was not a reduction in review time. It was an epistemic regime
shift. Without a specification, the reviewer produces probabilistic
conclusions: *"this looks like a bug"*. With a specification, it produces
deductive conclusions: *"this is a bug"*. The distinction is not academic.
Bug 1 — `forbidden` replaced by `ignored` — produces identical observable
runtime behavior in both cases. The difference exists only in the system's
design contract. Without the contract made explicit, a reviewer cannot confirm
with certainty whether a divergence is a bug or a design decision. The
specification collapses that ambiguity to a string comparison.

This regime shift matters most for the class of defects that Trenza was
designed to prevent: semantic violations with no visible runtime signature,
completeness gaps that are undetectable without knowing the full intended
contract. The `.trz` is not documentation that accompanies the code; it is the
ground truth against which the code is verified. That the ground truth is
machine-readable — that it can be traversed exhaustively by a model, not just
cited selectively — is what makes the verification deductive rather than
heuristic.

## 4.4 The Human as Architect

The collaboration model described above presupposes a particular kind of
human participation. The human is not a "code typist" who specifies
implementation details. Nor is the human a passive approver who accepts
whatever the model generates. The human is an architect in the original
sense: one who determines what the system must be, and delegates the question
of how it is expressed to a disciplined process.

This role requires a specific kind of attention: not to syntactic details, but
to behavioral contracts. The question is not "does this code look right?" but
"does the system's behavior in this state, under this event, from this role,
match what I intended?" That question is answerable from a `.trz` file by a
reader — human or model — with no implementation knowledge. It is not
consistently answerable from a thousand-line Rust file by any reader, however
experienced.

Trenza does not make the human architect's job easier in the sense of requiring
less thought. It makes it possible in the sense of making the relevant
questions precisely statable.

---

# §7 Conclusion

This paper began with a conversion and ends with an acknowledgement of
incomplete work.

The conversion was genuine: a practitioner with thirty years of experience in
distributed systems, resistant on principled grounds to AI-assisted code
generation, changed his mind in a single morning because the tool contributed
something he had not anticipated. The network monitor worked; the device vendor
lookup was a gift.

What the monitor did not have — and still does not have — is a formal state
model. The system that converted a skeptic remains, as of this writing, a set
of scripts and a Prometheus database. There is no `.trz` file for MonitoreoRed.
The transition from `DispositivoConocido` to `Ausente` to `AlertaActiva` is
still implicit, still unverifiable, still invisible to the system that is
supposed to track it.

This is not a failure of Trenza. It is the honest boundary of what this paper
can claim. Trenza exists and compiles. It has verified a sixteen-module
distributed reference system — the CronometroPSP — in under one hundred
milliseconds, detecting completeness violations, unreachable contexts, and
access-control breaches that would otherwise have been silent runtime defects.
It has demonstrated that the presence of a `.trz` specification transforms
LLM-assisted code review from a heuristic process into a deductive one. It has
been used to specify itself.

What remains is the transfer. The language that human-LLM collaboration
demanded into existence must now be applied to the system that motivated its
demand. That application is the second production case, and it is unfinished.

The thesis of this paper is not that Trenza solves the problem of LLM-generated
code. It is that the problem is structural, and that structural problems require
structural solutions. LLMs are not undisciplined because they lack capability.
They are undisciplined because the languages and practices in which they were
trained did not forbid indiscipline. Trenza imposes constraint on the
collaboration — on the human who might scatter state across four locations, and
on the model that would faithfully reproduce the pattern. The constraint is not
punitive. It is clarifying. A missing method implementation is easier to fix
than a missing guard condition, precisely because it is visible.

The adult in the room does not write the code. It makes certain code
unwritable.

---

*Draft — 2026-04-02.*
*§4 and §7 drafted from repository materials in mobile session.*
*The LLM review experiment results (§4.3 table) are drawn directly from*
*`history/chronicle/2026-03-23/04_experimento_revision_llm.md`.*
*To be reviewed and corrected by César Pérez-Chirinos before further development.*
