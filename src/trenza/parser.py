import os
import re
from typing import Dict, List, Optional
from .ast import (
    AstNode, Action, Handler, Role, Transition, Effect, 
    Context, SystemDecl, Manifest, TrenzaProject
)

class TrenzaParser:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir

    def _strip_comments(self, line: str) -> str:
        if '--' in line:
            line = line[:line.index('--')]
        return line.strip()

    def parse_system(self, filepath: str) -> SystemDecl:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        sys_name = "Unknown"
        initial = ""
        contexts = []
        composition = "exclusiva"
        concurrent = []
        overlays = []

        current_section = None

        for line in lines:
            line = self._strip_comments(line)
            if not line:
                continue

            if line.startswith('system '):
                # e.g. system CronometroPSP:
                m = re.match(r'system\s+([^:]+):', line)
                if m:
                    sys_name = m.group(1).strip()
            elif line.startswith('initial:'):
                initial = line.split(':', 1)[1].strip()
            elif line.startswith('contexts:'):
                current_section = 'contexts'
            elif line.startswith('composition:'):
                composition = line.split(':', 1)[1].strip()
                current_section = None
            elif line.startswith('concurrent:'):
                current_section = 'concurrent'
            elif line.startswith('overlays:'):
                current_section = 'overlays'
            elif current_section:
                # Items in lists
                item = line.strip()
                if item:
                    if current_section == 'contexts':
                        contexts.append(item)
                    elif current_section == 'concurrent':
                        concurrent.append(item)
                    elif current_section == 'overlays':
                        overlays.append(item)

        return SystemDecl(
            name=sys_name,
            initial=initial,
            contexts=contexts,
            composition=composition,
            concurrent=concurrent,
            overlays=overlays
        )

    def parse_context_file(self, filepath: str) -> Context:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        # Simplify by using a basic line-by-line block parser
        lines = content.split('\n')
        return self._parse_context_block(lines, 0)[0]

    def _parse_context_block(self, lines: List[str], start_idx: int, base_indent: int = -1) -> tuple[Context, int]:
        ctx_name = ""
        
        # Read the context header
        for i in range(start_idx, len(lines)):
            raw_line = lines[i]
            line = self._strip_comments(raw_line)
            if line.startswith('context '):
                if base_indent == -1:
                    base_indent = len(raw_line) - len(raw_line.lstrip())
                m = re.match(r'context\s+([^:]+):', line)
                if m:
                    ctx_name = m.group(1).strip()
                    start_idx = i + 1
                    break
            elif line:
                # Should not reach here before 'context' unless whitespace/comments
                pass

        ctx = Context(name=ctx_name)
        
        current_role = None
        current_section = None # 'transitions', 'effects', None
        
        i = start_idx
        while i < len(lines):
            raw_line = lines[i]
            line = self._strip_comments(raw_line)
            indent = len(raw_line) - len(raw_line.lstrip())
            
            if not line:
                i += 1
                continue

            # Check if this line unindents out of the current context block
            # For simplicity in this prototype, we assume contexts top-level are indent 0 or 4,
            # inner contexts are deeper. If we hit another 'context' and it's less/same indent, we are done
            # But Trenza contexts can be nested:
            if line.startswith('context '):
                if base_indent != -1 and indent <= base_indent:
                    # Sibling or parent context, we must exit this context parsing
                    break
                    
                subctx, new_i = self._parse_context_block(lines, i, indent)
                subctx.parent_name = ctx.name
                ctx.subcontexts[subctx.name] = subctx
                i = new_i
                continue

            if line.startswith('role '):
                current_section = None
                # role name: type
                m = re.match(r'role\s+([^:]+):\s*(.+)', line)
                if m:
                    rname = m.group(1).strip()
                    rtype = m.group(2).strip()
                    current_role = Role(name=rname, type_name=rtype)
                    ctx.roles[rname] = current_role
            elif line.startswith('on ') and current_role and current_section is None:
                # parsing an event handler for a role: on tap -> action
                m = re.match(r'on\s+(.+?)\s*->\s*(.+)', line)
                if m:
                    event = m.group(1).strip()
                    action_raw = m.group(2).strip()
                    
                    # split action into name and args if it has parens
                    action_name = action_raw
                    args = []
                    a_m = re.match(r'([^\(]+)\((.*)\)', action_raw)
                    if a_m:
                        action_name = a_m.group(1).strip()
                        args_raw = a_m.group(2).strip()
                        if args_raw:
                            args = [arg.strip() for arg in args_raw.split(',')]
                            
                    current_role.handlers.append(Handler(
                        event=event,
                        action=Action(name=action_name, args=args)
                    ))
            elif line.startswith('transitions:'):
                current_section = 'transitions'
                current_role = None
            elif line.startswith('effects:'):
                current_section = 'effects'
                current_role = None
            elif current_section == 'transitions' and line.startswith('on '):
                # on event -> state
                m = re.match(r'on\s+(.+?)\s*->\s*(.+)', line)
                if m:
                    event = m.group(1).strip()
                    estado = m.group(2).strip()
                    ctx.transitions.append(Transition(on_event=event, target_state=estado))
            elif current_section == 'effects' and line.startswith('['):
                # [al_entrar] -> external func
                m = re.match(r'\[(.*?)\]\s*->\s*(.+)', line)
                if m:
                    event = m.group(1).strip()
                    action_raw = m.group(2).strip()
                    
                    action_name = action_raw
                    args = []
                    if action_raw.startswith('external '):
                        action_raw = action_raw[9:].strip()
                    a_m = re.match(r'([^\(]+)\((.*)\)', action_raw)
                    if a_m:
                        action_name = a_m.group(1).strip()
                        args_raw = a_m.group(2).strip()
                        if args_raw:
                            args = [arg.strip() for arg in args_raw.split(',')]
                    else:
                        action_name = action_raw
                        
                    ctx.effects.append(Effect(on_event=event, action=Action(name=action_name, args=args)))

            # To handle early exit when we hit unindent logic in a real parser we'd check `indent`
            # For this MVP, nested contexts capture their blocks because they consume lines, 
            # and roles/transitions belong to whatever context is currently being parsed until EOF.
            
            i += 1
            
        return ctx, i
