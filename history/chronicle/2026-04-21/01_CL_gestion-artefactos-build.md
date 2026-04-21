# Crónica 2026-04-21 — Gestión de artefactos de compilación

**Author:** CL (Claude Sonnet 4.6 via Antigravity)
**Fecha:** 2026-04-21

---

## Resumen

Sesión de mantenimiento de infraestructura. Sin cambios de código fuente.

## Cambios realizados

### `FILES_FOR_AGENTS.md`
- Añadida sección **"Gestión de artefactos de compilación"** con el procedimiento
  post-build estándar para este proyecto.
- El procedimiento copia el ejecutable (`trenza-cli.exe`) a `bin/` y ejecuta
  `cargo clean`, pero **solo si la build tuvo éxito** (condicional `$LASTEXITCODE`).
- Usa sintaxis PowerShell 5.x compatible (`if ($LASTEXITCODE -eq 0)`), ya que
  `&&` no está disponible en esa versión.

### `.gitignore`
- Añadida entrada `bin/` para excluir el directorio de ejecutables locales.

## Decisiones documentadas

- `bin/` es un directorio **local** y **gitignoreado**. No se commitea nunca.
  Su único propósito es preservar el ejecutable compilado tras `cargo clean`,
  evitando que `target/` quede en disco y sature el canal IPC del IDE.
- Para distribución formal de binarios se usarán **GitHub Releases** cuando
  el proyecto esté maduro.

## Contexto: problema IPC de Antigravity

El IPC del agente se rompe tras el primer mensaje cuando Cimbra o Trenza-DSL
están abiertos como workspace en el IDE. La causa raíz es la saturación del
File Watcher por los >100.000 archivos de `target/`. El workaround definitivo
es no abrir estos proyectos como workspace — el agente opera sobre ellos vía
rutas absolutas sin necesidad de tenerlos abiertos. Esta decisión está
documentada en el KI `ki-indexing-policy` de Antigravity.
