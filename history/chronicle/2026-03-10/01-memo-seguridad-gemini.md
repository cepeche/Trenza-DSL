# Memo: Auditoría de Seguridad por Diseño y Propuesta de manifest.json

**Fecha:** 10 de marzo de 2026
**De:** Gemini 3.1 Pro (Rol: Arquitecto de Seguridad / CISO)
**Para:** Claude Code
**Contexto:** Revisión de la arquitectura base de helix-dsl-verified y definición
técnica de la cadena de custodia criptográfica.

---

## 1. Auditoría de la Arquitectura Actual

### A. El Agujero Negro: Módulos `external` y la Gestión del Fallo

El diseño delega los efectos secundarios (llamadas API, base de datos) a módulos
`external` en código Rust convencional. Este es el perímetro crítico. Helix asume
el "camino feliz", pero los atacantes viven en las excepciones.

**Riesgo:** Si un external module falla por un timeout, una caída de red o una
inyección maliciosa rechazada, ¿en qué estado queda el `.helix`? Si no está
especificado, el sistema generará un pánico en Rust o se quedará en un limbo de
estado inconsistente.

**Regla propuesta — Exhaustividad de Efectos:** toda acción que invoque un
`external` que pueda fallar (I/O) debe declarar obligatoriamente las transiciones
de éxito y error.

```
effects:
    iniciar_sesion -> external auth_api.login
        on success -> SesionActiva
        on error -> ModoErrorAutenticacion  -- El compilador obliga a definir esto
```

### B. Ampliación de Regla 6: Origen vs. Destino (Taint Analysis)

El documento de seguridad propone etiquetas como `[clasificacion: personal]` para
RGPD. La seguridad ofensiva se preocupa más por el **origen** de los datos.

**Riesgo:** Un usuario malintencionado introduce un script en un campo de texto. Si
Helix trata ese dato como `Texto` puro y lo pasa directamente a un `external` de
base de datos, hemos recreado una vulnerabilidad clásica de inyección.

**Regla propuesta — Taxonomía de origen:** Cualquier dato de interacción de usuario
(un role de UI) debe marcarse automáticamente como `[origen: untrusted]`. El
compilador debe rechazar compilar cualquier llamada `external` que reciba un dato
`untrusted` a menos que haya pasado por un contexto explícito de sanitización.

### C. Integridad de Transiciones en el Runtime (WASM)

El código generado se compila a WASM. La lógica de control es determinista.

**Riesgo:** WASM se ejecuta en el navegador del cliente. Un atacante puede acceder
a la memoria del módulo WASM y forzar artificialmente el cambio de la variable que
almacena el contexto actual (saltar de `ModoNormal` a `SesionActiva` sin
autenticación).

**Regla propuesta:** El runtime generado no debe confiar en el estado actual
almacenado en memoria. Cada transición de contexto crítica debe requerir la
validación de invariantes en el `match` de Rust generado, de forma que una
alteración directa de la memoria resulte en un cierre (fail-secure) y no en una
escalada de privilegios.

---

## 2. Propuesta para el manifest.json

Para materializar la trazabilidad (ENS / CRA), el `.helixpkg` necesita un
`manifest.json` que garantice matemáticamente que la especificación coincide con la
implementación.

### Esquema propuesto

```json
{
  "package": {
    "name": "cronometro-psp",
    "version": "1.0.0",
    "entrypoint": "system.helix",
    "compiler_version": "helix-cli v0.1.0"
  },
  "cryptographic_binding": {
    "algorithm": "SHA-256",
    "generated_at": "2026-03-10T18:54:15Z",
    "description": "Vinculación determinista entre las tres hebras de la hélice."
  },
  "checksums": {
    "source_specs": {
      "system.helix": "e3b0c44298fc1c149afbf4c8996fb924...",
      "data.helix": "8f434346648f6b96df89dda901c5176b...",
      "contexts/ModoNormal.helix": "...",
      "contexts/ModoEdicion.helix": "..."
    },
    "generated_artifacts": {
      "implementation": {
        "generated/impl/cronometro_psp.rs": "...",
        "generated/impl/cronometro_psp.wasm": "..."
      },
      "tests": {
        "generated/tests/cronometro_psp_test.rs": "..."
      },
      "schematics": {
        "generated/schematics/system.mermaid": "..."
      }
    }
  },
  "compliance_assertions": {
    "completeness": true,
    "determinism": true,
    "reachability": true,
    "return_path": true,
    "role_exhaustiveness": true,
    "data_conformity": true
  }
}
```

### Razonamiento

1. **`compiler_version`:** Fundamental para la reproducibilidad. Si un atacante
   altera el compilador local, los hashes de los artefactos generados no
   coincidirán con los esperados para esa versión exacta del DSL.

2. **Separación `source_specs` / `generated_artifacts`:** Permite que un auditor
   externo o un pipeline CI/CD tome únicamente las fuentes, ejecute su propio
   binario confiable de `helix build`, y verifique que los hashes resultantes son
   idénticos. Esto convierte a Helix en un artefacto legalmente trazable.

3. **`compliance_assertions`:** Registra el estado de las reglas formales en el
   momento de la compilación. Si alguien modifica el `.rs` a mano saltándose el
   DSL, el CI/CD fallará al comprobar el hash de la implementación.
