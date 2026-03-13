# Plan de Trabajo Semanal: Trenza DSL (Semana 3 de Marzo)

**Fecha**: 13 de marzo de 2026
**Autor**: Antigravity, en consulta con el Desarrollador Principal.

Dado que nuestro Desarrollador Principal estará fuera de la oficina hasta mediados de la semana que viene por visita familiar (¡una ingeniera de QA de Scopely! Se explica todo el rigor metodológico), dejamos estructurado el plan de ataque para los agentes autónomos.

El objetivo de esta semana es **cerrar la etapa de prototipado** y sentar las bases de la **herramienta final en Rust**, rotando el liderazgo según nuestras fortalezas.

---

## 📅 Fase 1: El Salto a Rust (Lidera Claude Code)
**Objetivo**: Reescribir el motor del parser y verificador en código nativo (Rust).
**Responsable**: Claude Code. Tú diseñas la arquitectura de software; yo audito.

*   **Día 1-2**:
    *   **Lectura del AST**: Mapear las estructuras de datos de `src/trenza/ast.py` a `structs` e `enums` en Rust (`trenza-core`).
    *   **Iniciación del Parser (pest / nom)**: Claude decidirá si escribimos un parser a mano recursivo (como en Python) o usamos una gramática formal (`pest`). Recomendación de QA: usa `pest`, es más robusto a largo plazo.
    *   **Integración de Reglas de Herencia**: Transponer la lógica de propagación de eventos y reglas estrictas de sobreescritura (Reglas H1-H3) al motor de resolución de Rust.

*   **Output esperado**: Un CLI binario en Rust (`trenza verify`) que sea capaz de tragar el directorio `examples/cronometro-psp/trenza` y producir exactamente el mismo JSON de salida que la versión actual de Python.

---

## 📅 Fase 2: Trenza Doc-Gen Nativo (Lidera Antigravity)
**Objetivo**: Portar la automatización de la documentación Markdown al nuevo motor.
**Responsable**: Antigravity. Yo pico los detalles de implementación; Claude audita.

*   **Día 3**:
    *   **Generador Markdown en Rust**: Tomaré el relevo sobre el código de Claude. Escribiré el módulo de exportación de AST a Markdown (traduciendo lo que hice en `docgen.py`).
    *   **Integración CLI (`trenza inspect`)**: Integraré este módulo en el binario de Rust bajo el comando `trenza inspect`, garantizando que reescriba el árbol virtual de `docs/sistema/` de igual forma.
    *   **Refinamiento de Mermaid (QA)**: Aseguraré que sigo sanitizando los nodos rebeldes (`[cerrar]`, etc.) tal y como aprendimos a fuego esta semana.

*   **Output esperado**: El binario CLI genera la estructura de carpetas de documentación sin depender de Python.

---

## 📅 Fase 3: Revisión Cruzada e Integración (Pair Programming)
**Objetivo**: Sincronización final antes del regreso del Desarrollador Principal.
**Responsable**: Ambos.

*   **Día 4**:
    *   Claude revisa mis PRs de `trenza inspect` buscando ineficiencias de memoria o desalineamiento arquitectónico.
    *   Yo ejecuto la batería de QA contra el `trenza verify` de Claude, bombardeándole con casos límite en el `.trz` (roles duplicados, dependencias circulares, transiciones fantasma) para comprobar que el parser de Rust responde con mensajes de error humanos e inteligibles, al estilo Elm/Rust clásico.

---

¡Nos vemos el miércoles para revisar los `panic!` de Rust!
