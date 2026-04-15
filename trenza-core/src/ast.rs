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
pub struct EnumDef {
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub is_public: bool,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Data(DataDef),
    External(ExternalDef),
    System(SystemDef),
    Context(ContextDef),
    Import(ImportDef),
    Enum(EnumDef),
}

impl Definition {
    pub fn name(&self) -> &str {
        match self {
            Definition::Data(d) => &d.name,
            Definition::External(e) => &e.name,
            Definition::System(s) => &s.name,
            Definition::Context(c) => &c.name,
            Definition::Import(i) => &i.name,
            Definition::Enum(e) => &e.name,
        }
    }

    pub fn sort_key(&self) -> (u8, &str) {
        match self {
            Definition::Import(_) => (0, self.name()),
            Definition::System(_) => (1, self.name()),
            Definition::Data(_) => (2, self.name()),
            Definition::Enum(_) => (3, self.name()),
            Definition::Context(_) => (4, self.name()),
            Definition::External(_) => (5, self.name()),
        }
    }
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
    Concurrent(Vec<ConcurrentEntry>),
    Overlays(Vec<String>),
    Events(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum ConcurrentEntry {
    Name(String),
    Anonymous(ContextDef),
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
    pub is_anonymous: bool,
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

// --- Serialización (ADR-021) ---

pub trait ToTrz {
    fn to_trz(&self) -> String;
}

impl ToTrz for Program {
    fn to_trz(&self) -> String {
        self.definitions.iter()
            .map(|d| d.to_trz())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl ToTrz for Definition {
    fn to_trz(&self) -> String {
        match self {
            Definition::Data(d) => d.to_trz(),
            Definition::External(e) => e.to_trz(),
            Definition::System(s) => s.to_trz(),
            Definition::Context(c) => c.to_trz(),
            Definition::Import(i) => i.to_trz(),
            Definition::Enum(e) => e.to_trz(),
        }
    }
}

impl ToTrz for ImportDef {
    fn to_trz(&self) -> String {
        format!("use {}#{}", self.name, self.hash)
    }
}

impl ToTrz for DataDef {
    fn to_trz(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.annotations {
            out.push_str(&format!("[{}: {}]\n", k, v));
        }
        if self.is_public { out.push_str("pub "); }
        out.push_str(&format!("data {}:\n", self.name));
        for field in &self.fields {
            out.push_str(&format!("  {}{}: {}\n", if field.mutable { "var " } else { "" }, field.name, field.datatype));
        }
        out
    }
}

impl ToTrz for EnumDef {
    fn to_trz(&self) -> String {
        let mut out = String::new();
        if self.is_public { out.push_str("pub "); }
        out.push_str(&format!("type {}:\n", self.name));
        for variant in &self.variants {
            out.push_str(&format!("  | {}\n", variant));
        }
        out
    }
}

impl ToTrz for ContextDef {
    fn to_trz(&self) -> String {
        let mut out = String::new();
        if self.is_public { out.push_str("pub "); }
        if !self.name.is_empty() {
            out.push_str(&format!("context {}:\n", self.name));
        }
        if !self.inputs.is_empty() {
            out.push_str("  input:\n");
            for input in &self.inputs {
                out.push_str(&format!("    {}{}: {}\n", if input.mutable { "var " } else { "" }, input.name, input.datatype));
            }
        }
        for role in &self.roles {
            out.push_str(&role.to_trz());
        }
        if !self.transitions.is_empty() {
            out.push_str("  transitions:\n");
            for trans in &self.transitions {
                out.push_str(&format!("    on {} -> {}\n", trans.event, trans.target));
            }
        }
        if !self.effects.is_empty() {
            out.push_str("  effects:\n");
            for effect in &self.effects {
                let trigger = match &effect.trigger {
                    EffectTrigger::Lifecycle(s) => format!("[{}]", s),
                    EffectTrigger::Event(s) => s.clone(),
                };
                out.push_str(&format!("    {} -> {}({})\n", trigger, effect.call.function, effect.call.args.join(", ")));
            }
        }
        for slot in &self.slots {
            out.push_str(&format!("  {}slot {}\n", if slot.is_public { "pub " } else { "" }, slot.name));
        }
        for fills in &self.fills {
            out.push_str(&format!("  fills {}.{}:\n", fills.target_context, fills.target_slot));
            for role in &fills.roles {
                out.push_str(&role.to_trz_indented(2));
            }
        }
        out
    }
}

impl ToTrz for RoleDef {
    fn to_trz(&self) -> String {
        self.to_trz_indented(1)
    }
}

impl RoleDef {
    fn to_trz_indented(&self, indent_level: usize) -> String {
        let indent = "  ".repeat(indent_level);
        let mut out = String::new();
        if self.is_public { out.push_str(&format!("{}pub ", indent)); } else { out.push_str(&indent); }
        out.push_str(&format!("role {}: {}", self.name, self.datatype));
        if let Some(b) = &self.binding {
            out.push_str(&format!(" = {}", b));
        }
        out.push_str("\n");
        for action in &self.actions {
            out.push_str(&format!("{}  on {} -> ", indent, action.event));
            match &action.target {
                ActionTarget::Call(c) => out.push_str(&format!("{}({})\n", c.function, c.args.join(", "))),
                ActionTarget::Ignored => out.push_str("ignore\n"),
                ActionTarget::Forbidden => out.push_str("forbidden\n"),
            }
        }
        out
    }
}

impl ToTrz for ExternalDef {
    fn to_trz(&self) -> String {
        let mut out = format!("external {}:\n", self.name);
        for action in &self.actions {
            let params = action.params.iter().map(|(n, t)| format!("{}: {}", n, t)).collect::<Vec<_>>().join(", ");
            out.push_str(&format!("  {}({}) -> {}\n", action.name, params, action.return_type));
        }
        out
    }
}

impl ToTrz for SystemDef {
    fn to_trz(&self) -> String {
        let mut out = format!("system {}:\n", self.name);
        out.push_str(&format!("  initial: {}\n", self.initial));
        for section in &self.sections {
            match section {
                SystemSection::Contexts(c) => out.push_str(&format!("  contexts:\n    {}\n", c.join("\n    "))),
                SystemSection::Concurrent(entries) => {
                    out.push_str("  concurrent:\n");
                    for entry in entries {
                        match entry {
                            ConcurrentEntry::Name(name) => out.push_str(&format!("    {}\n", name)),
                            ConcurrentEntry::Anonymous(ctx) => {
                                // Indent the anonymous context content
                                let content = ctx.to_trz();
                                for line in content.lines() {
                                    out.push_str(&format!("    {}\n", line));
                                }
                            }
                        }
                    }
                },
                SystemSection::Overlays(c) => out.push_str(&format!("  overlays:\n    {}\n", c.join("\n    "))),
                SystemSection::Events(c) => out.push_str(&format!("  events:\n    {}\n", c.join("\n    "))),
            }
        }
        out
    }
}
