//! Single source of truth for Trenza primitive types.
//!
//! Any change to this list must be coordinated with the grammar
//! (trenza.pest) and documented in an ADR.
//!
//! Generic container types (List<X>) are NOT listed here — they are
//! handled separately in the generator because they take a type parameter.

/// A primitive Trenza type and its mappings to target languages.
pub struct Primitive {
    pub name: &'static str,
    pub rust_type: &'static str,
    pub ts_type: &'static str,
}

/// Canonical list of Trenza primitives.
///
/// To add or remove entries: write an ADR first, update this list,
/// and verify that `cargo test -p trenza-core` still passes.
pub const PRIMITIVES: &[Primitive] = &[
    Primitive { name: "String",    rust_type: "String", ts_type: "string"  },
    Primitive { name: "Int",       rust_type: "i32",    ts_type: "number"  },
    Primitive { name: "Bool",      rust_type: "bool",   ts_type: "boolean" },
    Primitive { name: "ID",        rust_type: "String", ts_type: "string"  },
    Primitive { name: "Timestamp", rust_type: "u64",    ts_type: "number"  },
];

/// Returns true if `name` is a recognised scalar primitive.
///
/// Note: this does NOT recognise the generic container `List<X>`.
/// Callers that need to accept containers must handle that separately.
pub fn is_primitive(name: &str) -> bool {
    name == "List" || PRIMITIVES.iter().any(|p| p.name == name)
}

/// Rust type for a scalar primitive, or `None` if the name is not a primitive.
pub fn rust_type_of(name: &str) -> Option<&'static str> {
    PRIMITIVES.iter().find(|p| p.name == name).map(|p| p.rust_type)
}

/// TypeScript type for a scalar primitive, or `None` if the name is not a primitive.
pub fn ts_type_of(name: &str) -> Option<&'static str> {
    PRIMITIVES.iter().find(|p| p.name == name).map(|p| p.ts_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_primitive_recognises_canonical_list() {
        for p in PRIMITIVES {
            assert!(is_primitive(p.name), "expected {} to be primitive", p.name);
        }
    }

    #[test]
    fn is_primitive_recognises_list_generic() {
        assert!(is_primitive("List"));
    }

    #[test]
    fn is_primitive_rejects_unknown() {
        assert!(!is_primitive("Foo"));
        assert!(!is_primitive(""));
        // Primitives removed by the 2026-04-14 cleanup must no longer
        // be recognised (see removal decision in session chronicle).
        assert!(!is_primitive("None"));
        assert!(!is_primitive("Root"));
        assert!(!is_primitive("Instant"));
        assert!(!is_primitive("Decimal"));
    }

    #[test]
    fn rust_and_ts_mappings_cover_all_primitives() {
        for p in PRIMITIVES {
            assert_eq!(rust_type_of(p.name), Some(p.rust_type));
            assert_eq!(ts_type_of(p.name), Some(p.ts_type));
        }
    }
}
