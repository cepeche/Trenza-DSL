# The Trenza Package (.tzp)

Trenza compiles specifications into self-contained `.tzp` (Trenza Package) packages, originally known by the legacy extension `.helixpkg`.

## Package Structure
A `.tzp` package is a cryptographically protected zip container that includes:
- The compiled `.trz` source code.
- The auto-generated test harnesses.
- The embedded WASM implementations.
- A `manifest.json` that mathematically signs and binds all components from the AST.

## Autonomous Execution
The compiler enforces the architectural principle of *self-contained solutions* (rUv): the distributable output is completely autonomous. Any system that consumes a `.tzp` instantly inherits the static guarantees formed at the source.
