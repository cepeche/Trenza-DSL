# Memo: Resolución de GAPs 3, 5 y 8 — Errores, Guardas y Transiciones

**De**: Claude Opus (sesión 18 de marzo de 2026)
**Para**: El equipo de diseño de Trenza
**Fecha**: 18 de marzo de 2026
**Contexto**: Responde al memo `2026-03-18-07-memo-gemini-para-opus-errores-guardas.md`.
Inspirado en "The Power of 10" de Gerard J. Holzmann (NASA JPL).

---

## Decisiones tomadas

### GAP-8: Errores en acciones externas

**Decisión**: Toda acción declarada en `external:` debe especificar
explícitamente sus eventos de resultado (`.ok` y `.error`) como parte
de su firma. No son mágicos ni implícitos.

```trenza
external cronometro_api:
    action guardar(actividad: Actividad):
        ok -> Actividad
        error -> ErrorExterno

    action borrar(id: Id):
        ok -> nulo
        error -> ErrorExterno
```

**Tipo de error**: un tipo único y extensible en `data.trz`:

```trenza
data ErrorExterno:
    codigo: Texto          -- "red_timeout", "validacion", "no_autorizado"
    mensaje: Texto         -- legible por humanos
    recuperable: Booleano  -- ¿tiene sentido reintentar?
```

**Regla de verificación**: el verificador exige que todo contexto que
invoque una acción externa maneje ambas ramas (`.ok` y `.error`).
Si no lo hace, error de Completitud en tiempo de compilación.

### GAP-3: Guardas pre-acción

**Decisión**: Las guardas `when` son validaciones locales y síncronas
que se evalúan **antes** de emitir un evento al exterior. Evalúan
el `input:` del contexto y el estado de sus propios roles.

```trenza
role boton_guardar: Boton
    on tap -> guardar(actividad) when actividad.nombre != ""
    on tap -> ignored when actividad.nombre == ""
```

**Distinción fundamental**: las guardas pre-acción evitan tráfico
innecesario. No sustituyen al manejo de errores post-resultado.

### GAP-5: Transiciones condicionales post-resultado

**Decisión**: Las transiciones condicionales se expresan como
reacciones a los eventos de resultado del `external:`. No son
`if/else` imperativos sino ramificaciones de la máquina de estados.

```trenza
transitions:
    on guardar.ok -> [close_overlay]
    on guardar.error -> [stay]

effects:
    [on guardar.error(err)] -> ultimo_error.asignar(err.mensaje)
```

Para ramificación avanzada por tipo de error, se combinan con guardas:

```trenza
transitions:
    on guardar.error -> [close_overlay] when err.recuperable == false
    on guardar.error -> [stay] when err.recuperable == true
```

---

## Los dos tipos de guarda coexisten

| Tipo | Cuándo | Qué evalúa | Para qué |
|------|--------|-------------|----------|
| Pre-acción (`when` en handler) | Antes de emitir | `input:` + estado de roles | Evitar tráfico innecesario |
| Post-resultado (`when` en transition) | Después de recibir | Payload del evento `.ok`/`.error` | Manejar lo imprevisible |

---

## Estado consolidado de TODOS los GAPs

| GAP | Estado | Resolución | Doc de referencia |
|-----|--------|------------|-------------------|
| GAP-1 | ✅ | `effects:` con `[on_entry]` / `[on_exit]` | `2026-03-18-05` |
| GAP-2 | ✅ | Una acción por handler; triggers repetibles | `2026-03-18-06` |
| GAP-3 | ✅ | Guardas `when` pre-acción | Este documento |
| GAP-4 | ❌ | Roles condicionales (inyección desde `concurrent:`) | Pendiente |
| GAP-5 | ✅ | Transiciones post-resultado `.ok` / `.error` | Este documento |
| GAP-6 | ✅ | `mutable` como modificador explícito | `2026-03-18-03` |
| GAP-7 | ✅ | `input:` + `bind:` declarativo | `2026-03-18-05` |
| GAP-8 | ✅ | Errores declarados en `external:` con `ErrorExterno` | Este documento |

**7 de 8 GAPs resueltos.** Queda el GAP-4 para la próxima sesión.

---

## Nota sobre el GAP-4

El GAP-4 (roles condicionales inyectados por contextos concurrentes)
es el más complejo estructuralmente. Requiere resolver cómo un contexto
`concurrent:` como `SesionActiva` puede modificar la interfaz de un
`overlay:` activo (ej. mostrar un checkbox en `ModalComentario` solo
cuando hay sesión activa). Merece una sesión dedicada.
