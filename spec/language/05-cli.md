# Trenza CLI

The Trenza command-line interface allows you to initialize, verify, and compile projects statically without leaving the terminal.

## Main Tool Commands
- `trenza init <project>`: Creates the basic folder structure and `.trz` scaffolding.
- `trenza verify`: Performs a dry-run compilation and verifies the 6 formal rules. Emits readable errors if, for example, a handler is missing for a state.
- `trenza generate`: Generates the four strands (implementation, tests, schematics, and requirements). Generates Rust by default.
- `trenza package`: Compiles an entire subsystem, creating the signed `.tzp` package.
