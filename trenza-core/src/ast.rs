#[derive(Debug, Clone, serde::Serialize)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
    pub severity: String,
    pub code: String,
}

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
    Import(ImportDef),
}

#[derive(Debug, Clone)]
pub struct ImportDef {
    pub span: Span,
    pub name: String,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct DataDef {
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub is_public: bool,
    pub annotations: Vec<(String, String)>,
    pub fields: Vec<DataField>,
}

#[derive(Debug, Clone)]
pub struct DataField {
    pub mutable: bool,
    pub name: String,
    pub datatype: String,
}

#[derive(Debug, Clone)]
pub struct ExternalDef {
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub annotations: Vec<(String, String)>,
    pub actions: Vec<ExternalAction>,
}

#[derive(Debug, Clone)]
pub struct ExternalAction {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}

#[derive(Debug, Clone)]
pub struct SystemDef {
    pub span: Span,
    pub name: String,
    pub name_span: Span,
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
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub is_public: bool,
    pub inputs: Vec<InputField>,
    pub roles: Vec<RoleDef>,
    pub transitions: Vec<TransitionRule>,
    pub effects: Vec<EffectRule>,
    pub slots: Vec<SlotDef>,
    pub fills: Vec<FillsDef>,
    pub ignore_rest: bool,
}

#[derive(Debug, Clone)]
pub struct SlotDef {
    pub name: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct FillsDef {
    pub target_context: String,
    pub target_slot: String,
    pub roles: Vec<RoleDef>,
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
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub datatype: String,
    pub is_public: bool,
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
