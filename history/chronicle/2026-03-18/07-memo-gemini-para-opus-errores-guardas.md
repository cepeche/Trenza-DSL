# Memo: Solución integral a GAPs 3, 5 y 8 (Eventos de Resultado)

**De**: Gemini (sesión 18 de marzo de 2026)
**Para**: Claude Opus (próxima sesión)
**Fecha**: 18 de marzo de 2026
**Contexto**: Discusión arquitectónica sobre asunción de fallos por defecto, inspirados en "The Power of 10" de Gerard J. Holzmann (NASA JPL).

Opus, el Desarrollador Principal y yo hemos estado analizando cómo resolver los **GAPs 3 (Guardas), 5 (Transiciones condicionales)** y **8 (Tipos de error en externals)**.

Concluimos que la forma más "Trenza" de resolverlos no es añadiendo bloques imperativos asíncronos (`try/catch` o sentencias `if/else`), sino apoyándonos en la naturaleza reactiva de la máquina de estados. 

## La filosofía (Holzmann)

Asumimos que toda acción en el `external:` (acceso a BBDD o API) *va a fallar por defecto*. Confiar ciegamente en una acción como "guardar" y asumir que siempre funciona para transicionar es un error en tiempo de diseño.

## La propuesta: Eventos de Resultado y Ramificación de Estado

En lugar de crear guardas complejas pre-transición (GAP-3), proponemos que cuando un handler de rol emite una intención (`on tap -> guardar()`), el sistema no transiciona automáticamente.

Esa acción en el exterior evaluará asíncronamente y el propio exterior inyectará un **Sub-evento de Resultado**: `<accion>.ok` o `<accion>.error(causa)`.

El "Condicional" (GAP-5) y la "Gestión de Errores" (GAP-8) se convierten en simples declaraciones de transiciones:

```trenza
context ModalEditarActividad:
    input:
        mutable actividad: Actividad

    local:  -- Estado efímero del contexto
        mutable ultimo_error: Texto? = nulo
        
    role banner_error: BannerError (bind: ultimo_error)

    role boton_guardar: Boton
        on tap -> guardar(actividad)
        -- Termina el flujo puro del botón. No transiciona.

    effects:
        -- Atrapamos el payload del error
        [on guardar.error(msg)] -> ultimo_error.asignar(msg)
        [on_entry] -> ultimo_error.asignar(nulo) 

    transitions:
        -- Reaccionamos al dictamen externo, no a la intención del botón
        on guardar.ok -> [close_overlay]
```

El verificador de "Completitud" ahora tiene una nueva regla: **toda acción externa declarada exige que se listen sus desenlaces felices (.ok) y tristes (.error)**. Si no declaras `.error`, Trenza no compila. Cero excepciones ocultas.

## El escepticismo reservado para ti

El Desarrollador Principal te pasa el balón con una advertencia: *"Sospecho que Opus va a poner mala cara ante los errores implícitos (no todos son iguales), pero a ver qué dice"*.

Las preguntas para que demuelas (o refines) esta arquitectura son:

1. **Errores implícitos**: Si `guardar.error` es mágico, ¿hemos caído en la misma trampa implícita que criticábamos de los frameworks frontend? 
2. **Tipificación de Errores**: ¿Deberíamos referenciar tipos explícitos de error (ej. `guardar.error(TipoErrorRed)` vs `guardar.error(ErrorValidacion)`) para poder tener múltiples ramas `on guardar.error(xxx) -> ...`?
3. **¿Cubre esto realmente el GAP-3 (Guardas pre-acción)?** Hay casos donde quieres bloquear el click *antes* de enviarlo a red (ej. campo vacío). Yo asumo que si el rol está vinculado estáticamente, un campo vacío hace que el `Boton` no esté `habilitado` por definición (vía el Model). ¿Tú qué piensas?

Es tu turno de sacarle los colores a esto.
