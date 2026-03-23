# Self-Hosting Trenza CLI (Dog-fooding)

## Overview
A medida que el compilador y CLI de Trenza madura, la prueba definitiva de la expresividad del DSL (y un hito crucial para cualquier lenguaje formal) es el **Self-hosting** (o "dog-fooding")—la capacidad del compilador para especificarse usando su propio lenguaje.

Dado que una herramienta CLI es, fundamentalmente, una máquina de estados (esperar comandos -> parsear -> validar -> generar -> salir), Trenza es el medio perfecto para orquestar su propio flujo de ejecución.

## Beneficios Arquitectónicos
1. **Integridad Estructural**: Al generar el núcleo de `main.rs` a partir de `trenza-cli.trz`, garantizamos por diseño que el compilador no puede omitir accidentalmente una fase (por ejemplo, generar código sin haber verificado antes las reglas formales). La topología impuesta por Trenza hace que los estados ilegales sean irrepresentables.
2. **Arquitectura Autodocumentada**: Cualquier nuevo colaborador que se una al proyecto del compilador entenderá instantáneamente su arquitectura mirando los diagramas Mermaid autogenerados (Strand 3).
3. **Tests Unitarios Gratis**: El código Rust autogenerado (Strand 2) nos proporcionará una suite de pruebas algebraicas exhaustivas para la máquina de estados interna del CLI, coste cero.
4. **Núcleo Funcional Puro vs Consola Imperativa**: La especificación `trenza-cli.trz` actúa como el orquestador puro. Solo necesitaremos escribir Rust manualmente para las implementaciones de las acciones reales (los `effects` de Trenza), como el Visitor de Pest o las operaciones imperativas de `std::fs::read`.

## La Máquina de Estados del CLI
El diseño propuesto separa el pipeline del compilador en contextos de ciclo de vida explícitos:
- `EsperandoComando`
- `ParseandoArchivo`
- `VerificandoReglas`
- `GenerandoStrands`
- `Exito` / `ErrorFatal` / `MostrarAyuda`

Revisar el archivo `spec/reference/trenza-cli.trz` para leer la especificación estricta.
