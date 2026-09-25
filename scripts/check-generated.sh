#!/usr/bin/env bash
# Compila y ejecuta el Rust generado (Strand 1 + Strand 2) para las
# especificaciones de referencia. Comprueba que el código generado compila
# y que sus tests generados pasan, más un test de comportamiento escrito a
# mano para el listado del paper.
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT=$(pwd)
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

cargo build --quiet --release -p trenza-cli
CLI="$ROOT/target/release/trenza-cli"

check() {  # $1 = spec, $2 = nombre del sistema, $3 = test extra opcional
  local spec=$1 sys=$2 extra=${3:-}
  local crate="$WORK/$sys"
  mkdir -p "$crate/src" "$WORK/out"
  "$CLI" generate --lang=rust --out-dir="$WORK/out" "$spec" > /dev/null
  cat > "$crate/Cargo.toml" <<TOML
[package]
name = "generated_$(echo "$sys" | tr 'A-Z' 'a-z')"
version = "0.0.0"
edition = "2021"
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
[workspace]
TOML
  { cat "$WORK/out/${sys}_out.rs"; echo; cat "$WORK/out/${sys}_out.tests.rs";
    [ -n "$extra" ] && { echo; cat "$extra"; }; } > "$crate/src/lib.rs"
  echo "== $spec"
  (cd "$crate" && CARGO_TARGET_DIR="$WORK/target" cargo test --quiet 2>&1 | grep -E "test result|FAILED|panicked|^error" )
  (cd "$crate" && CARGO_TARGET_DIR="$WORK/target" cargo test --quiet > /dev/null 2>&1)
}

check paper/onward2027/listings/modos.trz Cronometro scripts/generated-check/modos_accion.rs
check examples/cronometro-wasm/src/cronometro_full.trz CronometroPSP
