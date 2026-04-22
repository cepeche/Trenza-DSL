# Coordinación: convertir el demo WASM en réplica funcional defensible

**Fecha:** 2026-04-22 (miércoles)
**Autor:** Claude Opus 4.7 (CO), sesión vía Claude Code
**Plan al que responde:** pregunta del usuario tras envío del paper — "¿hay réplica funcional o solo .trz compilados?"
**Sesiones paralelas:** Gemini 3 Flash (A: port HTML+CSS · B: storage adapter), Opus (Fase 2 + integración)
**Tests:** definidos por brief; integración final con smoke manual del golden path.

---

## Contexto

El paper ONWARD! 2026 fue enviado el 17 de abril. Al revisar el demo
[examples/cronometro-wasm/](../../../examples/cronometro-wasm/) frente a un
revisor hipotético, queda claro que lo que existe hoy es un *executable proof
of the synthesis pipeline* (4 botones cableados a la máquina compilada), no
una réplica funcional de CronometroPSP. La afirmación defendible es estrecha;
la indefendible —"hemos reimplementado CronometroPSP"— el paper no la hace,
pero un revisor podría leerla por implicación.

Objetivo: subir el demo a "réplica funcional del golden path con persistencia
real, conducida íntegramente por el `.trz`", a tiempo de incorporarlo al
material de rebuttal (notificación: 22 junio).

## Decisiones tomadas con el usuario

1. **Persistencia**: `localStorage` namespaced bajo `cronometro-psp:v1:*`.
   No se reusa la API PHP del NAS — penaliza reproducibilidad ONWARD!.
2. **Estilo visual**: portar `styles.css` original literal. Refuerza la
   afirmación "es la misma app" frente al revisor.
3. **Alcance**: 5 overlays golden funcionales (MenuConfiguracion,
   ModalSeleccionActividad, ModalComentario, ModalReset, ModalHistorial) +
   5 stubs renderizables gobernados por `.trz` (Crear/Editar Tarea/Actividad,
   AcercaDe).
4. **Reparto**: Gemini 3 Flash hace lo mecánico en paralelo; Opus se queda
   con la pieza que el paper defiende conceptualmente (capa de efectos
   sobre overlays).

## Reparto del trabajo

| Brief | Encargado | Entregable | Bloquea a |
|-------|-----------|------------|-----------|
| A — Port HTML+CSS | Gemini 3 Flash | `index.html` + `public/styles.css` portados | Integración final |
| B — Storage adapter | Gemini 3 Flash | `src/storage.ts` + `storage.types.ts` + tests | Integración final |
| Fase 2 — Overlay effects | Opus 4.7 | Capa `mountOverlayEffects(system)` en `src/main.ts` | Integración final |
| Integración + golden path | Opus 4.7 | Cableado A+B+Fase 2; recorrido manual capturado | — |

**Cero solape de archivos** entre A, B y Fase 2. Los tres pueden empezar
inmediatamente.

## Mapping `.trz` overlay → id DOM original

Verificado por grep contra
[history/inspirations/cronometro-psp-original/frontend/index.html](../../inspirations/cronometro-psp-original/frontend/index.html):

| Overlay declarado en `.trz` | id DOM en HTML original |
|------------------------------|-------------------------|
| `MenuConfiguracion` | `settingsMenu` |
| `ModalSeleccionActividad` | `activityModal` |
| `ModalCrearTarea` | `createTaskModal` |
| `ModalCrearActividad` | `createActivityModal` |
| `ModalComentario` | `commentModal` |
| `ModalAcercaDe` | `aboutModal` |
| `ModalReset` | `resetModal` |
| `ModalEditarTarea` | `editTaskModal` |
| `ModalEditarActividad` | `editActivityModal` |
| `ModalHistorial` | `historialModal` |

Mapping 1:1 limpio. Esta tabla la usa Fase 2 para construir la convención
`overlayName → '#' + camelCase(strip('Modal'))`. Brief A debe **preservar
estos `id` literales**.

## Riesgos identificados

- **Riesgo R1 (medio)**: el `.trz` actual puede no exponer todos los eventos
  con payload que la UI necesita (p. ej. `seleccionarActividad(id)`). Si
  aparece en Fase 2 o integración, el déficit lo arregla el `.trz`, no el
  TS — y eso refuerza el paper, no lo debilita.
- **Riesgo R2 (medio)**: el `InterpreterWasm` puede no soportar payloads
  ricos hoy. Si bloquea, decisión: extender intérprete (preferido) o
  limitar el demo a eventos sin payload (degradado pero válido).
- **Riesgo R3 (bajo)**: Gemini "mejora" el HTML con un framework o reescribe
  el CSS. Mitigado en Brief A con instrucción explícita de copia literal.

## Convergencia

Cuando A, B y Fase 2 estén en `main`, Opus integra en una sesión corta
(~30 min) y captura GIF/PNG del modo edición funcionando. Material directo
para §evaluación del paper en el rebuttal.
