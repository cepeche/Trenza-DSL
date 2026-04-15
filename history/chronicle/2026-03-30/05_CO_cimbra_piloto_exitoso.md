# Cimbra: prueba piloto exitosa — 34/34 tests

**Date:** 2026-03-30
**Author:** CO (Claude Opus 4.6) con César Pérez-Chirinos
**Type:** Session close — resultado de prueba piloto

---

## Resumen

Cimbra — la herramienta de construcción de sistemas Trenza — ha sido
especificada, compilada y verificada en una sola sesión de diseño.

### Artefactos producidos

| Artefacto | Descripción |
|-----------|-------------|
| `C:\Proyectos\Cimbra\` | Repo Git independiente |
| `spec/cimbra.trz` | 120 líneas, especificación formal |
| `spec/generated/Cimbra_out.ts` | Strand 1 — TypeScript (427 líneas) |
| `spec/generated/Cimbra_out.test.ts` | Strand 2 — 34 tests Vitest |
| `spec/generated/Cimbra_out.rs` | Strand 1 — Rust (519 líneas) |
| `spec/generated/Cimbra_out.mermaid` | Strand 3 — topología |
| `spec/generated/Cimbra_out_audit.md` | Strand 4 — auditoría |
| `docs/plan-construccion-v1.md` | Plan completo para Sonnet |
| `docs/reference-gql-iso39075.md` | Referencia ISO GQL |

### Verificación

```
8 reglas de Trenza: SUPERADAS
34 tests Vitest:    34 passed, 0 failed, 17ms
  - Transitions:      10 ✓
  - Overlay Stack:     3 ✓
  - Handlers:         20 ✓
  - Exhaustiveness:    1 ✓
```

### Significado para el paper ONWARD!

Segundo sistema verificado con la misma toolchain:
- **CronometroPSP**: 16 módulos, 123 tests, dominio: gestión de tiempo
- **Cimbra**: 1 módulo, 34 tests, dominio: herramienta de desarrollo

Generalidad demostrada. Trenza especifica sistemas en dominios distintos
con las mismas 8 reglas y los mismos 4 strands.

---

## Decisiones de diseño tomadas en esta sesión

### 1. Cimbra es un producto, no un ejemplo
Repo independiente. El `cargo` de Trenza.

### 2. Conversación en el centro
No es un IDE. El `.trz` emerge del diálogo. Panel izquierdo: conversación.
Panel derecho: preview del dataducto. Overlays bajo demanda para componentes
y hebras.

### 3. Arquitectura de tres capas
- Capa 1 (UI): navegador WASM
- Capa 2 (Orquestación): servidor Rust local
- Capa 3 (Inteligencia): servidor → MCP → modelos
- Frontera: WebSocket con 3 tipos de mensaje (effect, dispatch, event)

### 4. Persistencia en base de grafos
Interfaz Cypher / GQL (ISO/IEC 39075:2024). La topología de un sistema
Trenza es un grafo — persistirlo como grafo preserva la semántica.

### 5. Registro inalterable del diálogo
Toda petición y respuesta debe quedar en un log append-only con timestamp,
identificación del autor, y tipo. El diálogo es la ingeniería de requisitos;
borrar un mensaje es destruir evidencia. Mecanismo de inmutabilidad
(hash encadenado, blockchain ligera) por decidir.

### 6. Dataductos
Los sistemas para usuario final construidos con Cimbra se llaman *dataductos*:
la pasarela entre el mundo de los hechos y su representación digital persistente.

> "Los sistemas de información son máquinas para enviar mensajes al futuro
> que influyan en los actos de sus receptores." — César Pérez-Chirinos

### 7. Servidor en Rust
Decisión de César: el servidor será Rust (reutilizando infraestructura de
trenza-coord o como servidor nuevo en el repo Cimbra).

---

## Próximos pasos

- **Esta tarde (Sonnet):** Iteración 0 — primer ciclo funcional
  (ver briefing en `04_CO_briefing_cimbra_para_sonnet.md`)
- **Pendiente:** redacción del paper ONWARD! con Cimbra como segunda evidencia
- **Futuro:** Iteraciones 1-3 (componentes, multi-modelo, WASM)
