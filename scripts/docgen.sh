#!/usr/bin/env bash
# docgen.sh — Pipeline de generacion y verificacion de artefactos Trenza
#
# Genera codigo Rust y TypeScript a partir de las specs de referencia,
# luego valida los .ts generados con tsc --noEmit --strict.
#
# Uso:
#   bash scripts/docgen.sh          (desde la raiz del proyecto)
#   SKIP_TSC=1 bash scripts/docgen.sh   (omitir verificacion tsc)
#
# Requisitos:
#   - cargo (Rust toolchain)
#   - tsc (TypeScript compiler, global)

set -euo pipefail

# --- Configuracion -----------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

CLI_BIN="trenza-cli/target/release/trenza-cli"
# En Windows, el binario lleva .exe
if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* || "$(uname -s)" == CYGWIN* ]]; then
    CLI_BIN="${CLI_BIN}.exe"
fi

CRONOMETRO_SRC="spec/reference/cronometro-psp/trenza"
CRONOMETRO_OUT="spec/reference/cronometro-psp/generated"
CRONOMETRO_WASM_OUT="spec/reference/cronometro-psp/generated/wasm"

CLI_SRC="spec/reference/trenza-cli.trz"
CLI_OUT="spec/reference/trenza-cli/generated"

ERRORS=0
SUMMARY=""

# --- Funciones auxiliares ----------------------------------------------------

step_ok() {
    SUMMARY="${SUMMARY}\n  OK  $1"
}

step_fail() {
    SUMMARY="${SUMMARY}\n  FAIL  $1"
    ERRORS=$((ERRORS + 1))
}

run_step() {
    local label="$1"
    shift
    if "$@" ; then
        step_ok "$label"
    else
        step_fail "$label"
    fi
}

# --- 1. Compilar trenza-cli --------------------------------------------------

echo "=== [1/5] Compilando trenza-cli (cargo build --release) ==="
run_step "cargo build --release" cargo build --release --manifest-path trenza-cli/Cargo.toml

# --- 2. Generar artefactos CronometroPSP ------------------------------------

echo "=== [2/5] Generando CronometroPSP (Rust) ==="
run_step "CronometroPSP -> Rust" \
    ./$CLI_BIN generate --lang=rust --out-dir="$CRONOMETRO_OUT" "$CRONOMETRO_SRC"

echo "=== [2/5] Generando CronometroPSP (TypeScript) ==="
run_step "CronometroPSP -> TS" \
    ./$CLI_BIN generate --lang=ts --out-dir="$CRONOMETRO_OUT" "$CRONOMETRO_SRC"

echo "=== [2/5] Generando CronometroPSP (WASM/Rust) ==="
mkdir -p "$CRONOMETRO_WASM_OUT"
run_step "CronometroPSP -> WASM" \
    ./$CLI_BIN generate --lang=wasm --out-dir="$CRONOMETRO_WASM_OUT" "$CRONOMETRO_SRC"
# Nota: la compilación completa a .wasm requiere wasm-pack (ver scripts/build-wasm.sh)

# --- 3. Generar artefactos CLI_Trenza ----------------------------------------

echo "=== [3/5] Generando CLI_Trenza (Rust) ==="
run_step "CLI_Trenza -> Rust" \
    ./$CLI_BIN generate --lang=rust --out-dir="$CLI_OUT" "$CLI_SRC"

echo "=== [3/5] Generando CLI_Trenza (TypeScript) ==="
run_step "CLI_Trenza -> TS" \
    ./$CLI_BIN generate --lang=ts --out-dir="$CLI_OUT" "$CLI_SRC"

# --- 4. Verificacion TypeScript con tsc --------------------------------------

if [[ "${SKIP_TSC:-}" == "1" ]]; then
    echo "=== [4/5] tsc: OMITIDO (SKIP_TSC=1) ==="
    step_ok "tsc (omitido por SKIP_TSC=1)"
else
    echo "=== [4/5] Verificando .ts generados con tsc --noEmit --strict ==="

    TS_FILES=(
        "$CRONOMETRO_OUT/CronometroPSP_out.ts"
        "$CLI_OUT/CLI_Trenza_out.ts"
    )

    TSC_OK=true
    for ts_file in "${TS_FILES[@]}"; do
        if [[ ! -f "$ts_file" ]]; then
            step_fail "tsc: archivo no encontrado: $ts_file"
            TSC_OK=false
            continue
        fi

        echo "  tsc --noEmit --strict $ts_file"
        if npx tsc --noEmit --strict --target ES2020 --moduleResolution bundler "$ts_file" ; then
            step_ok "tsc $ts_file"
        else
            step_fail "tsc $ts_file"
            TSC_OK=false
        fi
    done

    if $TSC_OK; then
        echo "  Todos los .ts pasaron tsc."
    fi
fi

# --- 5. Resumen --------------------------------------------------------------

echo ""
echo "=== [5/5] Resumen ==="
echo -e "$SUMMARY"
echo ""

if [[ $ERRORS -gt 0 ]]; then
    echo "RESULTADO: $ERRORS error(es). Revisa los pasos marcados con FAIL."
    exit 1
else
    echo "RESULTADO: Todo correcto."
    exit 0
fi
