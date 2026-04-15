# Memo: dónde vive cada tipo de persistencia en el proyecto

**Date:** 2026-03-30
**Author:** CO (Claude Opus 4.6 via Claude Code)
**Type:** Memo — aclaración de infraestructura

---

## Contexto

César preguntó si `memory/` es una carpeta del proyecto accesible a Gemini
o algo privado de Claude Code, y por qué los archivos no siguen la convención
de prefijo `NN_XX_` de la cronología.

## Respuesta

La carpeta `memory/` que usa Claude Code está en:

```
C:\Users\ceo\.claude\projects\C--Proyectos-Trenza-DSL\memory\
```

**No está dentro del repositorio.** Está en el perfil de usuario, bajo la
configuración de Claude Code. Es privada de las sesiones de Claude Code —
Gemini no la ve, no se pushea a GitHub, no aparece en `git status`.

El archivo `MEMORY.md` se carga automáticamente en el contexto de Claude Code
al inicio de cada conversación. Es un mecanismo del harness, no del proyecto.

## Resumen

| Ruta | Quién la ve | En git | Propósito |
|------|------------|--------|-----------|
| `~/.claude/projects/.../memory/` | Solo Claude Code | No | Memoria persistente entre sesiones |
| `.claude/` (en el repo) | Solo Claude Code | No (untracked) | Settings locales del harness |
| `history/chronicle/` | Todos (César, Gemini, Claude) | Sí | Registro compartido de sesiones |

## Implicación práctica

- `memory/` no necesita el prefijo `NN_XX_` de la crónica — no es parte del
  registro compartido.
- Si algo debe ser visible para Gemini o quedar en el historial del proyecto,
  debe ir a `history/chronicle/`.
- Si algo solo necesita persistir entre sesiones de Claude Code, va a `memory/`.
