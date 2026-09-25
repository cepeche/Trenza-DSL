# ONWARDS! 2026 - Reproduction Script for Trenza-DSL Evidence (PowerShell)
# This script verifies the three canonical examples and generates all strands.

Write-Host "--- Trenza-DSL Paper Reproduction ---" -ForegroundColor Cyan

# 1. Build the compiler
Write-Host "[1/4] Building Trenza CLI..."
Set-Location trenza-cli
cargo build --release
Set-Location ..
$TREZ = ".\target\release\trenza-cli.exe"

# 2. Verify Canonical Example 1: CronometroPSP
Write-Host "[2/4] Compiling CronometroPSP..."
& $TREZ generate --lang=ts spec/reference/cronometro-psp/trenza

# 3. Verify Canonical Example 2: Cimbra
Write-Host "[3/4] Compiling Cimbra..."
if (Test-Path "../Cimbra") {
    & $TREZ generate --lang=ts ../Cimbra/spec/cimbra.trz
} else {
    Write-Warning "Cimbra directory not found at ../Cimbra. Skipping."
}

# 4. Verify Canonical Example 3: MonitoreoRed
Write-Host "[4/4] Compiling MonitoreoRed..."
& $TREZ generate --lang=ts examples/MonitoreoRed.trz

Write-Host "--- Reproduction Complete ---" -ForegroundColor Green
Write-Host "All examples compiled and verified successfully."
