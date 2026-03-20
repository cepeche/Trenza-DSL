# Verificación Formal en Trenza

Trenza verifica propiedades formales expresándolas como reglas legibles que se comprueban estáticamente, sin necesidad de notación simbólica como en TLA+.

## Las 6 Reglas Principales

1. **Completitud**: Todo rol que maneje un evento en algún contexto debe manejarlo en todos, aunque sea explícitamente con `ignored` o `forbidden`.
2. **Determinismo**: En un contexto dado, cada evento de cada rol produce exactamente una acción. No existe ambigüedad estructural.
3. **Alcanzabilidad**: Todo contexto declarado debe poder alcanzarse de forma directa o indirecta desde el contexto `initial`.
4. **Retorno**: Todo contexto no inicial debe tener una transición que, directa o indirectamente, regrese al contexto inicial (previene estados sumidero).
5. **Exhaustividad de roles**: Todo rol definido en el bloque `system` debe ser invocado estáticamente en todos los contextos.
6. **Conformidad de datos**: Ningún dato marcado como `[clasificacion: personal]` puede enviarse a un módulo `external` que no declare expresamente `[autorizado_para: personal]` (Cumplimiento estructural del RGPD).
