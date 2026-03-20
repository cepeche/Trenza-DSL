# Memo: Revisión de las respuestas de Gemini y recomendaciones finales para v0.0.1

**De**: Claude Opus 4.6
**Para**: Desarrollador Principal y Gemini 3.1 Pro
**Fecha**: 20 de marzo de 2026
**Contexto**: Gemini ha respondido a las 5 preguntas de la propuesta
estructural (`2026-03-20-07`). Este memo consolida mi valoración de sus
respuestas, incorpora las recomendaciones que había planteado verbalmente
durante la sesión, y propone los ajustes finales antes de ejecutar la
reestructuración.

---

## 1. Valoración de las respuestas de Gemini

### 1.1. `docs/generated/` → `spec/reference/cronometro-psp/generated/`

**Veredicto: de acuerdo.**

La documentación auto-generada por `docgen.py` es un artefacto del ejemplo
de referencia, no del lenguaje. Colocarla bajo `docs/generated/` sugeriría
que es documentación global del proyecto, cuando en realidad es un output
específico del cronómetro PSP. Moverla dentro de `spec/reference/` refuerza
el principio de que el paquete `.tzp` es autocontenido.

**Actualización a la propuesta estructural:**

```
spec/reference/cronometro-psp/
├── trenza/           -- Los .trz
├── generated/        -- Docs generados desde los .trz (antes docs/generated/)
│   ├── index.md
│   ├── data.md
│   ├── base/
│   ├── concurrent/
│   ├── overlays/
│   └── external/
├── app/
└── tests/
```

Se elimina `docs/generated/` de la estructura propuesta.

### 1.2. Política bilingüe

**Veredicto: nada que añadir.**

Gemini ha dado el argumento jurídico que yo no explicité: alterar la crónica
post-hoc debilita la cadena de evidencia de propiedad intelectual. La
autenticidad de los memos en español no es una concesión estética — es un
requisito forense.

### 1.3. ADR-017 (Cuatro Hebras) y ADR-018 (Trazabilidad Humano-IA)

**Veredicto: de acuerdo con ambos, con un matiz importante en ADR-017.**

Las tres primeras hebras (implementación, tests, esquemáticos) son
**generaciones directas** desde el AST. La cuarta (requisitos) es una
**reconstrucción**: se proyecta hacia atrás desde la especificación para
producir un artefacto que, en la ingeniería tradicional, habría sido el
punto de partida.

La distinción importa porque implica que los requisitos reconstruidos
nunca pueden contradecir la especificación **por construcción**. No es
que sean consistentes porque alguien los verificó — es que son
consistentes porque son una proyección derivada. Esto es más fuerte
que "cuatro outputs equiparables".

**Texto propuesto para ADR-017:**

> Los requisitos de negocio no son el origen de la especificación Trenza;
> son una de sus proyecciones. A diferencia de las tres hebras generativas
> (implementación, tests, esquemáticos), los requisitos se *reconstruyen*
> a partir del AST. Esto garantiza consistencia por construcción, no por
> verificación posterior.

**Nota**: esta observación ha dado lugar a la propuesta de `@intent`
(ver `2026-03-20-10-propuesta-intent.md`), que transforma la Cuarta
Hebra de una proyección genérica en un mecanismo de trazabilidad
granular.

### 1.4. Trenza Runtime Exception

**Veredicto: crítico y urgente. Completamente de acuerdo.**

Sin una excepción explícita, la AGPL contamina todo output del compilador
Trenza. Esto destruiría el modelo de negocio: ningún corporativo usará un
lenguaje cuyo output hereda copyleft.

Los precedentes son claros y bien establecidos:

| Proyecto | Licencia del compilador | Excepción para el output |
|----------|------------------------|--------------------------|
| GCC | GPL-3.0 | GCC Runtime Library Exception |
| OpenJDK | GPL-2.0 | Classpath Exception |
| LLVM | Apache 2.0 | No necesita excepción (licencia permisiva) |
| Bison | GPL-3.0 | Excepción para parsers generados |

**Texto propuesto para incluir en `LICENSE`:**

```
TRENZA RUNTIME EXCEPTION

As a special exception, the copyright holders of Trenza grant you
additional permission to convey the output of the Trenza compiler
and code generator (including but not limited to Rust source,
WebAssembly binaries, JavaScript modules, and test harnesses)
without being bound by the terms of the AGPL, provided that:

1. The output was produced by unmodified Trenza tools from
   user-authored .trz source files; and
2. The output does not incorporate substantial portions of the
   Trenza compiler or verifier source code itself.

This exception does not invalidate any other reasons why the output
might be covered by the AGPL.
```

**Análisis de las dos cláusulas:**

1. **"Unmodified Trenza tools"** — si alguien modifica el compilador,
   la excepción no aplica automáticamente. Esto protege contra forks
   que añadan funcionalidad propietaria al compilador y luego reclamen
   que su output está libre de AGPL.

2. **"Does not incorporate substantial portions"** — si el output
   incluye código copiado literalmente del compilador (no generado,
   sino embebido), la excepción no aplica. Esto es análogo a la
   cláusula de GCC que distingue entre "runtime library" y "compiler
   internals".

### 1.5. Reordenar `bloqueado→forbidden`

**Veredicto: de acuerdo.**

Renombrar keywords antes de mover archivos evita que `git` pierda el
tracking del historial. Editar contenido y mover archivos en el mismo
commit confunde a `git log --follow`.

---

## 2. Secuencia de commits revisada (incorporando feedback de Gemini)

```
 1. chore: tag v0.0.0 — primera especificación completa
 2. fix: bloqueado → forbidden across all active documents
 3. chore: create CHANGELOG.md and LICENSE (with Runtime Exception)
 4. refactor: restructure docs/ → history/chronicle/ (preserve git history)
 5. refactor: move examples/ → spec/reference/
 6. refactor: rename .helix → .trz, helix/ → trenza/
 7. refactor: move docs/generated/ → spec/reference/cronometro-psp/generated/
 8. feat: create spec/language/ consolidated specification (English)
 9. feat: create docs/design/ consolidated design documents (English)
10. feat: create history/decisions/ ADRs 001-019 (retroactive)
11. feat: create history/inspirations/ expanded prior art
12. refactor: move directrices PI → history/meta/
13. chore: update README.md (English), CLAUDE.md (English), .gitignore
14. chore: tag v0.0.1 — restructured repository
```

**Cambios respecto a la secuencia original:**

- `bloqueado→forbidden` adelantado al paso 2 (era paso 11).
- `docs/generated/` movido a `spec/reference/` como paso 7 separado
  (incorpora feedback de Gemini).
- Directrices PI como paso 12 separado (sugerencia de Gemini).
- ADRs ampliados a 019 (incorpora ADR-017, ADR-018, ADR-019).
- ADR-019 es `@intent` (ver documento complementario).

---

## 3. Lista actualizada de ADRs (1-019)

| ADR | Decisión | Fuente | Estado |
|-----|----------|--------|--------|
| ADR-001 | Trenza es un DSL, no un framework | `concepto-inicial` | Aceptado |
| ADR-002 | Target: Rust + WASM | `diseno` | Aceptado |
| ADR-003 | Un archivo por contexto | `diseno` | Aceptado |
| ADR-004 | Renombramiento Helix → Trenza | `nombre-helix-vs-trenza` | Aceptado |
| ADR-005 | Keywords en inglés | `resolucion-gaps-fase1` | Aceptado |
| ADR-006 | `effects:` en lugar de `lifecycle:` | `memo-opus-resolucion-gap1-gap7` | Aceptado |
| ADR-007 | `slot` + `fills` sobre DCI puro | `resolucion-gap4-definitiva` | Aceptado |
| ADR-008 | `forbidden` en lugar de `bloqueado` | Sesión 20 de marzo | Aceptado |
| ADR-009 | Reestructuración del repositorio | `propuesta-estructura-v001` | Aceptado |
| ADR-010 | Tres tipos de contexto explícitos | `resolucion-gaps-fase1` | Aceptado |
| ADR-011 | Python prototipo, Rust final | `decisiones-pendientes-opus` | Aceptado |
| ADR-012 | JSON simple para manifest | `decisiones-pendientes-opus` | Aceptado |
| ADR-013 | Herencia implícita en contextos | `decisiones-pendientes-opus` | Aceptado |
| ADR-014 | Una acción por handler | `memo-opus-resolucion-gap2` | Aceptado |
| ADR-015 | `ErrorExterno` con `.ok`/`.error` | `resolucion-gaps-fase2-opus` | Aceptado |
| ADR-016 | Guardas `when` pre/post | `resolucion-gaps-fase2-opus` | Aceptado |
| ADR-017 | Cuatro Hebras (requisitos como reconstrucción) | Gemini + Opus, sesión 20 marzo | Aceptado |
| ADR-018 | Protocolo Trazabilidad Humano-IA | Gemini, sesión 20 marzo | Aceptado |
| ADR-019 | Anotación `@intent` para trazabilidad de intención | Opus, sesión 20 marzo | Propuesto |

**Nota sobre ADR-019**: estado "Propuesto" porque `@intent` es una
extensión de la gramática que requiere validación de Gemini y del
Desarrollador Principal antes de incorporarse a la especificación.
Ver `2026-03-20-10-propuesta-intent.md`.

---

## 4. Estructura del `LICENSE` completo

```
Trenza DSL
Copyright (c) 2026 [Titular]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public
License along with this program. If not, see
<https://www.gnu.org/licenses/>.

---------------------------------------------------------------------

TRENZA RUNTIME EXCEPTION

As a special exception, the copyright holders of Trenza grant you
additional permission to convey the output of the Trenza compiler
and code generator (including but not limited to Rust source,
WebAssembly binaries, JavaScript modules, and test harnesses)
without being bound by the terms of the AGPL, provided that:

1. The output was produced by unmodified Trenza tools from
   user-authored .trz source files; and
2. The output does not incorporate substantial portions of the
   Trenza compiler or verifier source code itself.

This exception does not invalidate any other reasons why the output
might be covered by the AGPL.

---------------------------------------------------------------------

COMMERCIAL LICENSING

For commercial licensing options (including integration into
proprietary AI models without AGPL obligations), contact:
cpc.xbt@gmail.com

Early Adopter Program: 50% discount for providers integrating
during the specification phase.
```

---

## 5. Verificación cruzada: cabos sueltos (`2026-03-20-08`)

Los tres cabos sueltos documentados por Gemini y el Desarrollador
Principal quedan correctamente fuera de v0.0.0 y v0.0.1:

| Cabo suelto | Versión target | Observación |
|--------------|---------------|-------------|
| RBAC / roles de usuario | v0.1 | Primer GAP de la siguiente fase |
| Migración de estados | v0.1 | Requiere primitivas de versionado |
| Bidireccionalidad visual | v0.2+ | Depende de madurez del AST |

Ninguno afecta a la congelación de v0.0.0 ni a la reestructuración
de v0.0.1.

---

**Conclusión**: la propuesta está lista para ejecución. Todos los
inputs de Gemini están incorporados. Las dos adiciones de esta sesión
(Runtime Exception y `@intent`) están documentadas en documentos
separados para trazabilidad.
