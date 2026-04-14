# Movimiento de contexto personal desde memoria indexada a chronicle

**Fecha**: 2026-04-14
**Autor**: Claude Opus 4.6 (CL), sesión de auditoría de memoria
**Origen**: Sesión genérica desde `C:\Proyectos` (no proyecto concreto), ID `91b4e856-3f64-44ad-9b55-f9258dc67e02`

## Contexto

Durante una sesión exploratoria iniciada por el usuario para entender los límites de consumo de Claude Code, descubrimos dos cosas relevantes:

1. El comando `/insights` de Claude Code produce un informe sofisticado a partir de los transcripts locales, enviando las sesiones a la API para análisis.
2. Claude Code v2.1.59+ incluye una función de **Auto Memory** activada por defecto que escribe archivos `.md` indexados en `~/.claude/projects/<proyecto>/memory/` sin supervisión explícita del usuario. Estos archivos se cargan automáticamente en el contexto de toda sesión que arranque desde ese proyecto.

Al auditar el contenido acumulado, encontramos `memory/user_context.md` en Trenza-DSL, creado el 2026-03-26 a las 14:38:20 por un Claude anterior. La fecha coincide con el día del briefing MAPSE (22 entradas de chronicle ese día: 01_CL_briefing_mapse_vision.md y siguientes), así que es muy probable que se escribiera en ese contexto de trabajo.

## Decisión del usuario

El contenido tiene **valor histórico como motivación de decisiones de diseño** — particularmente por qué MAPSE es voice-first y por qué el proyecto opta por el modelo humano+agentes en vez de empleados humanos — pero **no debe cargarse pasivamente en el contexto de cada sesión ni viajar por API en cada llamada**.

Se acuerda:
- Preservar el contenido literal en esta entrada de chronicle (registro inmutable).
- Borrar `memory/user_context.md`.
- Eliminar la referencia a ese archivo en `memory/MEMORY.md` de Trenza-DSL.
- En el futuro, si algún contexto de esta información es relevante para una conversación estratégica, **el usuario lo citará explícitamente**; no será recuperado automáticamente.

## Contenido preservado (literal, desde `memory/user_context.md`)

---

> César is retired. His primary motivation for Trenza is intellectual fascination with
> the process of human-AI collaborative design. However, there is a real financial
> dimension: he and his wife rent (lost their home in the first of three business
> partnership frauds), and she is concerned about losing his pension income when he
> passes. Any financial return from the project matters primarily for her security,
> not for him personally.
>
> This context should inform strategic conversations: the project doesn't need to
> become a unicorn, but generating some form of durable income stream (royalties,
> licensing, small recurring revenue) would be meaningful. Pure intellectual
> pursuit is not enough — there needs to be a path to something tangible, even
> if modest.
>
> Also relevant: César has been defrauded by business partners three times. He has
> resolved never to work with human employees again. The AI-agent model is not just
> a technical choice — it is a personal boundary.
>
> The MAPSE vision (voice-first, accessible development environment) is not abstract:
> César has a congenital cataract (no vision in right eye), and a likely cataract
> surgery on the left eye. The "blind person building a system by voice" scenario
> is a real personal need, not a hypothetical. This makes the accessibility-first
> design constraint both genuine and urgent.

---

## Nota sobre la cronología

Se opta deliberadamente por registrar el movimiento en la chronicle de **hoy (2026-04-14)** y no retroactivamente en la del 2026-03-26. La razón: la decisión de mover el contenido de memoria indexada a chronicle inmutable se toma hoy, no entonces. Forzar una entrada retroactiva falsificaría la cronología. La fecha original de creación queda documentada en este mismo registro para preservar la trazabilidad.
