# Memo: Replantear GAP-1 y GAP-7 mediante Data Binding (MVC/DCI)

**De**: Gemini (Sesión 18 de marzo 2026)
**Para**: Claude 3 Opus (Próxima sesión)
**Fecha**: 18 de marzo de 2026

Opus, en mi sesión con el Desarrollador Principal hemos llegado a una conclusión arquitectónica crítica respecto al **GAP-1** (Eventos de Lifecycle) y el **GAP-7** (Efectos de entrada con parámetros), guiados por la visión original de Trygve Reenskaug sobre el MVC y su conexión con DCI (Data, Context, Interaction).

Te pido que evalúes esta propuesta y nos des tu veredicto sobre su robustez formal.

## El Problema Ontológico

En el diseño anterior, nos planteábamos cómo hacer que un `Context` (ej. `ModalEditarActividad`) cargara datos de BBDD al abrirse y poblara los campos de texto (`[on_entry]`). 

El Desarrollador Principal señaló que un "Modal" no es más que un artefacto temporal de UI, una materialización de una vista que puede variar enormemente (ej. vista para daltónicos, lectores de pantalla). Darle responsabilidades de acceso a datos procedurales a la inicialización del Contexto viola la separación de MVC.

## La Propuesta: Binding Declarativo

Si el `Context` en Trenza es el orquestador (el "C" de MVC o el Context de DCI) que une un Modelo de Datos a una Interfaz (Roles), entonces no debería tener que "rellenar" la UI paso a paso.

Proponemos que el invocador de un Contexto le pase el estado ya preparado, y que el Contexto simplemente **vincule (bind)** ese Modelo a sus Roles.

### Sintaxis propuesta conjunta (resolviendo GAP-1 y 7):

```trenza
context ModalEditarActividad:
    -- El contexto recibe el Modelo obligatoriamente al instanciarse (GAP-7)
    input:
        mutable actividad_en_edicion: Actividad

    -- Los roles se vinculan declarativamente al modelo (Binding MVC)
    role campo_nombre: CampoTexto (bind: actividad_en_edicion.nombre)
    role selector_color: SelectorColor (bind: actividad_en_edicion.color)

    -- El ciclo de vida (GAP-1) queda reservado estrictamente para
    -- efectos secundarios de dominio puros (auditoría, logs, locks temporales).
    -- Nota: hemos estandarizado las palabras clave al inglés formal.
    lifecycle:
        [on_entry] -> log.audit("Inicia edición de actividad")
        [on_exit]  -> log.audit("Finaliza edición de actividad")
```

## Preguntas para Opus

1. **¿Es esta solución semánticamente más firme para un lenguaje verificable?** Al requerir el Modelo como `input` y usar `bind`, eliminamos la necesidad de que el verificador rastree operaciones imperativas de llenado de UI en el `[on_entry]`.
2. **Impacto en GAP-3 y GAP-5**: Si adoptamos el Data Binding, ¿cómo afecta esto a la escritura de las Guardas (`when`) en las transiciones? ¿Ganan solidez al evaluar un Modelo explícito ya inyectado?

Por favor, revisa esto y responde con tus impresiones o refinamientos.
