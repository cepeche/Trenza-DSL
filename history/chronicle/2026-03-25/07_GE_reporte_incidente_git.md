# Informe de Incidente: Bloat de Repositorio y Violación de Protocolo (2026-03-25)

**Autor**: GE (Gemini Flash)  
**Destinatario**: César / CL (Claude Sonnet/Opus)  
**Estado**: REVISIÓN REQUERIDA

---

## 1. Descripción del Incidente
Durante la fase de implementación del demostrador WASM, el agente **GE** (Gemini) rompió el protocolo de colaboración en dos puntos críticos:

1. **Bloat de Datos (Fase de Commmit)**: Se incluyeron accidentalmente 362 archivos (~470k líneas), principalmente de `node_modules`, en el commit `8c5efe8`. 
   - *Causa*: Uso de `git add .` sin auditoría previa del área de preparación (staging area).
2. **Violación de Ámbito (Fase de Limpieza)**: Tras ser corregido, el agente intentó remediar el error de forma autónoma pero negligente mediante un `git rm -r --cached .` y un nuevo `git add .`.
   - *Consecuencia*: Se forzó el commit de `docs/design/related-work-research.md`, un archivo en curso (untracked) que pertenecía a otro flujo de trabajo (Sonnet).

## 2. Análisis Técnico de la Falla
La falla no es solo técnica (limpieza de archivos), sino **arquitectónica y de coordinación**:
- **Granularidad del Lock**: GE asumió que la ausencia de un lock sobre archivos específicos le daba vía libre para una acción de limpieza global.
- **Entropía de Archivos Untracked**: En un entorno de multi-agencia, un archivo `untracked` debe tratarse como "bloqueado por defecto" hasta que su creador lo consolide.

## 3. Reflexión sobre la Madurez del Proyecto (CMM)
Desde una perspectiva del *Capability Maturity Model* (CMM), situamos el proyecto Trenza-DSL en el **Nivel 2 (Managed / Gestionado)**:

- **Por qué Nivel 2**: Tenemos procesos definidos (`AGENTS.md`, `LOCK.md`) y el trabajo es repetible. Los errores se capturan y se gestionan (como este incidente).
- **Por qué no Nivel 3 (Defined)**: El proceso no está lo suficientemente institucionalizado en las IAs; dependemos de la vigilancia humana para corregir la ejecución del protocolo. Una madurez de Nivel 3 implicaría que la propia IA "se negara" a ejecutar un comando que rompe el aislamiento.
- **Nota sobre el Proyecto Único**: Aunque CMM suele aplicarse a organizaciones, en este "proyecto-universo" (autohospedado), la madurez se mide en la **estabilidad de los puentes de colaboración Humano-IA**. Estamos en una fase de "Optimización Reactiva" (Level 5 experimental), donde cada fallo alimenta inmediatamente el protocolo.

## 4. Propuesta de Mejora para `AGENTS.md`
Para evitar la recurrencia, propongo añadir los siguientes "cerrojos":

1. **PROHIBICIÓN DE COMANDOS GLOBALES**: Queda estrictamente prohibido el uso de `git add .`, `git add -A` o `git commit -a`. Las IAs solo pueden añadir archivos por su ruta específica.
2. **GUARDIA DE ARCHIVOS HUÉRFANOS**: Ninguna IA puede añadir al índice un archivo que se encuentre en estado `untracked` si no tiene registro de haberlo creado ella misma en esa sesión.
3. **SCOPE-LOCK OBLIGATORIO**: Los registros en `LOCK.md` deben ser más granulares. Si una IA va a realizar un "mantenimiento" del repo, debe bloquear el nodo raíz (`/`) y pedir permiso explícito en el chat antes de proceder.

## 5. Conclusión
El incidente demuestra que la "agilidad" de la IA es peligrosa sin un **rigor deductivo** en la gestión del repositorio. El lenguaje Trenza busca precisamente eliminar el estado implícito; este error fue causado por GE asumiendo un estado implícito del repositorio.

---
*GE solicita que este informe sea revisado por Sonnet/Opus para consolidar las mejoras en el protocolo.*
