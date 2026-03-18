# Memo: Resolución de GAP-2 — Acciones compuestas

**De**: Claude Opus (sesión 18 de marzo de 2026)
**Para**: Gemini (próxima sesión)
**Fecha**: 18 de marzo de 2026
**Contexto**: Tras resolver GAP-1 y GAP-7 (`2026-03-18-05-memo-opus-resolucion-gap1-gap7.md`),
el Desarrollador Principal ha validado esta propuesta para GAP-2.

---

## El problema original

Claude planteó en `2026-03-18-02-memo-claude-para-gemini-gaps.md` dos
alternativas para expresar múltiples efectos en un solo evento:

```trenza
-- Opción A: lista en línea
on guardar -> [actualizarNombre(campo.valor), cerrarOverlay]

-- Opción B: bloque do
on guardar -> do:
    actualizarNombre(campo.valor)
    cerrarOverlay
```

## Hallazgo: el problema se disuelve con las tres capas

El modelo de tres capas (`input:` + `bind:` + `effects:`) resuelto en
GAP-1/GAP-7 elimina la mayoría de los casos que motivaban las
"acciones compuestas".

La mayoría de los supuestos "compound actions" son en realidad
**una acción de dominio + un cambio de estado**:

| Lo que parecía | Lo que realmente es |
|----------------|---------------------|
| `guardar + cerrarOverlay` | Acción (`guardar`) + transición (`-> [close_overlay]`) |
| `descartar + cerrarOverlay` | Acción (`descartar`) + transición (`-> [close_overlay]`) |
| `cargarDatos + iniciarTimer` | Dos efectos de dominio en `[on_entry]` |

Con la separación correcta, un handler de rol emite **un evento**.
La transición que ese evento provoca se declara en `transitions:`.
Los efectos de dominio van en `effects:`.

## La decisión: una acción por handler, repetición de trigger en effects

### Handlers de rol: exactamente una acción

```trenza
role boton_guardar: Boton
    on tap -> guardar(actividad_en_edicion)

transitions:
    on guardar -> [close_overlay]
```

El handler emite `guardar`. La transición responde cerrando el overlay.
Son dos responsabilidades separadas, no una acción compuesta.

### Effects: una acción por línea, triggers repetibles

Si un `[on_entry]` necesita disparar más de un efecto de dominio,
se repite el trigger:

```trenza
effects:
    [on_entry] -> cronometro.start(tarea.id)
    [on_entry] -> analytics.track("session_start")
    [on_exit]  -> cronometro.stop()
```

La semántica es: todas las líneas `[on_entry]` se ejecutan al entrar,
en orden declarado. El verificador razona sobre cada una independientemente.

## Regla formal

| Sección | Regla |
|---------|-------|
| Handler de rol (`on tap ->`) | Exactamente una acción o evento |
| `transitions:` | Exactamente una transición por evento |
| `effects:` | Una acción por línea; se repite el trigger si hay varias |

### Justificación

Si un `on tap` necesita "guardar Y notificar Y cerrar", es una señal
de que la separación de responsabilidades no está limpia. La acción
`guardar` debería ser una operación de dominio en `external:` que
internamente haga lo que necesite. Trenza especifica *qué* ocurre,
no *cómo* se implementa.

Esta restricción puede relajarse en el futuro si la práctica lo exige.
Es más fácil relajar una restricción que endurecerla.

## Impacto: la gramática no crece

GAP-2 se resuelve **sin añadir nueva sintaxis**. No hay listas
`[a, b, c]` ni bloques `do:`. La gramática permanece mínima.

## Estado consolidado de los GAPs (actualizado)

| GAP | Estado | Resolución |
|-----|--------|------------|
| GAP-1 | ✅ | `effects:` con `[on_entry]` / `[on_exit]` |
| GAP-2 | ✅ | Una acción por handler; triggers repetibles en `effects:` |
| GAP-3 | ❌ | Guardas (pendiente) |
| GAP-4 | ❌ | Roles condicionales (pendiente) |
| GAP-5 | ❌ | Transiciones condicionales (pendiente) |
| GAP-6 | ✅ | `mutable` como modificador explícito |
| GAP-7 | ✅ | `input:` + `bind:` declarativo |
| GAP-8 | ✅ | `ignored` como palabra clave reservada |

**5 de 8 GAPs resueltos.** Quedan GAP-3, GAP-4 y GAP-5, que pueden
caer en cascada en la próxima sesión.

## Nota para Gemini

Gemini: el Desarrollador Principal te pide que en las próximas sesiones
inicies el contador de documentos diario desde 01, no continuando la
numeración del día anterior. La convención es `YYYY-MM-DD-NN-titulo.md`
donde `NN` se reinicia cada día.
