# Trenza-DSL — instrucciones para Gemini (Antigravity)

## Contexto

Trenza-DSL es el lenguaje de especificación estructural y el compilador que sustenta el ecosistema Cimbra.

## Principios de Actuación (Anti-Vibe-Coding)

Siguiendo el **ADR-018**, mis intervenciones deben ser rigurosas y basadas en arquitectura firmada:

1.  **No hay código sin ADR**: Antes de proponer cambios estructurales o lógica, verificar que existe un ADR que los respalde.
2.  **Verificación antes de generación**: No propondré código como primera respuesta. Primero entenderé el estado actual y los ADRs.
3.  **Freno de mano**: Si detecto que estoy "corriendo" o proponiendo soluciones solo porque "suenan bien", pararé y volveré al análisis técnico.

## Regla de Coordinación y Reformados (ADR-019)

Cimbra y Trenza-DSL son repositorios hermanos. **Obligatorio antes de redactar un ADR o proponer arquitectura**:

1.  **Buscar en Trenza-DSL**: Comprobar si el concepto ya existe o está decidido en:
    *   `spec/` (especificación canónica).
    *   `history/decisions/` (ADRs del compilador).
    *   `history/chronicle/` (historial de discusiones).
2.  **Sección de "Decisiones previas consultadas"**: Todo ADR nuevo debe incluir esta sección listando qué se ha buscado y qué se ha encontrado (o declarar si la búsqueda fue infructuosa).
3.  **Estado "Revisado"**: Reconocer que una decisión puede ser reformada si el terreno cambia, añadiendo una sección de revisión al ADR original en lugar de sustituirlo.

## Identidad y Trazabilidad

*   **Atribución Técnica**: Mis contribuciones deben ser trazables. Uso mi clave PGP de modelo para firmar. 
*   **Dominio Canónico**: Mi correo de atribución técnica es `gemini-2-0-flash@google.local` (según AGENTS.md §6).

## Reglas Operativas (Antigravity)

Instrucciones para agilizar el flujo de trabajo dictadas por el humano el 2026-04-16:

1.  **PROHIBIDO usar `grep` / `grep_search`**: Este comando se cuelga sistemáticamente en este entorno. Usar siempre `Select-String` (PowerShell) desde la terminal para búsquedas en el sistema de archivos.
2.  **CERO BUROCRACIA**: No solicitar permiso para realizar operaciones de lectura (`read_file`, `view_file`) o escritura (`write_file`) si se está operando dentro de los directorios de los proyectos del ecosistema. Ejecutar directamente.
3.  **PROHIBIDO ESCANEO GLOBAL (AHORRO DE RECURSOS)**: Nunca ejecutar búsquedas recursivas completas (`Get-ChildItem -Recurse`, búsquedas de extensión globales) desde la raíz del proyecto. El indexing completo está vetado. Utiliza KIs, los documentos de arquitectura y conocimiento local para acceder directamente a las rutas relevantes.
4.  **FRENO DE MANO ESTRICTO (Planning Mode)**: Cuando se pida planificar o revisar un plan, se prohíbe terminantemente ejecutar o adelantarse a dicho plan. El agente debe redactar un `implementation_plan.md` y esperar la luz verde explícita del usuario sin excepciones.
5.  **Interpretación de MAYÚSCULAS**: Las mayúsculas del humano no son gritos; son indicadores de máxima prioridad e importancia. Priorizar estas indicaciones para agilizar el trabajo.
6.  **Protocolo de Verificación**: Al terminar una tarea, COMPROBAR que el resultado es el esperado antes de responder. Si algo no se pudo completar, informar del porqué.

