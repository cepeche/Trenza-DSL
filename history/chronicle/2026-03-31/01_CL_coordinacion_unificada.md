# Crónica: Protocolo de Coordinación Unificado y FILES_FOR_AGENTS

**Fecha**: 2026-03-31
**Autor**: CL (Claude Sonnet 4.6 via Claude Code)

---

## Contexto

Sesión de revisión metodológica en el repo Cimbra, con impacto directo en Trenza-DSL.
El objetivo era unificar los protocolos de coordinación de ambos repositorios antes
de enviar el resultado a Gemini.

---

## Cambios realizados en este repo

### `AGENTS.md` — reescrito y unificado

El `AGENTS.md` de Trenza-DSL era la versión más madura de los dos repos. Se ha
reescrito para ser **idéntico** al de Cimbra, incorporando:

- Referencia explícita a `FILES_FOR_AGENTS.md` como primer paso de la Fase 0.
- Protocolo de Briefing (Relevo) con 4 elementos obligatorios.
- Patrón CO sin acceso a ficheros documentado explícitamente.
- Regla de archivos `untracked` ajenos (no añadir al índice si no los creaste tú).
- Validación obligatoria antes de push.
- "Quien ensucia, limpia" con especificidad.
- Sección de Resolución de Conflictos.

La única diferencia respecto al `AGENTS.md` de Cimbra es el ejemplo del semáforo
LOCK, que usa rutas de este proyecto (`trenza-cli/src/generator.rs`, `editors/vscode/`).

### `FILES_FOR_AGENTS.md` — nuevo

Creado para separar el protocolo genérico (AGENTS.md) del mapa de proyecto específico.
Contiene:
- Jerarquía de autoridad con rutas concretas de este repo.
- Descripción de todos los directorios clave (crates Rust, compilador Python, spec, docs).
- Tabla de archivos generados (no editar manualmente).
- Archivos de instrucción y sus propietarios.
- Comandos de build y test.
- Tabla de strands con extensiones de salida.
- Documentos de diseño relevantes (incluyendo `strand5-gql-property-graph.md`).
- Referencia cruzada a `C:/Proyectos/Cimbra/`.

---

## Contexto de la decisión

La separación AGENTS.md / FILES_FOR_AGENTS.md emergió de la observación de que los
ADRs de Cimbra no existían como directorio, aunque las decisiones sí estaban
documentadas en crónicas y memoria de sesión. El directorio `history/decisions/`
de Cimbra se creó en esta sesión con 12 ADRs formalizados.

Esto abre una pregunta relevante para Trenza-DSL: ¿debería el compilador generar
*esqueletos* de ADR identificando decisiones implícitas en el `.trz`? Ver el briefing
para Opus en la crónica de Cimbra del mismo día.

---

## Estado de artefactos al cierre

| Artefacto | Estado |
|-----------|--------|
| `AGENTS.md` | Actualizado ✓ |
| `FILES_FOR_AGENTS.md` | Creado ✓ |
