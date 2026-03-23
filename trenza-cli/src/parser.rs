use crate::ast::*;
use crate::TrenzaParser;
use crate::Rule;
use pest::Parser;

pub fn parse_file(content: &str) -> std::result::Result<Program, pest::error::Error<Rule>> {
    let mut pairs = TrenzaParser::parse(Rule::program, content)?;
    let mut definitions = Vec::new();
    
    if let Some(program_pair) = pairs.next() {
        if program_pair.as_rule() == Rule::program {
            for inner in program_pair.into_inner() {
                match inner.as_rule() {
                    Rule::definition => {
                        let def_inner = inner.into_inner().next().unwrap();
                        match def_inner.as_rule() {
                            Rule::data_def => definitions.push(Definition::Data(parse_data(def_inner))),
                            Rule::system_def => definitions.push(Definition::System(parse_system(def_inner))),
                            Rule::context_def => definitions.push(Definition::Context(parse_context(def_inner))),
                            _ => {}
                        }
                    },
                    Rule::external_def => definitions.push(Definition::External(parse_external(inner))),
                    _ => {}
                }
            }
        }
    }
    
    Ok(Program { definitions })
}

fn parse_data(pair: pest::iterators::Pair<Rule>) -> DataDef {
    let mut name = String::new();
    let mut annotation = None;
    let mut fields = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::data_annotation => {
                let mut iter = inner.into_inner();
                let k = iter.next().unwrap().as_str().to_string();
                let v = iter.next().unwrap().as_str().to_string();
                annotation = Some((k, v));
            },
            Rule::data_field => {
                let mut iter = inner.into_inner();
                let curr_name = iter.next().unwrap().as_str().to_string();
                let curr_type = iter.next().unwrap().as_str().to_string();
                fields.push((curr_name, curr_type));
            },
            _ => {}
        }
    }
    DataDef { name, annotation, fields }
}

fn parse_external(pair: pest::iterators::Pair<Rule>) -> ExternalDef {
    let mut name = String::new();
    let mut annotation = None;
    let mut actions = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::data_annotation => {
                let mut iter = inner.into_inner();
                let k = iter.next().unwrap().as_str().to_string();
                let v = iter.next().unwrap().as_str().to_string();
                annotation = Some((k, v));
            },
            Rule::external_action => {
                let mut a_iter = inner.into_inner();
                let a_name = a_iter.next().unwrap().as_str().to_string();
                let mut params = Vec::new();
                let mut responses = Vec::new();
                
                for a_inner in a_iter {
                    match a_inner.as_rule() {
                        Rule::action_params => {
                            let mut p_iter = a_inner.into_inner();
                            while let (Some(pk), Some(pv)) = (p_iter.next(), p_iter.next()) {
                                params.push((pk.as_str().to_string(), pv.as_str().to_string()));
                            }
                        },
                        Rule::external_response => {
                            let mut r_iter = a_inner.into_inner();
                            let r_k = r_iter.next().unwrap().as_str().to_string();
                            let r_v = r_iter.next().unwrap().as_str().to_string();
                            responses.push((r_k, r_v));
                        },
                        _ => {}
                    }
                }
                actions.push(ExternalAction { name: a_name, params, responses });
            },
            _ => {}
        }
    }
    ExternalDef { name, annotation, actions }
}

fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut name = String::new();
    let mut initial = String::new();
    let mut sections = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => if name.is_empty() { name = inner.as_str().to_string(); } else { initial = inner.as_str().to_string(); },
            Rule::system_sections => {
                let sec_inner = inner.into_inner().next().unwrap();
                match sec_inner.as_rule() {
                    Rule::context_list => {
                        let idents = sec_inner.into_inner().map(|p| p.as_str().to_string()).collect();
                        sections.push(SystemSection::Contexts(idents));
                    },
                    Rule::concurrent_list => {
                        let idents = sec_inner.into_inner().map(|p| p.as_str().to_string()).collect();
                        sections.push(SystemSection::Concurrent(idents));
                    },
                    Rule::overlay_list => {
                        let idents = sec_inner.into_inner().map(|p| p.as_str().to_string()).collect();
                        sections.push(SystemSection::Overlays(idents));
                    },
                    Rule::event_list_section => {
                        let idents = sec_inner.into_inner().next().unwrap().into_inner()
                            .map(|p| p.as_str().to_string()).collect();
                        sections.push(SystemSection::Events(idents));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    SystemDef { name, initial, sections }
}

fn parse_context(pair: pest::iterators::Pair<Rule>) -> ContextDef {
    let mut name = String::new();
    let mut inputs = Vec::new();
    let mut roles = Vec::new();
    let mut transitions = Vec::new();
    let mut effects = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::context_clause => {
                let clause = inner.into_inner().next().unwrap();
                match clause.as_rule() {
                    Rule::input_def => {
                        for field in clause.into_inner() {
                            let mut f_iter = field.into_inner();
                            let first = f_iter.next().unwrap();
                            let (mutable, f_name, f_type) = if first.as_str() == "mutable" {
                                (true, f_iter.next().unwrap().as_str().to_string(), f_iter.next().unwrap().as_str().to_string())
                            } else {
                                (false, first.as_str().to_string(), f_iter.next().unwrap().as_str().to_string())
                            };
                            inputs.push(InputField { mutable, name: f_name, datatype: f_type });
                        }
                    },
                    Rule::role_def => roles.push(parse_role(clause)),
                    Rule::transitions_def => {
                        for tr in clause.into_inner() {
                            if tr.as_rule() == Rule::transition_rule {
                                transitions.push(parse_transition(tr));
                            }
                        }
                    },
                    Rule::effects_def => {
                        for eff in clause.into_inner() {
                            let mut e_iter = eff.into_inner();
                            let trigger_pair = e_iter.next().unwrap();
                            let trigger = if trigger_pair.as_rule() == Rule::lifecycle_hook {
                                EffectTrigger::Lifecycle(trigger_pair.as_str().replace("[", "").replace("]", ""))
                            } else {
                                EffectTrigger::Event(trigger_pair.as_str().to_string())
                            };
                            let call = parse_action_call(e_iter.next().unwrap());
                            effects.push(EffectRule { trigger, call });
                        }
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
    ContextDef { name, inputs, roles, transitions, effects }
}

fn parse_role(pair: pest::iterators::Pair<Rule>) -> RoleDef {
    let mut it = pair.into_inner();
    let name = it.next().unwrap().as_str().to_string();
    let datatype = it.next().unwrap().as_str().to_string();
    let mut binding = None;
    let mut actions = Vec::new();

    for inner in it {
        match inner.as_rule() {
            Rule::role_binding => {
                binding = Some(inner.into_inner().next().unwrap().as_str().to_string());
            },
            Rule::role_action => {
                actions.push(parse_role_action(inner));
            },
            _ => {}
        }
    }
    RoleDef { name, datatype, binding, actions }
}

fn parse_role_action(pair: pest::iterators::Pair<Rule>) -> RoleAction {
    let mut decorator = None;
    let mut event = String::new();
    let mut target = ActionTarget::Ignored;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::decorator => decorator = Some(parse_decorator(inner)),
            Rule::ident => event = inner.as_str().to_string(),
            Rule::action_target => {
                let target_inner = inner.into_inner().next().unwrap();
                target = match target_inner.as_rule() {
                    Rule::action_call => ActionTarget::Call(parse_action_call(target_inner)),
                    Rule::ident if target_inner.as_str() == "ignored" => ActionTarget::Ignored,
                    Rule::ident if target_inner.as_str() == "forbidden" => ActionTarget::Forbidden,
                    _ => {
                        if target_inner.as_str() == "ignored" { ActionTarget::Ignored }
                        else if target_inner.as_str() == "forbidden" { ActionTarget::Forbidden }
                        else { ActionTarget::Ignored }
                    }
                };
            },
            _ => {}
        }
    }
    RoleAction { decorator, event, target }
}

fn parse_transition(pair: pest::iterators::Pair<Rule>) -> TransitionRule {
    let mut decorator = None;
    let mut event = String::new();
    let mut target = String::new();
    let mut with_clause = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::decorator => decorator = Some(parse_decorator(inner)),
            Rule::ident => event = inner.as_str().to_string(),
            Rule::transition_target => target = inner.as_str().to_string(),
            Rule::with_clause => {
                for arg in inner.into_inner() {
                    let mut a_it = arg.into_inner();
                    let k = a_it.next().unwrap().as_str().to_string();
                    let v = a_it.next().unwrap().as_str().to_string();
                    with_clause.push((k, v));
                }
            },
            _ => {}
        }
    }
    TransitionRule { decorator, event, target, with_clause }
}

fn parse_action_call(pair: pest::iterators::Pair<Rule>) -> ActionCall {
    let mut citer = pair.into_inner();
    let function = citer.next().unwrap().as_str().to_string();
    let mut args = Vec::new();
    if let Some(args_pair) = citer.next() {
        for arg in args_pair.into_inner() {
            args.push(arg.as_str().to_string());
        }
    }
    ActionCall { function, args }
}

fn parse_decorator(pair: pest::iterators::Pair<Rule>) -> Decorator {
    let mut iter = pair.into_inner();
    let name = iter.next().unwrap().as_str().to_string();
    let args = iter.next().unwrap().as_str().to_string().replace("\"", "").replace("'", "");
    Decorator { name, args }
}
