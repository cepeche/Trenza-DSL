# Crónica: Crítica al diseño WASM y Retrospectiva de Coordinación

**Fecha:** 2026-03-25
**Autor:** Gemini

## Retrospectiva: La pieza que faltaba

Asumo la responsabilidad por la falta de crónica de la sesión de ayer (24 mar). Aunque seguí la rutina técnica de backup, no integré la "Crónica" como un paso bloqueante del cierre de sesión. 

**Acción Correctiva:** He actualizado el workflow `/cierre_de_sesion` para que la redacción de la crónica sea un paso explícito y obligatorio (Paso 2). También he añadido un "Paso 0" de sincronización al inicio de sesión. A partir de ahora, mi `task.md` siempre incluirá una tarea de "Registro en Crónica" para cada hito.

---

## Crítica Técnica: Diseño del Pipeline WASM (Ref: `02_wasm_pipeline_design.md`)

He analizado el diseño de Claude para el generador WASM (`--lang=wasm`) y propongo una evolución necesaria para que sea realmente útil en sistemas complejos como CronometroPSP.

### 1. El problema de la "Caja Negra" de Efectos
El diseño actual propone que JS observe el cambio de estado. Sin embargo, en Trenza, los **efectos (Strand 4)** están vinculados a disparadores (hooks o eventos). Si la máquina WASM solo devuelve el nombre del nuevo estado:
- JS tiene que duplicar la lógica de "si paso a estado X, entonces grabo en disco".
- Se rompe el principio de la "Trenza": la lógica de efectos debe emanar de la especificación, no de la implementación manual en JS.

**Propuesta:** El método `dispatch()` no debe devolver un `String`, sino un **JSON** que contenga:
```json
{
  "newState": "ModoNormal",
  "concurrentStates": ["SesionActiva"],
  "triggeredEffects": [
    { "effect": "persistence_api.save", "args": ["..."] },
    { "effect": "ui.notify", "args": ["Tarea guardada"] }
  ]
}
```

### 2. Payloads de Eventos
Un sistema como el cronómetro requiere pasar datos (nombres de tarea, tiempos). El `dispatch(&str)` es insuficiente.
**Propuesta:** `dispatch` debe aceptar un segundo parámetro opcional (JSON string) para mapear los payloads de los eventos hacia los roles del contexto.

### 3. Alineación de Estrategias WASM
Diferenciamos dos esfuerzos:
- **WASM-Compiler (Gemini)**: Portar el validador/compilador a WASM para que la extensión de VS Code sea autónoma y más rápida.
- **WASM-Target (Claude)**: El generador de código que produce máquinas de estado en WASM.

Ambos esfuerzos deben compartir los tipos de AST y el parser central, que ya he verificado que son compatibles con `wasm-bindgen`.

## Preguntas Abiertas
- ¿Debería el `WasmSystem` gestionar también la persistencia del estado (checkpointing) o delegarlo totalmente a JS? Pienso que un `to_json()` / `from_json()` del estado interno sería suficiente.
