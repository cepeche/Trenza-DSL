use crate::ast::*;
use crate::TrenzaParser;
use crate::Rule;
use pest::Parser;
 
fn get_span(pair: &pest::iterators::Pair<Rule>) -> Span {
    let span = pair.as_span();
    let (line_start, col_start) = span.start_pos().line_col();
    let (line_end, col_end) = span.end_pos().line_col();
    Span {
        start: Pos { line: line_start, col: col_start },
        end: Pos { line: line_end, col: col_end },
    }
}

pub fn parse_file(content: &str) -> std::result::Result<Program, pest::error::Error<Rule>> {
    let mut pairs = TrenzaParser::parse(Rule::program, content)?;
    let mut definitions = Vec::new();
    
    if let Some(program_pair) = pairs.next() {
        if program_pair.as_rule() == Rule::program {
            for inner in program_pair.into_inner() {
                match inner.as_rule() {
                    Rule::definition => {
                        let mut def_inner = inner.into_inner();
                        let mut is_public = false;
                        let first = def_inner.next().unwrap();
                        
                        let actual_def = if first.as_rule() == Rule::pub_kw {
                            is_public = true;
                            def_inner.next().unwrap()
                        } else {
                            first
                        };

                        match actual_def.as_rule() {
                            Rule::data_def => definitions.push(Definition::Data(parse_data(actual_def, is_public))),
                            Rule::system_def => definitions.push(Definition::System(parse_system(actual_def))),
                            Rule::context_def => definitions.push(Definition::Context(parse_context(actual_def, is_public))),
                            Rule::import_def => definitions.push(Definition::Import(parse_import(actual_def))),
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

fn parse_data(pair: pest::iterators::Pair<Rule>, is_public: bool) -> DataDef {
    let mut name = String::new();
    let mut name_span = get_span(&pair); // Fallback
    let mut annotations = Vec::new();
    let mut fields = Vec::new();

    let span = get_span(&pair);
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                name = inner.as_str().to_string();
                name_span = get_span(&inner);
            },
            Rule::data_annotation => {
                let mut iter = inner.into_inner();
                let k = iter.next().unwrap().as_str().to_string();
                let v = iter.next().unwrap().as_str().to_string();
                annotations.push((k, v));
            },
            Rule::data_field => {
                let mut iter = inner.into_inner();
                let curr_mutable;
                let curr_name;
                let curr_type;
                let first = iter.next().unwrap();
                if first.as_str() == "mutable" {
                    curr_mutable = true;
                    curr_name = iter.next().unwrap().as_str().to_string();
                    curr_type = iter.next().unwrap().as_str().to_string();
                } else {
                    curr_mutable = false;
                    curr_name = first.as_str().to_string();
                    curr_type = iter.next().unwrap().as_str().to_string();
                }
                fields.push(DataField { mutable: curr_mutable, name: curr_name, datatype: curr_type });
            },
            _ => {}
        }
    }
    DataDef { span, name, name_span, is_public, annotations, fields }
}

fn parse_import(pair: pest::iterators::Pair<Rule>) -> ImportDef {
    let span = get_span(&pair);
    let mut it = pair.into_inner();
    let _ = it.next(); // skip use_kw
    let name = it.next().unwrap().as_str().to_string();
    let hash = it.next().unwrap().as_str().to_string();
    ImportDef { span, name, hash }
}

fn parse_external(pair: pest::iterators::Pair<Rule>) -> ExternalDef {
    let mut name = String::new();
    let mut name_span = get_span(&pair); // Fallback
    let mut annotations = Vec::new();
    let mut actions = Vec::new();

    let span = get_span(&pair);
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                name = inner.as_str().to_string();
                name_span = get_span(&inner);
            },
            Rule::data_annotation => {
                let mut iter = inner.into_inner();
                let k = iter.next().unwrap().as_str().to_string();
                let v = iter.next().unwrap().as_str().to_string();
                annotations.push((k, v));
            },
            Rule::external_action => {
                let mut a_iter = inner.into_inner();
                let a_name = a_iter.next().unwrap().as_str().to_string();
                let mut params = Vec::new();
                let mut return_type = String::new();
                
                for a_inner in a_iter {
                    match a_inner.as_rule() {
                        Rule::action_params => {
                            let mut p_iter = a_inner.into_inner();
                            while let (Some(pk), Some(pv)) = (p_iter.next(), p_iter.next()) {
                                params.push((pk.as_str().to_string(), pv.as_str().to_string()));
                            }
                        },
                        Rule::type_ident => {
                            return_type = a_inner.as_str().to_string();
                        },
                        _ => {}
                    }
                }
                actions.push(ExternalAction { name: a_name, params, return_type });
            },
            _ => {}
        }
    }
    ExternalDef { span, name, name_span, annotations, actions }
}

fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut name = String::new();
    let mut name_span = get_span(&pair); // Fallback
    let mut initial = String::new();
    let mut sections = Vec::new();

    let span = get_span(&pair);
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                let s = inner.as_str().to_string();
                if name.is_empty() { 
                    name = s;
                    name_span = get_span(&inner);
                } else { 
                    initial = s; 
                }
            },
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
    SystemDef { span, name, name_span, initial, sections }
}

fn parse_context(pair: pest::iterators::Pair<Rule>, is_public: bool) -> ContextDef {
    let mut name = String::new();
    let mut name_span = get_span(&pair); // Fallback
    let mut inputs = Vec::new();
    let mut roles = Vec::new();
    let mut transitions = Vec::new();
    let mut effects = Vec::new();
    let mut slots = Vec::new();
    let mut fills = Vec::new();
    let mut ignore_rest = false;

    let span = get_span(&pair);
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                name = inner.as_str().to_string();
                name_span = get_span(&inner);
            },
            Rule::context_clause => {
                let mut c_iter = inner.into_inner();
                let first = c_iter.next().unwrap();
                let (is_pub, clause_pair) = if first.as_rule() == Rule::pub_kw {
                    (true, c_iter.next().unwrap())
                } else {
                    (false, first)
                };

                match clause_pair.as_rule() {
                    Rule::input_def => {
                        for field in clause_pair.into_inner() {
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
                    Rule::role_def => roles.push(parse_role(clause_pair, is_pub)),
                    Rule::transitions_def => {
                        for tr in clause_pair.into_inner() {
                            if tr.as_rule() == Rule::transition_rule {
                                transitions.push(parse_transition(tr));
                            }
                        }
                    },
                    Rule::effects_def => {
                        for eff in clause_pair.into_inner() {
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
                    },
                    Rule::slot_def => {
                        let mut s_iter = clause_pair.into_inner();
                        let slot_name = s_iter.next().unwrap().as_str().to_string();
                        slots.push(SlotDef { name: slot_name, is_public: is_pub });
                    },
                    Rule::fills_def => {
                        fills.push(parse_fills(clause_pair));
                    },
                    Rule::role_wildcard => {
                        ignore_rest = true;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    ContextDef { span, name, name_span, is_public, inputs, roles, transitions, effects, slots, fills, ignore_rest }
}


fn parse_role(pair: pest::iterators::Pair<Rule>, is_public: bool) -> RoleDef {
    let span = get_span(&pair);
    let mut it = pair.into_inner();
    let name_pair = it.next().unwrap();
    let name = name_pair.as_str().to_string();
    let name_span = get_span(&name_pair);
    let mut datatype = String::new();
    let mut annotations = Vec::new();
    let mut binding = None;
    let mut actions = Vec::new();
 
    for inner in it {
        match inner.as_rule() {
            Rule::type_ident if datatype.is_empty() => datatype = inner.as_str().to_string(),
            Rule::role_annotation => {
                let mut a_it = inner.into_inner();
                let k = a_it.next().unwrap().as_str().to_string();
                let v = a_it.next().unwrap().as_str().to_string();
                annotations.push((k, v));
            },
            Rule::role_binding => {
                binding = Some(inner.into_inner().next().unwrap().as_str().to_string());
            },
            Rule::role_action => {
                actions.push(parse_role_action(inner));
            },
            _ => {}
        }
    }
    RoleDef { span, name, name_span, datatype, is_public, annotations, binding, actions }
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
    
    for inner in citer {
        if inner.as_rule() == Rule::action_args {
            for arg in inner.into_inner() {
                args.push(arg.as_str().to_string());
            }
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

fn parse_fills(pair: pest::iterators::Pair<Rule>) -> FillsDef {
    let mut target_context = String::new();
    let mut target_slot = String::new();
    let mut roles = Vec::new();
    let mut effects = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::slot_ref => {
                let ref_str = inner.as_str();
                let parts: Vec<&str> = ref_str.split('.').collect();
                if parts.len() == 2 {
                    target_context = parts[0].to_string();
                    target_slot = parts[1].to_string();
                } else {
                    target_context = ref_str.to_string();
                    target_slot = "".to_string(); // Will naturally fail Rule 7 later
                }
            },
            Rule::fills_clause => {
                let mut c_iter = inner.into_inner();
                let first = c_iter.next().unwrap();
                let (is_pub, clause_pair) = if first.as_rule() == Rule::pub_kw {
                    (true, c_iter.next().unwrap())
                } else {
                    (false, first)
                };

                match clause_pair.as_rule() {
                    Rule::role_def => roles.push(parse_role(clause_pair, is_pub)),
                    Rule::effects_def => {
                        for eff in clause_pair.into_inner() {
                            if eff.as_rule() != Rule::effect_rule { continue; }
                            let mut e_iter = eff.into_inner();
                            let trigger_pair = e_iter.next().unwrap();
                            let trigger = if trigger_pair.as_rule() == Rule::lifecycle_hook {
                                EffectTrigger::Lifecycle(
                                    trigger_pair.as_str()
                                        .replace("[", "").replace("]", "")
                                )
                            } else {
                                EffectTrigger::Event(trigger_pair.as_str().to_string())
                            };
                            let call = parse_action_call(e_iter.next().unwrap());
                            effects.push(EffectRule { trigger, call });
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    FillsDef { target_context, target_slot, roles, effects }
}
