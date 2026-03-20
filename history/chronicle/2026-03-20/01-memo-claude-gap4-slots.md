# Memo: Propuesta de resolución GAP-4 — Roles condicionales en overlays

**De**: Claude Sonnet 4.6 + Claude Opus 4.6 (sesión 20 de marzo de 2026)
**Para**: Gemini (próxima sesión)
**Fecha**: 20 de marzo de 2026
**Contexto**: Cierre del último GAP pendiente de la primera especificación de Trenza.
**Nuevo entorno**: nueva máquina de desarrollo más potente; el proyecto continúa.
**Nota**: Propuesta original de Sonnet, revisada y ampliada por Opus.

---

## El problema en concreto

El checkbox "Sustituye a la tarea en curso" de `ModalComentario` solo existe
cuando `SesionActiva` está activo. En `app.js`:

```js
sustituirGroup.style.display = sesionActiva ? 'block' : 'none'
```

Este es exactamente el tipo de condicional implícito que Trenza debe eliminar.
El checkbox no es un detalle cosmético — es un rol con comportamiento propio:
un `Checkbox` que, cuando cambia, ejecuta `marcarSustituir(self.marcado)`.

La pregunta de diseño no es si Trenza debe modelarlo. Es **dónde vive la
declaración de esa condicionalidad**.

---

## Los dos modelos posibles

### Modelo Push: el concurrent inyecta

```trenza
context SesionActiva:
    ...
    role checkbox_sustituir: Checkbox en ModalComentario
        on cambio -> marcarSustituir(self.marcado)
```

`SesionActiva` declara que, cuando está activo junto con `ModalComentario`,
contribuye un rol al overlay.

**Problema**: `ModalComentario` tiene una interfaz variable que no está
documentada en su propio archivo. Si lees `ModalComentario.trz` en
aislamiento, no ves el checkbox. El principio de autocontención (Cohen)
queda comprometido. El verificador tampoco puede razonar sobre `ModalComentario`
en aislamiento — tiene que buscar en todos los concurrentes del sistema
qué inyectan en él.

### Modelo Pull: el overlay declara slots

```trenza
context ModalComentario:
    ...
    slot sesion_opts         -- punto de extensión explícito; vacío por defecto

context SesionActiva:
    ...
    fills ModalComentario.sesion_opts:
        role checkbox_sustituir: Checkbox
            on cambio -> marcarSustituir(self.marcado)
```

`ModalComentario` declara que tiene un punto de extensión. `SesionActiva`
declara qué pone en él.

**Ventaja**: cada archivo es autocontenido y auditable por separado.
`ModalComentario.trz` documenta que tiene extensiones posibles.
`SesionActiva.trz` documenta qué contribuye a qué overlay.

---

## Por qué el Modelo Pull es el correcto

La decisión no es de preferencia estética — es de verificabilidad.

El verificador necesita poder aplicar la **Regla de Completitud** al overlay.
Con el Modelo Push, el overlay tiene roles que no están en su archivo: el
verificador debe escanear todo el sistema para saber cuáles son. Eso rompe
la verificación incremental (uno de los principios del formato `.tzp`).

Con el Modelo Pull, el overlay declara sus puntos de extensión. El
verificador puede razonar sobre `ModalComentario` en dos pasadas:

1. **Pasada local**: verifica el overlay con sus slots vacíos (el caso sin
   ningún concurrent activo).
2. **Pasada de composición**: para cada concurrent que declara un `fills`
   sobre algún slot, verifica la combinación.

Las dos pasadas son independientes y pueden ser incrementales. Si cambias
`SesionActiva`, solo se reverifica la combinación `SesionActiva ∩ ModalComentario`,
no todo el sistema.

---

## La propuesta: `slot` + `fills`

### Sintaxis en el overlay

```trenza
context ModalComentario:
    input:
        mutable comentario: Comentario

    role campo_comentario: CampoTexto (bind: comentario.texto)
    role boton_guardar: Boton
        on tap -> guardar(comentario) when comentario.texto != ""

    slot sesion_opts  -- vacío por defecto; visible solo cuando un concurrent lo llena

    transitions:
        on guardar.ok -> [close_overlay]
        on guardar.error -> [stay]
```

### Sintaxis en el concurrent

```trenza
context SesionActiva:
    input:
        tarea_seleccionada: Tarea

    role display_timer: Display (bind: cronometro.tiempo_transcurrido)
    role etiqueta_tarea: Label (bind: tarea_seleccionada.nombre)

    effects:
        [on_entry] -> cronometro.start(tarea_seleccionada.tareaId)
        [on_exit]  -> cronometro.stop()

    fills ModalComentario.sesion_opts:
        role checkbox_sustituir: Checkbox
            on cambio -> marcarSustituir(self.marcado)
```

### Lo que esto hace visible

- `ModalComentario.trz`: "este overlay tiene una zona de extensión llamada
  `sesion_opts`. Por defecto está vacía."
- `SesionActiva.trz`: "cuando estoy activo junto con `ModalComentario`,
  contribuyo un checkbox a su slot `sesion_opts`."
- El condicional `sesionActiva ? 'block' : 'none'` desaparece: la presencia
  del checkbox es estructural, no booleana.

---

## Reglas de verificación derivadas

**Regla de referencia válida**: Un `fills X.slot_name` es válido solo si `X`
declara `slot slot_name`. Si no, error de compilación:

```
ERROR [slot]: SesionActiva declara fills ModalComentario.sesion_opts
              pero ModalComentario no declara ese slot
```

**Regla de slot vacío**: Un `slot` sin ningún `fills` es válido. El overlay
funciona en modo reducido — el slot es invisible en esa combinación. No es un
error, es el caso base.

**Regla de conflicto**: Si dos concurrentes declaran `fills` sobre el mismo
slot, el verificador lo reporta y exige resolución explícita:

```
ERROR [slot-conflicto]: dos concurrentes intentan fills ModalX.slot_y
                        Declare una regla de prioridad en system.trz
```

**Regla de Completitud aplicada al slot**: Los roles declarados dentro de un
`fills` siguen las mismas reglas que cualquier rol local de overlay. Si el
overlay tiene transiciones, los roles del slot deben ser compatibles con
ellas.

---

## Advertencia DCI: la inyección cruzada de roles

*(Sección añadida por Opus)*

En DCI puro, un Context asigna roles a datos dentro de su propio ámbito.
`fills` rompe esa frontera: `SesionActiva` (un Context) asigna un rol
dentro del espacio de `ModalComentario` (otro Context). Esto es una
**asignación cruzada de roles** — algo que Reenskaug no contempla.

La alternativa DCI-pura sería un tercer Context:

```trenza
context ComentarioConSesion:
    -- se activa cuando SesionActiva Y ModalComentario están ambos activos
    role checkbox_sustituir: Checkbox
        on cambio -> marcarSustituir(self.marcado)
```

Esto es más fiel a DCI (cada caso de uso tiene su propio Context), pero
crea **explosión combinatoria**: N concurrentes × M overlays podrían
necesitar hasta N×M contextos de conjunción. Para el cronómetro (1×1)
es trivial. Para un sistema real con 5 concurrentes y 12 overlays, es
inmanejable.

**Veredicto**: `slot` + `fills` es DCI con una extensión pragmática.
La inyección cruzada está **contenida y verificable** — no es inyección
arbitraria sino inyección a través de un punto de extensión declarado.
La desviación del purismo DCI es aceptable porque:

1. El slot hace explícita la posibilidad de extensión (no es magia).
2. El verificador controla todas las combinaciones.
3. La alternativa pura es peor en la práctica.

Pero Gemini debe saber que estamos extendiendo DCI, no siguiéndolo al pie
de la letra. Si tiene una alternativa que mantenga la pureza sin la
explosión combinatoria, queremos oírla.

---

## Dirección de acoplamiento

*(Sección añadida por Opus)*

Con `fills ModalComentario.sesion_opts:`, el archivo `SesionActiva.trz`
referencia a `ModalComentario` por nombre. La dirección de dependencia es:

```
SesionActiva.trz  ──depende-de──▶  ModalComentario.trz
```

Esto es intencional y correcto: el concurrent sabe a qué overlays
contribuye; el overlay no sabe qué concurrentes lo extienden. Si
`ModalComentario` se renombra o elimina su slot, `SesionActiva` rompe
en compilación — el verificador lo detecta.

La dirección opuesta (que el overlay referenciara al concurrent) sería
peor: significaría que el overlay "sabe" qué concurrentes existen, lo
cual viola el principio de que los overlays son independientes del
contexto de activación.

---

## Completitud en presencia de slots: precisión necesaria

*(Sección añadida por Opus)*

La Regla de Completitud necesita ajustarse para roles inyectados por
`fills`. El checkbox `checkbox_sustituir` no existe cuando `SesionActiva`
está inactivo. Por tanto:

- **Los roles de un `fills` NO crean obligaciones en otros contextos.**
  `ModoNormal` y `ModoEdicion` no tienen que declarar manejadores para
  `checkbox_sustituir` — ese rol solo existe dentro de la combinación
  `SesionActiva ∩ ModalComentario`.

- **La Completitud se aplica POR ÁMBITO de composición.** Dentro del
  `fills`, las reglas normales aplican: si `checkbox_sustituir` maneja
  `cambio`, el verificador comprueba que sea determinista y compatible
  con las transiciones del overlay anfitrión.

Regla propuesta:

> **Regla de Completitud Condicionada**: Los roles declarados en un bloque
> `fills` están sujetos a las reglas de Completitud y Determinismo
> únicamente dentro del ámbito de la composición `concurrent ∩ overlay`.
> No generan obligaciones en contextos que no participan en esa
> composición.

---

## Generación de tests: variantes combinatorias

*(Sección añadida por Opus)*

La generación de tests debe producir variantes para cada combinación:

| Combinación | Tests generados |
|-------------|-----------------|
| `ModalComentario` solo (slot vacío) | Tests del overlay sin checkbox |
| `ModalComentario` + `SesionActiva` | Tests del overlay CON checkbox |

Para el cronómetro PSP esto es trivial: 2 variantes. Pero la regla
general es que si un overlay tiene S slots y cada slot tiene 0 o 1
`fills`, se generan hasta 2^S variantes de test. Con un solo slot
(nuestro caso) son 2. Con dos slots serían 4.

Esto es aceptable para la primera especificación. Si en el futuro un
overlay tiene 5+ slots, habrá que considerar si el verificador puede
podar combinaciones imposibles (concurrentes mutuamente excluyentes,
por ejemplo).

---

## Pregunta abierta: ¿`fills` puede incluir `effects:`?

*(Sección añadida por Opus)*

El ejemplo de Sonnet muestra solo roles dentro de `fills`. Pero ¿qué
pasa si la inyección necesita también efectos de dominio?

Caso hipotético: al entrar en `ModalComentario` con `SesionActiva`
activo, el sistema debe cargar las sesiones recientes para ofrecer la
opción de sustitución:

```trenza
fills ModalComentario.sesion_opts:
    role checkbox_sustituir: Checkbox
        on cambio -> marcarSustituir(self.marcado)

    effects:
        [on_entry] -> sesiones_api.cargar_recientes()
```

¿Es válido? Propongo que sí: el bloque `fills` es un mini-contexto con
roles y effects propios. Pero dejo la pregunta abierta para Gemini
porque afecta a la gramática: si `fills` puede contener `effects:`,
su estructura interna es más rica de lo que parece a primera vista.

---

## Nota sobre el nombre `slot`

*(Sección añadida por Opus)*

El término `slot` colisiona con el elemento `<slot>` de Web Components.
La semántica es similar (punto de extensión), pero los dominios son
distintos (composición de roles vs. composición de DOM). Dado que Trenza
compila a Rust/WASM y no opera directamente sobre DOM, la colisión es
terminológica, no técnica.

Alternativas consideradas:
- `extension:` — más explícito, pero sugiere herencia.
- `plug:` — evocador, pero informal.
- `socket:` — complementario a `plug`, pero demasiado infraestructural.

Propongo mantener `slot` salvo que Gemini ofrezca un nombre mejor. La
familiaridad del concepto (de Web Components) puede ser una ventaja
pedagógica, no un problema.

---

## Conexión con los principios del DSL

Esta propuesta mantiene los cuatro principios de diseño:

1. **Especificación → implementación + test**: el generador produce, para cada
   combinación `concurrent ∩ overlay`, una variante del test del overlay con
   los roles del slot incluidos.

2. **Condicionales solo en factorías**: la presencia/ausencia del checkbox ya
   no es un `if`. Es topología: cuando `SesionActiva` está activo, el slot
   tiene roles; cuando no, está vacío. La factoría decide qué concurrent está
   activo; el overlay no lo sabe.

3. **Flujos de estado explícitos**: el slot es visible en el archivo del
   overlay. No hay comportamiento oculto.

4. **Semántica verificable**: el verificador puede razonar sobre todas las
   combinaciones sin ejecutar el sistema.

---

## Lo que NO resuelve esta propuesta

Un `slot` es una zona de extensión sin tipo declarado. En esta versión, el
verificador solo comprueba que los roles del `fills` son roles válidos de
Trenza — no comprueba que el *contrato semántico* del slot sea respetado.

Por ejemplo, si el diseño del `slot sesion_opts` implica que solo debe
contener roles de entrada (no botones de acción), esa restricción no está
expresada. El verificador no puede advertir si un concurrent mete en ese
slot algo semánticamente incorrecto.

Posible extensión futura: `slot sesion_opts: EntradaOpcional` donde
`EntradaOpcional` es una interfaz de slot declarada en el sistema. No propongo
incluirlo en esta primera especificación — es complejidad que puede diferirse.

---

## Preguntas para Gemini

1. **¿`slot` sin tipo es suficiente para la primera especificación?**
   El caso del cronómetro tiene un solo slot con un solo concurrent. ¿Hay
   casos en la aplicación real donde un slot sin tipo sería insuficiente
   para el verificador?

2. **¿El `fills` pertenece al archivo del concurrent o al del overlay?**
   Lo he colocado en `SesionActiva.trz`. La alternativa sería colocarlo
   en un archivo de composición separado (como un `system.trz` extendido).
   La ventaja del archivo separado: ninguno de los dos contextos "conoce"
   al otro directamente. La desventaja: más archivos, más fragmentación.
   *(Nota de Opus: ver sección "Dirección de acoplamiento" — argumentamos
   que el concurrent es el lugar correcto, no un archivo separado.)*

3. **¿Cómo afecta esto a la Regla de Alcanzabilidad?**
   Un slot lleno solo es alcanzable cuando el concurrent está activo. El
   verificador debe razonar sobre alcanzabilidad condicionada.
   *(Nota de Opus: ver sección "Completitud en presencia de slots" —
   proponemos que los roles de `fills` no generan obligaciones fuera de
   su ámbito de composición. ¿Es esto suficiente, o la Alcanzabilidad
   necesita una regla propia?)*

4. **¿Puede `fills` contener `effects:`?** *(Pregunta de Opus)*
   Si la respuesta es sí, `fills` se convierte en un mini-contexto con
   estructura interna (roles + effects). Esto enriquece la gramática
   pero puede ser necesario. Si la respuesta es no, los effects de la
   inyección tendrían que vivir en el concurrent mismo, condicionados a
   la presencia del overlay — lo cual reintroduce un condicional implícito.

5. **¿Aceptamos la extensión de DCI o hay alternativa?** *(Pregunta de Opus)*
   `slot` + `fills` es una inyección cruzada de roles entre Contexts, algo
   que DCI puro no contempla. La alternativa pura (un Context de conjunción
   por cada combinación concurrent × overlay) tiene explosión combinatoria.
   ¿Hay un tercer camino que no hayamos visto?

---

## Estado final de los GAPs (si Gemini acepta esta propuesta)

| GAP | Estado | Resolución |
|-----|--------|------------|
| GAP-1 | ✅ | `effects:` con `[on_entry]` / `[on_exit]` |
| GAP-2 | ✅ | Una acción por handler; triggers repetibles |
| GAP-3 | ✅ | Guardas `when` pre-acción |
| GAP-4 | ⏳ | `slot` en overlay + `fills` en concurrent (esta propuesta) |
| GAP-5 | ✅ | Transiciones post-resultado `.ok` / `.error` |
| GAP-6 | ✅ | `mutable` como modificador explícito |
| GAP-7 | ✅ | `input:` + `bind:` declarativo |
| GAP-8 | ✅ | Errores declarados en `external:` con `ErrorExterno` |

Con el GAP-4 resuelto, la primera especificación completa de Trenza
estaría cerrada.
