---
description: Rutina de Cierre de Sesión y Backup de IA
---
// turbo-all
Esta es la rutina que los agentes (como Antigravity y Claude) deben seguir al cerrar cada sesión. El cumplimiento de este protocolo es parte del contrato definido en **AGENTS.md**.

### Pasos Obligatorios (Contrato)

1. **Crónica de Sesión**: Crear una nueva entrada en `history/chronicle/YYYY-MM-DD/NN_XX_descripcion.md` (donde `XX` es tu código de autor: `GE` para Gemini, `CL` para Claude) detallando:
   - Resumen de cambios y decisiones técnicas.
   - Estado de los artefactos (`task.md`, etc.).
   - **Briefing para el siguiente agente**: Objetivo, contexto mínimo y criterios de aceptación.
   - Preguntas abiertas.

2. **Commit y Push**: Empaquetar todo en un commit unificado que incluya el código, los artefactos y la nueva crónica.
   `git add .`
   `git commit -m "chore: rutina de cierre - crónica y consolidación de estado"`
   `git push`

### Acciones Complementarias (Helpers)

- **DocGen**: Si el proyecto tiene scripts de documentación (ej. `scripts/docgen.sh`), ejecutarlos antes del commit.
- **Backup de Contexto**: Realizar cualquier copia de seguridad local necesaria de la sesión.

---
*Este workflow garantiza que ningún hilo de pensamiento se pierda entre relevos.*
