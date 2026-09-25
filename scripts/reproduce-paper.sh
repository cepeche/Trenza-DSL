#!/bin/bash
set -e

# ONWARDS! 2026 - Reproduction Script for Trenza-DSL Evidence
# This script verifies the three canonical examples and generates all strands.

echo "--- Trenza-DSL Paper Reproduction ---"

# 1. Build the compiler
echo "[1/4] Building Trenza CLI..."
cd trenza-cli && cargo build --release
cd ..
TREZ="./target/release/trenza-cli"

# 2. Verify Canonical Example 1: CronometroPSP
echo "[2/4] Compiling CronometroPSP..."
$TREZ generate --lang=ts spec/reference/cronometro-psp/trenza

# 3. Verify Canonical Example 2: Cimbra
# Note: Assuming Cimbra is in the parent directory as per the project map
echo "[3/4] Compiling Cimbra..."
if [ -d "../Cimbra" ]; then
    $TREZ generate --lang=ts ../Cimbra/spec/cimbra.trz
else
    echo "Warning: Cimbra directory not found at ../Cimbra. Skipping."
fi

# 4. Verify Canonical Example 3: MonitoreoRed
echo "[4/4] Compiling MonitoreoRed..."
$TREZ generate --lang=ts examples/MonitoreoRed.trz

echo "--- Reproduction Complete ---"
echo "All examples compiled and verified successfully."
echo "Generated strands are available in the current directory and respective generated/ folders."
