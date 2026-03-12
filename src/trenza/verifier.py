from typing import Dict, List, Optional
from .ast import TrenzaProject, Context, Role

class VerifierError(Exception):
    pass

class TrenzaVerifier:
    def __init__(self, project: TrenzaProject):
        self.project = project
        self.notes: List[str] = []

    def verify(self):
        # Apply H1, H2, H3 rules
        for ctx_name, ctx in self.project.parsed_contexts.items():
            self._verify_context(ctx)

    def _verify_context(self, ctx: Context):
        # Validate that if this context overrode anything from its parent:
        # 1. The full role is re-declared
        # 2. No new events were added to inherited roles
        
        # In a real parser tree, we'd traverse down from the root so parents are processed first
        # Here we just apply the logic assuming parent is raw from parser or already processed.

        if ctx.parent_name and ctx.parent_name in self.project.parsed_contexts:
            parent = self.project.parsed_contexts[ctx.parent_name]
            
            # Inherit roles
            for role_name, parent_role in parent.roles.items():
                if role_name in ctx.roles:
                    # H2 / Explicit override: Child re-declared it
                    child_role = ctx.roles[role_name]
                    
                    if child_role.type_name != parent_role.type_name:
                        raise VerifierError(
                            f"Context {ctx.name} invalidly changes type of inherited role {role_name} "
                            f"from {parent_role.type_name} to {child_role.type_name}"
                        )
                        
                    child_role.is_local = False # It's an override of an inherited role
                    
                    # Check for new events
                    parent_events = {h.event for h in parent_role.handlers}
                    child_events = {h.event for h in child_role.handlers}
                    
                    new_events = child_events - parent_events
                    if new_events:
                        raise VerifierError(
                            f"Context {ctx.name} invalidly adds new events {new_events} to inherited role {role_name}. "
                            f"Use a local role instead."
                        )
                    
                    # Log overrides
                    for ch in child_role.handlers:
                        ph = next((h for h in parent_role.handlers if h.event == ch.event), None)
                        if ph and ph.action.name != ch.action.name:
                            self.notes.append(
                                f"NOTA [herencia]: {ctx.name} sobrescribe {role_name}.{ch.event} "
                                f"(padre: {ph.action.name} -> hijo: {ch.action.name})"
                            )
                else:
                    # H1: Implicit inheritance
                    # Copy the parent role to the child
                    import copy
                    copied_role = copy.deepcopy(parent_role)
                    copied_role.is_local = False
                    ctx.roles[role_name] = copied_role

        # Recursively verify children
        for sub_name, sub_ctx in ctx.subcontexts.items():
            sub_ctx.parent_name = ctx.name
            self._verify_context(sub_ctx)
