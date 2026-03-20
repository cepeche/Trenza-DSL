> Nota histórica: Trenza se llamó "Helix" hasta marzo de 2026. Los memos anteriores al renombramiento usan el nombre original. Ver `history/decisions/ADR-004-helix-to-trenza-rename.md` para la justificación del cambio.

# Ciclo de Vida Generalizado y "Effects"

Trenza desecha los acoplamientos a ecosistemas específicos.

Los prototipos conceptuales originalmente barajaban ciclos de vida semánticos idénticos a los del desarrollo frontend web (ej. el gancho "OnViewDidMount"). La abstracción evolucionó hacia una directiva totalmente universal catalogada como `effects:` (ADR-006). En lugar de atar el DSL al ciclo de vida del frontend o React/Vue/Android nativo, los efectos expresan side-effects de dominio genéricos (`GET /api/session`) independientes a sus integraciones host.
