#!/usr/bin/env bash
# Benchmark reproducible del verificador (paper Onward! 2027, §Validation).
# Registra el entorno y ejecuta trenza-core/examples/bench_verify.rs.
set -euo pipefail
cd "$(dirname "$0")/.."
echo "## Entorno"
echo "- fecha: $(date -u +%Y-%m-%dT%H:%MZ)"
echo "- commit: $(git rev-parse --short HEAD)"
echo "- rustc: $(rustc --version)"
echo "- SO: $(uname -srm)"
echo "- CPU: $( (lscpu 2>/dev/null | sed -n 's/^Model name:\s*//p') || sysctl -n machdep.cpu.brand_string 2>/dev/null || echo desconocida)"
echo "- núcleos lógicos: $(getconf _NPROCESSORS_ONLN 2>/dev/null || echo ?)"
echo
echo "## Resultados"
cargo run --quiet --release -p trenza-core --example bench_verify
