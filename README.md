# Trenza DSL

A DSL (formerly known as Helix) designed to be useful to both human developers
and LLMs. It is inspired by the structure of a **four-strand braid**: every
specification fragment simultaneously generates and consolidates its
**implementation**, **tests**, **schematic/documentation** and **requirements**,
so that all four verify each other and can never fall out of sync.

## Current status

**Current version:** v0.0.1

This is the first complete and validated specification of Trenza. All initial
design documents have been consolidated and hierarchically restructured to lay
the foundations for development.

## Origin

A conversation between a developer with experience since 1975 and Claude
(Sonnet 4.6), on 4 March 2026, prompted by the difficulty of diagnosing a
touch/click event bug in a web application. Three observations emerged:

- The accidental complexity of modern software makes diagnosis difficult even
  for LLMs with full access to the source code.
- Scattered conditionals (`if editMode`) are a structural bug vector.
- Current languages do not explicitly express state flows or guards, forcing
  the reader (human or LLM) to mentally reconstruct execution.

## Design hypotheses

1. **The Braid**: the specification of a requirement generates *four complementary
   artefacts* (the "four strands") — implementation, tests, schematic and
   **requirements** — which are unified projections of the same strict root
   artefact.
2. **Conditionals in factories**: all conditional code lives in factory methods.
   The rest of the code is polymorphic and does not know what state the system
   is in; it only sends messages. This makes it impossible to forget a case.
3. **Explicit state flows**: transitions between states and their lifecycle
   events are first-class citizens of the language, not implicit logic scattered
   across functions.
4. **Formal verifiability**: the DSL semantics must be sufficiently constrained
   to allow formal reasoning, not just execution.

## Project structure

Following the v0.0.0 consolidation, documentation and specification are divided
into:

- `spec/language/` — Formal language specification (grammar, verification, etc.)
- `spec/reference/` — Reference implementations and examples (e.g. the
  cronómetro-psp project)
- `docs/manual/` — User manuals and quick-start guides
- `docs/design/` — Design rationale documents
- `examples/` — Canonical `.trz` source examples
- `history/chronicle/` — Chronological record of the design evolution
  (including the foundational conversations), kept in Spanish
- `history/decisions/` — Architectural Decision Records (ADRs)
- `history/meta/` — Intellectual property guidelines and meta-reflections

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual license: AGPL-3.0 with a commercial option for AI model providers.
See [LICENSE](LICENSE) for details.
