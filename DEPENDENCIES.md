# Trenza-DSL — Entorno y Dependencias

Este repositorio aloja el compilador y la especificación del lenguaje Trenza-DSL, y requiere las siguientes herramientas para su desarrollo y coordinación multi-agente.

## Core Infrastructure

| Dependencia | Versión | Propósito |
| :--- | :--- | :--- |
| **PowerShell** | **7.0+** | **Requerido** para scripts de automatización multiplataforma (uso de `&&`, `||`). |
| **Rust** | 2024 Edition | El compilador nativo de Rust (rustc / cargo). |
| **Node.js** | 20+ (LTS) | Soporte para `trenza-ts` y compilación WASM. |
| **SurrealDB** | 2.1+ | (Opcional) Persistencia de grafos (Strand 5). |

## Desarrollo y Compilación

- **Cargo**: Gestor de dependencias de Rust.
- **Vite**: Para la visualización de reportes locales.
- **Tauri/WASM**: (Según el target de despliegue).

## Instalación de PowerShell 7
```powershell
winget install --id Microsoft.PowerShell --source winget
```
