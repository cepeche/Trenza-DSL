> Nota histórica: Trenza se llamó "Helix" hasta marzo de 2026. Los memos anteriores al renombramiento usan el nombre original. Ver `history/decisions/ADR-004-helix-to-trenza-rename.md` para la justificación del cambio.

# DCI y Reenskaug en Trenza

Data, Context, and Interaction (DCI) es el manifiesto arquitectónico propuesto por Trygve Reenskaug que separa tajantemente la estructura central de dominio (Data) de los flujos de casos de uso situacionales (Context) y el comportamiento dinámico (Interaction interactivo).

## Adaptación en Trenza
Trenza incorpora a fuego la arquitectura DCI puro, resolviendo el clásico infierno de la Herencia en Diamante.
En Trenza, los datos (`data`) se disocian del comportamiento en todo momento. Un dato solo es inyectado en un Contexto asumiendo un Rol fugaz. Además, se añade la flexibilidad de los `slots` que un `Context` puede requerir rellenar a nivel topológico (ADR-007).
