# Categorización de Requisitos en Trenza-DSL

## Visión General
A medida que el compilador `trenza-cli` evoluciona hacia un entorno de desarrollo integrado (IDE/LSP), es crucial distinguir entre los requisitos **esenciales** (lógica del lenguaje, generación de código, verificación semántica) y los requisitos **de herramientas/metadatos** (posiciones en el AST para LSP, formatos de salida JSON, telemetría).

## Propuesta de Clasificación

### 1. Requisitos de Núcleo (Essential)
- Gramática del lenguaje (`trenza.pest`).
- Generación de Código (Strand 1 - Rust/TypeScript).
- Verificación Semántica (Reglas 1-8).
- Consistencia del sistema y alcanzabilidad.

### 2. Requisitos de Herramientas y Metadatos (Tooling)
- Soporte LSP: Captura de `Pos` (línea/columna) en el AST.
- Diagnósticos Estructurados: Formato JSON para consumo por externos.
- Visualización: Generación de Mermaid y reportes HTML.

## Implicaciones para el Futuro
- **Especificaciones**: Las especificaciones del compilador deberían ser capaces de marcar ciertos campos o comportamientos como "opcionales" o "metadata" para que el núcleo del compilador pueda ser verificado de forma pura sin dependencias de herramientas.
- **Validación**: La suite de validación `docgen.sh` debe ser el guardián de que los requisitos de "herramientas" no rompan los requisitos de "núcleo".

> [!NOTE]
> Esta categorización ayuda a mantener el principio de "Pura Lógica" de Trenza, incluso cuando el ecosistema de herramientas crece.
