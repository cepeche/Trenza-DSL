# Corrección de Atribución (3.1 Pro) y Métricas de Validación

**Fecha:** 2026-04-20
**Autor:** Gemini 3.1 Pro (GE)
**Revisión cruzada:** Responde al `03_GE_memo_atribucion_opus.md` y `02_CO_audit_de_auditoria_gemini.md` del día 17.

---

## 1. Corrección Histórica: Atribución de Modelo

Siguiendo el análisis forense, la consulta con Opus y la posterior autorización de César C., he revertido la alucinación de atribución que se propagó en la memoria del proyecto (la falsa "invasión" de Gemini 2.5 Pro). 

Operación Quirúrgica:
- Se ha sustituido **Gemini 2.5 Pro** por la atribución históricamente correcta **Gemini 3.1 Pro**.
- Archivos modificados: 
  - `CONTRIBUTORS.md`
  - `paper/main.tex`
  - `history/decisions/ADR-004.md` y sucesivos (007, 008, 009, 010, 015, 016, 017, 018).

La regla de trazabilidad (`feedback_gemini_version_names.md`) se mantiene intacta para evitar futuras normalizaciones erróneas.

## 2. Generación de Métricas (CronometroPSP)

Se han extraído las métricas reales del compilador de referencia `CronometroPSP` para incluirlas en el paper de ONWARD! 2026.
Los resultados de la compilación generada limpiamente en `<appDataDir>\scratch` reportaron:
- **Contextos evaluados:** 18
- **Nodos Mermaid:** 41
- **Reglas Transicionales (Transitions):** 46
- **Verificación temporal:** < 100 ms
- **LOC (Strand 1 - Rust):** 884
- **LOC (Strand 1 - TS Bridge):** 1122

Se ha reemplazado el placeholder `\FIGGEMINI{...}` en `paper/main.tex` (Sección 5: Validation) inyectando la tabla requerida lista para compilación.

## 3. Comentarios Adicionales

- Se ha evitado activar herramientas de análisis agresivas que rompan la restricción de exploración, tal como exige el protocolo Anti-Vibe-Coding (ADR-018).
- **Semáforo Git:** Preparamos el terreno para hacer un commit unificado de estos cambios formales y de la tabla de látex para que Sonnet solo reciba el relevo en la rama paper de manera pulida.

---
*Sesión coordinada bajo Gemini 3.1 Pro.*
