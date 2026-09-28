# 28 sep 2026: cierre del día

## Hecho
- **Compilador:**
  - avisos no fatales (`--deny-warnings`);
  - R10 (transiciones muertas);
  - `pending`;
  - `[replace] X`;
  - `events:` como señales externas.
- **Demo:** pasa el e2e (14/14).
- **MonitoreoRed:** sin comodines.
- **Documentación:** manual, guía y §3 del paper actualizados.
- **`tools/atlas-mcp`:** servidor MCP local hacia ATLAS, comprobado por Code
  en el portátil de César (Windows, Claude Code y Claude Desktop).
- **Réplica del experimento con `gpt-oss-120b` en ATLAS**
  (`experiments/2026-09-atlas/`, tres pre-registros):

  | | Resultado |
  |---|---|
  | A (JS) | 19/20 |
  | B (Trenza, material original) | 4/20 |
  | B′ (material limpio) | 13/20 |
  | B″ (T1 con operaciones estructuradas) | 3/10 |

  - La mayor barrera era la presentación del material y la edición
    textual. Con operaciones no se rechazó ninguna edición.
  - Los 7 fallos de B″ son el mismo error: una acción producida sin
    transición en `ModoPausa`.

## Pregunta abierta de César
¿Siguen aportando algo los DSL como Trenza ahora que los modelos revisan
mejor? Respuesta provisional:
- con Sonnet, en tareas pequeñas, no aportan;
- con un modelo débil, solo si hay edición estructurada y reglas entre
  hermanos.

## Mañana
1. Implementar R11 (coherencia de transiciones entre hermanos) como aviso
   en `trenza-core/src/validator.rs`, con tests. El prototipo post hoc está
   en `experiments/2026-09-atlas/posthoc_r11.py`: 0 falsos positivos en B,
   B′ y B″.
2. Pre-registrar B‴ (= B″ + R11 en la comprobación que ve el modelo; T1,
   10 réplicas). Ojo: la comprobación del modelo usa el `trenza-cli`
   congelado de 2689bcb, así que hay que decidir cómo añadir R11 sin romper
   la comparabilidad. Por ejemplo, añadiendo la salida de R11 a la de
   `check`.
3. Sigue pendiente: el paper (tesis revisada), las 5 casillas `pending` de
   MonitoreoRed y el generador de Rust para MonitoreoRed.

**Presupuesto:** César indicó 122 $ restantes a media mañana. Las réplicas
corren en ATLAS y no gastan crédito.
