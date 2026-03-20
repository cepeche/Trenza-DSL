> Nota histórica: Trenza se llamó "Helix" hasta marzo de 2026. Los memos anteriores al renombramiento usan el nombre original. Ver `history/decisions/ADR-004-helix-to-trenza-rename.md` para la justificación del cambio.

# Seguridad por Diseño (RGPD Estructural)

Trenza transfiere las responsabilidades de observabilidad del flujo de la información privada a las verificaciones del AST en la capa de compilación.

## Conformidad en Privacidad
Si la arquitectura central anota `[clasificacion: personal]` en una variable, el compilador verificador previene de base la inyección o salida de dicho parámetro hacia ninguna `external action` si este "puerto" objetivo carece de la bandera `[autorizado_para: personal]`.
Esta aproximación garantiza topológicamente que ningún error por obviedad ni refactor imprevisto filtre PII (Personally Identifiable Information).
