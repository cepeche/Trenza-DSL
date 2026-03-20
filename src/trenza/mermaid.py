from typing import Dict, List, Set
from .ast import TrenzaProject, Context

class MermaidGenerator:
    def __init__(self, project: TrenzaProject):
        self.project = project
        self.output = []
        self.notes = []
        self._indent = 0

    def _add(self, line: str):
        self.output.append(" " * self._indent + line)

    def generate(self) -> str:
        self.output = []
        self._add("graph TD")
        self._indent += 4
        
        system_nodes = []
        
        # 1. Base Contexts (mutually exclusive inside the System)
        sys_name = self.project.system_decl.name
        self._add(f"subgraph {sys_name}")
        self._indent += 4
        
        for ctx_name in self.project.system_decl.contexts:
            if ctx_name in self.project.parsed_contexts:
                self._generate_context_state(self.project.parsed_contexts[ctx_name])
                system_nodes.append(ctx_name)
                
        initial = self.project.system_decl.initial
        if initial:
            # Flowchart doesn't have `[*]` but we can make a start node
            self._add(f"Start((Inicio)) --> {initial}")
            
        self._indent -= 4
        self._add("end")
        
        # 2. Concurrent Contexts
        if self.project.system_decl.concurrent:
            self._add(f"subgraph CO_{sys_name} [Concurrent Contexts]")
            self._indent += 4
            for ctx_name in self.project.system_decl.concurrent:
                if ctx_name in self.project.parsed_contexts:
                    self._generate_context_state(self.project.parsed_contexts[ctx_name])
                    system_nodes.append(ctx_name)
            self._indent -= 4
            self._add("end")
            
        # 3. Overlays
        if self.project.system_decl.overlays:
            self._add(f"subgraph OV_{sys_name} [Overlay Contexts]") 
            self._indent += 4
            for ctx_name in self.project.system_decl.overlays:
                if ctx_name in self.project.parsed_contexts:
                    self._generate_context_state(self.project.parsed_contexts[ctx_name])
                    system_nodes.append(ctx_name)
            self._indent -= 4
            self._add("end")

        # 4. Global Transitions
        for ctx_name in system_nodes:
            if ctx_name in self.project.parsed_contexts:
                self._generate_transitions(self.project.parsed_contexts[ctx_name])

        # 5. Global Notes
        for note_line in self.notes:
            self._add(note_line)

        return "\n".join(self.output)

    def _generate_context_state(self, ctx: Context):
        # We need a node representation for the context. In graph, subgraphs group things, nodes are states.
        if not ctx.subcontexts:
            # It's a leaf node
            roles_html = ""
            if ctx.roles:
                local_roles = [f"{r.name}: {r.type_name}" for r in ctx.roles.values() if r.is_local]
                if local_roles:
                    roles_html = "<br/>" + "<br/>".join(local_roles)
            
            slots_html = ""
            if ctx.slots:
                slots_html = "<br/><i>Slots:</i><br/>" + "<br/>".join([f"({s.name})" for s in ctx.slots])
                
            self._add(f'{ctx.name}["{ctx.name}{roles_html}{slots_html}"]')
        else:
            # It's a composite state (subgraph)
            self._add(f"subgraph {ctx.name}")
            self._indent += 4
            
            # Roles for this subgraph, attached to a dummy node because subgraphs can't have notes/labels easily in all renderers
            if ctx.roles:
                local_roles = [f"{r.name}: {r.type_name}" for r in ctx.roles.values() if r.is_local]
                if local_roles:
                    roles_html = "<br/>".join(local_roles)
                    self._add(f'{ctx.name}_roles["{roles_html}"]')
                    self._add(f"style {ctx.name}_roles fill:#f9f,stroke:#333,stroke-width:2px")
            
            # Nested subcontexts
            for sub_name, sub_ctx in ctx.subcontexts.items():
                self._generate_context_state(sub_ctx)
                
            self._indent -= 4
            self._add("end")

    def _safe_node(self, name: str) -> str:
        if name.startswith("[") and name.endswith("]"):
            safe_id = "SYS_" + name[1:-1]
            return f'{safe_id}(("{name}"))'
        return name

    def _generate_transitions(self, ctx: Context):
        # We only generate transitions between contexts (not effects)
        for t in ctx.transitions:
            # We can label transitions with the event
            target = self._safe_node(t.target_state)
            self._add(f"{ctx.name} -->|{t.on_event}| {target}")
            
        # Global: Fills (injections)
        for fill in ctx.fills:
            self._add(f"{ctx.name} -.fills.-> {fill.target_context}")
            
        # Also generate transitions for subcontexts
        for sub_name, sub_ctx in ctx.subcontexts.items():
            for t in sub_ctx.transitions:
                target = self._safe_node(t.target_state)
                self._add(f"{ctx.name} -->|{t.on_event}| {target}")
            self._generate_transitions(sub_ctx)
