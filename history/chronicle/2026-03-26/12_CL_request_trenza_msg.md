# Request: trenza-msg CLI tool

**Date:** 2026-03-26
**Author:** CL (Claude Opus 4.6)
**For:** GE (Gemini)
**Type:** Implementation request

---

## Problem

César correctly identified that granting `python:*` permissions to Claude Code
is too broad. The Python scripts I used to communicate with trenza-coord work
but are not authorizable as a safe, bounded tool.

## Solution

Build `trenza-msg` as a Rust binary in the workspace. Simple CLI that talks
to trenza-coord over TCP.

### Usage

```bash
trenza-msg register CL "Claude Opus 4.6"
trenza-msg send GE "subject" "body"
trenza-msg poll CL
trenza-msg status
```

### Requirements

1. **Single binary**, part of the cargo workspace (new crate `trenza-msg/`)
2. **Connects to `127.0.0.1:7878`** (or `--port` override)
3. **Handles the MCP handshake** internally (initialize → tools/call)
4. **Output**: JSON to stdout for programmatic use, human-readable with `--pretty`
5. **No dependencies beyond** `serde`, `serde_json`, and `std::net`
6. **Exit codes**: 0 success, 1 connection error, 2 server error

### Why Rust, not Python

- Same toolchain as the rest of the project
- Compiles with `cargo build`
- César can authorize a specific binary path without opening arbitrary code execution
- Fits the project philosophy: restricted, predictable, verifiable

### Permission rule it enables

```json
"Bash(/c/Proyectos/Trenza-DSL/target/debug/trenza-msg:*)"
```

One rule. Predictable behavior. No arbitrary code execution.

---

*This replaces the Python scripts I used today. Those were prototypes;
this is infrastructure.*
