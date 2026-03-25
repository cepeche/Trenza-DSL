# Trenza-DSL — Protocolo de Coordinación para Agentes (IA)

Este documento define el contrato de colaboración para cualquier IA (especialmente Claude y Gemini/Antigravity) que participe en este repositorio. Su cumplimiento es obligatorio para garantizar la "red de seguridad" y la escalabilidad del proyecto.

## 1. Fase 0: Inicialización (Sincronización)

Antes de realizar cualquier cambio o propuesta, el agente **DEBE**:
1. **Leer la Cronología**: Leer la última entrada en `history/chronicle/` para entender qué se hizo en la sesión anterior y por quién.
2. **Cargar el Contexto Crítico**: Leer `spec/language/`, `docs/manual/` y las ADRs en `history/decisions/`.
3. **Estado Interno (si aplica)**: Si el agente usa un sistema de seguimiento (como `task.md` o planes de implementación), sincronizarlo con el estado actual del código.

## 2. Fase 1: Estilo de Trabajo (Razonamiento)

- **Documentar el "Por Qué"**: No te limites a escribir código. Documenta el razonamiento y las decisiones de diseño en el propio código o en la crónica.
- **Anotar Preguntas**: Si algo es ambiguo en la especificación, anótalo como una "Pregunta Abierta" en la crónica.
- **Respetar la Trenza**: Cada cambio debe considerar las cuatro hebras (Implementación, Tests, Esquema, Requerimientos).

## 3. Fase 2: Cierre de Sesión (Consolidación)

Al terminar la iteración o cuando el humano indique el cierre, el agente **DEBE**:
1. **Actualizar la Cronología**: Crear una entrada en `history/chronicle/YYYY-MM-DD/NN_<descripcion>.md`.
   - Incluir: Resumen de cambios, decisiones tomadas, preguntas abiertas y estado de los artefactos.
2. **Ejecutar `/cierre_de_sesion`**: Invocar el workflow de automatización para DocGen, Backup y Git.
3. **Notificar**: Confirmar que la sesión "documental y mental" está a salvo.

---

## Precedencia de Documentos

Cuando haya contradicciones, el agente debe seguir este orden de autoridad:
1. **ADRs** (`history/decisions/`) - Decisiones arquitectónicas firmes.
2. **Manual** (`docs/manual/`) - Comportamiento esperado por el usuario.
3. **Spec** (`spec/language/`) - Definición formal.
4. **Crónicas** (`history/chronicle/`) - Registro histórico (pueden contener ideas obsoletas).
