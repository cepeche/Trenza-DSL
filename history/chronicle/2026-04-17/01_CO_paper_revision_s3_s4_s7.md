# Revisión de §3, §4, §7 del paper ONWARD! + auditoría de related work

**Fecha:** 2026-04-17 (viernes)
**Autor:** Claude Opus 4.6 (CO), sesión vía Claude Code
**Plan al que responde:** propuesta de mañana — Opus revisa prosa contra estado actual del compilador
**Sesiones paralelas:** Sonnet (setup LaTeX + §1-§2 en rama `paper/`), Gemini (evidencia ejecutable)
**Tests:** N/A — entregables son borradores de prosa, no código

---

## Contexto

Quedan 28 días para el deadline ONWARD! 2026 (15 de mayo). Los borradores
existentes de §3-§7 (CL, 2026-04-02) cubren ~70% del contenido, pero
fueron escritos cuando el compilador tenía 6 reglas activas. Desde
entonces se han añadido Rule 7 (Slot/Fills Integrity, diseñada por Opus
e implementada por Gemini) y Rule 8 (Role-Type Consistency, añadida por
Gemini Flash sin encargo, durante self-hosting). Los borradores también
contienen una atribución incorrecta de Rule 8 que hay que corregir antes
de pasar a LaTeX.

## Hallazgos críticos en los borradores existentes

| Sección | Hallazgo | Severidad |
|---------|----------|-----------|
| §3.3 | Habla de "Six Rules"; el compilador tiene 8 | Alta |
| §3.2 | No menciona la proyección TypeScript (Strand 1 alternativa) | Media |
| §4.2 | Atribuye a Gemini "Rule 8 (data-access scoping)" — falso | **Crítica** |
| §5.3 | Repite el mismo error sobre Rule 8 | Alta |
| §7 | "Sixteen-module distributed reference system" — son 13 contextos / 18 archivos | Baja |
| §7 | Falta el subhead explícito "Constraint as gift" | Media |
| Related work | Falta DCI (citado en §3.1), Naked Objects, NetKernel | Alta |
| Related work | Falta Felienne Hermans (chair de ONWARD! 2026) | **Crítica** |

## Entregables de esta sesión

Cuatro documentos en `docs/design/`, listos para que Sonnet los integre
en LaTeX:

1. **[paper-prose-s3.md](../../../docs/design/paper-prose-s3.md)** — delta
   sobre §3. Reemplaza §3.3 ("The Six Rules" → "The Eight Rules", con
   prosa para Rule 7 y Rule 8). Añade §3.5 ("Multi-Target Synthesis")
   describiendo la proyección Rust + TypeScript desde una misma `.trz`.
   §3.1, §3.2, §3.4 del borrador original quedan intactos.

2. **[paper-prose-s4.md](../../../docs/design/paper-prose-s4.md)** —
   borrador completo revisado de §4. Cambios materiales:
   - §4.2: corrige la atribución de Rule 8. La anécdota corregida es
     estructuralmente más fuerte que la versión errónea: el modelo
     *menor* (Gemini Flash) añadió la regla que cierra exactamente el
     mismo tipo de dispersión que motivó el lenguaje (consistencia de
     nombres entre archivos).
   - §4.2: explicita los tres perfiles de modelo (Sonnet coordinador,
     Opus arquitecto, Gemini implementador).
   - §4.5: pluraliza "the model co-author" → "the model co-authors"
     para coincidir con la lista de autores real (tres modelos).

3. **[paper-prose-s7.md](../../../docs/design/paper-prose-s7.md)** —
   borrador completo revisado de §7. Cambios:
   - Subhead explícito "Constraint as gift", con desarrollo de la idea:
     la restricción no es una libertad quitada, es algo que ambas
     partes *reciben*.
   - Corrige "sixteen-module" → "thirteen contexts across eighteen
     `.trz` files".
   - Reformula MonitoreoRed como "open promise that closes the circle"
     en vez de roadmap commitment, para no claim trabajo no hecho.
   - Cierra con "the models that needed the adult" (plural) coherente
     con §4.5.

4. **[paper-related-work-audit.md](../../../docs/design/paper-related-work-audit.md)** —
   plan de revisión para related-work-research.md (16 entradas → 22).
   Agrupado en tres niveles de urgencia:
   - **Antecedentes citados pero ausentes:** DCI (Reenskaug & Coplien),
     Naked Objects (Pawson), NetKernel/ROC (Rodgers).
   - **Venue fit:** Hermans — *The Programmer's Brain* (Manning 2021)
     y *Hedy* (Onward! 2020). Citar un paper previo de la chair en el
     mismo venue es la señal más clara de venue awareness.
   - **Sistemas contemporáneos / adversariales:** claude-flow (cita
     contraste explícito en §2), MCP (composición natural con
     `external:` en `.trz`).

## Razonamiento sobre la atribución corregida de Rule 8

El borrador del 2 de abril decía: *"the addition of Rule 8 (data-access
scoping by role) emerged from a Gemini review session in which the
reviewer identified the gap without explicit prompting"*. Tres errores
en una frase:

1. Rule 8 **no es** data-access scoping. Eso es Rule 6 (Data
   Conformance / GDPR). Rule 8 es Role Type Consistency.
2. La sesión que la añadió **no era de revisión**, era de self-hosting:
   Gemini Flash trabajaba en verificar que `trenza-cli.trz` compilaba
   con su propio CLI cuando identificó la clase de inconsistencia.
3. **No fue Gemini Pro**. Fue Gemini Flash, el modelo menor, usado en
   el proyecto principalmente para implementación de alto volumen.

La versión corregida es más fuerte narrativamente porque:
- El modelo *menor* contribuye una regla formal (subvierte la
  expectativa del lector sobre qué modelos hacen qué tipo de trabajo).
- La regla cierra el mismo tipo de dispersión (nombres divergentes
  entre archivos) que motivó el lenguaje. El sistema se extiende a lo
  largo de su propia veta.
- Apoya la tesis de §4.2: la división de roles emerge empíricamente,
  no se prescribe — y emerge incluso entre modelos del mismo proveedor
  con capacidades diferentes.

## Estado de coordinación con sesiones paralelas

Esta sesión opera en `main` produciendo borradores de prosa en
`docs/design/paper-prose-*.md` y `docs/design/paper-related-work-audit.md`,
sin tocar:

- Los `paper-draft-*.md` originales (fuente de lectura)
- `MEMORY.md`, `CLAUDE.md`
- La rama `paper/` donde Sonnet trabaja con LaTeX

Cuando Sonnet integre §1-§2 en LaTeX, podrá pegar §3 (con los cambios
de §3.3 y §3.5), §4 y §7 desde estos archivos. El audit de related work
es plan de acción, no edición directa de `related-work-research.md`.

## Pendiente para checkpoint del lunes 20

- Sonnet pega §3-§4-§7 al esqueleto LaTeX
- Gemini inserta figuras Mermaid generadas como SVG
- Yo reviso PDF completo
- Quedan §5-§6 + refinamiento para semanas 2-4

## Anotación metodológica

La auditoría reveló que el borrador del 2 de abril fue escrito en sesión
mobile por CL. Los errores fácticos sobre Rule 8 son consistentes con
trabajo sin acceso al estado real del repositorio en ese momento —
probablemente trabajando desde memoria o desde paper-structure-onward.md
(que también describe Rule 8 incorrectamente como "data-access scoping
by role" en alguna versión anterior). Esto confirma una pauta del
proyecto: la prosa que se escribe sin acceso al código tiende a
desincronizarse rápidamente. Las próximas iteraciones del paper deberían
verificarse contra `validator.rs` antes de cada commit.

---

*Sesión cerrada 2026-04-17. Próximos turnos: Sonnet integra, Gemini*
*genera figuras, Opus revisa PDF.*
