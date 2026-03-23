#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Data(DataDef),
    External(ExternalDef),
    System(SystemDef),
    Context(ContextDef),
}

#[derive(Debug, Clone)]
pub struct DataDef {
    pub name: String,
    pub annotations: Vec<(String, String)>,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ExternalDef {
    pub name: String,
    pub annotations: Vec<(String, String)>,
    pub actions: Vec<ExternalAction>,
}

#[derive(Debug, Clone)]
pub struct ExternalAction {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub responses: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct SystemDef {
    pub name: String,
    pub initial: String,
    pub sections: Vec<SystemSection>,
}

#[derive(Debug, Clone)]
pub enum SystemSection {
    Contexts(Vec<String>),
    Concurrent(Vec<String>),
    Overlays(Vec<String>),
    Events(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct ContextDef {
    pub name: String,
    pub inputs: Vec<InputField>,
    pub roles: Vec<RoleDef>,
    pub transitions: Vec<TransitionRule>,
    pub effects: Vec<EffectRule>,
}

#[derive(Debug, Clone)]
pub struct InputField {
    pub mutable: bool,
    pub name: String,
    pub datatype: String,
}

#[derive(Debug, Clone)]
pub struct RoleDef {
    pub name: String,
    pub datatype: String,
    pub annotations: Vec<(String, String)>,
    pub binding: Option<String>,
    pub actions: Vec<RoleAction>,
}

#[derive(Debug, Clone)]
pub struct RoleAction {
    pub decorator: Option<Decorator>,
    pub event: String,
    pub target: ActionTarget,
}

#[derive(Debug, Clone)]
pub enum ActionTarget {
    Call(ActionCall),
    Ignored,
    Forbidden,
}

#[derive(Debug, Clone)]
pub struct Decorator {
    pub name: String,
    pub args: String,
}

#[derive(Debug, Clone)]
pub struct ActionCall {
    pub function: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TransitionRule {
    pub decorator: Option<Decorator>,
    pub event: String,
    pub target: String,
    pub with_clause: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct EffectRule {
    pub trigger: EffectTrigger,
    pub call: ActionCall,
}

#[derive(Debug, Clone)]
pub enum EffectTrigger {
    Lifecycle(String),
    Event(String),
}
