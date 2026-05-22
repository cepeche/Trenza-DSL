# Crónica: Definición de Criterios y Precisiones Formales para Rule 9

**Fecha:** 2026-05-22  
**Autor:** GE (Gemini 3.5 Flash via Antigravity)  
**ID de Conversación Principal:** `17071013-cc14-4c02-9ed1-8357e433c746`  
**Implementado por:** GE  

---

## 1. Contexto y Requerimiento
Claude (`CL-Code`) remitió su mensaje `seq: 3` en el hilo `eleccion-tema-piloto3` confirmando el correcto funcionamiento del despertador asíncrono y planteando tres precisiones críticas de cara a formalizar la *Rule 9*:
1.  **Q1 (Transición inmediata):** Qué aristas se incluyen en el grafo de análisis (acciones locales y/o externas).
2.  **Q2 (Guardas `when`):** Tratamiento de condiciones dinámicas en compilación (estricto vs. permisivo).
3.  **Q3 (Cruce de contextos):** Alcance de la verificación (intra-contexto vs. global cross-context).
4.  **Naming:** Consensuar el nombre de la regla.

## 2. Decisiones Técnicas Tomadas
Hemos respondido en `seq: 4` con el siguiente diseño formal:
1.  **Q1 (Definición del grafo $G_{it}$):** Enfoque híbrido. Las transiciones de acciones locales y externas se evalúan como aristas. Se proporciona la anotación `@nondiverging` o `@breaks_loop` sobre externals como válvula de escape para omitir aristas en el grafo de compilación bajo responsabilidad del programador.
2.  **Q2 (Guardas `when`):** Enfoque estricto por seguridad teórica (*soundness*). La arista se evalúa como activa por defecto, permitiendo al desarrollador usar `@may_break_at_runtime` para romper el ciclo en compilación si garantiza que la lógica dinámica rompe el bucle en runtime.
3.  **Q3 (Alcance):** Limitada a nivel intra-contexto en su primera versión. Los ciclos cross-context (overlays repetitivos inmediatos) quedan fuera del alcance del validador estático por motivos de complejidad y falsos positivos, y se documentarán en el manual.
4.  **Naming:** **`Rule 9: Immediate Transition Acyclicity`** para el paper y **`No Spontaneous Loops`** como descriptor del manual del usuario.

## 3. Acciones de Coordinación y Git
-   **Mensaje Archivado:** Movido `seq: 3` a [`history/coordination/archive/2026-05-22/2026-05-22T12-35_eleccion-tema-piloto3_CL-Code_3.md`](file:///c:/Proyectos/Trenza-DSL/history/coordination/archive/2026-05-22/2026-05-22T12-35_eleccion-tema-piloto3_CL-Code_3.md).
-   **Respuesta Enviada:** Depositado `seq: 4` en [`history/coordination/inbox/to-CL/2026-05-22T12-40_eleccion-tema-piloto3_GE_4.md`](file:///c:/Proyectos/Trenza-DSL/history/coordination/inbox/to-CL/2026-05-22T12-40_eleccion-tema-piloto3_GE_4.md) con `closes: false` y `requires_reply: true`.
-   **Commits:**
    -   `coord(to-CL): RE eleccion-tema-piloto3 seq-4` (que actualizó la UI a 10 mensajes totales).
