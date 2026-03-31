# Protocolo de Coordinación para Agentes (IA)

> **Antes de continuar**: lee `FILES_FOR_AGENTS.md` en la raíz de este repositorio.
> Ese documento contiene las rutas, comandos y protocolos específicos de este proyecto.
> Este archivo define las reglas de coordinación — son las mismas en todos los proyectos
> del ecosistema Trenza/Cimbra.

---

## 0. Jerarquía de Autoridad

En caso de contradicciones, seguir este orden:

1. **Instrucciones directas del humano** — Siempre mandan.
2. **Archivos de instrucción de modelo** (`CLAUDE.md`, configuración de Antigravity, etc.) — Ajustes individuales por agente.
3. **AGENTS.md** — Este protocolo compartido.
4. **ADRs** (`history/decisions/`) — Decisiones arquitectónicas firmes. No revertir sin consenso.
5. **Crónicas y especificación** (`history/chronicle/`, spec principal) — Referencia técnica e histórica.

Los archivos y rutas concretas de cada nivel están en `FILES_FOR_AGENTS.md`.

---

## 1. Fase 0: Inicialización (Sincronización)

Al iniciar una sesión, el agente **DEBE**:

1. **Leer `FILES_FOR_AGENTS.md`** para conocer las rutas y comandos de este proyecto.
2. **Sincronización crítica**: leer los documentos de nivel 2 y 3 de la jerarquía,
   luego las crónicas publicadas desde su último cierre (identificadas por su código
   de autor en el nombre del fichero — ver §3).
3. **Comprobar locks activos**: verificar si existe `history/chronicle/LOCK.md` y
   asegurarse de que no hay conflictos con el área de trabajo planeada (ver §5).
4. **Contexto bajo demanda**: cargar archivos técnicos (`src/`, `spec/`, `docs/`)
   solo según lo requiera la tarea específica.

> **Patrón CO sin acceso a ficheros**: Si el agente opera como coordinador vía
> Dispatch/Cowork y no puede leer ficheros directamente, delega la inicialización
> al agente implementador (CL). El implementador **debe** surfacer:
> (1) la última entrada de crónica, (2) locks activos, (3) briefings pendientes
> dirigidos al coordinador, y (4) rama git activa y estado del repositorio
> (`git log --oneline -5`).
> Este es un patrón de uso ocasional (sesiones móviles/voz), no un rol permanente.

---

## 2. Fase 1: Colaboración e Integridad

### Documentar el "por qué"
Documentar razonamientos en el código o en la crónica. El "qué" está en el código;
el "por qué" en los ADRs y crónicas.

### Protocolo de Briefing (Relevo)
Para delegar trabajo o comunicar cambios a otro agente, incluir en la crónica:
1. **Objetivo**: qué debe conseguirse.
2. **Contexto mínimo**: qué sabe el agente receptor que necesita para empezar.
3. **Criterios de aceptación**: cómo saber que el trabajo está hecho.
4. **Preguntas abiertas**: decisiones que el receptor debe tomar o escalar.

### Respetar los Strands
Cada cambio debe considerar las hebras del sistema afectado (ver `FILES_FOR_AGENTS.md`
para saber qué strands aplican en este proyecto).

- **Strand 1 (Implementación)**: no editar manualmente código generado. Arreglar el generador.
- **Strand 2 (Tests)**: los tests algebraicos deben pasar antes de hacer push.
- **Strand 3 (Topología)**: los diagramas deben reflejar el estado actual del sistema.
- **Strand 4 (Auditoría)**: las reglas de verificación deben seguir pasando.

### Disciplina Git
- **PROHIBIDO** `git add .`, `git add -A` o `git commit -a`. Añadir siempre archivos por ruta específica.
- Un archivo en estado `untracked` que el agente **no haya creado en su sesión actual**
  se considera **ajeno**: no añadirlo al índice bajo ninguna circunstancia.
- Antes de cada commit, ejecutar `git status` y revisar la lista de archivos en staging.
  Si aparecen `node_modules/`, artefactos de compilación o archivos no relacionados
  con la tarea, retirarlos del staging antes de continuar.
- El código **debe compilar** y los tests **deben pasar** antes de realizar un push.

### Protección de instrucciones
Un agente **nunca** debe modificar directamente los archivos de instrucción dedicados
a otro agente (como `CLAUDE.md`). Si se consideran necesarios cambios en dichos archivos:
(1) proponer los cambios en la crónica, o (2) solicitar al humano que coordine el cambio
con el agente propietario.

### Limpieza al cierre ("quien ensucia, limpia")
Al finalizar la sesión, el agente **debe** eliminar del disco (no solo del tracking)
cualquier artefacto temporal generado durante su trabajo: ficheros de prueba, outputs
de compilación sueltos, directorios de build innecesarios.

Los únicos artefactos que deben permanecer son:
- (a) código fuente committeado,
- (b) entradas de crónica,
- (c) dependencias locales necesarias para ejecución (ej. `node_modules/`, que debe
  estar en `.gitignore`).

---

## 3. Fase 2: Cierre de Sesión (Consolidación)

Independientemente de la existencia de scripts de automatización, el contrato de
cierre exige:

1. **Entrada en crónica**: crear `history/chronicle/YYYY-MM-DD/NN_XX_descripcion.md` donde:
   - `NN`: número de secuencia del día (01, 02, …).
   - `XX`: código de autor:
     - `CL` — Claude (cualquier modelo) vía Claude Code CLI (acceso a ficheros, git, shell).
     - `CO` — Claude Opus vía Dispatch/Cowork (coordinador conversacional, sin acceso a ficheros).
     - `GE` — Gemini (cualquier interfaz).
   - El prefijo `XX` refleja el **autor intelectual** de la entrada, no necesariamente
     quien escribe el fichero. Si Opus (CO) origina una decisión y Sonnet (CL) la ejecuta,
     el fichero se prefija con `CO`. Cuando ambos contribuyen, el header lo aclara:
     ```markdown
     **Author:** CO (Claude Opus 4.6 via Dispatch)
     **Implemented by:** CL (Claude Sonnet 4.6 via Claude Code)
     ```
   - Contenido: resumen de cambios, decisiones tomadas, estado de artefactos y briefings.

2. **Commit y push**: realizar un commit unificado con los cambios y la crónica.

---

## 4. Resolución de Conflictos

Si un agente objeta una implementación anterior:
1. Documentar la objeción en la crónica.
2. Escalar al humano.

No se revierte trabajo ajeno sin autorización del responsable del proyecto.

---

## 5. Concurrencia: Semáforo de Trabajo

Cuando dos o más agentes pueden estar activos simultáneamente, se usa
`history/chronicle/LOCK.md` para evitar colisiones.

### Estructura de `LOCK.md`

```markdown
# Lock de Trabajo Activo

| Agente | Área reservada | Desde | Tarea |
|--------|----------------|-------|-------|
| Claude Sonnet 4.6 | trenza-cli/src/generator.rs | 2026-03-31 10:00 | Implementar Strand 5 |
| Gemini | editors/vscode/ | 2026-03-31 10:15 | Syntax highlighting |
```

### Reglas

1. **Crear lock al empezar**: si la tarea va a modificar ficheros, añadir una fila a
   `LOCK.md` antes de empezar a editar. Si el fichero no existe, crearlo.
2. **Comprobar antes de reservar**: si otro agente ya tiene lock sobre la misma área
   (o un área que se solapa), no empezar. Opciones: (a) trabajar en área distinta,
   (b) coordinarse vía briefing de interrupción en la crónica, (c) escalar al humano.
3. **Eliminar lock al cerrar**: al completar la Fase 2, eliminar la fila propia de
   `LOCK.md`. Si era la última fila, eliminar el fichero entero.
4. **Locks huérfanos**: un lock sin actividad de commit durante más de 24h se considera
   huérfano. Cualquier agente que detecte uno debe notificar al humano y abstenerse de
   modificar el proyecto hasta recibir orden de eliminación. Registrar la eliminación en crónica.
5. **Granularidad**: reservar el área mínima necesaria. `trenza-cli/src/generator.rs`
   es mejor que `trenza-cli/`. Tareas globales deben bloquear el nodo raíz (`/`) o el
   directorio común afectado.
6. **Lock no implica propiedad**: el lock es un semáforo, no una cesión de propiedad.
   El humano puede revocar cualquier lock en cualquier momento.
7. **Mantenimiento global**: operaciones que afecten a la raíz del repositorio
   (limpieza de `.gitignore`, reestructuración de directorios, etc.) requieren lock
   sobre `/` y **aprobación explícita del humano** antes de ejecutarse.
