# Cimbra: visión, arquitectura de UI y arranque del proyecto

**Date:** 2026-03-30
**Author:** CO (Claude Opus 4.6 via Claude Code)
**Type:** Session close — diseño conceptual + decisión de proyecto

---

## 1. Decisión: Cimbra es un producto, no un ejemplo

Cimbra es la herramienta de construcción por defecto de sistemas de información
especificados en Trenza-DSL. Repo independiente, ciclo de vida propio.

Analogía: Rust tiene `rustc` (compilador) y `cargo` (build tool).
Trenza tiene `trenza-cli` y Cimbra.

## 2. Intenciones fundacionales (César)

**A — Composición de componentes.**
Facilitar la reutilización de componentes especificados en Trenza. Agrupar y
desagrupar para permitir composiciones diferentes. No un chip supercomplejo:
celdas componibles en un SoC.

**B — Dataductos.**
Los sistemas para usuario final construidos con Cimbra se denominan *dataductos*.
Un dataducto es la pasarela entre el mundo de los hechos y su representación
digital persistente. Los sistemas de información son máquinas para enviar
mensajes al futuro que influyan en los actos de sus receptores.

**C — WASM + MCP.**
Las primeras iteraciones serán aplicaciones WASM ejecutadas en un navegador
local contra un servidor MCP arrancado en la máquina del desarrollador.
El embrión del servidor MCP ya existe en `trenza-coord/`.

## 3. Interfaz de usuario: la conversación en el centro

Cimbra **no es un IDE**. Un IDE pone el código en el centro. Cimbra pone la
conversación en el centro. El `.trz` no se escribe — emerge del diálogo.

```
┌─────────────────────────┬──────────────────────────┐
│  DIÁLOGO                │  APLICACIÓN              │
│                         │                          │
│  ┌───────────────────┐  │  (preview en vivo del    │
│  │ Petición humana   │  │   dataducto que estamos  │
│  └───────────────────┘  │   especificando)         │
│                         │                          │
│  ┌───────────────────┐  │                          │
│  │ Respuesta modelo  │  │                          │
│  └───────────────────┘  │                          │
│  ┌───────────────────┐  │                          │
│  │ Respuesta modelo  │  │                          │
│  └───────────────────┘  │                          │
│                         │                          │
├─────────────────────────┴──────────────────────────┤
│  [Componentes]  [Hebras]     (bajo demanda)        │
└────────────────────────────────────────────────────┘
```

Tres niveles de atención:
1. **Primario** — el diálogo (siempre visible, izquierda)
2. **Primario** — la aplicación viva (siempre visible, derecha)
3. **Secundario** — navegador de componentes y visor de hebras (bajo demanda)

## 4. Mapeo a Trenza

La interfaz mapea naturalmente al modelo de Trenza:

- **Contextos base**: `Dialogo`, `VistaPrevia`
- **Overlays**: `NavegadorComponentes`, `VisorHebras`
- **Roles**: `Autor` (humano), `Modelo` (LLMs), `Sistema` (compilador + runtime)
- Flujo: petición → cristalización .trz → compilación → actualización vista previa

El flujo principal es conversar hasta que el arco se sostiene solo.
Cuando lo hace, la cimbra se retira.

## 5. Roles naturales de Cimbra

| Rol | Quién | Responsabilidad |
|-----|-------|-----------------|
| Autor | Humano | Aporta intención, valida, decide |
| Modelo | LLM(s) | Cristaliza .trz desde el diálogo |
| Integrador | Humano o Modelo | Agrupa/desagrupa celdas |
| Sistema | Compilador + runtime | Valida, genera, sirve |

## 6. Próximos pasos

- [x] Crear repo `Cimbra` en `C:\Proyectos\Cimbra\`
- [ ] Escribir `cimbra.trz` — especificación del flujo de trabajo
- [ ] Compilar con `trenza-cli` — producir 4 strands
- [ ] Documentar como segunda evidencia ejecutable para el paper ONWARD!
- [ ] (Iteración 2+) WASM frontend + MCP integration

---

> "Los sistemas de información son máquinas para enviar mensajes al futuro
> que influyan en los actos de sus receptores." — César Pérez-Chirinos
