# Propuesta: Estructura del repositorio para v0.0.1

**De**: Claude Opus 4.6 (sesión 20 de marzo de 2026)
**Para**: Desarrollador Principal + Gemini 3.1 Pro (validación)
**Fecha**: 20 de marzo de 2026
**Contexto**: La primera especificación completa de Trenza está cerrada
(8/8 GAPs resueltos). Es el momento de congelar v0.0.0 y reorganizar el
repositorio para v0.0.1 con una estructura profesional que refleje la
naturaleza peculiar de este proyecto.
**Decisiones incorporadas**: Licencia dual AGPL + Comercial (ver sección
correspondiente) y política de idiomas bilingüe (ver sección final).

---

## Principio rector de la reestructuración

Este proyecto tiene dos productos, no uno:

1. **El lenguaje Trenza** — su especificación, gramática, reglas de
   verificación y herramientas.

2. **El proceso de diseño** — las interacciones entre humano y LLMs,
   las inspiraciones, las decisiones tomadas y sus razones. Este proceso
   es tan valioso como el resultado, y debe quedar como traza auditable.

La estructura del repositorio debe hacer justicia a ambos sin mezclarlos.

---

## Plan de ejecución

### Paso 1: Congelar v0.0.0

Antes de tocar nada:

```bash
git tag -a v0.0.0 -m "Primera especificación completa de Trenza (8/8 GAPs resueltos)"
git push origin v0.0.0
```

El tag marca el estado actual del repositorio — con todas sus
imperfecciones, nombres en Helix, y estructura provisional — como
hito auditable. Cualquier investigador puede hacer `git checkout v0.0.0`
y ver exactamente cómo llegamos hasta aquí.

### Paso 2: Reestructurar para v0.0.1

La reestructuración se hace en commits atómicos, cada uno con un
propósito claro, para que `git log` cuente la historia.

---

## Estructura propuesta

```
Trenza-DSL/
│
├── README.md                         -- Carta de presentación del proyecto
├── CHANGELOG.md                      -- Historial de versiones (semántico)
├── LICENSE                           -- Licencia del proyecto
├── CLAUDE.md                         -- Instrucciones para agentes IA
├── .gitignore
│
├── spec/                             -- LA ESPECIFICACIÓN (artefacto primario)
│   ├── language/                     -- Definición del lenguaje
│   │   ├── 01-overview.md            -- Qué es Trenza, para quién, por qué
│   │   ├── 02-grammar.md            -- Gramática completa
│   │   ├── 03-verification.md       -- Las 6+5 reglas de verificación
│   │   ├── 04-package-format.md     -- Formato .tzp
│   │   └── 05-cli.md               -- Interfaz de línea de comandos
│   │
│   └── reference/                    -- Ejemplo canónico
│       └── cronometro-psp/           -- El banco de pruebas original
│           ├── trenza/               -- Los .trz (especificación)
│           │   ├── system.trz
│           │   ├── data.trz
│           │   ├── contexts/
│           │   └── external/
│           ├── app/                  -- La aplicación real (backend + frontend)
│           │   ├── backend/
│           │   └── frontend/
│           └── tests/
│
├── src/                              -- HERRAMIENTAS (prototipo Python)
│   └── trenza/
│       ├── __init__.py
│       ├── ast.py
│       ├── parser.py
│       ├── verifier.py
│       ├── docgen.py
│       ├── mermaid.py
│       └── cli.py
│
├── docs/                             -- DOCUMENTACIÓN PUBLICABLE
│   ├── manual/                       -- Manual de usuario
│   │   └── trenza-manual.md         -- El manual (hoy 2026-03-20-04)
│   │
│   ├── generated/                    -- Docs auto-generados por docgen
│   │   └── sistema/                  -- Los .md generados desde .trz
│   │       ├── index.md
│   │       ├── data.md
│   │       ├── base/
│   │       ├── concurrent/
│   │       ├── overlays/
│   │       └── external/
│   │
│   └── design/                       -- Decisiones de diseño consolidadas
│       ├── principles.md             -- Los 4 principios (consolidado)
│       ├── dci-influences.md         -- DCI y Reenskaug (consolidado)
│       ├── security-by-design.md     -- Seguridad y RGPD (consolidado)
│       └── lifecycle.md              -- Ciclo de vida y versionado
│
├── history/                          -- TRAZA AUDITABLE (el archivo vivo)
│   │
│   ├── README.md                     -- Explicación del archivo histórico
│   │
│   ├── chronicle/                    -- Crónica del proyecto (los memos)
│   │   ├── 2026-03-04/              -- Un directorio por día
│   │   │   ├── 01-concepto-inicial.md
│   │   │   ├── 02-diseno.md
│   │   │   ├── 03-arte-previo.md
│   │   │   ├── 04-influencias-dci.md
│   │   │   └── 05-metafora-circuitos.md
│   │   ├── 2026-03-05/
│   │   │   ├── 01-memo-gemini.md
│   │   │   ├── ...
│   │   │   └── 06-memo-claude-respuesta-gemini-ronda2.md
│   │   ├── 2026-03-06/
│   │   ├── 2026-03-10/
│   │   ├── 2026-03-12/
│   │   ├── 2026-03-13/
│   │   ├── 2026-03-18/
│   │   └── 2026-03-20/
│   │       ├── 01-memo-claude-gap4-slots.md
│   │       ├── 02-memo-gemini-respuesta-gap4.md
│   │       ├── 03-resolucion-gap4-definitiva.md
│   │       ├── 04-manual-usuario-trenza.md
│   │       └── 05-propuesta-estructura-v001.md
│   │
│   ├── decisions/                    -- Registro de decisiones (estilo ADR)
│   │   ├── README.md                -- Qué es un ADR y cómo se usa aquí
│   │   ├── ADR-001-dsl-not-framework.md
│   │   ├── ADR-002-rust-wasm-target.md
│   │   ├── ADR-003-one-file-per-context.md
│   │   ├── ADR-004-helix-to-trenza-rename.md
│   │   ├── ADR-005-english-keywords.md
│   │   ├── ADR-006-effects-not-lifecycle.md
│   │   ├── ADR-007-slot-fills-over-pure-dci.md
│   │   ├── ADR-008-forbidden-over-bloqueado.md
│   │   └── ...
│   │
│   ├── inspirations/                 -- Fuentes de inspiración y arte previo
│   │   ├── README.md                -- Índice con estado de cada comparación
│   │   ├── xstate.md                -- Statecharts de Harel via XState
│   │   ├── elm-tea.md               -- The Elm Architecture
│   │   ├── eiffel-dbc.md            -- Design by Contract
│   │   ├── tlaplus.md               -- TLA+ / PlusCal
│   │   ├── dafny.md                 -- Microsoft Dafny
│   │   ├── self-spec.md             -- Self-Spec (OpenReview 2025)
│   │   ├── alloy-llms.md            -- Alloy + LLMs (Cunha & Macedo)
│   │   ├── reenskaug-dci.md         -- DCI y OORam
│   │   ├── cohen-ruv.md             -- Soluciones autocontenidas (rUv)
│   │   ├── holzmann-power-of-ten.md -- NASA JPL
│   │   └── parnas-rational.md       -- Parnas & Clements 1986
│   │
│   ├── conversations/               -- Backups de conversaciones IA
│   │   └── 2026-03-18_antigravity_brain_backup.zip
│   │
│   └── meta/                        -- Reflexiones sobre el proceso
│       ├── metafisica-de-trenza.md  -- El loro estocástico
│       ├── colaboracion-llms.md     -- Multi-LLM como equipo
│       └── reflexiones-equipo.md    -- Observaciones del equipo
│
└── .agents/                          -- Automatización de agentes IA
    └── workflows/
        └── cierre_de_sesion.md
```

---

## Justificación de las decisiones estructurales

### Por qué `spec/` en lugar de mantener `examples/`

La especificación ES el artefacto primario de Trenza. No es documentación
sobre el código; es el código. Colocarla bajo `spec/` comunica esta
jerarquía. El cronómetro PSP no es un "ejemplo" — es la **implementación
de referencia** que demuestra que la especificación funciona.

### Por qué `history/` como directorio de primer nivel

En un proyecto convencional, los memos entre desarrolladores irían a un
wiki o se perderían en Slack. Este proyecto es distinto: la traza de cómo
tres LLMs y un humano co-diseñaron un lenguaje es un artefacto de
investigación con valor propio. Merece visibilidad de primer nivel, no un
subdirectorio olvidado de `docs/`.

### Por qué `chronicle/` con subdirectorios por día

Los memos actuales usan el prefijo `YYYY-MM-DD-NN-titulo.md`. Eso escala
mal: 42 archivos en un directorio plano ya es incómodo. Agrupar por día
mantiene el orden cronológico sin perder la numeración interna.

Los nombres de archivo pierden el prefijo de fecha (redundante con el
directorio) pero conservan el número secuencial y el título:
`docs/2026-03-18-05-memo-opus-resolucion-gap1-gap7.md` se convierte en
`history/chronicle/2026-03-18/05-memo-opus-resolucion-gap1-gap7.md`.

### Por qué ADRs (Architecture Decision Records)

Los ADRs son un formato estándar (propuesto por Michael Nygard) para
documentar decisiones arquitectónicas de forma concisa y auditable.
Cada ADR tiene:

- **Contexto**: qué problema se enfrentaba.
- **Decisión**: qué se decidió.
- **Consecuencias**: qué implica la decisión.
- **Estado**: propuesto / aceptado / sustituido.
- **Participantes**: quién participó (humano, Sonnet, Opus, Gemini).

Los ADRs se extraen retroactivamente de los memos existentes. No
sustituyen a los memos — los indexan. Un ADR dice "se decidió X por
las razones Y"; el memo en `chronicle/` tiene la discusión completa.

### Por qué `inspirations/` separado del arte previo

El documento actual (`2026-03-04-03-arte-previo.md`) es un snapshot
del día 1. Las inspiraciones crecen: Holzmann se añadió el 18 de marzo;
pueden aparecer más. Un directorio dedicado permite que cada fuente tenga
su propio archivo con:

- Resumen de la iniciativa.
- Qué tomamos de ella.
- En qué nos diferenciamos.
- Referencias y URLs.

El `README.md` del directorio funciona como índice y tabla comparativa.

### Por qué `docs/design/` consolida y `history/chronicle/` preserva

Los memos son material bruto, valioso por su proceso. Pero un lector
nuevo no debería tener que leer 42 memos para entender el diseño.
`docs/design/` contiene documentos consolidados que sintetizan las
decisiones, con notas al pie que enlazan a los memos originales en
`history/chronicle/`.

Ejemplo de nota al pie:

```markdown
## Efectos de dominio

La sección `effects:` reemplaza propuestas anteriores de `lifecycle:`.[^1]

[^1]: Ver discusión completa en
[history/chronicle/2026-03-18/05-memo-opus-resolucion-gap1-gap7.md].
Gemini propuso `lifecycle:`; Opus argumentó en contra por las connotaciones
de frameworks frontend. El Desarrollador Principal validó `effects:`.
```

---

## Tratamiento de referencias a Helix

### Regla general

En documentos que se **reescriben** (README, spec, docs/design), se usa
"Trenza" sin excepción. Se añade una nota al principio de `docs/design/`:

> Nota histórica: Trenza se llamó "Helix" hasta marzo de 2026. Los memos
> anteriores al renombramiento usan el nombre original. Ver
> `history/decisions/ADR-004-helix-to-trenza-rename.md` para la
> justificación del cambio.

### En `history/chronicle/`

Los memos originales NO se modifican. Son registro histórico. Si dicen
"Helix", dicen "Helix". La autenticidad de la traza es más importante
que la consistencia cosmética.

### En `spec/reference/cronometro-psp/`

El directorio `helix/` dentro del ejemplo se renombra a `trenza/`.
Los archivos `.helix` se renombran a `.trz`. El contenido se actualiza
(palabras clave: `bloqueado` → `forbidden`, etc.).

El directorio original `helix/` con los `.helix` se preserva en una
nota del CHANGELOG:

```markdown
## [0.0.1] — 2026-03-20

### Cambiado
- Renombrado Helix → Trenza en toda la especificación y herramientas.
- `bloqueado` → `forbidden` en el vocabulario reservado.
- Archivos `.helix` → `.trz`; `.helixpkg` → `.tzp`.
- Reestructuración completa del repositorio (ver ADR-009).

### Nota
El estado previo al renombramiento está preservado íntegramente en el
tag `v0.0.0`.
```

---

## Tratamiento de `bloqueado` → `forbidden`

Además del cambio en los documentos de hoy, la v0.0.1 debe actualizar:

| Archivo | Cambio |
|---------|--------|
| `docs/2026-03-04-02-diseno.md` → `spec/language/02-grammar.md` | `bloqueado` → `forbidden` en la consolidación |
| `examples/cronometro-psp/trenza/*.trz` | Cualquier uso de `bloqueado` |
| `src/trenza/parser.py` | Si el parser reconoce `bloqueado` como keyword |
| `src/trenza/ast.py` | Constantes de keywords |

Los memos en `history/chronicle/` que mencionan `bloqueado` NO se tocan.

---

## Lista completa de ADRs propuestos (retroactivos)

Estos ADRs se extraen de las discusiones ya documentadas en los memos:

| ADR | Decisión | Fuente |
|-----|----------|--------|
| ADR-001 | Trenza es un DSL, no un framework | `concepto-inicial` |
| ADR-002 | Target: Rust + WASM | `diseno` |
| ADR-003 | Un archivo por contexto | `diseno` |
| ADR-004 | Renombramiento Helix → Trenza | `nombre-helix-vs-trenza` |
| ADR-005 | Keywords en inglés (`ignored`, `on_entry`) | `resolucion-gaps-fase1`, `resolucion-gap1-gap7` |
| ADR-006 | `effects:` en lugar de `lifecycle:` | `memo-opus-resolucion-gap1-gap7` |
| ADR-007 | `slot` + `fills` sobre DCI puro | `resolucion-gap4-definitiva` |
| ADR-008 | `forbidden` en lugar de `bloqueado` | Esta sesión (20 de marzo) |
| ADR-009 | Reestructuración del repositorio para v0.0.1 | Este documento |
| ADR-010 | Tres tipos de contexto explícitos en `system.trz` | `resolucion-gaps-fase1` |
| ADR-011 | Python para prototipo, Rust para herramienta final | `decisiones-pendientes-opus` |
| ADR-012 | Formato JSON simple para manifest (no OPC) | `decisiones-pendientes-opus` |
| ADR-013 | Herencia implícita en contextos (H1-H5) | `decisiones-pendientes-opus` |
| ADR-014 | Una acción por handler | `memo-opus-resolucion-gap2` |
| ADR-015 | `ErrorExterno` con ramas `.ok` / `.error` | `resolucion-gaps-fase2-opus` |
| ADR-016 | Guardas `when` pre-acción y post-resultado | `resolucion-gaps-fase2-opus` |

---

## Migración de archivos: mapa completo

### Archivos que se MUEVEN (rename/move, preserva historial git)

| Origen | Destino |
|--------|---------|
| `docs/2026-03-*.md` (42 archivos) | `history/chronicle/YYYY-MM-DD/NN-titulo.md` |
| `docs/historial_ias/` | `history/conversations/` |
| `docs/inspiration_sources/` | `history/inspirations/` |
| `docs/Directrices_PI_IA.md` | `history/meta/directrices-pi-ia.md` |
| `docs/backup_conversaciones.py` | `.agents/scripts/backup_conversaciones.py` |
| `docs/sistema/` | `docs/generated/sistema/` |
| `docs/2026-03-13-09-metafisica-de-trenza.md` | `history/meta/metafisica-de-trenza.md` |
| `examples/cronometro-psp/` | `spec/reference/cronometro-psp/` |

### Archivos que se CREAN (nuevos)

| Archivo | Contenido |
|---------|-----------|
| `CHANGELOG.md` | Historial de versiones |
| `LICENSE` | Licencia (a decidir por el Desarrollador Principal) |
| `spec/language/01-overview.md` | Consolidado desde concepto-inicial + README |
| `spec/language/02-grammar.md` | Consolidado desde diseno (actualizado) |
| `spec/language/03-verification.md` | Consolidado desde diseno + GAPs |
| `spec/language/04-package-format.md` | Consolidado desde diseno |
| `spec/language/05-cli.md` | Consolidado desde diseno |
| `docs/design/principles.md` | Consolidación de los 4 principios |
| `docs/design/dci-influences.md` | Consolidación de DCI |
| `docs/design/security-by-design.md` | Consolidación de seguridad |
| `docs/design/lifecycle.md` | Ciclo de vida (existente, reubicado) |
| `docs/manual/trenza-manual.md` | Manual de usuario (hoy 2026-03-20-04) |
| `history/README.md` | Explicación del archivo histórico |
| `history/decisions/ADR-001..016.md` | ADRs retroactivos |
| `history/inspirations/*.md` | Descomposición de arte-previo |
| `history/inspirations/README.md` | Tabla comparativa |

### Archivos que se ELIMINAN

| Archivo | Razón |
|---------|-------|
| `examples/cronometro-psp/helix/` | Reemplazado por `trenza/`; preservado en tag v0.0.0 |

### Archivos que se ACTUALIZAN in-place

| Archivo | Cambio |
|---------|--------|
| `README.md` | Reescrito para Trenza (no Helix) |
| `CLAUDE.md` | Actualizado con nueva estructura |
| `.gitignore` | Ajustado a nueva estructura |
| `src/trenza/*.py` | `bloqueado` → `forbidden` donde aplique |

---

## Secuencia de commits propuesta

```
1. chore: tag v0.0.0 — primera especificación completa
2. chore: create CHANGELOG.md and LICENSE
3. refactor: restructure docs/ → history/chronicle/ (preserve git history)
4. refactor: move examples/ → spec/reference/
5. refactor: rename .helix → .trz, helix/ → trenza/ in reference example
6. feat: create spec/language/ consolidated specification
7. feat: create docs/design/ consolidated design documents
8. feat: create history/decisions/ ADRs (retroactive)
9. feat: create history/inspirations/ expanded prior art
10. chore: update README.md, CLAUDE.md, .gitignore for new structure
11. fix: bloqueado → forbidden across all active documents
12. chore: tag v0.0.1 — restructured repository
```

Cada commit es reversible y tiene un propósito único.

---

## Lo que NO cambia

- **`src/trenza/`** permanece donde está. Es código, no documentación.
- **`.agents/workflows/`** permanece. Se le añade `scripts/`.
- **La convención de nombrado de memos** se preserva dentro de
  `history/chronicle/`. Solo se reorganiza por directorios de día.
- **El contenido de los memos no se modifica.** La historia es sagrada.

---

## Licencia: Dual AGPL-3.0 + Comercial

**Decisión tomada** (20 de marzo de 2026): el proyecto adopta licencia
dual. Ver `docs/2026-03-20-06-modelo-de-negocio.md` para la
justificación completa.

### Componente open-source: AGPL-3.0

El código fuente (parser, verificador, generador), la especificación
del lenguaje y la documentación se publican bajo **GNU Affero General
Public License v3.0**.

La AGPL extiende la GPL con la "cláusula de red": si una empresa
integra Trenza en un servicio accesible por red (un modelo de IA servido
por API, un SaaS), debe publicar el código completo de su integración.
Esto crea presión natural hacia la licencia comercial para proveedores
que no quieran abrir su integración.

### Componente comercial

Los proveedores de modelos de IA que quieran integrar Trenza en
productos propietarios sin obligación de publicar su código pueden
adquirir una licencia comercial. Incluye:

- Derecho a integrar sin AGPL.
- Acceso prioritario a nuevas versiones.
- Derecho al sello "Trenza Verified" (sujeto a certificación).

### Programa de Early Adopters

Descuento del 50% sobre tarifa estándar para proveedores que se
integren durante la fase de especificación. Anthropic y Google tienen
descuento preferente como proveedores de los modelos que co-diseñaron
el lenguaje.

### Archivo LICENSE

El archivo `LICENSE` en la raíz contendrá el texto completo de la
AGPL-3.0, con un encabezado que indique la disponibilidad de la
licencia comercial:

```
Trenza DSL
Copyright (c) 2026 [Titular]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, version 3.

For commercial licensing options (including integration into
proprietary AI models without AGPL obligations), contact:
[dirección de contacto]
```

---

## Política de idiomas

**Decisión tomada** (20 de marzo de 2026): el repositorio es bilingüe
por capas. El criterio es pragmático: cada capa usa el idioma que
maximiza su utilidad para su audiencia.

### Inglés (documentos orientados al exterior)

| Capa | Razón |
|------|-------|
| `README.md` | Carta de presentación pública; audiencia global |
| `CHANGELOG.md` | Estándar de la industria |
| `spec/language/` | La especificación debe ser legible por cualquier ingeniero o proveedor de IA |
| `docs/manual/` | El manual es la puerta de entrada; primero en inglés |
| `docs/design/` | Los documentos consolidados tienen audiencia internacional |
| `history/decisions/` (ADRs) | Son documentos de referencia técnica |
| `history/inspirations/` | Las comparaciones con XState, TLA+, etc. tienen audiencia global |
| `CLAUDE.md` | Los agentes IA operan mejor en inglés |
| Keywords del DSL | Ya decidido: `ignored`, `forbidden`, `when`, `slot`, `fills`... |

### Español (traza histórica auténtica)

| Capa | Razón |
|------|-------|
| `history/chronicle/` | Los memos se escribieron en español. La traza es auténtica o no sirve. **No se traducen.** |
| `history/meta/` | Las reflexiones filosóficas (`metafisica-de-trenza.md`, etc.) son inseparables de su idioma |

### El nombre del proyecto

**Trenza** se mantiene en español. Es identidad, no obstáculo. Se
explica en el README en una línea:

> *"Trenza" (Spanish for "braid") — from a single specification, three
> strands are woven: implementation, tests, and schematics.*

Como "Ubuntu" es zulú y nadie lo traduce. El nombre es memorable, no
tiene colisiones, y la metáfora de los tres cabos es universal.

### Impacto en la secuencia de commits

La transición al inglés en los documentos consolidados (`spec/`,
`docs/design/`, ADRs) se ejecuta durante la creación de esos documentos
(commits 6-9 de la secuencia propuesta). No es un paso adicional; es
parte del trabajo de consolidación.

Los documentos nuevos a partir de v0.0.1 se escriben en inglés por
defecto, salvo los memos de crónica (`history/chronicle/`) que pueden
seguir en el idioma de la sesión.

---

## Preguntas para Gemini 3.1 Pro

1. **¿Ves algún problema estructural en la organización propuesta?**
   Especialmente en la separación `spec/` vs `docs/` vs `history/`.
   ¿Hay redundancias o zonas grises que no hayamos visto?

2. **¿La política bilingüe es sostenible?** ¿O crees que a medio plazo
   deberíamos migrar toda la crónica a inglés para coherencia? (Nuestra
   posición: la autenticidad de la traza vale más que la uniformidad.)

3. **¿Los ADRs retroactivos cubren todas las decisiones significativas?**
   Revisa la lista de 16 ADRs propuestos y señala si falta alguna
   decisión que debería estar documentada.

4. **¿La licencia dual AGPL + Comercial es la correcta para el modelo
   de negocio descrito en `2026-03-20-06-modelo-de-negocio.md`?** ¿Hay
   riesgos legales o de posicionamiento que no hayamos considerado?

5. **¿Tienes sugerencias para la secuencia de commits?** ¿Cambiarías
   el orden o agruparías algún paso?
