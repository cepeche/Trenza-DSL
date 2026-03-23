use crate::ast::*;
use crate::TrenzaParser;
use crate::Rule;
use pest::Parser;

pub fn parse_file(content: &str) -> std::result::Result<Program, pest::error::Error<Rule>> {
    let mut pairs = TrenzaParser::parse(Rule::program, content)?;
    let mut definitions = Vec::new();
    
    if let Some(program_pair) = pairs.next() {
        if program_pair.as_rule() == Rule::program {
            for def_pair in program_pair.into_inner() {
                if def_pair.as_rule() == Rule::definition {
                    let inner = def_pair.into_inner().next().unwrap();
                    match inner.as_rule() {
                        Rule::data_def => definitions.push(Definition::Data(parse_data(inner))),
                        Rule::system_def => definitions.push(Definition::System(parse_system(inner))),
                        Rule::context_def => definitions.push(Definition::Context(parse_context(inner))),
                        _ => {}
                    }
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

fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut name = String::new();
    let mut initial = String::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => if name.is_empty() { name = inner.as_str().to_string(); } else { initial = inner.as_str().to_string(); },
            Rule::system_sections => {},
            _ => {}
        }
    }
    SystemDef { name, initial }
}

fn parse_context(pair: pest::iterators::Pair<Rule>) -> ContextDef {
    let mut name = String::new();
    let mut roles = Vec::new();
    let mut transitions = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::context_clause => {
                let clause = inner.into_inner().next().unwrap();
                match clause.as_rule() {
                    Rule::role_def => roles.push(parse_role(clause)),
                    Rule::transitions_def => {
                        for tr in clause.into_inner() {
                            if tr.as_rule() == Rule::transition_rule {
                                transitions.push(parse_transition(tr));
                            }
                        }
                    },
                    Rule::effects_def => {}
                    _ => {}
                }
            },
            _ => {}
        }
    }
    ContextDef { name, roles, transitions }
}

fn parse_role(pair: pest::iterators::Pair<Rule>) -> RoleDef {
    let mut iter = pair.into_inner();
    let name = iter.next().unwrap().as_str().to_string();
    let datatype = iter.next().unwrap().as_str().to_string();
    let mut actions = Vec::new();

    for inner in iter {
        if inner.as_rule() == Rule::role_action {
            actions.push(parse_role_action(inner));
        }
    }
    RoleDef { name, datatype, actions }
}

fn parse_role_action(pair: pest::iterators::Pair<Rule>) -> RoleAction {
    let mut decorator = None;
    let mut event = String::new();
    let mut call = ActionCall { function: "".to_string(), args: vec![] };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::decorator => decorator = Some(parse_decorator(inner)),
            Rule::ident => event = inner.as_str().to_string(),
            Rule::action_call => {
                let mut citer = inner.into_inner();
                let function = citer.next().unwrap().as_str().to_string();
                let mut args = Vec::new();
                if let Some(args_pair) = citer.next() {
                    for arg in args_pair.into_inner() {
                        args.push(arg.as_str().to_string());
                    }
                }
                call = ActionCall { function, args };
            },
            _ => {}
        }
    }
    RoleAction { decorator, event, call }
}

fn parse_transition(pair: pest::iterators::Pair<Rule>) -> TransitionRule {
    let mut decorator = None;
    let mut event = String::new();
    let mut target = String::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::decorator => decorator = Some(parse_decorator(inner)),
            Rule::ident => {
                if event.is_empty() { event = inner.as_str().to_string(); }
                else { target = inner.as_str().to_string(); }
            },
            _ => {}
        }
    }
    TransitionRule { decorator, event, target }
}

fn parse_decorator(pair: pest::iterators::Pair<Rule>) -> Decorator {
    let mut iter = pair.into_inner();
    let name = iter.next().unwrap().as_str().to_string();
    let args = iter.next().unwrap().as_str().to_string().replace("\"", "");
    Decorator { name, args }
}
