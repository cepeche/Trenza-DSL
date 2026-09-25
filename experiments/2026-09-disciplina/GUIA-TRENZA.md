# Guía rápida de Trenza

Trenza especifica el comportamiento de una interfaz como una **máquina de
estados por roles**. De un archivo `.trz` se generan la implementación, los
tests y los diagramas. Esta guía cubre lo necesario para modificar una
especificación existente.

## Capas

```
data Tarea:                 -- qué son las cosas (sin comportamiento)
    tareaId: Id

system Cronometro:          -- qué contextos hay y cuál es el inicial
    initial: ModoNormal
    contexts:               -- contextos BASE: exactamente uno activo
        ModoNormal
        ModoEdicion
    overlays:               -- se apilan sobre el base (diálogos, menús)
        ModalAlgo
    concurrent:             -- activos a la vez que el base
        SesionActiva

context ModoNormal:         -- qué hacen las cosas AQUÍ
    role tarjeta: Tarea
        on tap -> iniciarTarea(self.tareaId)
    role boton_edicion: Boton
        on tap -> activarEdicion
    transitions:
        on activarEdicion -> ModoEdicion
```

Los comentarios empiezan por `--`. La indentación no es significativa.

## Roles, eventos y acciones

- Un **rol** asocia un nombre a un tipo de dato: `role tarjeta: Tarea`.
- Cada línea `on <evento> -> <destino>` dice qué hace ese rol cuando recibe
  ese evento **en ese contexto**. El destino es:
  - una **acción**: `iniciarTarea(self.tareaId)` o simplemente `activarEdicion`;
  - `ignored`: el evento no hace nada, y así se ha decidido;
  - `forbidden`: recibir el evento es un error (no debería poder ocurrir).
- El mismo rol puede hacer cosas distintas en contextos distintos: esa es la
  idea central. No hay condicionales del tipo `if (modoEdicion)`.

## Transiciones

`transitions:` asocia **acciones** (no eventos) a un destino:

```
transitions:
    on activarEdicion -> ModoEdicion      -- cambia de contexto base
    on abrirMenu      -> MenuConfig       -- abre un overlay (se apila)
    on cerrar         -> [close_overlay]  -- cierra el overlay actual
    on algo           -> [stay]           -- se queda donde está
```

Cuando un rol produce una acción y el contexto activo tiene una transición
para esa acción, se ejecuta la transición. Si la acción no tiene
transición, no cambia de contexto.

Un overlay puede declarar `initial: SubContexto`: al abrirlo, se entra
directamente en ese sub-contexto.

## Reglas que comprueba el verificador

`trenza-cli check archivo.trz` comprueba, entre otras:

1. **Completitud (R1)** y **exhaustividad de roles (R5)**: los contextos
   **hermanos** deben declarar los mismos roles y los mismos pares
   rol·evento. Son hermanos: todos los contextos base entre sí; y los
   sub-contextos de un mismo overlay entre sí. Un overlay no necesita
   declarar los roles del base que tapa. Si en un contexto un rol no debe
   hacer nada, se escribe explícitamente `on tap -> ignored` (o
   `forbidden`).
2. **Determinismo (R2)**: un rol no puede tener dos manejadores del mismo
   evento en el mismo contexto.
3. **Alcanzabilidad (R3, aviso)**: todo contexto debe ser alcanzable desde
   el inicial.
4. **Retorno (R4)**: desde todo contexto se debe poder volver al inicial.
5. **Consistencia de tipos (R8)**: un nombre de rol tiene el mismo tipo en
   todos los contextos.

Si la verificación pasa, imprime `Verificación Semántica: Superada`. Si no,
lista los diagnósticos con línea y columna.
