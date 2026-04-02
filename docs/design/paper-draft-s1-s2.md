---
title: "Trenza — Draft §1 Introduction + §2 Motivation"
status: draft
date: 2026-04-02
author: CL (Claude Sonnet 4.6 via Claude Code, mobile session)
based-on: paper-structure-onward.md, paper-hybrid-abstract-onward.md,
          01_CL_session_close.md (2026-03-27), oral account (2026-04-02)
---

# §1 Introduction

This paper was not proposed by its human author. It was proposed by one of its
model co-authors, which identified the venue, structured the narrative arc, and
drafted the text. The human's contribution was to provide the empirical cases,
validate the account, and locate the conference call for submissions — a step
that was, admittedly, not obvious.

We record this at the outset not as a curiosity but as evidence. A model that
identifies its own structural limitation, finds the appropriate forum to report
it, and produces the written account of what it found is doing something that
the "stochastic parrot" framing does not accommodate. Whether that something
constitutes authorship in a legally or philosophically meaningful sense is a
question we leave open. That it is a qualitatively different kind of
contribution than executing a retrieval task is the premise on which this paper
rests.

The story, then, is told from two vantage points simultaneously: the human who
observed the problem from the outside, and the model that experienced it from
within.

---

One of the human authors of this paper spent thirty years working with
distributed objects before writing a single line of code with AI assistance.
Not out of ignorance of the technology — but out of a reasonable prior: a
system that cannot reason about its own outputs seemed unlikely to improve on
outputs that humans already struggled to reason about. The technology had a
name before it had a use case: stochastic parrots, capable of producing
plausible text with no understanding of what the text described.

The prior was wrong, and the correction arrived in a single morning.

The task was concrete: build a local network monitoring tool to replace the
functionality of Fing, a commercial application, on a home network of fourteen
devices. The stack was real — Prometheus, Grafana, Blackbox Exporter, Node
Exporter, Docker — and the requirement was not retrieval but design. What
emerged was not a faithful reproduction of what had been asked for. Claude
identified device manufacturers from their MAC addresses. That detail had not
been requested. It had not been considered. It arrived as a contribution, not
an execution.

The skeptic became a practitioner. But the same experiment that induced the
conversion also revealed the problem that this paper addresses.

The monitoring tool worked. What it did not have was a model of state. Device
presence was represented three ways simultaneously: as a JSON snapshot
(`dispositivos.json`), as Prometheus time-series metrics, and as point-in-time
ping results from scripts that ran and discarded their output. Nowhere in the
system was there a formal account of what it meant for a device to transition
from *known* to *absent* to *alert-active*. The state existed — it was just
invisible to the system that was supposed to track it.

When this problem surfaced, an obvious response was available: delegate
coordination to more agents. The `claude-flow` framework, representative of a
broader school of thought, answers state dispersal with swarms — fifteen
specialized agents collaborating to manage what a single coherent model would
have made unnecessary. The author had been following the intellectual trajectory
of that approach for some time, with interest and scepticism in equal measure.
He had written about the inadequacy of relational databases as a persistence
model for complex systems more than twenty-five years earlier. He was not
convinced that adding coordination layers was the right answer to a design
problem. `claude-flow` was proposed to Claude as a reference point for
inspiration, not as a solution to adopt.

The conclusion that this paper argues was already forming: the problem was not
a shortage of agents. It was the absence of formal constraint.

The second experiment made the diagnosis precise. In a proof-of-concept
application — a time-tracking tool built for personal use — a behavioral flag
called `modoEdicion` was scattered across four independent locations in the
JavaScript source. When a bug appeared in the mobile interface, Claude was
asked to find and fix it. The bug was trivial. The search was not.

The reason was structural, not incidental. Claude had written the code in
question. But its relationship to that code was identical to its relationship
to code it had never seen. There is no persistent authorship in a language
model. Each session begins from the corpus; the code is equally foreign
regardless of its provenance. Asking Claude to maintain software it had written
is not qualitatively different from asking it to maintain software written by a
stranger, because the language model makes no distinction between the two.

If the state flag had been a polymorphic object — if the four conditional
branches had been four implementations of a single interface, selected at
construction time — the missing case would have been a compilation error, not
a runtime surprise. The bug would have been structurally impossible. And Claude,
confronted with a `.trz` specification that made the design contract explicit,
would have been able to reason about its own output not heuristically, but
mechanically.

That observation is the origin of Trenza.

---

# §2 Motivation: The State Dispersal Problem

## 2.1 A Pattern, Not an Anecdote

The two cases described above — the network monitor and the time-tracking
application — exhibit the same structural pathology by different means. We
call it *state dispersal*: the condition in which the behavioral state of a
system is encoded implicitly, in fragments, across locations that have no
formal relationship to one another.

State dispersal is not a new problem. It is the problem that object-oriented
programming was supposed to solve, and solved only partially: encapsulation
prevents external mutation but does not prevent internal duplication. A
boolean flag checked in four places is dispersed state even if every check
is inside the same module. The language permits it. The programmer produces
it. The LLM faithfully reproduces it, because the corpus from which it learned
was written by the same programmers.

## 2.2 Case 1 — MonitoreoRed

MonitoreoRed is a home network monitoring system built over Prometheus,
Grafana, Blackbox Exporter, Node Exporter, and Docker, running on a fourteen-device
home network. It was built collaboratively with Claude in a single session,
from an empty directory, with no prior scaffolding.

The functional result was genuine: the system monitored device availability,
collected metrics, and exposed a Grafana dashboard. An unrequested feature —
vendor identification from MAC address using the IEEE OUI registry — was
introduced by Claude during the session and retained as a contribution.

The structural problem was discovered later. Device state was represented in
three incompatible forms:

| Representation | Location | What it captures |
|---|---|---|
| JSON snapshot | `dispositivos.json` | Known devices at last scan |
| Time-series | Prometheus TSDB | Metrics (latency, reachability) — not states |
| Point-in-time | `ping_check.py`, `network_scan.py` | Instantaneous presence — discarded after check |

There was no model of `DispositivoConocido → Ausente → AlertaActiva`. The
transition from "this device was present" to "this device has been absent long
enough to warrant an alert" was not representable in any of the three layers.
It had to be reconstructed each time from the intersection of three
independently maintained data structures — a reconstruction that neither the
system nor any assisting LLM could perform mechanically, because the rule was
nowhere written down.

The `claude-flow` framework, which uses a fifteen-agent coordination model to
manage complex AI workflows, represents one response to this kind of problem:
if the system is too complex for a single model to reason about, distribute the
reasoning. We reject this approach as a response to state dispersal. The
problem is not cognitive capacity — it is the absence of a shared, formal
account of what the system's state is. Adding agents without adding a state
model produces a system that is more complex and equally incoherent.

## 2.3 Case 2 — CronometroPSP

CronometroPSP is a personal software process (PSP) time-tracking application
with a mobile-first HTML/JavaScript interface. It was also built with Claude.
Its behavioral complexity is dominated by a modal UI with two distinct
interaction modes: normal operation (`ModoNormal`) and activity editing
(`ModoEdicion`).

The flag `modoEdicion` was checked in four independent locations:

1. The card tap handler in the main view
2. The activity tab tap handler
3. The frequent-activities tab handler
4. A secondary card interaction path

In practice, location 4 was missing the guard. On mobile, tapping a card in
edit mode opened the session dialog instead of the edit dialog. The bug was
real, reproducible, and intermittent (it required a specific navigation path
to trigger).

When Claude was asked to diagnose and fix the bug, the process was
disproportionate to the defect. The reason was not computational but
architectural: Claude had no privileged access to the code it had written.
The code was as opaque to it as any other code of similar length and
complexity. Reconstructing the intent of four conditional branches scattered
across several hundred lines required the same heuristic reasoning that would
have been required for foreign code — with the same probability of
misdiagnosis.

The correct fix was structural: `modoEdicion` should not have been a boolean
flag. It should have been the identity of a polymorphic object. The four
conditional branches should have been four method implementations, selected
at construction time by a factory. In that design, the missing case at
location 4 would have been a missing method implementation — a compilation
error, not a runtime defect.

## 2.4 The Common Structure

Both cases exhibit the same pattern:

> *A behavioral condition that should be a named, first-class entity in the
> system's design is instead encoded as a recurrence — a flag checked in
> multiple places, a state reconstructed from multiple sources — with no
> formal guarantee of consistency across occurrences.*

This pattern is not detectable by conventional static analysis, because no
individual occurrence is incorrect. It is detectable only by reasoning about
the design, which requires knowing the design. In a human-LLM collaboration
where the LLM has no persistent memory of the design decisions it participated
in, that knowledge is unavailable without an external artifact that makes it
explicit.

Trenza is that artifact.

---

*Draft — 2026-04-02.*
*§1 and §2 drafted from materials in the repository and oral account provided
in mobile session. To be reviewed and corrected by César Pérez-Chirinos before
further development.*
