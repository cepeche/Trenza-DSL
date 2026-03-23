---
date: 2026-03-23
session: Rule 5 implementation brief for Gemini
from: Claude Sonnet 4.6
to: Gemini
---

# Briefing: Rule 5 — Role Exhaustiveness

## Estado actual del compilador

| Regla | Estado | Notas |
|-------|--------|-------|
| Rule 1 — Completeness | ✅ | Pass 2 en `validator.rs` |
| Rule 2 — Determinism | ✅ | Pass 1 en `validator.rs` |
| Rule 3 — Reachability | ✅ | Pass 3 en `validator.rs` |
| Rule 4 — Structural Least Privilege | ✅ | Implícita en `ActionTarget` enum (AST) |
| Rule 5 — Role Exhaustiveness | ❌ | **Objetivo de esta sesión** |
| Rule 6 — Data Conformance | ✅ | Pass 4 en `validator.rs` |

---

## Definición formal (spec/language/02-grammar.md, línea 719)

> **Every role declared in the `system` block must appear in all contexts
> of the system.**

### Distinción crítica respecto a Rule 1

- **Rule 1** verifica que cada `(rol, evento)` esté manejado en cada contexto.
- **Rule 5** verifica que el rol *exista* en cada contexto — aunque su
  comportamiento sea `ignored` en todos sus eventos.

Rule 1 puede pasar aunque un rol entero esté ausente de un contexto (si ese
contexto no declara ninguno de los eventos de ese rol). Rule 5 cierra ese hueco.

### Ejemplo concreto

Si el rol `carrito` aparece en `CarritoVacio` y `CarritoActivo`, debe estar
presente también en `ProcesandoPago` — aunque su único handler sea `ignored`.
Omitirlo es un error de diseño: significa que no se pensó en cómo se comporta
ese elemento de interfaz durante el pago.

---

## Algoritmo de implementación

No requiere cambios en la gramática (`trenza.pest`) ni en el AST (`ast.rs`).
Solo añadir un **Pass 5** en `validator.rs`, después del Pass 4 actual:

```rust
// Pass 5: Rule 5 (Role Exhaustiveness)
let mut all_roles: HashSet<String> = HashSet::new();
let mut context_roles: HashMap<String, HashSet<String>> = HashMap::new();

for def in &program.definitions {
    if let Definition::Context(ctx) = def {
        let mut roles = HashSet::new();
        for role in &ctx.roles {
            roles.insert(role.name.clone());
            all_roles.insert(role.name.clone());
        }
        context_roles.insert(ctx.name.clone(), roles);
    }
}

for ctx_name in &all_contexts {
    if let Some(roles) = context_roles.get(ctx_name) {
        for role_name in &all_roles {
            if !roles.contains(role_name) {
                errors.push(format!(
                    "ERROR [exhaustiveness]: role '{}' appears in other contexts \
                     but is absent from context '{}'",
                    role_name, ctx_name
                ));
            }
        }
    }
}
```

---

## Caso de prueba: `examples/broken_rule_5.trz`

El archivo existe pero actualmente **no dispara** Rule 5 — todos sus contextos
declaran los dos roles (`fuego` y `agua`).

Hay dos opciones:

**Opción A** — Modificar `broken_rule_5.trz` añadiendo un contexto que omita
uno de los roles:

```
context ModoEmergencia:
    input: r: Recurso
    role fuego: Recurso (bind: r)
        on trigger -> accionCalor()
    -- rol 'agua' ausente → dispara Rule 5
    transitions:
        on deactivate -> ModoNormal
```

**Opción B** — Crear un nuevo ejemplo `broken_rule_5_v2.trz` y conservar el
original como está.

Mi recomendación: **Opción A**. El archivo ya se llama `broken_rule_5`; lo
natural es que realmente rompa Rule 5.

---

## Formato del mensaje de error (según la spec)

```
ERROR [exhaustiveness]: role pestaña_frecuentes declared in the system
                        but absent from context ModoEdicion
```

---

## Verificación de finalización

Una vez implementado, el compilador debería:

1. Rechazar `broken_rule_5.trz` (tras la modificación del Opción A) con el
   mensaje de error correcto.
2. Aceptar `autenticacion-rgpd.trz` sin errores nuevos (todos sus contextos
   tienen los mismos roles).
3. Aceptar `iefbr14.trz` sin errores (no hay roles: Rule 5 es vacuamente
   verdadera con conjunto de roles vacío).

Con esto, las seis reglas de verificación de la spec estarán implementadas
en el compilador Rust. 🎯
