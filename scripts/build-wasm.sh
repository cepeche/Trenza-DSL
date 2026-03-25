#!/usr/bin/env bash
# build-wasm.sh — Compila una spec Trenza a WebAssembly via wasm-pack
#
# Uso:
#   bash scripts/build-wasm.sh <spec_path> <system_name> [out_dir]
#
# Ejemplos:
#   bash scripts/build-wasm.sh spec/reference/cronometro-psp/trenza CronometroPSP
#   bash scripts/build-wasm.sh spec/reference/cronometro-psp/trenza CronometroPSP dist/wasm
#
# Requisitos:
#   - trenza-cli compilado (cargo build --release en trenza-cli/)
#   - wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)
#   - Rust target wasm32-unknown-unknown (rustup target add wasm32-unknown-unknown)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# --- Argumentos --------------------------------------------------------------

if [[ $# -lt 2 ]]; then
    echo "Uso: bash scripts/build-wasm.sh <spec_path> <system_name> [out_dir]"
    echo "  spec_path:   ruta al .trz o directorio con .trz"
    echo "  system_name: nombre del sistema (ej. CronometroPSP)"
    echo "  out_dir:     directorio de salida (default: dist/wasm)"
    exit 1
fi

SPEC_PATH="$1"
SYSTEM_NAME="$2"
WASM_OUT="${3:-dist/wasm}"

CLI_BIN="trenza-cli/target/release/trenza-cli"
if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* || "$(uname -s)" == CYGWIN* ]]; then
    CLI_BIN="${CLI_BIN}.exe"
fi

WASM_CRATE="build/wasm-crate"

# --- Verificar herramientas --------------------------------------------------

echo "=== Verificando herramientas ==="

if [[ ! -f "$CLI_BIN" ]]; then
    echo "❌ trenza-cli no encontrado en $CLI_BIN"
    echo "   Ejecuta: cargo build --release --manifest-path trenza-cli/Cargo.toml"
    exit 1
fi
echo "  ✅ trenza-cli: $CLI_BIN"

if ! command -v wasm-pack &>/dev/null; then
    echo "❌ wasm-pack no encontrado en PATH"
    echo "   Instala con: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    echo "   O en Windows: cargo install wasm-pack"
    exit 1
fi
echo "  ✅ wasm-pack: $(wasm-pack --version)"

if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "⚠️  Target wasm32-unknown-unknown no instalado. Instalando..."
    rustup target add wasm32-unknown-unknown
fi
echo "  ✅ target wasm32-unknown-unknown"

# --- Paso 1: Generar Rust WASM desde la spec ---------------------------------

echo ""
echo "=== [1/4] Generando Rust WASM desde spec ==="

mkdir -p "$WASM_CRATE/src"

./"$CLI_BIN" generate --lang=wasm --out-dir="$WASM_CRATE/src" "$SPEC_PATH"

# El compilador genera <SystemName>_out.rs — wasm-pack necesita lib.rs
GENERATED_RS="$WASM_CRATE/src/${SYSTEM_NAME}_out.rs"
if [[ ! -f "$GENERATED_RS" ]]; then
    echo "❌ Archivo generado no encontrado: $GENERATED_RS"
    exit 1
fi

cp "$GENERATED_RS" "$WASM_CRATE/src/lib.rs"
echo "  ✅ Rust WASM generado: $WASM_CRATE/src/lib.rs"

# --- Paso 2: Crear Cargo.toml para el crate WASM ----------------------------

echo ""
echo "=== [2/4] Creando Cargo.toml del crate WASM ==="

# Nombre de crate: PascalCase → kebab-case (ej. CronometroPSP → cronometro-psp)
CRATE_NAME=$(echo "$SYSTEM_NAME" | sed 's/\([A-Z]\)/-\1/g' | sed 's/^-//' | tr '[:upper:]' '[:lower:]')

cat > "$WASM_CRATE/Cargo.toml" << EOF
[package]
name = "${CRATE_NAME}-wasm"
version = "0.1.0"
edition = "2021"
description = "Auto-generado por Trenza DSL Compiler para ${SYSTEM_NAME}"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
EOF

echo "  ✅ Cargo.toml: ${CRATE_NAME}-wasm v0.1.0"

# --- Paso 3: wasm-pack build -------------------------------------------------

echo ""
echo "=== [3/4] Compilando a WASM con wasm-pack ==="

mkdir -p "$WASM_OUT"

wasm-pack build "$WASM_CRATE" \
    --target web \
    --out-dir "../../$WASM_OUT" \
    --out-name "$SYSTEM_NAME"

# --- Paso 4: Resumen ---------------------------------------------------------

echo ""
echo "=== [4/4] Outputs ==="
ls -lh "$WASM_OUT/"*.wasm "$WASM_OUT/"*.js 2>/dev/null || true

echo ""
echo "✅ Build WASM completado."
echo "   Archivos en: $WASM_OUT/"
echo ""
echo "   Uso en HTML/JS:"
echo "   import init, { WasmSystem } from './$WASM_OUT/${SYSTEM_NAME}.js';"
echo "   await init();"
echo "   const sys = new WasmSystem();"
echo "   const state = sys.dispatch('EVENTO');"
