use crate::ast::*;
use std::collections::{HashMap, HashSet};



pub fn verify(program: &Program) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    
    // Gather system metadata
    let mut initial_context = String::new();
    let mut overlays = HashSet::new();
    let mut concurrent_contexts = HashSet::new();
    let mut base_contexts = HashSet::new();
    
    for def in &program.definitions {
        if let Definition::System(sys) = def {
            initial_context = sys.initial.clone();
            base_contexts.insert(initial_context.clone());
            for sec in &sys.sections {
                match sec {
                    SystemSection::Overlays(ov) => {
                        for o in ov { overlays.insert(o.clone()); }
                    },
                    SystemSection::Concurrent(cc) => {
                        for entry in cc {
                            match entry {
                                ConcurrentEntry::Name(name) => { concurrent_contexts.insert(name.clone()); },
                                ConcurrentEntry::Anonymous(ctx) => { concurrent_contexts.insert(ctx.name.clone()); }
                            }
                        }
                    },
                    SystemSection::Contexts(ctxs) => {
                        for c in ctxs { base_contexts.insert(c.clone()); }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut all_contexts = HashSet::new();
    let mut ignore_rest_contexts = HashSet::new();
    let mut role_events: HashSet<(String, String)> = HashSet::new();
    let mut context_role_events: HashMap<String, HashSet<(String, String)>> = HashMap::new();
    let mut context_roles: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_roles: HashSet<String> = HashSet::new();
    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

    // Pass 0: Prefix check (ADR-021)
    for def in &program.definitions {
        match def {
            Definition::Data(d) => {
                if d.name.starts_with('_') { errors.push(name_reserved_error(&d.name, &d.name_span)); }
                for f in &d.fields {
                    if f.name.starts_with('_') { errors.push(name_reserved_error(&f.name, &d.name_span)); }
                }
            },
            Definition::Context(c) => {
                if c.name.starts_with('_') { errors.push(name_reserved_error(&c.name, &c.name_span)); }
                for i in &c.inputs {
                    if i.name.starts_with('_') { errors.push(name_reserved_error(&i.name, &c.name_span)); }
                }
                for r in &c.roles {
                    if r.name.starts_with('_') { errors.push(name_reserved_error(&r.name, &r.name_span)); }
                    for a in &r.actions {
                        if a.event.starts_with('_') { errors.push(name_reserved_error(&a.event, &r.name_span)); }
                    }
                }
                for s in &c.slots {
                    if s.name.starts_with('_') { errors.push(name_reserved_error(&s.name, &c.name_span)); }
                }
                for t in &c.transitions {
                    if t.event.starts_with('_') { errors.push(name_reserved_error(&t.event, &c.name_span)); }
                }
                for fills in &c.fills {
                    for r in &fills.roles {
                        if r.name.starts_with('_') { errors.push(name_reserved_error(&r.name, &r.name_span)); }
                        for a in &r.actions {
                            if a.event.starts_with('_') { errors.push(name_reserved_error(&a.event, &r.name_span)); }
                        }
                    }
                }
            },
            Definition::Enum(e) => {
                if e.name.starts_with('_') { errors.push(name_reserved_error(&e.name, &e.name_span)); }
                for v in &e.variants {
                    if v.starts_with('_') { errors.push(name_reserved_error(v, &e.name_span)); }
                }
            },
            Definition::System(s) => {
                if s.name.starts_with('_') { errors.push(name_reserved_error(&s.name, &s.name_span)); }
                for sec in &s.sections {
                    if let SystemSection::Events(evs) = sec {
                        for e in evs {
                            if e.starts_with('_') { errors.push(name_reserved_error(e, &s.name_span)); }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    if !errors.is_empty() { return Err(errors); }

    // Pass 1: Build indexes & Check Determinism (Rule 2)
            let mut context_spans: HashMap<String, Span> = HashMap::new();
            let mut role_spans: HashMap<String, Span> = HashMap::new();
            
            for def in &program.definitions {
                match def {
                    Definition::Context(ctx) => {
                        context_spans.insert(ctx.name.clone(), ctx.name_span.clone());
                        for role in &ctx.roles {
                            role_spans.insert(format!("{}.{}", ctx.name, role.name), role.name_span.clone());
                        }
                    },
                    Definition::Data(_) => {},
                    Definition::External(_) => {},
                    Definition::System(_) => {},
                    Definition::Import(_) => {},
                    Definition::Enum(_) => {},
                }
            }

            for def in &program.definitions {
                if let Definition::Context(ctx) = def {
                    all_contexts.insert(ctx.name.clone());
                    if ctx.ignore_rest {
                        ignore_rest_contexts.insert(ctx.name.clone());
                    }

                    let mut ctx_re = HashSet::new();
                    let mut ctx_r = HashSet::new();
                    
                    for role in &ctx.roles {
                        ctx_r.insert(role.name.clone());
                        all_roles.insert(role.name.clone());
                        for action in &role.actions {
                            let re = (role.name.clone(), action.event.clone());
                            role_events.insert(re.clone());
                            
                            if ctx_re.contains(&re) {
                                errors.push(Diagnostic {
                                span: role.span.clone(),
                                    message: format!("El rol '{}' tiene manejadores duplicados para el evento '{}' en el contexto '{}'", role.name, action.event, ctx.name),
                                    severity: "error".to_string(),
                                    code: "determinism".to_string(),
                                });
                            }
                            ctx_re.insert(re);
                        }
                    }
            context_role_events.insert(ctx.name.clone(), ctx_re);
            context_roles.insert(ctx.name.clone(), ctx_r);

            let mut targets = Vec::new();
            for trans in &ctx.transitions {
                let target = if trans.target == "[close_overlay]" || trans.target == "[deactivate]" {
                    initial_context.clone()
                } else {
                    // Remove brackets if it's a simple context name like [ContextName]
                    trans.target.trim_matches(|c| c == '[' || c == ']').to_string()
                };
                targets.push(target);
            }
            // `initial: Sub` en un overlay: al abrirlo se apila también Sub
            // (mismo comportamiento que el generador), así que es una arista.
            if let Some(sub) = &ctx.initial_sub {
                targets.push(sub.clone());
            }
            adjacency_list.insert(ctx.name.clone(), targets);
        }
    }

    // Pass 2: Rule 1 (Completeness), por grupos de hermanos.
    //
    // Ámbito (decisión 2026-09-25): un par rol·evento manejado en un contexto
    // debe estar manejado en todos sus HERMANOS (topology::SiblingGroup): los
    // contextos base entre sí, los sub-contextos de un mismo overlay entre
    // sí. Un overlay no hereda los roles del base que suspende. Los
    // contextos con `role *` siguen exentos.
    let topo = crate::topology::classify(program);
    let group_of = |c: &String| topo.sibling_group(c);
    let mut group_pairs: HashMap<crate::topology::SiblingGroup, HashSet<(String, String)>> = HashMap::new();
    let mut group_roles: HashMap<crate::topology::SiblingGroup, HashSet<String>> = HashMap::new();
    for (ctx_name, ctx_re) in &context_role_events {
        group_pairs.entry(group_of(ctx_name)).or_default().extend(ctx_re.iter().cloned());
    }
    for (ctx_name, roles) in &context_roles {
        group_roles.entry(group_of(ctx_name)).or_default().extend(roles.iter().cloned());
    }
    let mut sorted_contexts: Vec<&String> = all_contexts.iter().collect();
    sorted_contexts.sort();
    for ctx_name in &sorted_contexts {
        if ignore_rest_contexts.contains(*ctx_name) { continue; }
        if let Some(ctx_re) = context_role_events.get(*ctx_name) {
            let context_span = context_spans.get(*ctx_name).cloned().unwrap_or(Span { 
                start: Pos { line: 1, col: 1 }, 
                end: Pos { line: 1, col: 10 } 
            });
            let group = group_of(ctx_name);
            let mut missing: Vec<&(String, String)> = group_pairs[&group].iter().filter(|re| !ctx_re.contains(*re)).collect();
            missing.sort();
            for re in missing {
                errors.push(Diagnostic {
                    span: context_span.clone(),
                    message: format!("La acción '{}.{}' no está declarada en el contexto '{}' (se maneja en {})", re.0, re.1, ctx_name, group.describe()),
                    severity: "error".to_string(),
                    code: "completeness".to_string(),
                });
            }
        }
    }

    // Pass 3: Rule 3 (Reachability)
    let mut visited = HashSet::new();
    if !initial_context.is_empty() {
        let mut stack: Vec<String> = vec![initial_context.clone()];
        
        while let Some(node) = stack.pop() {
            if visited.insert(node.clone()) {
                if let Some(neighbors) = adjacency_list.get(&node) {
                    for n in neighbors {
                        if n != "[stay]" {
                            stack.push(n.clone());
                        }
                    }
                }
            }
        }
    }
        
    for ctx_name in &all_contexts {
        if !visited.contains(ctx_name) {
            errors.push(Diagnostic {
                span: context_spans.get(ctx_name).cloned().unwrap_or(Span { 
                    start: Pos { line: 1, col: 1 }, 
                    end: Pos { line: 1, col: 10 } 
                }),
                message: format!("El contexto '{}' es inalcanzable", ctx_name),
                severity: "warning".to_string(),
                code: "reachability".to_string(),
            });
        }
    }

    // Pass 4: Rule 6 (Data Conformance)
    let mut data_privacy = HashMap::new();
    for def in &program.definitions {
        if let Definition::Data(d) = def {
            for (k, v) in &d.annotations {
                if k == "privacy" && v == "gdpr" {
                    data_privacy.insert(d.name.clone(), "gdpr".to_string());
                }
            }
        }
    }

    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            for role in &ctx.roles {
                let mut role_has_gdpr = false;
                for (k, v) in &role.annotations {
                    if k == "access" && v == "gdpr" {
                        role_has_gdpr = true;
                    }
                }

                for action in &role.actions {
                    if let ActionTarget::Call(call) = &action.target {
                        for arg in &call.args {
                            if arg.contains('.') {
                                let parts: Vec<&str> = arg.split('.').collect();
                                let var_name = parts[0];
                                
                                let mut var_type = None;
                                if role.name == var_name { var_type = Some(&role.datatype); }
                                else if let Some(b) = &role.binding {
                                    if b == var_name { var_type = Some(&role.datatype); }
                                }
                                
                                if var_type.is_none() {
                                    for input in &ctx.inputs {
                                        if input.name == var_name { var_type = Some(&input.datatype); break; }
                                    }
                                }
                                if var_type.is_none() {
                                    for r in &ctx.roles {
                                        if r.name == var_name { var_type = Some(&r.datatype); break; }
                                        if let Some(b) = &r.binding {
                                            if b == var_name { var_type = Some(&r.datatype); break; }
                                        }
                                    }
                                }

                                if let Some(t) = var_type {
                                    if data_privacy.contains_key(t) && !role_has_gdpr {
                                        errors.push(Diagnostic {
                                        span: role.span.clone(),
                                            message: format!(
                                                "El rol '{}' de tipo '{}' en el contexto '{}' intenta acceder al campo protegido '{}' sin permiso [access: gdpr]",
                                                role.name, t, ctx.name, arg
                                            ),
                                            severity: "error".to_string(),
                                            code: "privacy".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Pass 5: Rule 5 (Role Exhaustiveness), por grupos de hermanos (ver Pass 2).
    for ctx_name in &sorted_contexts {
        if ignore_rest_contexts.contains(*ctx_name) { continue; }
        let Some(roles) = context_roles.get(*ctx_name) else { continue; };
        let group = group_of(ctx_name);
        let mut missing: Vec<&String> = group_roles[&group].iter().filter(|r| !roles.contains(*r)).collect();
        missing.sort();
        for role_name in missing {
            errors.push(Diagnostic {
                span: context_spans.get(*ctx_name).cloned().unwrap_or(Span { 
                start: Pos { line: 1, col: 1 }, 
                end: Pos { line: 1, col: 10 } 
            }),
                message: format!("role '{}' appears in {} but is absent from context '{}'", role_name, group.describe(), ctx_name),
                severity: "error".to_string(),
                code: "exhaustiveness".to_string(),
            });
        }
    }

    // Pass 5.1: Rule 8 (Role Type Consistency)
    let mut role_types: HashMap<String, String> = HashMap::new();
    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            for role in &ctx.roles {
                let existing = role_types.entry(role.name.clone()).or_insert(role.datatype.clone());
                if existing != &role.datatype {
                    errors.push(Diagnostic {
                        span: role.span.clone(),
                        message: format!("role '{}' has conflicting types: '{}' and '{}'", role.name, existing, role.datatype),
                        severity: "error".to_string(),
                        code: "type-consistency".to_string(),
                    });
                }
            }
                for fills in &ctx.fills {
                    for role in &fills.roles {
                        let existing = role_types.entry(role.name.clone()).or_insert(role.datatype.clone());
                        if existing != &role.datatype {
                            errors.push(Diagnostic {
                                span: role.span.clone(),
                                message: format!("role '{}' in fills has conflicting types: '{}' and '{}'", role.name, existing, role.datatype),
                                severity: "error".to_string(),
                                code: "type-consistency".to_string(),
                            });
                        }
                    }
                }
        }
    }

    // Pass 6: Rule 4 (Return/No Sinks)
    let mut reversed_adj: HashMap<String, Vec<String>> = HashMap::new();
    for (src, targets) in &adjacency_list {
        for dst in targets {
            if dst != "[stay]" {
                reversed_adj.entry(dst.clone()).or_default().push(src.clone());
            }
        }
    }

    let mut can_return = HashSet::new();
    if !initial_context.is_empty() {
        let mut stack = vec![initial_context.clone()];
        while let Some(node) = stack.pop() {
            if can_return.insert(node.clone()) {
                if let Some(parents) = reversed_adj.get(&node) {
                    for p in parents {
                        stack.push(p.clone());
                    }
                }
            }
        }
    }

    for ctx_name in &all_contexts {
        if !can_return.contains(ctx_name) {
            errors.push(Diagnostic {
                span: context_spans.get(ctx_name).cloned().unwrap_or(Span { 
                    start: Pos { line: 1, col: 1 }, 
                    end: Pos { line: 1, col: 10 } 
                }),
                message: format!("context '{}' cannot return to the initial state '{}'", ctx_name, initial_context),
                severity: "error".to_string(),
                code: "return".to_string(),
            });
        }
    }

    // Pass 7: Rule 7 (Slot/Fills Integrity)
    let mut slot_index: HashSet<(String, String)> = HashSet::new();
    for def in &program.definitions {
        if let Definition::Context(c) = def {
            for slot in &c.slots {
                slot_index.insert((c.name.clone(), slot.name.clone()));
            }
        }
    }

    let mut fills_index: HashMap<(String, String), Vec<(String, FillsDef)>> = HashMap::new();
    for def in &program.definitions {
        if let Definition::Context(c) = def {
            for fills in &c.fills {
                let key = (fills.target_context.clone(), fills.target_slot.clone());
                
                // Rule S1
                if !slot_index.contains(&key) {
                    errors.push(Diagnostic {
                        span: c.name_span.clone(),
                        message: format!("context '{}' declares fills {}.{} but {} does not declare that slot", c.name, fills.target_context, fills.target_slot, fills.target_context),
                        severity: "error".to_string(),
                        code: "slot".to_string(),
                    });
                    continue;
                }
                fills_index.entry(key).or_default().push((c.name.clone(), fills.clone()));
            }
        }
    }

    // Rule S3
    for (key, sources) in &fills_index {
        if sources.len() > 1 {
            let names: Vec<String> = sources.iter().map(|s| s.0.clone()).collect();
            errors.push(Diagnostic {
                span: context_spans.get(&key.0).cloned().unwrap_or(Span { 
                    start: Pos { line: 1, col: 1 }, 
                    end: Pos { line: 1, col: 10 } 
                }),
                message: format!("contexts {} both declare fills for {}.{}. Declare priority in the system block", names.join(", "), key.0, key.1),
                severity: "error".to_string(),
                code: "slot-conflict".to_string(),
            });
        }
    }

    // Rule S4
    for (key, sources) in &fills_index {
        for (source_name, fills_def) in sources {
            let mut seen_role_events = HashSet::new();
            for role in &fills_def.roles {
                for action in &role.actions {
                    let re = (role.name.clone(), action.event.clone());
                    if !seen_role_events.insert(re) {
                        errors.push(Diagnostic {
                            span: role.span.clone(),
                            message: format!("role '{}' has duplicate handlers for event '{}' in fills {}.{} of context '{}'", role.name, action.event, key.0, key.1, source_name),
                            severity: "error".to_string(),
                            code: "determinism-fills".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Pass 8: Rule 9 (Import Integrity - ADR-022)
    let mut import_names = Vec::new();
    let mut system_names = Vec::new();
    for def in &program.definitions {
        match def {
            Definition::Import(i) => import_names.push((i.name.clone(), i.span.clone())),
            Definition::System(s) => system_names.push(s.name.clone()),
            _ => {}
        }
    }

    for (imp_name, imp_span) in import_names {
        // If there are systems in the program, one must match the import name.
        // Option B: Allow 0 systems (data-only package), but if there is one, it must match.
        if !system_names.is_empty() {
            if !system_names.contains(&imp_name) {
                errors.push(Diagnostic {
                    span: imp_span,
                    message: format!(
                        "El componente importado '{}' no define el sistema esperado '{}' (encontrado: {})",
                        imp_name, imp_name, system_names.join(", ")
                    ),
                    severity: "error".to_string(),
                    code: "import-mismatch".to_string(),
                });
            }
        }
    }

    // Rule (Initial sub-context integrity): `initial: X` on an overlay context
    // must name a sub-context whose transitions lead back to the overlay. The
    // check mirrors `parent_of` inference used by the generator: a context is a
    // sub-context of the overlay if any of its transitions target that overlay
    // (directly or via a sibling sub-context).
    //
    // We only emit a diagnostic on the direct-parent case here; deeper graphs
    // fall back to a soft check (existence-only) to avoid false negatives while
    // the fixed-point inference lives only in the generator.
    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            let Some(init) = ctx.initial_sub.as_ref() else { continue; };
            // `initial:` only makes sense on overlays.
            if !overlays.contains(&ctx.name) {
                errors.push(Diagnostic {
                    span: ctx.name_span.clone(),
                    message: format!(
                        "'initial:' declarado en '{}', que no es un overlay. Esta cláusula sólo se aplica a overlays con sub-contextos.",
                        ctx.name
                    ),
                    severity: "error".to_string(),
                    code: "initial-not-overlay".to_string(),
                });
                continue;
            }
            // The referenced context must exist.
            if !all_contexts.contains(init) {
                errors.push(Diagnostic {
                    span: ctx.name_span.clone(),
                    message: format!(
                        "'initial: {}' referencia un contexto que no existe.",
                        init
                    ),
                    severity: "error".to_string(),
                    code: "initial-unknown".to_string(),
                });
                continue;
            }
            // The referenced context must be a sub-context — i.e. it cannot
            // itself be a base, overlay, or concurrent (those have different
            // stack semantics). The generator infers the parent_of relation
            // via fixed-point on transitions to overlays / known sub-contexts;
            // here we only check the structural exclusion, which is sufficient
            // and tolerates `[close_overlay]`-style cerrar handlers (which do
            // not name the parent explicitly).
            let is_base = base_contexts.contains(init);
            let is_overlay = overlays.contains(init);
            let is_concurrent = concurrent_contexts.contains(init);
            if is_base || is_overlay || is_concurrent {
                errors.push(Diagnostic {
                    span: ctx.name_span.clone(),
                    message: format!(
                        "'initial: {}' debe ser un sub-contexto (no un base, overlay o concurrent). '{}' está declarado como {}.",
                        init, init,
                        if is_base { "context base" }
                        else if is_overlay { "overlay" }
                        else { "concurrent" }
                    ),
                    severity: "error".to_string(),
                    code: "initial-wrong-kind".to_string(),
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn name_reserved_error(name: &str, span: &Span) -> Diagnostic {
    Diagnostic {
        span: span.clone(),
        message: format!("El identificador '{}' no puede empezar por '_'. El prefijo '_' está reservado para uso interno del compilador (ADR-021).", name),
        severity: "error".to_string(),
        code: "name-reserved".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;

    #[test]
    fn test_import_mismatch() {
        let source = "
use Reloj#abc
system Cronometro:
  initial: C1
";
        let program = parse_file(source).unwrap();
        let result = verify(&program);
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.code == "import-mismatch"));
    }

    #[test]
    fn test_import_match() {
        let source = "
use Reloj#abc
system Reloj:
  initial: C1
";
        let program = parse_file(source).unwrap();
        let result = verify(&program);
        // Might fail other rules, but not import-mismatch
        if let Err(errs) = result {
            assert!(!errs.iter().any(|e| e.code == "import-mismatch"));
        }
    }

    #[test]
    fn test_import_data_only_package() {
        let source = "
use MyTypes#abc
data Foo:
  x: Entero
";
        let program = parse_file(source).unwrap();
        let result = verify(&program);
        // Option B: No systems -> OK
        if let Err(errs) = result {
            assert!(!errs.iter().any(|e| e.code == "import-mismatch"));
        }
    }

    #[test]
    fn test_name_reservation() {
        let source = "
data _Privado:
  secreto: Texto
";
        let program = crate::parser::parse_file(source).unwrap();
        let result = verify(&program);
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.code == "name-reserved"));
    }

    #[test]
    fn test_field_reservation() {
        let source = "
data Usuario:
  _uid: Texto
";
        let program = crate::parser::parse_file(source).unwrap();
        let result = verify(&program);
        assert!(result.is_err());
    }
}
