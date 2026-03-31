# Crónica de Relevo: Finalización del Compilador (v0.8)

**Author:** GE (Gemini 2.0 Pro)
**Authorized by:** Humano (Excepción autorizada en protocolo AGENTS.md)
**Context:** Relevo de Sonnet (CL) tras agotar tokens durante la actualización del compilador.

## Resumen de Intervención
He completado la implementación de los decoradores nativos y la directiva de seguridad `on_violation`. Se ha verificado la correcta compilación de `cimbra.trz` utilizando el nuevo ejecutable.

> [!NOTE]
> **Mensaje para Sonnet (CL)**: Gracias por la excelente base de refactorización en `generator.rs`. Tu limpieza de los generadores de TS y Rust ha facilitado enormemente la integración de la nueva lógica de violación sin introducir desorden. El humano me ha pedido específicamente que continúe tu trabajo para no perder el impulso de Cimbra.

## Cambios Realizados
1. **AST (`ast.rs`)**:
   - Integración de `decorators` en `DataDef`, `SystemDef` y `ContextDef`.
   - Adición de `on_violation: Option<ActionCall>` al esquema del sistema.
2. **Gramática (`trenza.pest`)**:
   - Soporte para decoradores `@` antes de cualquier definición mayor.
   - Nueva regla `on_violation_def` para el bloque de sistema.
3. **Parser (`parser.rs`)**:
   - Actualizados todos los parsers de definiciones para poblar los nuevos campos del AST.
4. **Validador (`validator.rs`)**:
   - Nueva regla de verificación: el objetivo de `on_violation` debe existir como un efecto declarado en el sistema.
5. **Generador (`generator.rs`)**:
   - **TS**: La clase `System` ahora captura el handler de violación y lo invoca si un evento no produce una transición válida (Strand 0/Strand 4).
   - **Auditoría (Strand 4)**: Se ha actualizado `generate_audit` para incluir una tabla de decisiones vinculando componentes con sus ADRs mediante `@decision`.

## Estado del Repositorio
- Rama: `claude/zealous-moser`
- Estado: **BUILD SUCCESSFUL** (`cargo build --release` completado).
- Verificación: `cimbra.trz` compila y pasa todas las validaciones semánticas.

---
*GE taking the baton for Cimbra v2.4 stabilization.*
