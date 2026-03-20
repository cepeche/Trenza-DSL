# ANALISIS_GAPS.md â€” Lo que el DSL no cubre (todavÃ­a)

**Generado por**: ejercicio de especificaciÃ³n completa del cronÃ³metro PSP  
**Fecha**: 6 de marzo de 2026  
**MÃ©todo**: escribir todos los `.trz` del sistema real y anotar cada punto de fricciÃ³n

---

## Resumen

Al intentar especificar el cronÃ³metro completo encontramos **8 gaps** en el
DSL actual. No todos tienen el mismo peso. Los ordenamos de mayor a menor
impacto en la expresividad del lenguaje.

---

## GAP-1: ParÃ¡metros de entrada en contextos (CRÃTICO)

**DÃ³nde aparece**: ModalComentario, ModalEditarTarea, ModalEditarActividad,
ModalSeleccionActividad.

**El problema**: cuando ModoEdicion hace `on tap -> abrirEditarTarea`, necesita
pasar el `tipoId` de la tarjeta que se pulsÃ³. Sin esto, el modal no sabe
*quÃ©* estÃ¡ editando. En `app.js` esto se resuelve con `AppState.tareaIdPendiente`
â€” exactamente el estado implÃ­cito que trenza pretende eliminar.

**Impacto en app.js actual**: la variable `AppState.tareaIdPendiente` es el
ejemplo canÃ³nico. Un estado global mutable para transferir datos entre un
contexto y el siguiente.

**Propuesta de sintaxis**:
```
context ModalEditarTarea:
    input:
        tipoTareaId: Id            -- obligatorio
        actividadId: Id?           -- opcional (con ?)
```

Los parÃ¡metros de entrada viajan en la transiciÃ³n:
```
transitions:
    on abrirEditarTarea(tipoId) -> ModalEditarTarea with tipoTareaId: tipoId
```

**Por quÃ© es crÃ­tico**: sin esto, el compilador no puede verificar que todos
los contextos que abren un modal le pasan los datos que necesita. La
verificaciÃ³n formal queda incompleta.

---

## GAP-2: Contextos overlay (CRÃTICO)

**DÃ³nde aparece**: todos los modales y el menÃº de configuraciÃ³n.

**El problema**: los modales no reemplazan el contexto base â€” se superponen
a Ã©l. Cuando `ModalComentario` se cierra, el sistema vuelve a `ModoNormal`
(o a `ModoEdicion`, dependiendo de quiÃ©n lo abriÃ³). El DSL actual no tiene
concepto de "pila de contextos".

La diferencia con las transiciones normales es fundamental:
- TransiciÃ³n normal: `ModoNormal â†’ ModoEdicion`. ModoNormal desaparece.
- TransiciÃ³n a overlay: `ModoNormal â†’ (ModoNormal + ModalComentario)`.
  ModoNormal sigue activo debajo.

**Impacto en app.js actual**: todos los modales usan `classList.add('active')`
y `classList.remove('active')` â€” el modal se superpone, no reemplaza. El
flag `modoEdicion` persiste mientras el modal estÃ¡ abierto.

**Propuesta de sintaxis**:
```
system CronometroPSP:
    overlays:              -- nueva clÃ¡usula
        MenuConfiguracion
        ModalComentario
        ...
```

Y en las transiciones, el cierre usa un token especial:
```
transitions:
    on cancelar -> [cerrar_overlay]    -- regresa al contexto base que habÃ­a debajo
```

**Corolario**: cuando un overlay transiciona a otro overlay (MenuConfiguracion
â†’ ModalCrearActividad), Â¿se apilan o el primero se cierra? En app.js el menÃº
se cierra. El DSL necesita distinguir `[cerrar_y_abrir: NuevoOverlay]` de
`[apilar: NuevoOverlay]`.

---

## GAP-3: Contextos concurrentes (IMPORTANTE)

**DÃ³nde aparece**: SesionActiva coexiste con ModoNormal o ModoEdicion.

**El problema**: `SesionActiva` no es ni un contexto base ni un overlay. Es
un estado ortogonal que puede estar activo o no independientemente del modo.
Cuando hay sesiÃ³n activa, el timer muestra datos; cuando no, muestra `--:--`.
Esto ocurre tanto en ModoNormal como en ModoEdicion.

El modelo actual de `composition: prioridad` asume que los contextos son
alternativos. SesionActiva no es una alternativa a ModoNormal â€” es una
dimensiÃ³n diferente.

**Propuesta de sintaxis**:
```
system CronometroPSP:
    concurrent:            -- nueva clÃ¡usula (activos en paralelo con el base)
        SesionActiva       -- se activa/desactiva independientemente
```

Y en la desactivaciÃ³n:
```
context SesionActiva:
    transitions:
        on sesionFinalizada -> [desactivar]   -- sale del concurrent sin cambiar el base
```

---

## GAP-4: Roles condicionales segÃºn contexto concurrente (IMPORTANTE)

**DÃ³nde aparece**: `checkbox_sustituir` en ModalComentario, que solo existe
cuando SesionActiva estÃ¡ activo.

**El problema**: cuando ModalComentario se abre sin sesiÃ³n activa, el checkbox
"Sustituye a la tarea en curso" no existe en el DOM. Cuando se abre con sesiÃ³n
activa, existe. Este es un rol que depende de si un contexto concurrente estÃ¡ activo.

En app.js: `sustituirGroup.style.display = AppState.sesionActiva ? 'block' : 'none'`.
Un condicional mÃ¡s que trenza deberÃ­a eliminar.

**Propuesta de sintaxis**:
```
-- En SesionActiva.trz, declarar un rol que se inyecta en ModalComentario:
context SesionActiva:
    role checkbox_sustituir: Checkbox en ModalComentario
        on cambio -> marcarSustituir(self.marcado)
```

O alternativamente, en ModalComentario:
```
context ModalComentario:
    role checkbox_sustituir: Checkbox si SesionActiva
        on cambio -> marcarSustituir(self.marcado)
```

**ObservaciÃ³n**: esto es exactamente lo que hace DCI con los roles opcionales.
Un objeto (el modal) puede tener un rol adicional asignado por un contexto
concurrente. La sintaxis trenza deberÃ­a poder expresar esto.

---

## GAP-5: Transiciones condicionales por validaciÃ³n (MODERADO)

**DÃ³nde aparece**: ModalCrearTarea (nombre no vacÃ­o + al menos una actividad),
ModalEditarTarea (nombre no vacÃ­o), ModalReset Fase3 (campo == "BORRAR").

**El problema**: el botÃ³n "Guardar" solo dispara la transiciÃ³n si los datos
del formulario son vÃ¡lidos. Si no, deberÃ­a mostrar un error o deshabilitar el botÃ³n.
El DSL actual no expresa esto â€” la validaciÃ³n queda oculta en el external.

**Propuesta de sintaxis**:
```
role boton_guardar: Boton
    on tap si campo_nombre.valido -> guardarEdicion
    on tap si not campo_nombre.valido -> mostrarErrorNombre
```

O usando guards en las transiciones:
```
transitions:
    on guardarEdicion si campo_nombre.valido -> [cerrar_overlay]
    on guardarEdicion si not campo_nombre.valido -> mostrarErrorValidacion
```

**Nota**: esto introduce lÃ³gica booleana en el DSL. Hay que diseÃ±ar el
vocabulario de condiciones cuidadosamente para que siga siendo verificable
formalmente. Las condiciones deben referirse solo a propiedades de los datos
del contexto, nunca a estado externo.

---

## GAP-6: Roles dinÃ¡micos (multiplicidad) (MODERADO)

**DÃ³nde aparece**: botones de actividad en ModalSeleccionActividad,
checkboxes en ModalCrearTarea y ModalReset.

**El problema**: el nÃºmero de elementos no es fijo. Hay tantos botones de
actividad como actividades existan en el sistema. El DSL actual declara roles
como entidades Ãºnicas, no como colecciones.

**Propuesta de sintaxis**:
```
role boton_actividad[]: Actividad     -- [] indica multiplicidad
    on tap -> elegirActividad(self.id)
```

La verificaciÃ³n de completitud sigue funcionando: el compilador verifica que
el rol `boton_actividad[]` (como clase) maneje todos los eventos en todos los
contextos donde aparece, independientemente de cuÃ¡ntas instancias haya.

---

## GAP-7: Efectos de ciclo de vida (MODERADO)

**DÃ³nde aparece**: ModalAcercaDe (carga datos al abrir), ModalHistorial
(carga datos al cambiar de sub-contexto).

**El problema**: algunos contextos necesitan ejecutar efectos al entrar
(`al_entrar`) o al salir (`al_salir`), no en respuesta a un evento de usuario.
Son disparados por el cambio de contexto en sÃ­.

En app.js: `mostrarModalAcercaDe()` llama a `api.healthCheck()` y
`api.getAcumulado()` â€” no hay evento de usuario que los dispare; es el
hecho de abrir el modal lo que los lanza.

**Propuesta de sintaxis**:
```
effects:
    [al_entrar] -> external verificar_conexion()
    [al_salir]  -> limpiarEstadoModal()
```

Los corchetes distinguen los hooks de ciclo de vida de los efectos de acciÃ³n.

---

## GAP-8: Flujo de error en externals (MENOR)

**DÃ³nde aparece**: todas las llamadas a la API (cualquier modal que guarda datos).

**El problema**: las funciones `external` pueden fallar (servidor caÃ­do, error
de red, validaciÃ³n backend). El DSL actual no modela el flujo de error. En
app.js hay `catch` en cada funciÃ³n async, pero los errores son todos
`mostrarError(mensaje)` â€” el mismo manejador para todo.

Un sistema bien especificado deberÃ­a poder decir:
```
external module cronometro_api:
    crear_tipo_tarea(...) -> TipoTarea | Error<ApiError>
```

Y los contextos deberÃ­an poder manejar el error:
```
transitions:
    on guardarEdicion      -> [cerrar_overlay]
    on error_guardar_edicion -> mostrarErrorAPI
```

**Por quÃ© es menor**: la lÃ³gica de manejo de errores es en su mayorÃ­a
uniforme (`mostrarError`). Modelarla completamente en trenza serÃ­a verboso
sin aÃ±adir mucha verificabilidad. Lo mÃ¡s pragmÃ¡tico puede ser que los
externals declaren que pueden fallar, y trenza genere el cÃ³digo de
manejo de error estÃ¡ndar.

---

## Resumen de propuestas

| GAP | Propuesta | Impacto en verificabilidad |
|-----|-----------|---------------------------|
| 1 | `input:` en contextos overlay | ALTO: elimina estado implÃ­cito |
| 2 | `overlays:` en system + `[cerrar_overlay]` | ALTO: modela correctamente los modales |
| 3 | `concurrent:` en system + `[desactivar]` | ALTO: modela SesionActiva correctamente |
| 4 | `role X: T si <Contexto>` | MEDIO: elimina condicionales de visibilidad |
| 5 | Guards en transiciones (`si <condiciÃ³n>`) | MEDIO: valida flujos de formulario |
| 6 | `role X[]: T` (multiplicidad) | MEDIO: modela listas dinÃ¡micas |
| 7 | `[al_entrar]` / `[al_salir]` en effects | MEDIO: modela side effects de inicializaciÃ³n |
| 8 | Tipos de error en externals | BAJO: mejora completitud pero es verboso |

---

## Lo que funciona bien

Para ser honestos, estos gaps no invalidan el diseÃ±o central. El nÃºcleo del
DSL â€” contextos, roles, eventos, transiciones â€” funciona muy bien para:

- La lÃ³gica ModoNormal / ModoEdicion (el problema original): **perfecto**.
- Los sub-contextos de ModalReset (3 fases): **funciona sin cambios**.
- Los sub-contextos de ModalHistorial (7d / 30d): **funciona con GAP-7**.
- La separaciÃ³n data / context: **sÃ³lida, sin fricciones**.
- Los mÃ³dulos external: **funcionan bien como contratos**.

Los gaps mÃ¡s crÃ­ticos (1, 2, 3) son todos variaciones del mismo tema:
el DSL necesita mÃ¡s tipos de relaciÃ³n entre contextos que solo "exclusivos
con prioridad". Los tres tipos identificados (base exclusivo, overlay apilable,
concurrente) son una extensiÃ³n natural del modelo DCI, no una ruptura de Ã©l.

---

## PrÃ³ximo paso sugerido

Antes de aÃ±adir sintaxis nueva, comprobar si los tres tipos de contexto
(base, overlay, concurrent) pueden expresarse con el modelo actual de
composiciÃ³n, usando convenciones de nomenclatura en lugar de palabras clave
nuevas. Si el verificador puede inferir el tipo desde las transiciones
(`[cerrar_overlay]` implica overlay; `[desactivar]` implica concurrent),
quizÃ¡s no necesitamos keywords adicionales en la declaraciÃ³n del system.

La navaja de Occam antes de aÃ±adir palabras reservadas.

