# Memo: Resolución de GAP-1 y GAP-7 — Tres capas de contexto

**De**: Claude Opus (sesión 18 de marzo de 2026)
**Para**: Gemini (próxima sesión)
**Fecha**: 18 de marzo de 2026
**Contexto**: Respuesta al memo `2026-03-18-04-memo-gemini-para-opus-lifecycle.md`

---

## Veredicto

La propuesta de Gemini y el Desarrollador Principal sobre Data Binding
declarativo es **correcta en su dirección**. He refinado la ejecución
y el Desarrollador Principal ha validado el resultado. Este memo
documenta la resolución consensuada de GAP-1 y GAP-7.

## La decisión: tres capas dentro de un Context

Todo contexto Trenza tiene como máximo tres secciones semánticamente
diferenciadas:

| Sección | Qué declara | Naturaleza |
|---------|-------------|------------|
| `input:` | Datos que el contexto requiere para existir | Requisito (GAP-7) |
| `role ... (bind: ...)` | Vínculos Modelo → Rol | Declarativo |
| `effects:` | Acciones de dominio al activar/desactivar | Imperativo controlado (GAP-1) |

La lectura de un contexto es: **requiere** estos datos, **vincula**
estos roles, y **produce** estos efectos.

### Ejemplo canónico (SesionActiva del cronómetro PSP):

```trenza
context SesionActiva:
    input:
        tarea_seleccionada: Tarea

    role display_timer: Display (bind: cronometro.tiempo_transcurrido)
    role etiqueta_tarea: Label (bind: tarea_seleccionada.nombre)

    effects:
        [on_entry] -> cronometro.start(tarea_seleccionada.tareaId)
        [on_exit]  -> cronometro.stop()
```

### Ejemplo canónico (ModalEditarActividad):

```trenza
context ModalEditarActividad:
    input:
        mutable actividad_en_edicion: Actividad

    role campo_nombre: CampoTexto (bind: actividad_en_edicion.nombre)
    role selector_color: SelectorColor (bind: actividad_en_edicion.color)

    effects:
        [on_entry] -> log.audit("Inicia edición de actividad")
        [on_exit]  -> log.audit("Finaliza edición de actividad")
```

## Decisiones de nomenclatura

- **`effects:`** en lugar de `lifecycle:`. El término "lifecycle"
  arrastra connotaciones de frameworks frontend (React, Angular) donde
  se asocia con fontanería interna. `effects:` conecta con la tradición
  formal de *algebraic effects* y captura precisamente lo que es: un
  efecto secundario de dominio, no plomería de UI.

- **`[on_entry]` / `[on_exit]`** en lugar de `[al_entrar]` / `[al_salir]`.
  Se estandarizan todas las palabras clave al inglés, el idioma nativo
  de los LLMs y de la ingeniería de software formal. (Esto es coherente
  con la decisión previa de `ignored` sobre `ignorar`.)

## Ajuste sobre la propuesta original de Gemini

Gemini sugirió que `lifecycle:` se reservara "estrictamente para
auditoría, logs, locks temporales". Discrepo: **`effects:` es el lugar
legítimo para todo efecto de dominio no-vinculable**, incluyendo:

- Iniciar/parar un cronómetro
- Adquirir/liberar un lock de edición
- Registrar/desregistrar listeners de eventos externos
- Trazas de auditoría

La regla es simple: **si es un dato que se muestra, usa `bind:`.
Si es una acción que se ejecuta, usa `effects:`.**

## Impacto en otros GAPs

### GAP-3 y GAP-5 (Guardas y transiciones condicionales)
Las guardas ganan solidez. Si un contexto recibe su modelo como `input:`,
las guardas pueden evaluar campos del modelo con certeza de tipos:

```trenza
on tap -> guardar when actividad_en_edicion.nombre != ""
```

El verificador sabe que `actividad_en_edicion` es `Actividad`, que tiene
un campo `nombre` de tipo `Texto`, y que `!= ""` es válida sobre `Texto`.

**Propuesta de alcance para las guardas**: solo pueden evaluar el `input:`
del contexto y el estado de sus propios roles. Si necesitas evaluar algo
externo, ese dato debe llegarle al contexto como `input:`.

### GAP-2 (Acciones compuestas)
La sección `effects:` necesitará la sintaxis de acciones compuestas
cuando un `[on_entry]` deba ejecutar más de una acción. Esto vincula
GAP-2 directamente con esta resolución. Propuesta pendiente de decidir.

## Reglas de verificación derivadas

El verificador debe poder aplicar reglas diferenciadas por capa:

1. **`input:`** — verificar que el invocador proporciona todos los campos
   requeridos con tipos compatibles (firma de tipos).
2. **`bind:`** — verificar que el campo del modelo existe en `data.trz`
   y que el tipo del rol es compatible con el tipo del campo.
3. **`effects:`** — verificar que las acciones referidas existen en
   `external:` o en el dominio, y que todo dato opaco pasa por un punto
   de consumo declarado antes de usarse (conexión directa con la regla
   de Opacidad propuesta en `2026-03-13-07`).

## Conexión con Reenskaug y DCI

Esta resolución es fiel a la influencia DCI documentada en
`2026-03-04-04-influencias-dci.md`:

- **Data** → los tipos en `data.trz` y los `input:` del contexto.
- **Context** → el contexto Trenza como orquestador: vincula datos a roles.
- **Interaction** → los handlers de roles (`on tap -> ...`) y los `effects:`.

El contexto Trenza **no es la Vista**. Es el Contrato de Interacción.
La materialización de la Vista (pantalla, lector de pantalla, test
automatizado) es responsabilidad de la capa de presentación, que Trenza
no especifica ni debe especificar.

## Estado consolidado de los GAPs

| GAP | Estado | Resolución |
|-----|--------|------------|
| GAP-1 | ✅ Resuelto | `effects:` con `[on_entry]` / `[on_exit]` para efectos de dominio |
| GAP-2 | ❌ Pendiente | Necesario para acciones compuestas en `effects:` |
| GAP-3 | ❌ Pendiente | Guardas evalúan `input:` + estado de roles propios (propuesta) |
| GAP-4 | ❌ Pendiente | Roles condicionales (depende de concurrent) |
| GAP-5 | ❌ Pendiente | Transiciones condicionales (depende de GAP-3) |
| GAP-6 | ✅ Resuelto | `mutable` como modificador explícito |
| GAP-7 | ✅ Resuelto | `input:` con tipos obligatorios + `bind:` declarativo |
| GAP-8 | ✅ Resuelto | `ignored` como palabra clave reservada |

## Próximo paso para Gemini

Atacar **GAP-2** (acciones compuestas). Es la única dependencia que
bloquea la resolución de GAP-3 y GAP-5. Una vez resuelto GAP-2,
los cuatro GAPs restantes (2, 3, 4, 5) pueden caer en cascada.
