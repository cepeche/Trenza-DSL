//! Clasificación topológica de los contextos de un programa.
//!
//! Fuente única para el validador y el generador (antes el generador tenía
//! su propia copia). Cada contexto es exactamente uno de:
//!
//! - **base**: el `initial:` del sistema y los listados en `contexts:`.
//!   Mutuamente excluyentes; exactamente uno activo.
//! - **overlay**: listados en `overlays:`. Se apilan sobre el base.
//! - **concurrent**: listados en `concurrent:`. Coexisten con el base.
//! - **sub-contexto**: cualquier otro. Si pertenece a un overlay, `parent_of`
//!   lo indica (ver `classify`).
//!
//! Sobre esta clasificación se definen los **grupos de hermanos**
//! (`sibling_group`), que son el ámbito de las Reglas 1 y 5 (decisión del
//! 2026-09-25, ver history/chronicle/2026-09-25/).

use crate::ast::*;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct Topology {
    pub initial: String,
    pub bases: HashSet<String>,
    pub overlays: HashSet<String>,
    pub concurrents: HashSet<String>,
    pub sub_contexts: HashSet<String>,
    /// Sub-contexto → overlay al que pertenece.
    pub parent_of: BTreeMap<String, String>,
}

/// Grupo de contextos hermanos: comparten la obligación de declarar los
/// mismos roles (R5) y los mismos pares rol·evento (R1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SiblingGroup {
    /// Todos los contextos base del sistema.
    Base,
    /// Los sub-contextos de un mismo overlay.
    SubsOf(String),
    /// Contexto sin hermanos (overlay, concurrent o sub-contexto huérfano).
    Alone(String),
}

impl SiblingGroup {
    pub fn describe(&self) -> String {
        match self {
            SiblingGroup::Base => "los contextos base".to_string(),
            SiblingGroup::SubsOf(p) => format!("los sub-contextos de '{}'", p),
            SiblingGroup::Alone(c) => format!("'{}'", c),
        }
    }
}

impl Topology {
    pub fn sibling_group(&self, ctx: &str) -> SiblingGroup {
        if self.bases.contains(ctx) {
            SiblingGroup::Base
        } else if let Some(p) = self.parent_of.get(ctx) {
            SiblingGroup::SubsOf(p.clone())
        } else {
            SiblingGroup::Alone(ctx.to_string())
        }
    }
}

pub fn classify(program: &Program) -> Topology {
    let mut t = Topology::default();
    for def in &program.definitions {
        if let Definition::System(sys) = def {
            t.initial = sys.initial.clone();
            t.bases.insert(sys.initial.clone());
            for sec in &sys.sections {
                match sec {
                    SystemSection::Contexts(v) => t.bases.extend(v.iter().cloned()),
                    SystemSection::Overlays(v) => t.overlays.extend(v.iter().cloned()),
                    SystemSection::Concurrent(entries) => {
                        for e in entries {
                            match e {
                                ConcurrentEntry::Name(n) => { t.concurrents.insert(n.clone()); }
                                ConcurrentEntry::Anonymous(c) => { t.concurrents.insert(c.name.clone()); }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    for def in &program.definitions {
        if let Definition::Context(c) = def {
            if !t.bases.contains(&c.name) && !t.overlays.contains(&c.name) && !t.concurrents.contains(&c.name) {
                t.sub_contexts.insert(c.name.clone());
            }
        }
    }

    // parent_of por punto fijo: primero `initial: Sub` en un overlay, luego
    // transiciones directas (`on cerrar -> Overlay`) y, por último,
    // transiciones a un sub-contexto hermano cuyo padre ya se conoce.
    for def in &program.definitions {
        if let Definition::Context(ctx) = def {
            if !t.overlays.contains(&ctx.name) { continue; }
            if let Some(sub) = &ctx.initial_sub {
                t.parent_of.insert(sub.clone(), ctx.name.clone());
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for def in &program.definitions {
            if let Definition::Context(ctx) = def {
                if !t.sub_contexts.contains(&ctx.name) { continue; }
                if t.parent_of.contains_key(&ctx.name) { continue; }
                let mut found = ctx.transitions.iter()
                    .find(|tr| t.overlays.contains(&tr.target))
                    .map(|tr| tr.target.clone());
                if found.is_none() {
                    found = ctx.transitions.iter()
                        .find_map(|tr| t.parent_of.get(&tr.target).cloned());
                }
                if let Some(p) = found {
                    t.parent_of.insert(ctx.name.clone(), p);
                    changed = true;
                }
            }
        }
        // Hacia delante: un sub-contexto al que se llega desde el overlay o
        // desde un sub-contexto de padre conocido pertenece a ese overlay
        // (p. ej. un asistente Paso1 -> Paso2 cuyo Paso2 sólo sabe cerrar).
        for def in &program.definitions {
            if let Definition::Context(ctx) = def {
                let owner = if t.overlays.contains(&ctx.name) {
                    Some(ctx.name.clone())
                } else {
                    t.parent_of.get(&ctx.name).cloned()
                };
                let Some(owner) = owner else { continue; };
                for tr in &ctx.transitions {
                    if t.sub_contexts.contains(&tr.target) && !t.parent_of.contains_key(&tr.target) {
                        t.parent_of.insert(tr.target.clone(), owner.clone());
                        changed = true;
                    }
                }
            }
        }
    }
    t
}
