---
title: "Trenza: A Role-Based State Machine DSL for Human-AI Collaborative Specification and Synthesis"
authors: ["César Pérez-Chirinos¹", "Claude Sonnet 4.6²", "Claude Opus 4.6²", "Gemini 2.5 Pro³"]
status: draft
date: 2026-03-23
event: ONWARD! Essays 2026
---

# Abstract

Software defects caused by implicit, scattered state management and poorly governed data access are among the most costly to diagnose in practice. This work began with a concrete instance: a proof-of-concept application in which a single behavioral flag (`modoEdicion`) was scattered across four independent code locations—invisible to any static analysis, untestable in isolation, and fatal to long-term maintenance. Trenza was designed so that class of defect would instead be a compile-time error. We present **Trenza**, a domain-specific language in which system behavior is expressed as a set of named contexts, each declaring the roles, transitions, and effects that are valid within it. The language is deliberately restrictive: every role-event pair must be handled in every context (completeness), no handler may be duplicated (determinism), every context must be reachable, and data access is strictly scoped structurally (least privilege). 

These constraints are enforced in milliseconds at compile time by a Rust-based toolchain executing seven formal verification rules—including explicit GDPR data conformance and strict view composition constraints (`slot`/`fills`). This transforms behavioral errors that typically surface at runtime into errors that surface before a single line of implementation code is generated. Validated specifications undergo multi-strand synthesis simultaneously, projecting an algebraically tested Rust runtime, topological Mermaid diagrams, and narrative audit reports for complete semantic traceability. 

Crucially, Trenza is designed to be reasoned about collaboratively by both human developers and large language models (LLMs), repositioning the human from a "code typist" to a system "Architect" supported by an infallible "Logical CAD" environment. We validate this approach by formally verifying the **CronometroPSP** reference specification—a 16-module distributed system—demonstrating that 100+ logical dependencies and cross-context invariants can be mechanically verified in less than 100ms. Rather than merely reducing review time, the specification induced a shift in epistemic regime: review without the specification was heuristic; review with it became a mechanical, exhaustive, one-to-one deductive check. We argue that this regime shift—from "does this look correct?" to "does this mathematically correspond?"—represents a qualitative imperative for the verifiability of LLM-generated code. Finally, we explore Trenza's hybrid synthesis model for concurrent state machines, seamlessly generating both single-threaded composed states for browser environments and `mpsc`-driven threaded actors for high-performance backends.

