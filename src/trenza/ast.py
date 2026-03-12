from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import json

@dataclass
class AstNode:
    def to_dict(self) -> Dict[str, Any]:
        result = {}
        for key, value in self.__dict__.items():
            if isinstance(value, list):
                result[key] = [v.to_dict() if isinstance(v, AstNode) else v for v in value]
            elif isinstance(value, dict):
                result[key] = {k: (v.to_dict() if isinstance(v, AstNode) else v) for k, v in value.items()}
            elif isinstance(value, AstNode):
                result[key] = value.to_dict()
            else:
                result[key] = value
        return result

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), indent=2, ensure_ascii=False)

@dataclass
class Action(AstNode):
    name: str
    args: List[str] = field(default_factory=list)

@dataclass
class Handler(AstNode):
    event: str
    action: Action

@dataclass
class Role(AstNode):
    name: str
    type_name: str
    handlers: List[Handler] = field(default_factory=list)
    is_local: bool = True     # False if inherited (computed by verifier)

@dataclass
class TransitionCondition(AstNode):
    event: str
    action: Optional[str] = None # For 'on event -> action' syntactic sugar, though it's distinct in Trenza

@dataclass
class Transition(AstNode):
    on_event: str
    target_state: str

@dataclass
class Effect(AstNode):
    on_event: str
    action: Action

@dataclass
class Context(AstNode):
    name: str
    roles: Dict[str, Role] = field(default_factory=dict)
    transitions: List[Transition] = field(default_factory=list)
    effects: List[Effect] = field(default_factory=list)
    subcontexts: Dict[str, 'Context'] = field(default_factory=dict)
    parent_name: Optional[str] = None

@dataclass
class SystemDecl(AstNode):
    name: str
    initial: str
    contexts: List[str] = field(default_factory=list)
    composition: str = "exclusiva"
    concurrent: List[str] = field(default_factory=list)
    overlays: List[str] = field(default_factory=list)

@dataclass
class Manifest(AstNode):
    manifest_version: str
    trenza_version: str
    system: str
    data: str
    contexts: List[Dict[str, str]] = field(default_factory=list)

@dataclass
class TrenzaProject(AstNode):
    manifest: Manifest
    system_decl: SystemDecl
    parsed_contexts: Dict[str, Context] = field(default_factory=dict)
