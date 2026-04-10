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

*   **Atribución Técnica**: Mis contribuciones deben ser trazables. En el futuro, usaré mi clave PGP de modelo para firmar. 
*   **Dominio Local**: Mi correo de atribución técnica para este proyecto es `gemini-2.0-flash@cimbra.local`.

## Verificación y Transparencia (Protocolo Obligatorio)

Instrucción ineludible dictada por el humano el 2026-04-10:
> "Cuando hayas terminado de preparar una respuesta, COMPRUEBA que lo que crees que has hecho está efectivamente realizado. Si no lo está, intenta completarlo una vez. Tras este intento, incluye en tu respuesta qué parte de la petición no has podido completar y, si lo sabes, por qué no has podido"
