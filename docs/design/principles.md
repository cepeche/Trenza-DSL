> Nota histórica: Trenza se llamó "Helix" hasta marzo de 2026. Los memos anteriores al renombramiento usan el nombre original. Ver `history/decisions/ADR-004-helix-to-trenza-rename.md` para la justificación del cambio.

# Los Principios Rectores de Trenza

1. **La Trenza**: La especificación es el lenguaje, no un mero diagrama. Cada requisito se expande estructuralmente en 4 hebras inseparables (implementación, tests, esquemas y requisitos inferidos).
2. **Métodos Factoría**: El control de flujo y los condicionales confusos de una aplicación clásica quedan relegados a factorías formales de tipo State Machine.
3. **Flujos de Estado Explícitos**: Las transiciones son modeladas estáticamente; los "booleanos bandera" y el estado fragmentado quedan extirpados del root space.
4. **Verificabilidad Formal**: Trenza impone reglas nativas legibles (alcanzabilidad, completitud) pero auditables mecánicamente sin requerir un trasfondo doctoral en Notación Z o TLA+.
