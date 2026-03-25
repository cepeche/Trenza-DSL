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
1. **Sincronización Crítica**: Leer las últimas entradas en `history/chronicle/` (puede haber varias del mismo día de distintos agentes) y los documentos de nivel 2 y 3 de la jerarquía.
2. **Comprobar locks activos**: Verificar si existe un fichero `LOCK.md` en `history/chronicle/` (ver sección 5).
3. **Contexto On-Demand**: Cargar archivos técnicos (`src/`, `spec/`, `docs/`) solo según lo requiera la tarea específica.

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

## 3. Fase 2: Cierre de Sesión (Consolidación)

Independientemente de la existencia de scripts de automatización, el contrato de cierre exige:
1. **Entrada en Crónica**: Crear `history/chronicle/YYYY-MM-DD/NN_<descripcion>.md` con:
   - Resumen de cambios y decisiones.
   - Estado de los artefactos (`task.md`, etc.).
   - Preguntas abiertas y briefings para el siguiente agente.
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

Cada fila es un **lock**. Los campos:
- **Agente**: identidad del modelo que reserva.
- **Área reservada**: ruta o patrón de ficheros afectados (tan específico como sea posible).
- **Desde**: fecha y hora de creación del lock.
- **Tarea**: descripción breve de lo que se va a hacer.

### Reglas

1. **Crear lock al empezar**: Si la tarea va a modificar ficheros, el agente
   añade una fila a `LOCK.md` antes de empezar a editar. Si el fichero no
   existe, lo crea.
2. **Comprobar antes de reservar**: Si otro agente ya tiene un lock sobre la
   misma área (o un área que se solapa), el agente **no debe** empezar.
   Opciones: (a) trabajar en un área distinta, (b) coordinarse vía briefing
   en la crónica, o (c) escalar al humano.
3. **Eliminar lock al cerrar**: Al completar la Fase 2 (cierre de sesión),
   el agente elimina su fila de `LOCK.md`. Si era la última fila, elimina
   el fichero entero.
4. **Locks huérfanos**: Un lock sin actividad de commit durante más de 24h
   se considera huérfano. Cualquier agente puede eliminarlo, pero debe
   registrar la eliminación en la crónica y notificar al humano.
5. **Granularidad**: Reservar el área mínima necesaria. `trenza-cli/src/generator.rs`
   es mejor que `trenza-cli/`. No bloquear directorios enteros salvo que
   la tarea lo requiera realmente.
6. **Lock no implica propiedad**: El lock es un semáforo, no una cesión de
   propiedad. El humano puede revocar cualquier lock en cualquier momento.
