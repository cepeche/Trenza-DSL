# Sesión: Entornos APSE y Estrategia de Requisitos No Funcionales (NFRs)

**Fecha:** 2026-03-23
**Participantes:** Desarrollador Humano y Gemini

## Resumen de la Conversación

1. **Paralelismo con Ada y APSE (KAPSE/MAPSE):**
   - Discutimos la historia del entorno de soporte de Ada (APSE) diseñado en los años 80 para el DoD.
   - Analizamos por qué APSE no logró su promesa (limitaciones de hardware, bases de datos masivas como DIANA, y un modelo monolítico frente a la victoria de UNIX y el texto plano).
   - Concluimos que Trenza está en una posición histórica única para tener éxito donde APSE se atascó: hoy en día, generar las proyecciones (hebras) desde texto plano (`.trz`) en tiempo real no cuesta nada. Nuestra "base de datos" es Git y efímeros ASTs en memoria, lo que permite aprovechar a los LLMs para operar sobre la complejidad.

2. **Requisitos No Funcionales (NFRs) e Instrumentación:**
   - Surgió la necesidad de definir cómo Trenza debe manejar los NFRs (auditoría, telemetría y logs).
   - Acordamos formalmente que el compilador inyectará transversalmente la instrumentación.
   - Se definió una primera estrategia basada en entornos:
     - `--profile=pre` (Pre-producción / Desarrollo): Telemetría implícita masiva inyectada ciegamente por el compilador (volcando estado, eventos y payloads).
     - `--profile=pro` (Producción): Código hiper-optimizado sin telemetría implícita. Sin embargo, para auditoría de dominio ("Logs legales obligatorios"), se generarán únicamente los logs de acciones explícitamente marcadas en el `.trz` con algo como `@audit("Motivo Legal")`.

3. **Próximos Pasos Técnicos:**
   - Iniciar el desarrollo del compilador / CLI en **Rust**. Esto servirá como inicio formal del entorno KAPSE de Trenza en un lenguaje altamente eficiente y concurrente.
   - El objetivo fundacional es lograr compilar el equivalente en Trenza al mítico programa `IEFBR14` (el *dummy* de OS/360 que no hace nada y finaliza exitosamente) para probar el ciclo de vida completo del parser.
