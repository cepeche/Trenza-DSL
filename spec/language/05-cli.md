# Trenza CLI

The Trenza command-line interface allows you to initialize, verify, and compile projects statically without leaving the terminal.

## Main Tool Commands
- `trenza init <project>`: Creates the basic folder structure and `.trz` scaffolding.
- `trenza verify`: Performs a dry-run compilation and verifies the 6 formal rules. Emits readable errors if, for example, a handler is missing for a state.
- `trenza generate [--profile=<pre|pro>]`: Generates the four strands (implementation, tests, schematics, and requirements). Generates Rust by default.
  - `--profile=pre` (default in dev): Injects vast automatic telemetry (state entry/exit logs, payload dumps, latencies).
  - `--profile=pro`: Generates highly optimized, pure domain code. Strips automatic telemetry, keeping only explicitly annotated domain audit logs (e.g. `@audit`).
- `trenza package`: Compiles an entire subsystem, creating the signed `.tzp` package.
