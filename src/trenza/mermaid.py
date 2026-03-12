from typing import Dict, List, Set
from .ast import TrenzaProject, Context

class MermaidGenerator:
    def __init__(self, project: TrenzaProject):
        self.project = project
        self.output = []
        self._indent = 0

    def _add(self, line: str):
        self.output.append(" " * self._indent + line)

    def generate(self) -> str:
        self.output = []
        self._add("stateDiagram-v2")
        self._indent += 4
        
        system_nodes = []
        
        # 1. Base Contexts (mutually exclusive inside the System)
        sys_name = self.project.system_decl.name
        self._add(f"state {sys_name} {{")
        self._indent += 4
        
        initial = self.project.system_decl.initial
        if initial:
            self._add(f"[*] --> {initial}")
            
        for ctx_name in self.project.system_decl.contexts:
            if ctx_name in self.project.parsed_contexts:
                self._generate_context_state(self.project.parsed_contexts[ctx_name])
                system_nodes.append(ctx_name)
                
        self._indent -= 4
        self._add("}")
        
        # 2. Concurrent Contexts
        if self.project.system_decl.concurrent:
            self._add(f"state CO_{sys_name} {{") # A parallel state representation
            self._indent += 4
            self._add(f"note left of CO_{sys_name}: Concurrent Contexts")
            for ctx_name in self.project.system_decl.concurrent:
                if ctx_name in self.project.parsed_contexts:
                    self._generate_context_state(self.project.parsed_contexts[ctx_name])
                    system_nodes.append(ctx_name)
            self._indent -= 4
            self._add("}")
            
        # 3. Overlays
        if self.project.system_decl.overlays:
            self._add(f"state OV_{sys_name} {{") 
            self._indent += 4
            self._add(f"note left of OV_{sys_name}: Overlay Contexts")
            for ctx_name in self.project.system_decl.overlays:
                if ctx_name in self.project.parsed_contexts:
                    self._generate_context_state(self.project.parsed_contexts[ctx_name])
                    system_nodes.append(ctx_name)
            self._indent -= 4
            self._add("}")

        # 4. Global Transitions
        for ctx_name in system_nodes:
            if ctx_name in self.project.parsed_contexts:
                self._generate_transitions(self.project.parsed_contexts[ctx_name])

        return "\n".join(self.output)

    def _generate_context_state(self, ctx: Context):
        if not ctx.subcontexts and not ctx.roles:
            self._add(f"{ctx.name}")
            return
            
        self._add(f"state {ctx.name} {{")
        self._indent += 4
        
        # Add roles as a note
        if ctx.roles:
            roles_str = "\\n".join([f"{r.name}: {r.type_name}" for r in ctx.roles.values() if r.is_local])
            if roles_str:
                self._add(f"note right of {ctx.name}: {roles_str}")
        
        # Nested subcontexts
        for sub_name, sub_ctx in ctx.subcontexts.items():
            self._generate_context_state(sub_ctx)
            
        self._indent -= 4
        self._add("}")

    def _generate_transitions(self, ctx: Context):
        # We only generate transitions between contexts (not effects)
        for t in ctx.transitions:
            # We can label transitions with the event
            self._add(f"{ctx.name} --> {t.target_state} : {t.on_event}")
            
        # Also generate transitions for subcontexts
        for sub_name, sub_ctx in ctx.subcontexts.items():
            for t in sub_ctx.transitions:
                self._add(f"{ctx.name} --> {t.target_state} : {t.on_event}")
            self._generate_transitions(sub_ctx)
