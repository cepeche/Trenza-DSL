# FILES_FOR_AGENTS.md — Trenza-DSL

Mapa del proyecto para agentes IA. Leer durante la Fase 0 (Inicialización).
El protocolo de coordinación está en `AGENTS.md`.

---

## Proyecto

**Nombre**: Trenza-DSL
**Propósito**: Compilador y ecosistema del lenguaje Trenza-DSL para la especificación
formal de sistemas de información como grafos de propiedades verificables.

**Analogía**: `rustc` es a `cargo` lo que `trenza-cli` es a Cimbra.
Trenza-DSL contiene el compilador; Cimbra es la herramienta de construcción.

**Repo de la herramienta de construcción**: `C:/Proyectos/Cimbra/`
Si un cambio en el compilador es necesario para que Cimbra funcione, coordinar con el
agente responsable de Cimbra y documentar la dependencia cruzada en la crónica.

---

## Jerarquía de Autoridad (rutas concretas)

| Nivel | Documento | Ruta |
|-------|-----------|------|
| 1 | Instrucciones del humano | — (conversación directa) |
| 2 | Instrucciones de modelo | `CLAUDE.md` (Claude) · configuración Antigravity (Gemini) |
| 3 | Protocolo compartido | `AGENTS.md` |
| 4 | Decisiones arquitectónicas | `history/decisions/README.md` + `ADR-*.md` |
| 5 | Crónicas | `history/chronicle/YYYY-MM-DD/` |
| 5 | Especificación de referencia | `spec/reference/` |

---

## Directorios clave

| Directorio | Contenido |
|------------|-----------|
| `src/trenza/` | Compilador Python (parser, verifier, docgen, mermaid) |
| `trenza-cli/` | CLI Rust — punto de entrada principal (`trenza-cli generate`) |
| `trenza-cli/src/` | `main.rs`, `ts_output.rs` (generador TypeScript) |
| `trenza-core/` | Crate Rust con la lógica central |
| `trenza-coord/` | Crate Rust para coordinación MCP y persistencia |
| `trenza-msg/` | Crate Rust para mensajería |
| `spec/reference/` | Especificaciones de referencia (cronómetro, trenza-cli.trz) |
| `spec/reference/cronometro-psp/` | Sistema de referencia completo — no modificar |
| `docs/design/` | Documentos de diseño (strands, seguridad, lifecycle, Strand 5) |
| `history/decisions/` | ADRs formalizados |
| `history/chronicle/` | Crónicas de sesión |
| `editors/` | Integraciones con editores (VS Code syntax highlighting, etc.) |
| `examples/` | Ejemplos de uso del DSL |
| `dist/` | Artefactos de distribución — no editar a mano |
| `target/` | Build de Rust — no commitear |

---

## Archivos generados (no editar manualmente)

| Archivo | Generado por |
|---------|-------------|
| `dist/` | Pipeline de build |
| `target/` | `cargo build` |
| `trenza-cli/CronometroPSP_out.*` | `trenza-cli generate` sobre spec de referencia |
| `trenza-ast.json`, `trenza-ast.mermaid` | Herramientas de análisis |
| Cualquier `*_out.ts`, `*_out.rs`, `*_out.mermaid`, `*_out_audit.md` | `trenza-cli generate` |

Si hay un error en el código generado, **arreglar el generador** en
`trenza-cli/src/ts_output.rs` o `src/trenza/`, no el archivo generado.

---

## Archivos de instrucción (no editar cross-agent)

| Archivo | Propietario |
|---------|-------------|
| `CLAUDE.md` | Claude (CL/CO) |
| Configuración Antigravity | Gemini (GE) |

Para proponer cambios en un archivo de instrucción ajeno: documentar en crónica
y solicitar al humano que coordine con el agente propietario.

---

## Comandos de build y test

```bash
# Compilar el CLI Rust
cd trenza-cli && cargo build --release

# Ejecutar tests Rust
cargo test                          # todos los crates
cd trenza-cli && cargo test         # solo el CLI

# Compilar una especificación con el compilador Python
python -m trenza.cli generate --lang=ts spec/reference/cronometro-psp/trenza/system.trz

# Compilar con el CLI Rust (cuando esté disponible)
./target/release/trenza-cli generate --lang=ts <archivo.trz>

# Validación completa antes de push
cargo build && cargo test
```

---

## Gestión de artefactos de compilación

Tras una compilación `--release` exitosa, el agente **debe** ejecutar el siguiente
procedimiento antes de cerrar sesión:

```powershell
cargo build --release
if ($LASTEXITCODE -eq 0) {
    Copy-Item target\release\trenza-cli.exe bin\trenza-cli.exe
    cargo clean
}
```

**Motivo**: el directorio `target/` puede contener más de 100.000 archivos intermedios
de Rust. Tenerlos en disco mientras el proyecto está abierto como workspace en el IDE
satura el canal IPC del agente y causa crashes. Mantener solo el ejecutable en `bin/`
resuelve el problema sin perder el resultado de la build.

> `bin/` es un directorio **local** para preservar el ejecutable tras `cargo clean`.
> Está en `.gitignore` y **no debe commitarse**. Git no está diseñado para distribuir
> binarios de plataforma. Para releases formales, usar GitHub Releases con un tag versionado.

---

## Strands del compilador

El compilador genera los siguientes strands para cada especificación `.trz`:

| Strand | Tipo | Extensión de salida |
|--------|------|---------------------|
| Strand 1 | Implementación TypeScript | `*_out.ts` |
| Strand 2 | Tests algebraicos | `*_out.test.ts` |
| Strand 3 | Topología Mermaid | `*_out.mermaid` |
| Strand 4 | Auditoría / requisitos | `*_out_audit.md` |
| Strand 5 | Grafo GQL (en diseño) | `*_graph.json` + `*_graph.gql` |

El diseño de Strand 5 está documentado en `docs/design/strand5-gql-property-graph.md`.

---

## Documentos de diseño relevantes

| Documento | Contenido |
|-----------|-----------|
| `docs/design/strand5-gql-property-graph.md` | Diseño de Strand 5: GQL, JSON-Graph, event sourcing |
| `docs/design/lifecycle.md` | Ciclo de vida de sistemas Trenza |
| `docs/design/security-by-design.md` | Seguridad por diseño |
| `docs/design/dci-influences.md` | Influencias DCI en el modelo de roles |
| `history/meta/metafisica-de-trenza.md` | Fundamentos filosóficos del DSL |
| `history/meta/directrices-pi-ia.md` | Directrices de propiedad intelectual |
