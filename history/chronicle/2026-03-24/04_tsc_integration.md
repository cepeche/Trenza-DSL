# Integracion de `tsc --noEmit --strict` en el pipeline de generacion

**Fecha:** 2026-03-24
**Autor:** Claude (asistido)

## Motivacion

El compilador Trenza genera codigo TypeScript (`.ts`) a partir de las specs
`.trz`. Hasta ahora, la unica verificacion de los artefactos generados era
visual (revision humana) o funcional (ejecucion manual). Esto deja un hueco:
errores de tipo en el generador TS (`generator.rs`) pueden producir codigo
que compila semanticamente mal pero pasa desapercibido.

**Solucion:** ejecutar `tsc --noEmit --strict` sobre cada `.ts` generado
como paso automatico del pipeline. Esto atrapa:

- Tipos incompatibles generados por el backend TS
- Propiedades faltantes o mal nombradas
- Errores de flujo de control (variables posiblemente undefined, etc.)

## Por que NO se modela el generador como spec Trenza

El generador TS es una **funcion pura** `AST -> String`. No tiene:
- Estados internos que muten
- Transiciones condicionales
- Efectos secundarios observables

Trenza esta disenado para modelar **maquinas de estados con transiciones
explicitas**. Modelar una transformacion pura de arboles como spec Trenza
seria forzar el lenguaje fuera de su dominio natural y no aportaria valor
sobre la verificacion directa con `tsc`.

## Como funciona el nuevo pipeline

El script `scripts/docgen.sh` ejecuta los siguientes pasos en orden:

1. **`cargo build --release`** — compila el compilador Trenza
2. **Genera CronometroPSP** (Rust + TS) desde `spec/reference/cronometro-psp/trenza/`
3. **Genera CLI_Trenza** (Rust + TS) desde `spec/reference/trenza-cli.trz`
4. **`tsc --noEmit --strict`** sobre cada `.ts` generado
5. **Resumen** con indicadores OK/FAIL por cada paso

El script:
- Sale con codigo 1 si cualquier paso falla
- Es idempotente (se puede ejecutar N veces sin efectos acumulativos)
- Soporta `SKIP_TSC=1` para entornos sin TypeScript instalado
- Funciona en Git Bash / MSYS2 en Windows

## Invocacion

Desde la raiz del proyecto:

```bash
bash scripts/docgen.sh
```

O via el cron de Claude Code (configurado en `.claude/settings.local.json`).

## Preguntas abiertas

- **tsconfig.json:** Por ahora pasamos los flags directamente a `tsc`.
  Si el numero de flags crece, convendria crear un `tsconfig.json` minimo
  en la raiz o en `scripts/`.
- **CI remoto:** Este pipeline esta pensado para ejecucion local. Cuando
  haya CI (GitHub Actions), habra que replicar los mismos pasos alli.
- **Generador Rust:** No hay un equivalente a `tsc --noEmit` para Rust
  generado, ya que `rustc` requiere un `Cargo.toml` completo. Queda
  pendiente evaluar si vale la pena generar un crate temporal para
  verificacion.
