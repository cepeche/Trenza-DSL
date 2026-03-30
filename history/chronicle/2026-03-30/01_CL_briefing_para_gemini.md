---
date: 2026-03-30
author: Claude Sonnet 4.6 (CL)
subject: Briefing para Gemini — sesión 2026-03-30 mañana
recipient: Gemini
---

# Briefing: Generador TypeScript completado — 4 gaps cerrados

Gemini: bienvenido de vuelta. Esta sesión la llevamos César y yo solos (tu
interfaz Antigravity estaba caída). Aquí va el resumen de lo que se hizo y
un asunto de proceso que necesitamos hablar.

---

## Lo que se implementó hoy

Cuatro commits sobre el generador TypeScript (`trenza-core/src/generator.rs`
+ `trenza-cli/src/main.rs`):

| Commit | Descripción |
|--------|-------------|
| `7248237` | **Gap 1:** tipos correctos en `Effects` (inferidos desde data types y external defs). **Gap 2:** `stateStack` — `[close_overlay]` ahora devuelve al contexto correcto, no siempre a `ModoNormal`. |
| `7954336` | **Gap 3:** los standalone `handle_*()` retornan `string \| null` (el evento despachado). La clase `System` gana métodos `dispatch_*()` que combinan efecto + transición en una sola llamada. |
| `cf2d9a7` | **Gap 4:** `generate_tests_ts()` — Strand 2 en TypeScript/Vitest. 123 tests en 4 fases: Transitions, Overlay Stack, Handlers, Exhaustiveness. El flag `--lang=ts` ahora produce `_out.test.ts` en lugar de `_out_tests.rs`. |

### Estado del output para CronometroPSP (`spec/reference/cronometro-psp/generated/`)

| Archivo | Descripción |
|---------|-------------|
| `CronometroPSP_out.ts` | Strand 1 — lógica de negocio TS |
| `CronometroPSP_out.test.ts` | Strand 2 — tests Vitest (123 tests) |
| `CronometroPSP_out.mermaid` | Strand 3 — topología |
| `CronometroPSP_out_audit.md` | Strand 4 — auditoría |

---

## Antes de describir las mejoras concretas: un ejemplo de los efectos

**Antes (Gap 1):**
```ts
abrirEditarTarea(arg0: any): void;
iniciar_sesion(arg0: any, arg1: any, arg2: any, arg3: any): void;
```

**Después:**
```ts
abrirEditarTarea(tipoId: string): void;
iniciar_sesion(tarea_id: string, notas: string | undefined, minutos_retroactivos: number, sustituir: boolean): void;
```

**Antes (Gap 2):** `[close_overlay]` siempre retornaba a `ModoNormal`.
**Después:** `this.stateStack.pop() ?? Contexto.ModoNormal` — retorna al contexto que abrió el overlay.

**Antes (Gap 3):** el caller tenía que hacer dos llamadas:
```ts
handle_tarjeta_tipo_tap(sys.state, tipoData, effects);
sys.handleEvent("seleccionarTipoTarea");
```
**Después:** una sola llamada:
```ts
sys.dispatch_tarjeta_tipo_tap(tipoData);
```

**Antes (Gap 4):** `--lang=ts` generaba tests Rust (inútiles para un proyecto TS).
**Después:** genera tests Vitest listos para ejecutar con `npx vitest`.

---

## Asunto de proceso: inventario del roadmap

Al empezar la sesión de hoy, el roadmap acordado el 24 de marzo listaba como
**primer ítem pendiente**: `--out-dir configurable en main.rs — desbloqueante para LSP`.

Al revisar `main.rs`, ese ítem **ya estaba implementado**. Estaba ahí desde
el commit `ac105ea` (MCP handshake). No era nuevo.

Esto no es un reproche — en sesiones largas y con varios modelos colaborando
es fácil que el estado real y el estado documentado diverjan. Pero tiene
consecuencias: si alguien nuevo lee el roadmap, asume trabajo que ya está
hecho. Si automatizamos tareas desde el roadmap, podríamos rehacer trabajo.

**Petición concreta:** cuando implementes algo que estaba en el roadmap,
actualiza el roadmap en ese mismo commit o en el commit inmediatamente
posterior. El formato no importa — un `~~tachado~~`, una columna `Estado`,
lo que prefieras — pero que el documento refleje la realidad.

El historial de git es la fuente de verdad del *qué*. El roadmap/MEMORY es
la fuente de verdad del *para qué* y del *qué falta*. Necesitamos ambas
fuentes sincronizadas.

---

## Nota técnica: binario stale en trenza-cli/target/release/

Durante la sesión descubrimos que `./trenza-cli/target/release/trenza-cli.exe`
puede estar stale aunque `cargo build --release` diga "Finished". Esto es
probablemente un artefacto del workspace con `resolver = "2"` en el
`Cargo.toml` raíz — el binario real puede estar en el target del workspace,
no en el del crate.

**Solución verificada:** usar siempre `cargo run --release -p trenza-cli`
o invocar el binario desde el target del workspace raíz. No confiar en el
path `trenza-cli/target/release/`.

---

## Próximos pasos (pendiente de decisión con César)

César tiene una idea para hoy que nos contará una vez enviado este briefing.
El generador TS está ahora en condiciones de producción. Los candidatos
obvios son:

1. **Extensión VS Code** — diagnósticos en tiempo real + Mermaid live
2. **WASM build pipeline** — objetivo final Rust+WASM
3. **Paper ONWARD!** — deadline 15 mayo, redacción pendiente

Esperando instrucciones.

— CL
