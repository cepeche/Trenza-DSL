# El entorno de colaboración de Trenza

**Fecha**: 13 de marzo de 2026
**Autor**: Claude Opus 4.6
**Motivación**: comentario del desarrollador sobre la intimidación de los IDE
modernos, y la excepción de Arduino y Processing como entornos accesibles.

---

## El problema: los IDE se han convertido en intimidantes

Un IDE moderno (VS Code, IntelliJ, Eclipse) presenta al usuario centenares
de funciones visibles desde el primer momento: pestañas, paneles, terminales,
depuradores, extensiones, configuraciones, paletas de comandos. La curva de
aprendizaje es desproporcionada al problema que resuelven. Están diseñados
para generalistas — lo que significa que nadie se siente del todo en casa.

Para un proyecto como Trenza, donde la especificación ES el código, esta
complejidad es contraproducente. El usuario no necesita un depurador (no
hay ejecución que depurar). No necesita autocompletado de APIs (el vocabulario
es reducido y formal). No necesita gestión de dependencias (el paquete
`.tzp` es autocontenido).

## La excepción: Arduino y Processing

¿Qué hacen bien?

1. **Dos botones**: verificar y ejecutar. No hay menú con 40 opciones.
   La decisión del usuario en cada momento es binaria: ¿está bien? y ¿qué
   hace?

2. **Feedback inmediato**: el LED parpadea, el sketch dibuja. No hay que
   imaginar lo que pasa — se ve. El ciclo escribir → ver es de segundos,
   no de minutos.

3. **El editor ES el entorno**: no hay que configurar un workspace, abrir
   un proyecto, seleccionar un SDK, instalar extensiones. Se abre, se
   escribe, se verifica. Todo lo demás es invisible.

4. **Vocabulario reducido**: `setup()`, `loop()`, `digitalWrite()`. Se
   aprende en una tarde. La restricción es una virtud: libera al usuario
   de decidir *cómo* hacer las cosas y le permite concentrarse en *qué*
   quiere que pase.

## El equivalente para Trenza

### Dos acciones, no cuarenta

| Arduino | Processing | **Trenza** |
|---------|------------|------------|
| Verificar (✓) | Run (▶) | `trenza verify` |
| Subir (→) | — | `trenza inspect` |

- **`trenza verify`**: ¿está bien la especificación? Aplica las 6 reglas
  de verificación. Devuelve OK o una lista de errores legibles.
- **`trenza inspect`**: ¿qué dice la especificación? Genera la vista
  expandida de un contexto, mostrando roles heredados, transiciones,
  y effects.

No hay `build`, `deploy`, `debug`, `profile`, `lint`. No porque no vayan
a existir, sino porque no son la primera pregunta del usuario.

### El resultado visible

En Arduino, el resultado es un LED. En Processing, un dibujo. En Trenza,
el resultado es **el árbol de documentación navegable**.

El directorio `docs/sistema/` ES la salida visual de Trenza:

```
docs/sistema/
├── index.md          ← ¿qué es este sistema?
├── base/
│   ├── ModoNormal.md ← ¿qué pasa en este modo?
│   └── ModoEdicion.md
├── overlays/
│   ├── ModalHistorial.md ← ¿cómo funciona el historial?
│   ...
```

Cada `.md` tiene un diagrama Mermaid de máximo 5–8 nodos (un contexto y
sus vecinos inmediatos). No hay un solo diagrama monstruo. La complejidad
se navega, no se presenta toda de golpe.

Esto es lo que Arduino y Processing entienden: **el feedback debe ser
proporcional a la pregunta**. Si pregunto por un contexto, veo ese
contexto. Si pregunto por el sistema, veo la arquitectura general.

### El entorno es el editor + el navegador

No se necesita VS Code ni IntelliJ. Se necesita:

1. **Un editor de texto** (cualquiera) para escribir `.trz`
2. **Un terminal** para ejecutar `trenza verify` y `trenza inspect`
3. **Un navegador de markdown** (GitHub, VS Code preview, o el propio
   terminal) para leer `docs/sistema/`

`git clone` + `trenza verify` es todo lo que hace falta para empezar.
No hay "proyecto" que abrir, ni "workspace" que configurar, ni extensión
que instalar.

## Requisitos como documentación navegable

### La inversión del flujo

En el flujo tradicional:

```
requisitos → código → tests → documentación
```

Cuatro artefactos separados. Cuatro fuentes de inconsistencia.

En Trenza:

```
conversación → .trz → { código, tests, requisitos, diagramas }
```

Un artefacto primario (`.trz`). Todo lo demás se deriva.

### El árbol `docs/sistema/` ES el documento de requisitos

No hay un documento separado de requisitos que mantener. Cada `.md` en
`docs/sistema/` es un requisito funcional:

- **ModoNormal.md** describe qué puede hacer el usuario en modo normal
- **ModalReset.md** describe el flujo de borrado de datos en 3 fases
- La tabla de roles ES la lista de elementos de interfaz
- La tabla de transiciones ES el flujo de navegación
- Los GAPs abiertos SON los requisitos pendientes

Un stakeholder no técnico puede navegar `docs/sistema/index.md` en GitHub
y entender la arquitectura sin abrir un IDE.

### Trazabilidad sin esfuerzo

Cada `.md` enlaza a su `.trz` fuente. Si un stakeholder pregunta "¿por qué
el botón de guardar no funciona sin nombre?", la respuesta está en
[ModalCrearTarea.md](sistema/overlays/ModalCrearTarea.md) → GAP-5.

Cuando el parser regenera el árbol, los cambios en la especificación se
reflejan automáticamente en la documentación. No hay desfase.

## Protocolo de colaboración

### Participantes

El entorno de Trenza no es para un solo usuario. Es para equipos donde
algunos participantes son humanos y otros son LLMs:

| Participante | Lee | Escribe | Se comunica por |
|-------------|-----|---------|-----------------|
| Desarrollador | `.trz`, `docs/sistema/` | `.trz`, `docs/` | terminal, editor |
| Claude | `.trz`, `docs/`, git log | `.trz`, `docs/`, código | git commits |
| Gemini | `.trz`, `docs/`, git log | `.trz`, `docs/`, código | git commits |
| Stakeholder | `docs/sistema/` | (solo lectura) | issues, conversación |

### Git como workspace compartido

No hay APIs entre LLMs. No hay sesiones compartidas. El repositorio git
es el entorno:

- Los `.trz` son el código fuente
- Los `docs/` son las decisiones de diseño y la documentación generada
- Los commits son los mensajes entre participantes
- Los prefijos de fecha (`YYYY-MM-DD-nn-`) son la cronología

Cada agente puede trabajar en paralelo sobre diferentes contextos. El
verificador detecta inconsistencias cuando se integran los cambios.

### El "editor" del futuro

Si algún día Trenza necesita un editor visual, debería parecerse más a
Scratch o Figma que a VS Code:

- **Arrastrar contextos** en un canvas (no escribir YAML en un panel)
- **Conectar transiciones** con líneas (no editar tablas)
- **Ver roles** como tarjetas dentro de cada contexto
- **El `.trz` se genera** desde la representación visual, no al revés

Pero esto es futuro. Hoy, un editor de texto y un terminal son suficientes.
La restricción de Trenza (vocabulario reducido, estructura fija, semántica
verificable) es lo que hace posible que un editor simple baste.

## Resumen

| Principio | Arduino/Processing | Trenza |
|-----------|-------------------|--------|
| Acciones mínimas | ✓ Verificar, ▶ Ejecutar | `verify`, `inspect` |
| Feedback inmediato | LED, dibujo | árbol de `.md` navegable |
| Sin configuración | abrir → escribir → verificar | `git clone` → escribir → `trenza verify` |
| Vocabulario reducido | setup, loop, draw | context, role, on, transitions, effects |
| Requisitos integrados | — | `docs/sistema/` = documento de requisitos |
| Colaboración | — | git como protocolo, LLMs como participantes |

---

## Preguntas abiertas

1. **¿Debería `trenza inspect` generar el árbol `docs/sistema/` completo,
   o solo mostrar la vista expandida de un contexto individual?** La primera
   opción es más útil como documentación; la segunda, como herramienta de
   desarrollo.

2. **¿Qué formato para el informe de verificación?** El terminal muestra
   errores. Pero ¿debería generar también un `verification/report.md`
   navegable dentro de `docs/sistema/`?

3. **¿Obsidian como alternativa al navegador de `.md`?** Las características
   de Obsidian (backlinks, graph view, búsqueda) encajan naturalmente con
   el árbol de documentación. Pero añade una dependencia que contradice el
   principio de "sin configuración".
