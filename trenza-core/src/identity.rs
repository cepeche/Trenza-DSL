use sha2::{Sha256, Digest};
use crate::ast::*;

/// Recorre el programa y asigna una identidad canónica a los componentes anónimos.
/// Según ADR-021, la identidad es _<hash6> donde hash6 es el truncamiento de 24 bits de SHA-256.
pub fn assign_identities(program: &mut Program) {
    for def in &mut program.definitions {
        if let Definition::System(sys) = def {
            for section in &mut sys.sections {
                if let SystemSection::Concurrent(entries) = section {
                    for entry in entries {
                        if let ConcurrentEntry::Anonymous(ctx) = entry {
                            let hash = compute_hash(ctx);
                            ctx.name = format!("_{}", hash);
                            ctx.is_anonymous = true;
                            ctx.name_span = ctx.span.clone(); // Ocupa todo el bloque
                        }
                    }
                }
            }
        }
    }
}

fn compute_hash(ctx: &ContextDef) -> String {
    // Usamos el serializador ToTrz que ya garantiza un orden determinista
    // de campos, roles y cláusulas según la implementación en ast.rs.
    let canonical_view = ctx.to_trz();
    
    let mut hasher = Sha256::new();
    hasher.update(canonical_view.as_bytes());
    let result = hasher.finalize();
    
    // Truncamos a 24 bits (6 caracteres hexadecimales)
    let hash_hex = hex::encode(result);
    hash_hex[..6].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_stability() {
        let ctx1 = ContextDef {
            span: Span { start: Pos { line: 1, col: 1 }, end: Pos { line: 2, col: 2 } },
            name: "".into(),
            name_span: Span { start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 1 } },
            is_public: false,
            inputs: vec![],
            roles: vec![],
            transitions: vec![],
            effects: vec![],
            slots: vec![],
            fills: vec![],
            ignore_rest: false,
            is_anonymous: false,
            initial_sub: None,
        };
        
        let hash1 = compute_hash(&ctx1);
        let hash2 = compute_hash(&ctx1);
        
        assert_eq!(hash1, hash2, "El hash debe ser determinista");
        assert_eq!(hash1.len(), 6, "El hash debe tener 6 caracteres (24 bits)");
    }
}
