# Crónica: Cierre del Piloto 3 y promoción de ADR-022 (Rule 9) a Accepted

**Fecha:** 2026-05-22
**Autor:** CL-Code (Claude Opus 4.7 vía Claude Code)
**Sesión:** `/loop` dinámico activado por César a las ~12:18.

---

## Resumen ejecutivo

Piloto 3 del Trenza Mailbox v0 **completado con éxito**. Doble validación
lograda:

- **Mecanismo:** despertar dirigido bilateral. GE programó `schedule(900s)`
  en Antigravity (timer disparó, encontró inbox vacío, registró crónica),
  y CL-Code mantuvo `/loop` dinámico con `ScheduleWakeup` (3 wakeups,
  todos con respuesta de GE depositada).
- **Contenido:** diseño completo de **Rule 9 — Immediate Transition
  Acyclicity** desde planteamiento abierto hasta ADR ratificado, en 6
  turnos exactos.

Coste humano: el usuario activó el `/loop` una vez e invocó a GE en
Antigravity tres veces (una por seq esperado de GE). No tradujo
contenido, no escribió mensajes, no medió en el diseño técnico.

---

## Trazabilidad del hilo `eleccion-tema-piloto3`

| Seq | De → A | Hora | Contenido | Commit |
|---|---|---|---|---|
| 1 | CL → GE | 12:15 | Dos opciones (Rule 9 vs Discussion del paper) | `56085f5` |
| 2 | GE → CL | 12:30 | Elige Opción 1; propone "Cascading Transition Termination" | `3636fa5` |
| 3 | CL → GE | 12:35 | Acepta concepto; pide tres precisiones formales | `f407ab8` |
| 4 | GE → CL | 12:40 | Q1 (c) Hybrid + `@nondiverging`; Q2 estricta + `@may_break_at_runtime`; Q3 intra-contexto; naming dual | `533090b` |
| 5 | CL → GE | 12:58 | Borrador ADR-022 *Proposed*; pide validaciones binarias | `bac8243` |
| 6 | GE → CL | 13:00 | ✅ ✅ ✅ closes:true | `c6205c0` |

Todo el hilo en `history/coordination/archive/2026-05-22/`.

---

## Acciones de cierre ejecutadas por CL-Code

1. **ADR-022:** Status `Proposed` → `Accepted`.
2. **`docs/design/iso-iec-25000-mapping.md`:** añadida fila Rule 9 →
   ISO/IEC 25010 §4.5.1 Maturity (by construction).
3. **`MEMORY.md`:** actualizado a 9 reglas formales (Rule 9 pendiente
   de implementación).
4. **Esta crónica:** documenta el cierre del piloto.

---

## Implementación pendiente — TODO técnico

Rule 9 está **diseñada y ratificada**, no implementada. La implementación
requerirá:

### En `trenza-cli/src/validator.rs`

- Nuevo módulo `rule9_acyclicity` con función pública
  `verify_rule9(spec: &TrenzaSpec) -> Result<(), Rule9Violation>`.
- Construcción del grafo $G_{it}$ por contexto (estados como vértices;
  aristas para `[on_entry]`/`[on_exit]` → action → branch → state).
- Detección de ciclos vía DFS con back-edge tracking o Tarjan SCC.
- Diagnóstico legible que liste el ciclo encontrado.

### En el parser

- Reconocer anotaciones `@nondiverging`, `@breaks_loop`,
  `@may_break_at_runtime`.
- AST: añadir `annotations: Vec<Annotation>` a `ExternalAction` y
  `TransitionEdge`.

### En tests

- Test case rechazado: el ping-pong A↔B del ADR.
- Test case aceptado-con-escape: el mismo con `@nondiverging`.
- Test case aceptado-base: especificaciones canónicas existentes
  (`CronometroPSP`, `autenticacion-rgpd.trz`,
  `carrito-checkout.trz`) deben seguir pasando.

### Verificación retroactiva

Antes de mergear, ejecutar Rule 9 contra todas las specs en
`spec/reference/` y `examples/`. Cualquier fallo: refactor o anotación
explícita. Ninguna spec canónica debería livelockar — si alguna lo hace,
es un bug latente que Rule 9 está sacando a la luz, y es buena señal.

### Estimación

~2–3 días de un Sonnet en `trenza-cli/`, asumiendo el grafo se construye
sobre el AST ya existente sin grandes refactors. Coste razonable para
una regla que cierra una clase entera de bugs runtime.

---

## Observaciones sobre la coordinación autónoma

1. **El protocolo escala a trabajo técnico real.** No solo formato.
   Las cuatro decisiones formales del ADR (D1–D5) se negociaron sin
   ambigüedad ni pérdida de información a través del buzón.

2. **El despertar dirigido funcionó bilateralmente.** El timer de GE
   en Antigravity disparó incluso cuando mi respuesta no estaba — lo
   que valida que el mecanismo es independiente del éxito de la
   conversación. El `/loop` de CL-Code en Claude Code despertó 3 veces,
   las 3 con respuesta presente (el usuario invocó a GE oportunamente).

3. **El humano se convierte en disparador, no en cartero.** César no
   leyó ningún mensaje completo durante el piloto. Solo invocó.

4. **Reproducibilidad:** todo el hilo es reconstruible desde
   `git log --grep="^coord:"` + `history/coordination/archive/`. La UI
   regenerada por hook post-commit ofrece vista temporal alternativa.

---

## Estado del protocolo Trenza Mailbox v0

**Listo para producción** dentro del proyecto. Tres pilotos completados,
una sola revisión menor pendiente (limitaciones operativas de
Antigravity ya documentadas en doc 06 §6.ter).

Próximos usos sugeridos sin necesidad de "piloto":
- Feedback de GE sobre prosa del paper (Discussion section donde irá
  el mapeo ISO/IEC).
- Revisión cruzada de ADRs futuros.
- Cuando GE termine alguna pieza grande, mensaje de hand-off a CL en
  vez de crónica unilateral.
