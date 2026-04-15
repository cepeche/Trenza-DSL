use crate::ast::*;
use crate::runtime::*;

pub struct Interpreter {
    pub program: Program,
    pub state: SystemState,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let mut initial_state = String::new();
        let mut concurrent_states = Vec::new();

        for def in &program.definitions {
            if let Definition::System(sys) = def {
                initial_state = sys.initial.clone();
                for sec in &sys.sections {
                    if let SystemSection::Concurrent(cc) = sec {
                        for entry in cc {
                            match entry {
                                ConcurrentEntry::Name(name) => { concurrent_states.push(name.clone()); },
                                ConcurrentEntry::Anonymous(ctx) => { concurrent_states.push(ctx.name.clone()); }
                            }
                        }
                    }
                }
            }
        }

        Interpreter {
            program,
            state: SystemState {
                current_state: initial_state,
                concurrent_states,
            },
        }
    }

    pub fn dispatch(&mut self, event: &str, _payload: &str) -> DispatchResult {
        let mut triggered_effects = Vec::new();
        let mut new_state = self.state.current_state.clone();

        // 1. Find the current context definition
        let current_ctx = self.program.definitions.iter().find_map(|def| {
            if let Definition::Context(ctx) = def {
                if ctx.name == self.state.current_state { Some(ctx) } else { None }
            } else { None }
        });

        if let Some(ctx) = current_ctx {
            // 2. Look for transitions
            for trans in &ctx.transitions {
                if trans.event == event {
                    new_state = if trans.target == "[stay]" {
                        self.state.current_state.clone()
                    } else {
                        trans.target.trim_matches(|c| c == '[' || c == ']').to_string()
                    };
                    break;
                }
            }

            // 3. Look for effects (Strand 4)
            for effect_rule in &ctx.effects {
                if let EffectTrigger::Event(e) = &effect_rule.trigger {
                    if e == event {
                        triggered_effects.push(Effect {
                            name: effect_rule.call.function.clone(),
                            args: effect_rule.call.args.clone(),
                        });
                    }
                }
            }
        }

        // Update internal state
        self.state.current_state = new_state.clone();

        DispatchResult {
            new_state,
            concurrent_states: self.state.concurrent_states.clone(),
            triggered_effects,
        }
    }
}
