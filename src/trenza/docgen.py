import os
from typing import Dict, List
from .ast import TrenzaProject, Context

class DocGenerator:
    def __init__(self, project: TrenzaProject, output_dir: str):
        self.project = project
        self.output_dir = output_dir

    def _ensure_dirs(self):
        os.makedirs(self.output_dir, exist_ok=True)
        os.makedirs(os.path.join(self.output_dir, "base"), exist_ok=True)
        os.makedirs(os.path.join(self.output_dir, "concurrent"), exist_ok=True)
        os.makedirs(os.path.join(self.output_dir, "overlays"), exist_ok=True)

    def _get_context_path(self, ctx_name: str, relative_to: str = "") -> str:
        if ctx_name in self.project.system_decl.contexts:
            path = f"base/{ctx_name}.md"
        elif ctx_name in self.project.system_decl.concurrent:
            path = f"concurrent/{ctx_name}.md"
        elif ctx_name in self.project.system_decl.overlays:
            path = f"overlays/{ctx_name}.md"
        else:
            path = f"{ctx_name}.md" # Fallback
            
        if relative_to == "base" or relative_to == "concurrent" or relative_to == "overlays":
            return f"../{path}"
        elif relative_to == "root":
            return path
        return f"{path}"

    def generate(self):
        self._ensure_dirs()
        self._generate_index()
        
        for ctx_name, ctx in self.project.parsed_contexts.items():
            if ctx_name == self.project.system_decl.name: continue
            
            # Determine subfolder
            target_folder = ""
            if ctx_name in self.project.system_decl.contexts:
                target_folder = "base"
            elif ctx_name in self.project.system_decl.concurrent:
                target_folder = "concurrent"
            elif ctx_name in self.project.system_decl.overlays:
                target_folder = "overlays"
            else:
                continue # Skip nested for now
                
            self._generate_context_doc(ctx, target_folder)

    def _generate_index(self):
        sys_name = self.project.system_decl.name
        lines = [
            f"# {sys_name}",
            "",
            f"**Estado inicial**: [{self.project.system_decl.initial}](base/{self.project.system_decl.initial}.md)",
            "",
            "## Arquitectura",
            "",
            "```mermaid",
            "stateDiagram-v2",
            '    state "Base" as BASE {',
            f"        [*] --> {self.project.system_decl.initial}"
        ]
        
        # Base inner transitions
        for ctx_name in self.project.system_decl.contexts:
            if ctx_name in self.project.parsed_contexts:
                ctx = self.project.parsed_contexts[ctx_name]
                for t in ctx.transitions:
                    if t.target_state in self.project.system_decl.contexts:
                        lines.append(f"        {ctx_name} --> {t.target_state}")
        lines.append("    }")
        
        if self.project.system_decl.concurrent:
            lines.append('    state "Concurrentes" as CONC {')
            for c in self.project.system_decl.concurrent:
                lines.append(f"        {c}")
            lines.append("    }")
            
        if self.project.system_decl.overlays:
            lines.append('    state "Overlays" as OV {')
            for o in self.project.system_decl.overlays:
                lines.append(f"        {o}")
            lines.append("    }")
            
        lines.append("```")
        lines.append("")
        
        path = os.path.join(self.output_dir, "index.md")
        with open(path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))

    def _generate_context_doc(self, ctx: Context, folder: str):
        lines = [
            f"# {ctx.name}",
            "",
            f"**Tipo**: contexto {folder}",
            "",
            "## Roles",
            "",
            "| Rol | Tipo | Origen |",
            "|-----|------|--------|"
        ]
        
        for r_name, r in ctx.roles.items():
            origen = "Local" if r.is_local else "Heredado"
            lines.append(f"| {r_name} | {r.type_name} | {origen} |")
            
        if ctx.slots:
            lines.extend(["", "## Puntos de Extensión (Slots)", ""])
            for s in ctx.slots:
                lines.append(f"- `({s.name})`")
                
        if ctx.fills:
            lines.extend(["", "## Contribuciones (Fills)", ""])
            for f in ctx.fills:
                lines.append(f"- Llenando **{f.target_context}.{f.target_slot}** con:")
                for rname, r in f.roles.items():
                    lines.append(f"    - Rol `{rname}: {r.type_name}`")
                for e in f.effects:
                    lines.append(f"    - Efecto `[{e.on_event}] -> {e.action.name}`")

        lines.extend([
            "",
            "## Transiciones",
            "",
            "```mermaid",
            "stateDiagram-v2"
        ])
        
        for t in ctx.transitions:
            target = t.target_state
            if target.startswith("["): target = f"SYS_{target[1:-1]}"
            lines.append(f"    {ctx.name} --> {target} : {t.on_event}")
            
        lines.append("```")
        lines.append("")
        
        if ctx.transitions:
            lines.extend([
                "| Evento | Destino |",
                "|--------|---------|"
            ])
            for t in ctx.transitions:
                target_link = t.target_state
                if not t.target_state.startswith("["):
                    target_link = f"[{t.target_state}]({self._get_context_path(t.target_state, folder)})"
                lines.append(f"| {t.on_event} | {target_link} |")
        
        path = os.path.join(self.output_dir, folder, f"{ctx.name}.md")
        with open(path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))
