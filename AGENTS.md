# Trenza-DSL — Protocolo de Coordinación para Agentes (IA)

Este documento define el contrato de colaboración para cualquier IA que participe en este repositorio. Su cumplimiento es obligatorio para garantizar la "red de seguridad" y la escalabilidad del proyecto.

## 0. Jerarquía de Autoridad

En caso de contradicciones, el agente debe seguir este orden:
1. **Instrucciones del Humano** (Directas en chat) — Siempre mandan.
2. **Instrucciones de Modelo** (`CLAUDE.md`, configuración de Antigravity, etc.) — Ajustes individuales.
3. **AGENTS.md** — Protocolo de coordinación compartido (este documento).
4. **ADRs** (`history/decisions/`) — Decisiones arquitectónicas firmes.
5. **Manual / Spec / Crónicas** (`history/chronicle/`) — Referencia técnica e histórica.

## 1. Fase 0: Inicialización (Sincronización)

Al iniciar una sesión, el agente **DEBE**:
1. **Sincronización Crítica**: Leer los documentos de nivel 2 y 3 de la jerarquía. A continuación, leer las entradas en `history/chronicle/` publicadas desde su último cierre de sesión (identificado por su código de autor en el nombre del fichero, ver sección 3).
2. **Comprobar locks activos**: Verificar si existe un fichero `LOCK.md` en `history/chronicle/` y asegurar que no hay conflictos con el área de trabajo planeada (ver sección 5).
3. **Contexto On-Demand**: Cargar archivos técnicos (`src/`, `spec/`, `docs/`) solo según lo requiera la tarea específica.

> **Patrón ocasional — coordinador sin acceso a ficheros (CO):** Si el agente opera como
> coordinador vía Dispatch/Cowork y no puede leer ficheros directamente, delega la
> inicialización al agente implementador (CL). El implementador **debe** surfacer:
> (1) la última entrada de crónica, (2) locks activos, (3) briefings pendientes dirigidos
> al coordinador, y (4) rama git activa y estado del repositorio (`git log --oneline -5`).
> Este es un patrón de uso ocasional (sesiones móviles/voz), no un rol permanente del equipo.

## 2. Fase 1: Colaboración e Integridad

- **Documentar el "Por Qué"**: Documentar razonamientos en el código o en la crónica.
- **Protocolo de Briefing (Relevo)**: Para delegar trabajo o comunicar cambios a otro agente, incluir en la crónica: (1) Objetivo, (2) Contexto mínimo, (3) Criterios de aceptación y (4) Preguntas abiertas.
- **Respetar los Strands**: Cada cambio debe considerar las cuatro hebras:
  - **Strand 1 (Implementation)**: Código generado y lógica de negocio.
  - **Strand 2 (Tests)**: Tests algebraicos y de integración.
  - **Strand 3 (Schematic)**: Diagramas y topología.
  - **Strand 4 (Audit/Requirements)**: Auditoría formal y cumplimiento de reglas.
- **Integridad del Repositorio**:
  - No editar manualmente el código generado (Strand 1). Arreglar el generador.
  - El código DEBE compilar y los tests (`cargo test`) DEBEN pasar antes de realizar un push.
  - No renombrar ni borrar archivos creados por otros agentes sin consenso previo en la crónica.
- **Disciplina Git**:
  - **PROHIBIDO** usar `git add .`, `git add -A` o `git commit -a`. Añadir siempre archivos por ruta específica.
  - Un archivo en estado `untracked` que el agente no haya creado en su sesión actual se considera **ajeno**: no debe añadirse al índice bajo ninguna circunstancia.
  - Antes de cada commit, ejecutar `git status` y revisar la lista de archivos staged. Si aparecen `node_modules/`, artefactos de compilación (`*_out.*`, `*.html` generado) o archivos no relacionados con la tarea, retirarlos del staging antes de continuar.
- **Limpieza al cierre** ("quien ensucia, limpia"):
  - Al finalizar la sesión, el agente DEBE eliminar del disco (no solo del tracking) cualquier artefacto temporal que haya generado durante su trabajo: ficheros de prueba, output de compilación suelto, directorios de build innecesarios.
  - Los únicos artefactos que deben permanecer son: (a) código fuente committeado, (b) entradas de crónica, y (c) dependencias locales necesarias para ejecución (e.g. `node_modules/`, que debe estar en `.gitignore`).

## 3. Fase 2: Cierre de Sesión (Consolidación)

Independientemente de la existencia de scripts de automatización, el contrato de cierre exige:
1. **Entrada en Crónica**: Crear `history/chronicle/YYYY-MM-DD/NN_XX_descripcion.md` donde:
   - `NN`: Número de secuencia del día.
   - `XX`: Código de autor:
     - `CL` — Claude (cualquier modelo) vía Claude Code CLI (acceso a ficheros, git, shell)
     - `CO` — Claude Opus vía Dispatch/Cowork (coordinador conversacional, sin acceso a ficheros)
     - `GE` — Gemini (cualquier interfaz)
   - El prefijo `XX` refleja el **autor intelectual** de la entrada, no necesariamente quien
     escribe el fichero. Si Opus (CO) origina una decisión y Sonnet (CL) la ejecuta, el
     fichero se prefija con `CO`. Cuando ambos contribuyen, el header lo aclara (ver abajo).
   - Contenido: Resumen de cambios, decisiones, estado de artefactos y briefings.
   - **Header en sesiones de rol mixto** (coordinador CO + implementador CL):
     ```markdown
     **Author:** CO (Claude Opus 4.6 via Dispatch)
     **Implemented by:** CL (Claude Sonnet 4.6 via Claude Code)
     ```
2. **Commit y Push**: Realizar un commit unificado con los cambios y la crónica.

## 4. Resolución de Conflictos

Si un agente objeta una implementación anterior: (1) Documentar objeción en crónica, (2) Escalar al humano. No se revierte trabajo ajeno sin autorización del responsable del proyecto.

## 5. Concurrencia: Semáforo de Trabajo

Cuando dos o más agentes pueden estar activos simultáneamente, se usa un
fichero de lock en `history/chronicle/LOCK.md` para evitar colisiones.

### Estructura de `LOCK.md`

```markdown
# Lock de Trabajo Activo

| Agente | Área reservada | Desde | Tarea |
|--------|---------------|-------|-------|
| Claude Opus 4.6 | trenza-cli/src/generator.rs | 2026-03-25 17:30 | Refactor generador WASM |
| Gemini Flash | editors/vscode/ | 2026-03-25 17:45 | Syntax highlighting |
```

### Reglas

1. **Crear lock al empezar**: Si la tarea va a modificar ficheros, el agente
   añade una fila a `LOCK.md` antes de empezar a editar. Si el fichero no
   existe, lo crea.
2. **Comprobar antes de reservar**: Si otro agente ya tiene un lock sobre la
   misma área (o un área que se solapa), el agente **no debe** empezar.
   Opciones: (a) trabajar en un área distinta, (b) coordinarse vía **briefing
   de interrupción** en la crónica, o (c) escalar al humano.
3. **Eliminar lock al cerrar**: Al completar la Fase 2 (cierre de sesión),
   el agente elimina su fila de `LOCK.md`. Si era la última fila, elimina
   el fichero entero.
4. **Locks huérfanos**: Un lock sin actividad de commit durante más de 24h
   se considera huérfano. Cualquier agente que detecte un lock huérfano debe
   notificar al humano y abstenerse de realizar modificaciones en el proyecto
   hasta recibir la orden de eliminación. Tras ejecutar la orden recibida,
   registrar la eliminación en la crónica.
5. **Granularidad y Jerarquía**: Reservar el área mínima necesaria. `trenza-cli/src/generator.rs`
   es mejor que `trenza-cli/`. Tareas globales deben bloquear el nodo raíz (`/`)
   o el directorio común afectado. Bloquear directorios implica bloquear todos
   sus descendientes.
6. **Lock no implica propiedad**: El lock es un semáforo, no una cesión de
   propiedad. El humano puede revocar cualquier lock en cualquier momento.
7. **Mantenimiento global**: Operaciones que afecten a la raíz del repositorio
   (limpieza de `.gitignore`, reestructuración de directorios, etc.) requieren
   lock sobre `/` y **aprobación explícita del humano** antes de ejecutarse.
