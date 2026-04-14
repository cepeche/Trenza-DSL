# Bloque 2.3(A): Unificación de tipos primitivos

**Fecha:** 2026-04-14
**Author:** CO (Claude Opus via Dispatch — diseño y ejecución intelectual)
**Committed by:** CL (Claude Sonnet 4.6 via Claude Code)
**Tests:** 13/13 ✅

---

## Problema

Los tipos primitivos de Trenza estaban definidos de forma duplicada y divergente
en tres lugares distintos: `generator.rs` (match Rust), `generator.rs` (match TS)
y `pub_surface.rs` (match is_primitive). Cualquier adición o eliminación de un
primitivo requería actualizar tres matches de forma manual y sincronizada.

Adicionalmente, los primitivos históricos (`Texto`, `Entero`, `Decimal`, `Ninguno`,
`Root`, `Instant`) habían sido ya eliminados de la gramática, pero sus vestigios
permanecían en los matches.

## Solución: `trenza-core/src/primitives.rs`

Nueva fuente de verdad canónica para los tipos primitivos de Trenza.

```rust
pub struct Primitive {
    pub name: &'static str,
    pub rust_type: &'static str,
    pub ts_type: &'static str,
}

pub const PRIMITIVES: &[Primitive] = &[
    Primitive { name: "String",    rust_type: "String", ts_type: "string"  },
    Primitive { name: "Int",       rust_type: "i32",    ts_type: "number"  },
    Primitive { name: "Bool",      rust_type: "bool",   ts_type: "boolean" },
    Primitive { name: "ID",        rust_type: "String", ts_type: "string"  },
    Primitive { name: "Timestamp", rust_type: "u64",    ts_type: "number"  },
];
```

Funciones públicas: `is_primitive(name)`, `rust_type_of(name)`, `ts_type_of(name)`.

## Archivos modificados

| Archivo | Cambio |
|---------|--------|
| `trenza-core/src/primitives.rs` | **NUEVO** — fuente de verdad |
| `trenza-core/src/lib.rs` | `pub mod primitives;` añadido |
| `trenza-core/src/generator.rs` | `rust_type()` y `ts_type()` delegan a `primitives` |
| `trenza-core/src/pub_surface.rs` | `is_primitive()` local → `crate::primitives::is_primitive()` |
| `trenza-core/src/serializer.rs` | Tests actualizados a nombres canónicos (`String`, `Int`) |
| `AGENTS.md` | Mejoras al protocolo de resolución de conflictos y validación pre-push |

## Verificación de eliminación

Los primitivos eliminados (`None`, `Root`, `Instant`, `Decimal`) están
explícitamente testeados como NO-primitivos:

```rust
assert!(!is_primitive("None"));
assert!(!is_primitive("Root"));
assert!(!is_primitive("Instant"));
assert!(!is_primitive("Decimal"));
```

## Estado

Bloque 2.3(A) cerrado. 13/13 tests green.
