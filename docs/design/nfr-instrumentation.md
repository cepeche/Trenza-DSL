# Non-Functional Requirements (NFR) Instrumentation

## Overview
Traditionally, telemetry, observability, and audit logs pollute imperative business logic. In Trenza, the compiler possesses absolute context of state changes and transitions. This allows it to inject non-functional requirements (NFRs) automatically or selectively via decorators, keeping the domain logic pure.

## Environment Profiles
The compiler (`trenza generate --profile=<pre|pro>`) manages telemetry injection:

1. **Pre-production (`pre`)**: The compiler blindly injects vast telemetry:
   - State entry/exit traces (`[on_entry]`, `[on_exit]`).
   - Detailed transition payloads and external call latencies.
2. **Production (`pro`)**: The compiler strips all implicit telemetry, yielding highly optimized code. 

## Domain Audit Logs (`@audit`)
Certain logs are domain requirements (e.g. Legal: "Record when an operator cancels an order"). These must survive the `pro` compilation profile.
Trenza solves this by providing **Decorators** on transitions or states.

### Syntax Proposal
Decorators are prefixed with `@` and placed immediately before a transition definition.

```trenza
transitions:
    @audit("Legal: Registro de cancelación de pedido de cliente")
    on cancelar.ok -> PedidoCancelado
```

### Compiler Behavior
In production, the compiler ignores generic transitions but generates specific hook calls or infrastructure code for annotated interactions, guaranteeing immutable auditing without entangling the state machine definitions.
