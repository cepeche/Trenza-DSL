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
                    
                    // Rule 2: Determinism (duplicate checks)
                    if ctx_re.contains(&re) {
                        errors.push(format!("ERROR [determinism]: El rol '{}' tiene manejadores duplicados para el evento '{}' en el contexto '{}'", role.name, action.event, ctx.name));
                    }
                    ctx_re.insert(re);
                }
            }
            context_role_events.insert(ctx.name.clone(), ctx_re);

            // Graph for Reachability
            let mut targets = Vec::new();
            for trans in &ctx.transitions {
                targets.push(trans.target.clone());
            }
            adjacency_list.insert(ctx.name.clone(), targets);
        }
    }

    // Pass 2: Rule 1 (Completeness)
    // Every role+event pair seen anywhere must exist in ALL contexts.
    for ctx_name in &all_contexts {
        if let Some(ctx_re) = context_role_events.get(ctx_name) {
            for re in &role_events {
                if !ctx_re.contains(re) {
                    errors.push(format!("ERROR [completeness]: La acción '{}.{}' está definida en otro sitio pero olvidada en el contexto '{}'", re.0, re.1, ctx_name));
                }
            }
        }
    }

    // Pass 3: Rule 3 (Reachability)
    if !initial_context.is_empty() && all_contexts.contains(&initial_context) {
        let mut visited = HashSet::new();
        let mut stack = vec![initial_context.clone()];
        
        while let Some(node) = stack.pop() {
            if visited.insert(node.clone()) {
                if let Some(neighbors) = adjacency_list.get(&node) {
                    for n in neighbors {
                        stack.push(n.clone());
                    }
                }
            }
        }
        
        for ctx_name in &all_contexts {
            if !visited.contains(ctx_name) {
                errors.push(format!("ERROR [reachability]: El contexto '{}' es código muerto (inalcanzable desde el initial '{}')", ctx_name, initial_context));
            }
        }
    } else if !initial_context.is_empty() {
         errors.push(format!("ERROR [reachability]: El contexto inicial '{}' declarado en system no existe", initial_context));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
