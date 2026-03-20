# Trenza DSL: The Overview

Trenza is a Domain Specific Language (DSL), formerly known as Helix, designed from first principles to be equally useful to human developers and Large Language Models (LLMs).

## Core Philosophy
The core principle is that the formal specification of a system is the true source of truth. Trenza weaves together a "braid" (trenza) of four inseparable threads:
1. **Implementation** (Rust/WASM output)
2. **Tests** (Auto-generated algebraic unit tests)
3. **Schematics** (Mermaid statecharts)
4. **Requirements** (Formal intent mapping)

Instead of relying on scattered boolean flags in the host language (e.g. `if AppState.isEditing`), Trenza forces state management into declarative contexts, making missing event handlers a compile-time error.
