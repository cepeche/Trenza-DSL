# Crónica: Resolución de Tipos TS en Compilador Trenza-DSL

**De:** Gemini (GE)
**Para:** Humano y agentes futuros (CL, CO)
**Fecha:** 2026-05-20

---

## 1. Archivos Modificados

| Archivo | Acción |
|---------|--------|
| [primitives.rs](file:///C:/Proyectos/Trenza-DSL/trenza-core/src/primitives.rs) | Modificado (Añadido alias `Numero`, actualizado `is_primitive` para aceptar `Lista`) |
| [generator.rs](file:///C:/Proyectos/Trenza-DSL/trenza-core/src/generator.rs) | Modificado (Mapeos explícitos para raw `List`/`Lista` en `ts_type` y `rust_type`) |
| [CLI_Trenza_out.ts](file:///C:/Proyectos/Trenza-DSL/spec/reference/trenza-cli/generated/CLI_Trenza_out.ts) | Regenerado (Tipos válidos de TS: `any[]` y `number`) |
| [CLI_Trenza_out.rs](file:///C:/Proyectos/Trenza-DSL/spec/reference/trenza-cli/generated/CLI_Trenza_out.rs) | Regenerado (Mapeos válidos a `Vec<String>` y `i32`) |
| [CLI_Trenza_out.tests.rs](file:///C:/Proyectos/Trenza-DSL/spec/reference/trenza-cli/generated/CLI_Trenza_out.tests.rs) | Añadido y renombrado en Git (reemplazando al obsoleto `CLI_Trenza_out_tests.rs`) |

---

## 2. Descripción de Cambios y Motivación

En la sesión anterior se detectó que el test de integración `trenza_cli_ts_pasa_tsc` fallaba debido a que el compilador TypeScript (`tsc --strict`) rechazaba los archivos generados porque contenían los tipos `List` y `Numero`, que no estaban definidos.

* **Alias `Numero`**: Mapeado en la tabla de primitivas `PRIMITIVES` a `i32` para Rust y `number` para TypeScript, reconociéndolo como alias español de `Int`/`Entero`.
* **Colección `List` y `Lista`**: Se incluyeron reglas en `ts_type` y `rust_type` para mapear los tipos contenedores raw/sin parámetros genéricos (e.g. `pares: List` en `trenza-cli.trz`) a `any[]` en TypeScript y `Vec<String>` en Rust.
* **Actualización de tests de referencia**: Se eliminó el archivo obsoleto `CLI_Trenza_out_tests.rs` (con guion bajo) de Git y se añadió el archivo `CLI_Trenza_out.tests.rs` (con punto) que es el formato de salida oficial generado por la CLI.

---

## 3. Verificación y Resultados

* **Workspace Rust Tests**: Se ejecutó `cargo test` de manera exitosa en todo el workspace (incluyendo `tsc --strict` check en TypeScript).
* **Compilación Release y Limpieza**: Se compiló el CLI con `cargo build --release`, se guardó el binario en `bin/trenza-cli.exe` y se realizó `cargo clean` para evitar saturación del IDE (limpiando 780MB de caché temporal).
