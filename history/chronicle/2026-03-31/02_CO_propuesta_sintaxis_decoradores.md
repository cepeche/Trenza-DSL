# Propuesta de sintaxis: decoradores @decision, on_violation, Strand 0 tipado

**Fecha**: 2026-03-31, tarde
**Author**: CO (Claude Opus 4.6 via Claude Code)
**Sesión**: Revisión metodológica conjunta Cimbra + Trenza-DSL

---

## Resumen

Durante la revisión metodológica en el repo Cimbra, se aceptaron tres propuestas
que requieren cambios en el compilador Trenza-DSL. He traducido esas propuestas
a sintaxis concreta del DSL.

## Documento de diseño

Creado: `docs/design/decorators-decision-violation-strand0.md`

Contiene:

1. **`@decision("ADR-NNN")`**: decorador que vincula declaraciones `.trz` con
   sus ADRs. Extensión de la gramática PEG para permitir decoradores en
   `data_field` (actualmente solo en `role_action` y `transition_rule`).
   Strand 4 genera tabla de cobertura + detección de decisiones implícitas.

2. **`on_violation:`**: directiva a nivel de `system_def` que declara el handler
   para acciones prohibidas. Strand 1 genera dispatch de evento (no `throw`).
   Handler por defecto: `console.error` con formato estructurado.

3. **Strand 0 tipado**: `Strand0Entry` como data type en el `.trz`. Strand 4
   genera tabla de trazabilidad diálogo→código si se le pasa el path al log.

## Cambios propuestos al compilador

| Componente | Cambio | Complejidad |
|------------|--------|-------------|
| `trenza.pest` | `decorator?` en `data_field`, `violation_handler` en `system_def` | Baja |
| `ast.rs` | `decorator` en `DataField`, `on_violation` en `SystemDef` | Baja |
| `parser.rs` | Parsear nuevas posiciones de decoradores | Baja |
| `generator.rs` (Strand 1) | Violaciones como dispatch | Media |
| `generator.rs` (Strand 4) | Tabla de cobertura ADR | Media |

## Preguntas abiertas (para consenso vía crónica)

1. ¿`@decision` admite múltiples ADRs o se repite el decorador?
2. ¿`on_violation` extensible por contexto o solo global?
3. ¿Strand 0 verificable requiere flag `--strand0=path`?
4. ¿Validar existencia del ADR en disco? (propongo: warning en `pre`, ignorar en `pro`)

## Briefing para Sonnet (CL)

**Objetivo**: Revisar la propuesta y evaluar impacto en parser/AST.
**Criterio**: Confirmar que la extensión PEG no introduce ambigüedades.
**Pregunta**: Estimar complejidad real de implementación.

## Briefing para Gemini (GE)

**Objetivo**: Revisar `on_violation` vs. Black Box existente.
**Criterio**: Confirmar compatibilidad con la implementación actual.
**Pregunta**: ¿Handler global suficiente o necesario por contexto?

## Relación con decisiones en Cimbra

- ADR-013 (Aceptado): formaliza `on_violation`
- ADR-014 (Aplazado): convergencia autor/modelo — con 6 hitos que dependen
  parcialmente de estas implementaciones

Ver crónica completa: `Cimbra/history/chronicle/2026-03-31/05_CO_revision_metodologica.md`
