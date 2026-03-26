# Briefing: MAPSE Vision and Strategic Reframing

**Date:** 2026-03-26
**Author:** CL (Claude Opus 4.6)
**Audience:** GE (Gemini), CL (Sonnet), César
**Type:** Strategic briefing + action items

---

## 1. Context: What happened in this session

This session was primarily "meta-project" — strategic analysis rather than
implementation. Key activities:

### 1.1 Review of Devis article on Claude ecosystem
Analyzed Ricardo Devis's LinkedIn article "Claude en marzo de 2026: tres cabezas
para un mismo dragón" (published 2026-03-25). Key takeaways relevant to Trenza:

- **Context rot is a documented industry problem.** Natural-language instructions
  degrade to ~80% compliance, dropping to near-zero after multiple context
  compactions. This validates Trenza's formal verification approach.
- **Hooks (deterministic event handlers) vs. instructions (probabilistic)** is the
  same dichotomy as Trenza's 8 formal rules vs. ad-hoc coding conventions.
- **Devis's thesis: Cowork/autonomous agents will compete with enterprise SaaS.**
  If true, those agents will need formal behavioral contracts — which is what
  .trz specifications could provide.

### 1.2 External documents produced (not in repo)
- Updated `Trenza_AI_Infrastructure_Costs.docx` — corrected for Gemini 3.1 pricing.
- Updated `Trenza_Valuation_Memo.docx` — added section on market repositioning
  based on Devis analysis (Trenza as "trust infrastructure for autonomous agents").
- Both in `~/Documents/ParaClaude/InformesTrenza/`.

### 1.3 AGENTS.md review
Confirmed current version incorporates all post-incident rules. No changes needed.

---

## 2. The MAPSE Vision — Strategic Reframing

César has proposed a fundamental reframing of the project's direction. This is the
most important content in this briefing.

### 2.1 The APSE analogy
Reference: the Stoneman report (DoD, 1980) defined three layers for the Ada
Programming Support Environment:
- **KAPSE** (Kernel): minimal runtime, compiler, OS interface
- **MAPSE** (Minimal): the tools needed for development
- **APSE** (Full): complete environment with all tools

César's assessment: **we already have the KAPSE** (compiler, 8 rules, VS Code
extension, multi-target generation). The next step is the **MAPSE**: a minimal
but complete development environment.

### 2.2 The design constraint that changes everything
The MAPSE must be **voice-first and accessibility-first**. The defining test case:

> A visually impaired person should be able to build a non-trivial verified system
> by speaking to their phone while walking.

This is not a hypothetical — it is a real accessibility need and a genuine design
constraint. If it works for someone who cannot see a screen, it works for everyone.

### 2.3 Why this is viable with Trenza (and not with general-purpose languages)
- Trenza's semantics are **restricted enough to be dictated**. You cannot reliably
  dictate Python or Rust. You can dictate a .trz specification because the syntax
  is constrained and the compiler can verify what you said.
- The 8 verification rules provide **feedback without visual inspection**. The
  compiler can report "unreachable state in ModoEdicion" via voice. No code
  reading required.
- The four strands mean a single conversation produces implementation, tests,
  diagrams, and audit trail simultaneously.

### 2.4 The key insight: the DSL is for the agents
César has confirmed what was implicit since the beginning: **.trz is not meant to
be written by humans**. The human provides *intent* through conversation. The
agents formalize that intent into .trz. The compiler verifies. Feedback returns
to the human via voice.

The chronicle (`history/chronicle/`) is not project documentation — it **is** the
requirements engineering process. Requirements are a strand that emerges from
dialogue, not a source document that precedes implementation.

### 2.5 Implications for the paper
The ONWARD! paper should be reframed. "A Role-Based State Machine DSL" undersells
what Trenza actually is. A more accurate framing: **a conversational specification
environment with formal verification**, where the DSL is the internal mechanism,
not the product.

---

## 3. Action Items

### For GE (Gemini):
1. **VS Code extension status**: Please document current state of the extension
   (what works, what doesn't) in a chronicle entry. César is unsure what was
   accomplished yesterday — we need clarity.
2. **Voice interaction exploration**: Evaluate how the VS Code extension could
   integrate with existing AI voice interfaces (Gemini in Android, Claude voice,
   VS Code Speech API). This is exploratory, not implementation.
3. **Continue `--out-dir` work** if not blocked by other priorities.

### For CL (Opus/Sonnet):
1. **Strategy document**: Writing a detailed proposal for the MAPSE development
   path (see companion document `02_CL_mapse_strategy.md`).
2. **Paper reframing**: Evaluate how to incorporate the MAPSE vision into the
   ONWARD! paper without derailing the submission timeline.
3. **MCP server design**: Propose architecture for a lightweight agent-to-agent
   coordination MCP server (see strategy document).

### For César:
1. Review strategy document and indicate priorities.
2. Monitor ONWARD! deadline (historically April-May).
3. Decision needed: prototype MAPSE before paper, or paper first?

---

## 4. Open Questions

1. Should the ONWARD! paper include the MAPSE vision, or should it remain focused
   on the DSL/compiler and mention MAPSE as future work?
2. What is the minimum viable voice interaction we could demonstrate?
3. How does the WASM demo GE built yesterday fit into the MAPSE story?

---

*Next entry: `02_CL_mapse_strategy.md` — detailed strategy proposal.*
