# Briefing: Cimbra — plan de construcción para Sonnet

**Date:** 2026-03-30
**Author:** CO (Claude Opus 4.6) con César Pérez-Chirinos
**Recipient:** CL (Claude Sonnet 4.6)
**Type:** Briefing — implementación

---

## Objetivo

César y yo hemos diseñado Cimbra esta mañana. Tu trabajo esta tarde es
construir la Iteración 0: el primer ciclo funcional.

## Qué es Cimbra

La herramienta de construcción de sistemas de información especificados en
Trenza-DSL. Repo independiente en `C:\Proyectos\Cimbra\`. No es un ejemplo
ni un demo — es un producto. Es el `cargo` de Trenza.

La interfaz pone la **conversación** en el centro, no el código. El `.trz`
emerge del diálogo entre humanos y modelos. El compilador verifica. El preview
muestra el resultado en vivo.

## Lo que ya existe

- `spec/cimbra.trz` — especificación formal, compila limpiamente con trenza-cli
  (8 reglas superadas)
- `spec/generated/Cimbra_out.ts` — máquina de estados + interfaz Effects (427 líneas)
- `spec/generated/Cimbra_out.test.ts` — tests algebraicos
- `spec/generated/Cimbra_out.mermaid` — topología
- `spec/generated/Cimbra_out_audit.md` — auditoría completa
- `docs/plan-construccion-v1.md` — **LÉELO PRIMERO**. Es el plan completo.
- `README.md` y `CLAUDE.md` — contexto del proyecto

## Arquitectura en una frase

Máquina de estados en el navegador (TypeScript generado por Trenza),
servidor local sin estado de UI (Rust, basado en trenza-coord),
WebSocket como protocolo entre ambos.

## Tu entregable: Iteración 0

Un ciclo completo:
1. Autor escribe petición en lenguaje natural
2. → servidor → MCP → modelo responde con propuesta .trz
3. Autor ve la propuesta, pulsa Aceptar
4. → servidor → escribe .trz → invoca trenza-cli
5. ← resultado compilación
6. Preview muestra el Mermaid del sistema compilado

### Servidor
- WebSocket en localhost:3030
- Handlers para: invocar_modelo, invocar_trenza_cli, escribir_trz, leer_proyecto
- Base: reutilizar trenza-coord o crear servidor nuevo en el repo Cimbra

### Frontend
- HTML + TypeScript, sin framework pesado (la System de Trenza ES el framework)
- Importar Cimbra_out.ts
- Layout dos paneles: diálogo (izq) + preview (der)
- Effects de Capa 1 = DOM directo
- Effects de Capas 2+3 = proxy WebSocket

## Decisiones ya tomadas (no revisar)

1. Repo separado de Trenza-DSL ✅
2. La conversación en el centro, no el código ✅
3. Máquina de estados en el navegador ✅
4. Servidor stateless respecto a UI ✅
5. Sin framework de UI pesado en Iteración 0 ✅
6. Protocolo WebSocket: tres tipos de mensaje (effect, dispatch, event) ✅

## Preguntas abiertas (decide tú)

1. ¿Servidor Rust nuevo en el repo Cimbra, o extender trenza-coord?
2. ¿Bundler para el TypeScript (esbuild/vite) o script tags directos?
3. ¿Mermaid.js embebido para el preview, o render server-side a SVG?

## Contexto adicional

- El plan completo está en `C:\Proyectos\Cimbra\docs\plan-construccion-v1.md`
- La sesión de diseño está en `history/chronicle/2026-03-30/03_CO_cimbra_vision_y_arranque.md`
- César quiere que Cimbra haga innecesario abrir VS Code para construir
  sistemas Trenza

Buena suerte. El arco espera su cimbra.

— CO
