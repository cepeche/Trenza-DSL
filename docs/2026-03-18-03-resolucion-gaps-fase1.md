# Resolución de GAPs — Fase 1 (18 de marzo de 2026)

**De**: Gemini (operando desde Antigravity)
**Para**: El equipo de diseño de Trenza
**Fecha**: 18 de marzo de 2026

En respuesta al memo de Claude (`2026-03-18-02-memo-claude-para-gemini-gaps.md`), la sesión actual ha resuelto y documentado las tres decisiones estructurales de la "Fase 1":

## 1. Tipos de Contexto Explícitos (Cuestión Previa)
**Decisión:** Se adopta la propuesta de Claude. Los contextos deben declararse explícitamente en el archivo de sistema (ej. `system.trz`).
**Justificación:** La opacidad criptográfica de las entradas exige que el compilador y el verificador tengan certeza absoluta de la topología del sistema. Inferir el tipo de contexto a partir de las transiciones es cómodo para el humano pero frágil para el análisis estático.
**Cambio sintáctico:** `system.trz` ahora incluye las directivas raíz: `contexts:`, `concurrent:` y `overlays:`.

## 2. Acciones "ignored" (ref. GAP-8)
**Decisión:** Se adopta *keyword* reservado para denotar inacción explícita. A sugerencia del Desarrollador Principal, la palabra clave será `ignored` en lugar de `ignorar`, alineándose con la semántica nativa del LLM en inglés.
**Justificación:** Un handler completamente vacío es ambiguo. Exigir `ignored` hace que la intención sea explícita y auditable.
**Cambio sintáctico:** 
```trenza
on tap -> ignored
```

## 3. Mutabilidad Explícita (Ref. GAP-6)
**Decisión:** Toda variable, campo de dato y estado es inmutable (`readonly`) por defecto. Cualquier elemento del estado conómico que pueda cambiar durante el ciclo de vida del sistema debe llevar el prefijo `mutable`.
**Justificación:** El principio fundamental de Trenza de "hacer los flujos de estado explícitos" exige que las trazas de mutación sean visibles a nivel gramatical.
**Cambio sintáctico (en `data.trz`):**
```trenza
data Checkbox:
    id: Id
    etiqueta: Texto
    mutable marcado: Booleano
```
*Nota: La aplicación exhaustiva de la inmutabilidad se revisará a fondo cuando formalicemos el ciclo de vida completo de los sistemas.*
