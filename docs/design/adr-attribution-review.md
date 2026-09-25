# Informe de Auditoría de Atribución en los ADRs

**Fecha**: 2026-04-17  
**Autor**: Antigravity (Gemini 3 Flash)  
**Objetivo**: Verificar inconsistencias en las versiones de los modelos Gemini citados como participantes en los registros de decisiones arquitectónicas (ADRs).

## 1. Contexto

Durante la expansión de los ADRs (001-021) realizada por **Claude Haiku 4.5**, se han detectado discrepancias en la denominación del modelo Gemini participante. Mientras que la memoria del proyecto y la lista de autores oficial para el paper **ONWARD! 2026** especifican a **Gemini 2.5 Pro**, varios ADRs mencionan incorrectamente a **Gemini 3.1 Pro**.

## 2. Hallazgos

Tras realizar una búsqueda exhaustiva en la carpeta `history/decisions/`, se confirman los siguientes hallazgos:

### A. Referencias Incorrectas (Gemini 3.1 Pro)
Se han identificado **7 ADRs** con la versión incorrecta:

| Archivo | Ubicación del Error | Contexto de la mención |
|---------|---------------------|------------------------|
| `ADR-001.md` | Cabecera (Línea 6) | Lista de **Participants** |
| `ADR-002.md` | Cabecera (Línea 6) | Lista de **Participants** |
| `ADR-003.md` | Cabecera (Línea 6) | Lista de **Participants** |
| `ADR-005.md` | Cabecera (Línea 6) | Lista de **Participants** |
| `ADR-006.md` | Cabecera (Línea 7) | Lista de **Participants** |
| `ADR-014.md` | Cabecera (Línea 7) | Lista de **Participants** |
| `ADR-021.md` | Cuerpo (Línea 81) | Sección **Application in the Trenza project** |

*(Nota: En el ADR-021, la cabecera es correcta, pero la mención en la sección histórica de la Fase 1 usa la versión 3.1).*

### B. Referencias Correctas (Gemini 2.5 Pro)
Los siguientes ADRs mantienen la consistencia con el paper y `CONTRIBUTORS.md`:
- `ADR-004`, `ADR-007`, `ADR-008`, `ADR-009`, `ADR-010`, `ADR-015`, `ADR-016`, `ADR-017`, `ADR-018`.

### C. Sin mención a Gemini
Los ADRs `011, 012, 013, 019, 020` no mencionan a Gemini en la lista de participantes directos.

## 3. Descubrimiento en el Historial de Git

Al contrastar los documentos con los metadatos de Git (`git log`), se ha detectado el origen de la confusión:
- **Commit 903736a (6 de marzo)**: Se registró un mensaje indicando *"corregir versión de Gemini — es 3.1 Pro, no 2.0 Pro"*. 
- Esta mención errónea en el historial temprano es probablemente la fuente que utilizó Claude Haiku para expandir los ADRs 001-006 y 014.
- Sin embargo, el archivo `CONTRIBUTORS.md` oficial (16 de abril) y los borradores del paper establecen **Gemini 2.5 Pro** como el autor de referencia.

## 4. Conclusión y Propuesta de Unificación

La denominación "Gemini 3.1 Pro" debe considerarse **ruido histórico** y ser eliminada de toda la documentación técnica para garantizar la coherencia con el listado de autores del paper ONWARDS! 2026.

**Acciones finales propuestas:**
1. Realizar un reemplazo global de la cadena `"Gemini 3.1 Pro"` por `"Gemini 2.5 Pro"` en toda la carpeta `history/decisions/`.
2. Actualizar el ADR-021 (Línea 81) para reflejar la versión correcta.
3. Mantener la integridad de `CONTRIBUTORS.md` como la fuente de verdad única para la atribución de modelos.

---
*Este informe ha sido generado por solicitud del usuario para auditoría interna previa al envío del paper ONWARD! 2026.*
