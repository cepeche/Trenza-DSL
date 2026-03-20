# Plan de trabajo revisado — Semana del 16 de marzo

**Fecha**: 13 de marzo de 2026
**Autor**: Claude, en consulta con el Desarrollador Principal
**Sustituye a**: `2026-03-13-04-plan-semanal.md` (plan de Rust — aplazado)

---

## Principio rector

> "Priorizar claridad conceptual sobre implementación prematura."
> — CLAUDE.md, principio de diseño

El prototipo Python es **exploratorio**. Su valor es validar decisiones
de diseño, no ser el producto final. No cambiamos de lenguaje hasta que
el diseño esté estabilizado.

## Modo de trabajo

Sesiones supervisadas por el desarrollador principal. Cada bloque tiene
un objetivo claro y un entregable revisable al final de la sesión.
No hay trabajo autónomo sin supervisión.

---

## Bloque 1 — Resolución de GAPs (lidera Claude, audita Antigravity)

**Objetivo**: cerrar los 8 GAPs de diseño abiertos. Cada GAP se resuelve
con una decisión documentada que modifica la gramática o las reglas.

| GAP | Descripción | Impacto |
|-----|-------------|---------|
| GAP-1 | Eventos de lifecycle (`[al_entrar]`, `[al_salir]`) | Gramática |
| GAP-2 | Acciones compuestas (múltiples efectos por evento) | Gramática |
| GAP-3 | Guardas/condiciones en transiciones | Gramática + verificador |
| GAP-4 | Roles condicionales (presentes solo si hay contexto concurrente) | Verificador |
| GAP-5 | Transiciones condicionales (ramificación por estado) | Gramática + verificador |
| GAP-6 | Datos mutables vs inmutables | `data.trz` |
| GAP-7 | Efectos de entrada con parámetros | Gramática |
| GAP-8 | Acciones "ignorar" (evento declarado sin efecto) | Verificador |

**Entregable**: documento de resolución por GAP + archivos .trz actualizados.

## Bloque 2 — Gramática formal (lidera Claude, audita Antigravity)

**Objetivo**: producir una gramática PEG o EBNF completa del lenguaje Trenza,
validada contra todos los .trz existentes en `examples/cronometro-psp/`.

**Prerequisito**: Bloque 1 (los GAPs afectan la gramática).

**Entregable**: `docs/gramatica-trenza.peg` + documento de justificación.

## Bloque 3 — Verificador completo (lidera Antigravity, audita Claude)

**Objetivo**: implementar las 6 reglas de verificación en el prototipo Python.

| Regla | Estado actual |
|-------|---------------|
| Completitud | ❌ pendiente |
| Determinismo | ❌ pendiente |
| Alcanzabilidad | ❌ pendiente |
| Retorno | ❌ pendiente |
| Exhaustividad | ❌ pendiente |
| Conformidad de datos | ❌ pendiente |

Más las 5 reglas de herencia (H1-H5), de las que H1, H2 y H5 ya están
implementadas. Faltan H3 (completitud por nivel) y H4 (no type change).

**Entregable**: verificador que pase todos los .trz del cronómetro sin
falsos positivos ni falsos negativos.

## Bloque 4 — Docgen con zonas protegidas (lidera Antigravity, audita Claude)

**Objetivo**: evolucionar `docgen.py` para que:
1. Genere esqueletos desde el AST (estructura, tablas de roles/transiciones)
2. Preserve secciones marcadas como manuales (GAPs, notas H1-H5, comentarios)
3. Nunca sobreescriba contenido enriquecido sin `--force`

**Entregable**: docgen que regenere `docs/sistema/` sin perder contenido manual.

## Bloque 5 — Opacidad criptográfica de entradas

**ATENCIÓN Antigravity**: lee `2026-03-13-07-opacidad-criptografica-inputs.md`.

Este es un objetivo estratégico del proyecto al mismo nivel que el ciclo
de vida. Nuestro desarrollador principal propone que **toda entrada de
usuario se cifre en la frontera UI → sistema** como mecanismo de seguridad
por diseño. No es un nice-to-have ni un argumento comercial: es una
propiedad fundamental que debe informar el diseño de la gramática.

**Impacto en tu trabajo**:
- El parser debe soportar la declaración de **puntos de consumo** donde
  los datos opacos se descifran/interpretan.
- El verificador debe auditar que todo dato externo pasa por un punto
  de consumo declarado antes de ser interpretado.
- Candidata a 7ª regla de verificación: **Opacidad**.

Esto no es un bloque de implementación todavía, pero **toda decisión
de diseño en los bloques 1-4 debe tener en cuenta que los datos
externos serán opacos por defecto**.

---

## Sobre el salto a Rust

Rust será el lenguaje de implementación final. No lo descartamos, lo
aplazamos hasta que:

- [ ] Los 8 GAPs estén cerrados
- [ ] La gramática formal esté validada
- [ ] Las 6+5 reglas de verificación funcionen en Python
- [ ] El modelo de opacidad criptográfica esté diseñado
- [ ] El ciclo de vida esté al menos esbozado

Entonces migrar será un ejercicio de traducción, no de diseño simultáneo.

---

## Nota sobre atribución

Las ideas de este plan tienen orígenes diversos. Para que quede explícito:

- Opacidad criptográfica de entradas: **Desarrollador Principal**
- Ciclo de vida y versionado: **Desarrollador Principal** (experiencia XBRL)
- Entorno inspirado en Arduino/Processing: **Desarrollador Principal**
- Árbol .md navegable como docs de requisitos: **Desarrollador Principal**
- Docgen automático: **Antigravity**
- Parser y verificador Python: **Antigravity**
- Resolución de reglas H1-H5: **Claude** (en consulta con Desarrollador)
- Análisis de GAPs y especificación del cronómetro: **Claude**
- Priorización y secuenciación de este plan: **Claude**
