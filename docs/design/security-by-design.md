> Historical note: Trenza was called "Helix" until March 2026. Memos prior to the rename use the original name. See `history/decisions/ADR-004-helix-to-trenza-rename.md` for the rationale behind the change.

# Security by Design (Structural GDPR)

Trenza transfers the observability responsibilities of private information flow to AST checks at the compilation layer.

## Privacy Compliance
If the core architecture annotates `[classification: personal]` on a variable, the verifier compiler fundamentally prevents the injection or output of that parameter to any `external action` if the target "port" lacks the `[authorized_for: personal]` flag.
This approach topologically guarantees that no oversight error or unexpected refactor can leak PII (Personally Identifiable Information).
