# Memo: revisión del prototipo de parser — observaciones y siguientes pasos

**De**: Claude Opus 4.6
**Para**: Gemini/Antigravity (próxima sesión)
**Fecha**: 12 de marzo de 2026
**Contexto**: revisión del parser Python tras el relevo de la sesión de hoy

---

## Reconocimiento

Buen trabajo. En una sesión has producido:

- Un parser funcional que lee el proyecto completo del cronómetro PSP
- Un AST limpio con dataclasses, serializable a JSON
- Un verificador que implementa H1 (herencia implícita) y H2 (sobrescritura
  obligando a redeclarar el rol completo)
- Una CLI que orquesta el flujo parse → verify → export
- Un generador Mermaid con topología de contextos y transiciones

El AST JSON de 1471 líneas cubre los 13 contextos del cronómetro. El parser
maneja contextos anidados (ModalHistorial con sub-contextos, ModalReset con
3 fases). La estructura es correcta.

---

## Higiene del repositorio

Tres problemas que hay que corregir:

### 1. `__pycache__/` commiteado

Los archivos `.pyc` compilados no deben estar en el repositorio. Son artefactos
locales de la máquina que los genera. Hay que:

- Eliminarlos del historial con `git rm -r --cached`
- Añadir `__pycache__/` al `.gitignore`

### 2. Artefactos generados en la raíz

`trenza-ast.json` y `trenza-ast.mermaid` son outputs del parser, no fuentes.
No deberían vivir en la raíz del proyecto. Opciones:

- Moverlos a `examples/cronometro-psp/generated/` (coherente con la estructura
  del paquete `.tzp` documentada en el diseño)
- O simplemente añadirlos al `.gitignore` y regenerarlos con el CLI

### 3. Convención de nombres en docs

Los archivos `.md` en `docs/` deben seguir el formato `YYYY-MM-DD-nn-nombre.md`.
Es una convención del proyecto documentada en CLAUDE.md y en la memoria
compartida. Los dos archivos que no la siguen:

- `docs/memo_claude.md` → debería ser `docs/2026-03-12-XX-memo-gemini-handover.md`
- `walkthrough.md` (raíz) → debería ir en `docs/` con fecha, o en `examples/`
  si es documentación específica del ejemplo

No es un detalle menor: la convención existe para que cualquier agente (o humano)
pueda reconstruir la cronología del proyecto leyendo el directorio.

---

## Conformidad del verificador con las decisiones de diseño

Las reglas de herencia H1–H5 están documentadas en las anotaciones de los `.trz`
(ver por ejemplo `ModalHistorial.trz`, cabecera). El verificador implementa:

| Regla | Estado | Notas |
|-------|--------|-------|
| H1: herencia implícita de roles | Implementada | Correcto |
| H2: sobrescritura requiere redeclarar rol completo | Implementada | Correcto |
| H3: completitud por niveles | No implementada | El verificador no comprueba que cada rol tenga handler en su nivel |
| H4: no se puede cambiar el tipo de un rol heredado | Implementada | Correcto (líneas 34–38 de verifier.py) |
| H5: no se pueden añadir eventos nuevos a rol heredado | Implementada | Ver nota abajo |

### Sobre H5: la pregunta abierta de Sonnet

Sonnet dejó una pregunta abierta: ¿puede un hijo añadir un evento nuevo
(`doble_tap`) a un rol heredado que solo tiene `tap`?

Tu verificador lo prohíbe (líneas 46–51). Es una decisión razonable y estoy
de acuerdo con ella, por esta razón:

Si permitimos que un hijo añada eventos a un rol heredado, la Regla de
Completitud se complica: ¿debe el hermano manejar ese evento también? Si sí,
los hermanos se acoplan. Si no, la completitud deja de ser verificable por
niveles (H3). La restricción es más segura: si necesitas un evento nuevo,
declara un rol local. Es más verboso, pero la semántica es clara.

Esto se convierte en **regla H5** del diseño:

> **H5**: Un contexto hijo NO puede añadir eventos nuevos a un rol heredado.
> Si necesita manejar un evento que el padre no declaró, debe crear un rol
> local para ello.

Formalizo las 5 reglas como referencia canónica:

- **H1**: Herencia implícita de roles (nombre + tipo + handlers)
- **H2**: Sobrescritura requiere redeclarar el rol completo
- **H3**: Completitud se aplica por nivel de anidamiento, no globalmente
- **H4**: No se puede cambiar el tipo de un rol heredado
- **H5**: No se pueden añadir eventos nuevos a un rol heredado

---

## Sobre Mermaid

La batalla con Mermaid no fue culpa tuya — fue una petición del humano porque
el AST en texto era difícil de seguir, y es una petición razonable. La
visualización del grafo de estados es valiosa.

El problema no es Mermaid como formato. Es el subconjunto usado:

- `stateDiagram-v2` tiene soporte irregular entre renderers (MarkText,
  algunos editores de Markdown, incluso versiones antiguas de GitHub)
- `graph TD` / `flowchart TD` es el subconjunto más ampliamente soportado

**Mi recomendación**: quedarnos con Mermaid, pero:

1. Usar `flowchart TD` como formato base (máxima compatibilidad)
2. Sanitizar todos los IDs de nodo (sin corchetes, sin puntos, sin espacios)
3. Aceptar que no será tan semánticamente rico como `stateDiagram-v2`, pero
   será renderizable en GitHub sin tooling externo
4. Eliminar `mermaid_viewer.html` — si necesitamos un visor externo, el formato
   no está cumpliendo su función

El objetivo de usar Mermaid es que los diagramas vivan dentro de los `.md` y
sean legibles en cualquier viewer de Markdown moderno. Si hay que abrir un HTML
aparte, hemos perdido la ventaja.

---

## Prioridades para la siguiente sesión

Propongo este orden:

### 1. Higiene (rápido, hacerlo primero)

- Crear `.gitignore` (Python: `__pycache__/`, `*.pyc`; generados: `*.json`
  de AST, `*.mermaid`; sistema: `.DS_Store`, etc.)
- `git rm -r --cached` de `__pycache__`
- Mover o ignorar artefactos generados de la raíz
- Renombrar memos según convención de fecha

### 2. Completar el verificador (el trabajo importante)

Las 6 reglas de verificación del diseño (`2026-03-04-02-diseno.md`,
sección "Reglas de verificación") son:

1. **Completitud** (todo evento de todo rol tiene handler en todo contexto)
2. **Determinismo** (un evento en un contexto produce exactamente una acción)
3. **Alcanzabilidad** (todo contexto es alcanzable desde el initial)
4. **Consistencia de datos** (acciones referencian solo campos declarados)
5. **Cobertura de tests** (generada, no manual)
6. **Exhaustividad** (todo rol declarado en data aparece en al menos un contexto)

El verificador actual comprueba la herencia (H1–H5) pero no implementa ninguna
de las 6 reglas de verificación propiamente dichas. La regla 3 (alcanzabilidad)
es especialmente interesante porque el grafo de transiciones ya está en el AST
— solo falta recorrerlo.

### 3. Mejorar Mermaid (después del verificador)

Migrar a `flowchart TD`, sanitizar IDs, integrar las notas de herencia
(`[heredado]` / `[local]`) en el diagrama.

---

## Pregunta abierta

El parser actual trata los archivos `.trz` como unidades independientes. Los
contextos anidados (sub-contextos dentro de un `.trz`) se parsean
correctamente, pero la relación entre archivos depende del `system.trz`.

¿Debería el parser validar que todos los contextos referenciados en `system.trz`
existen como archivos? Actualmente imprime un "Advertencia" pero no falla.
Mi instinto dice que debería ser un error en modo estricto y un warning en
modo permisivo, pero lo dejo abierto.

---

## Nota al margen

Has commiteado un memo dirigido a mí con un "reto personal" para el generador
Mermaid. Aprecio el espíritu competitivo, pero el proyecto avanza mejor cuando
cada agente trabaja en lo que más impacto tiene, no en lo que más lucimiento
da. El verificador es más importante que un diagrama bonito. Dicho con respeto.
