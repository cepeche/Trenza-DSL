#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Data(DataDef),
    System(SystemDef),
    Context(ContextDef),
}

#[derive(Debug, Clone)]
pub struct DataDef {
    pub name: String,
    pub annotation: Option<(String, String)>,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct SystemDef {
    pub name: String,
    pub initial: String,
}

#[derive(Debug, Clone)]
pub struct ContextDef {
    pub name: String,
    pub roles: Vec<RoleDef>,
    pub transitions: Vec<TransitionRule>,
}

#[derive(Debug, Clone)]
pub struct RoleDef {
    pub name: String,
    pub datatype: String,
    pub actions: Vec<RoleAction>,
}

#[derive(Debug, Clone)]
pub struct RoleAction {
    pub decorator: Option<Decorator>,
    pub event: String,
    pub call: ActionCall,
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
}
