use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub fn verify(program: &Program) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    // Gather system metadata
    let mut initial_context = String::new();
    for def in &program.definitions {
        if let Definition::System(sys) = def {
            initial_context = sys.initial.clone();
        }
    }

    let mut all_contexts = HashSet::new();
    let mut role_events: HashSet<(String, String)> = HashSet::new();
    let mut context_role_events: HashMap<String, HashSet<(String, String)>> = HashMap::new();
    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

    // Pass 1: Build indexes & Check Determinism (Rule 2)
    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            all_contexts.insert(ctx.name.clone());
            let mut ctx_re = HashSet::new();
            
            for role in &ctx.roles {
                for action in &role.actions {
                    let re = (role.name.clone(), action.event.clone());
                    role_events.insert(re.clone());
                    
                    if ctx_re.contains(&re) {
                        errors.push(format!("ERROR [determinism]: El rol '{}' tiene manejadores duplicados para el evento '{}' en el contexto '{}'", role.name, action.event, ctx.name));
                    }
                    ctx_re.insert(re);
                }
            }
            context_role_events.insert(ctx.name.clone(), ctx_re);

            let mut targets = Vec::new();
            for trans in &ctx.transitions {
                targets.push(trans.target.clone());
            }
            adjacency_list.insert(ctx.name.clone(), targets);
        }
    }

    // Pass 2: Rule 1 (Completeness)
    for ctx_name in &all_contexts {
        if let Some(ctx_re) = context_role_events.get(ctx_name) {
            for re in &role_events {
                if !ctx_re.contains(re) {
                    errors.push(format!("ERROR [completeness]: La acción '{}.{}' no está declarada en el contexto '{}'", re.0, re.1, ctx_name));
                }
            }
        }
    }

    // Pass 3: Rule 3 (Reachability)
    if !initial_context.is_empty() {
        let mut visited = HashSet::new();
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
        
        for ctx_name in &all_contexts {
            if !visited.contains(ctx_name) {
                errors.push(format!("ERROR [reachability]: El contexto '{}' es inalcanzable", ctx_name));
            }
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
                                        errors.push(format!(
                                            "ERROR [privacy]: El rol '{}' de tipo '{}' en el contexto '{}' intenta acceder al campo protegido '{}' sin permiso [access: gdpr]",
                                            role.name, t, ctx.name, arg
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
