# DiseÃ±o del lenguaje â€” Trenza DSL

**Estado**: propuesta de diseÃ±o â€” en revisiÃ³n (actualizado con Seguridad por DiseÃ±o + decisiones pendientes resueltas)
**Fecha**: 4â€“6 de marzo de 2026; actualizado 12 de marzo de 2026
**Participantes**: desarrollador + Claude Sonnet 4.6 + Claude Opus 4.6 + Gemini 3.1 Pro
**Referencias de diseÃ±o**: Parnas & Clements (A Rational Design Process: How and Why to Fake It, 1986); Reenskaug (DCI)

---

## Principio rector: la legibilidad es un requisito formal

Los mÃ©todos formales tienen un problema de adopciÃ³n. No es que los ingenieros
de software no necesiten razonamiento formal â€” es que la notaciÃ³n habitual
(lÃ³gica temporal, Ã¡lgebra relacional, tipos dependientes) aleja a la mayorÃ­a
de profesionales que mÃ¡s se beneficiarÃ­an de ellos.

Trenza toma una posiciÃ³n deliberada: **las propiedades formales se expresan
como reglas legibles, no como fÃ³rmulas**. El rigor no viene de la notaciÃ³n
sino de la estructura. Un DSL bien diseÃ±ado puede ser tan verificable como
TLA+ sin necesitar que el usuario sepa quÃ© es un operador temporal.

Esto no es una concesiÃ³n a la simplicidad. Es una decisiÃ³n de diseÃ±o: si el
DSL es ilegible para un ingeniero de software medio, ha fracasado en su
objetivo, independientemente de cuÃ¡n formalmente correcto sea.

---

## Influencia: soluciones autocontenidas

El trabajo de Reuven Cohen (rUv) sobre soluciones autocontenidas aporta un
principio que encaja naturalmente con trenza.

Cohen construye sistemas que empaquetan todo lo necesario para funcionar y
verificarse en un solo artefacto. Su proyecto RuView (captura de movimiento
vÃ­a WiFi) es un ejemplo extremo: sensores ESP32 a 8 dÃ³lares que procesan
seÃ±ales WiFi localmente, sin internet, sin nube, sin dependencias externas.
El artefacto incluye modelo, runtime de inferencia y verificaciÃ³n
criptogrÃ¡fica â€” todo en un solo archivo binario (formato RVF).

El principio relevante para trenza no es tÃ©cnico sino arquitectÃ³nico:

> Cada artefacto debe contener todo lo necesario para verificar su propia
> correcciÃ³n, sin depender de nada externo.

En RuView, puedes ejecutar `./verify` con solo Python y numpy â€” sin hardware
WiFi â€” y obtener una validaciÃ³n completa del pipeline de procesamiento.

Trenza adopta este principio: **cada contexto es una unidad autocontenida**.
Contiene su especificaciÃ³n, genera su implementaciÃ³n, sus tests y su
esquemÃ¡tico, y puede verificarse en aislamiento. No necesitas ejecutar la
aplicaciÃ³n completa para saber si un contexto estÃ¡ bien definido.

Hay una conexiÃ³n adicional con la metodologÃ­a SPARC de Cohen (Specification,
Pseudocode, Architecture, Refinement, Completion): ambas parten de que la
especificaciÃ³n es el artefacto primario. Pero en SPARC, la especificaciÃ³n
es un documento que humanos y LLMs leen. En trenza, **la especificaciÃ³n es
el programa**. No hay brecha entre lo que se especifica y lo que se ejecuta.

---

## La unidad mÃ­nima de especificaciÃ³n: el contexto

La primera pregunta abierta del concepto inicial era: *Â¿quÃ© es la unidad
mÃ­nima de especificaciÃ³n?*

La respuesta es el **contexto** â€” en el sentido DCI de Reenskaug.

Un contexto es la menor porciÃ³n de especificaciÃ³n que es autocontenida y
verificable por sÃ­ misma. Contiene:

- Un **nombre** que corresponde a un caso de uso del dominio.
- Los **roles** que participan en ese caso de uso.
- Los **eventos** que cada rol puede recibir.
- Las **acciones** que resultan de cada evento.
- Las **transiciones** a otros contextos.
- Los **efectos** que se producen (llamadas API, cambios en DOM, etc.).

No existe una especificaciÃ³n trenza mÃ¡s pequeÃ±a que un contexto. Un evento
suelto no tiene significado. Un rol sin contexto no tiene comportamiento.
Un contexto es el Ã¡tomo del sistema â€” indivisible y autocontenido.

---

## Seguridad y privacidad por diseÃ±o (compliance estructural)

La legislaciÃ³n actual (RGPD Art. 25, ENS, CRA, NIS2) exige responsabilidad
sobre la seguridad y privacidad de los sistemas. Sin embargo, las herramientas
de desarrollo tradicionales no ofrecen trazabilidad de estas exigencias,
generando una "deuda tÃ©cnica" donde la responsabilidad es jurÃ­dicamente
exigible pero tÃ©cnicamente inescrutable.

Trenza convierte parte de estas aspiraciones legislativas en verificaciones
del compilador, haciendo que el cumplimiento normativo sea estructural y
demostrable â€” no documental. Esto no resuelve la deuda tÃ©cnica acumulada,
pero crea el tipo de artefacto sobre el que puede construirse responsabilidad
formal trazable.

### MÃ­nimo privilegio estructural (ENS / CRA)

En trenza, si un evento no estÃ¡ cableado a una acciÃ³n en un contexto
especÃ­fico, no existe. No es una polÃ­tica de control de acceso que pueda
fallar por omisiÃ³n â€” es topologÃ­a pura. El uso explÃ­cito de `bloqueado`
documenta y garantiza la denegaciÃ³n por defecto. La Regla de Completitud
garantiza que ningÃºn evento queda sin manejador declarado.

### Resiliencia y recuperaciÃ³n segura (CRA / NIS2)

Al ser una mÃ¡quina de estados estricta, el sistema siempre arranca en el
contexto `initial` declarado. No existen estados intermedios huÃ©rfanos
derivados de variables booleanas inconsistentes â€” el bug original del
cronÃ³metro que motivÃ³ este proyecto.

### Cadena de custodia criptogrÃ¡fica

El paquete `.tzp` con `manifest.json` y checksums vincula
matemÃ¡ticamente la especificaciÃ³n (`.trz`), la implementaciÃ³n generada
(`.wasm`) y los tests. Si el cÃ³digo en producciÃ³n no corresponde a la
especificaciÃ³n firmada, el checksum lo evidencia. Es un artefacto firmable
legalmente y auditable antes de que el cÃ³digo se ejecute.

### Trazabilidad de auditorÃ­a (RGPD Art. 30)

Como todas las transiciones de estado ocurren a travÃ©s del DSL, el
compilador *puede* inyectar automÃ¡ticamente llamadas a un mÃ³dulo de
auditorÃ­a en el cÃ³digo Rust generado. Para ello, `system.trz` debe
declarar el mÃ³dulo de auditorÃ­a destino:

```
system MiSistema:
    audit: external modulo_auditoria
```

Cuando se declara, ningÃºn desarrollador puede olvidar auditar una
transiciÃ³n de contexto â€” el cÃ³digo de auditorÃ­a es generado, no escrito.

### ClasificaciÃ³n de datos y conformidad de flujo (RGPD Art. 25.1)

Ver secciÃ³n "DeclaraciÃ³n de datos" y Regla 6 en "VerificaciÃ³n".

---

## GramÃ¡tica del DSL

La gramÃ¡tica de trenza usa indentaciÃ³n (como Python) y palabras clave legibles.
No hay operadores simbÃ³licos excepto `->` para indicar consecuencia.

### DeclaraciÃ³n de datos

Los datos se declaran fuera de los contextos. Son estructura pura â€” sin
comportamiento. Un dato es "lo que algo es"; un rol en un contexto es
"lo que algo hace aquÃ­".

```
data <nombre> [clasificacion: <etiqueta>]:
    <campo>: <tipo>
```

La anotaciÃ³n `[clasificacion:]` es opcional pero verificada: si un dato
tiene `[clasificacion: personal]`, el verificador aplicarÃ¡ la Regla 6
para garantizar que solo fluye hacia mÃ³dulos `external` explÃ­citamente
autorizados para esa clasificaciÃ³n.

### Estructura de un contexto

```
context <nombre>:
    [requires: <condiciones>]

    role <nombre_rol>: <tipo_dato>
        on <evento> -> <acciÃ³n>
        [on <evento> -> ignorar]

    [context <nombre_hijo>:       -- contexto anidado (mÃ¡x. 2 niveles)
        ...]

    [transitions:
        on <evento> -> <otro_contexto>]

    [effects:
        <acciÃ³n> -> <descripciÃ³n_del_efecto>]
```

### Reglas sintÃ¡cticas

- Los nombres de contexto empiezan con mayÃºscula: `ModoEdicion`, `SesionActiva`.
- Los nombres de dato empiezan con mayÃºscula: `TipoTarea`, `Actividad`.
- Los nombres de rol empiezan con minÃºscula: `tarjeta`, `pestaÃ±a_actividad`.
- Los eventos empiezan con minÃºscula: `tap`, `doble_tap`, `mantener`.
- Las acciones empiezan con minÃºscula: `mostrarModal`, `iniciarTarea`.
- La palabra `ignorar` significa "este evento estÃ¡ contemplado y no hace nada".
- Los comentarios usan `--` (doble guion, como SQL y Haskell).
- Los roles se vinculan a un tipo de dato con `:`. El `self` dentro del rol
  refiere a las propiedades de ese dato.

### Vocabulario reservado

| Palabra | Significado |
|---------|-------------|
| `system` | Declara el sistema completo con sus contextos |
| `data` | Declara un tipo de dato (estructura sin comportamiento) |
| `context` | Declara un contexto (caso de uso) |
| `role` | Declara un rol dentro de un contexto |
| `on` | Declara un manejador de evento |
| `->` | Indica consecuencia: evento -> acciÃ³n |
| `ignorar` | El evento estÃ¡ contemplado pero no produce acciÃ³n |
| `bloqueado` | El evento estÃ¡ explÃ­citamente prohibido en este contexto |
| `requires` | CondiciÃ³n necesaria para que el contexto estÃ© activo |
| `transitions` | Declara cambios de contexto |
| `effects` | Declara efectos secundarios asociados a acciones |
| `external` | Marca una acciÃ³n implementada en cÃ³digo convencional |

---

## Ejemplo completo: el cronÃ³metro PSP

El cronÃ³metro-psp tiene cinco condicionales dispersos que dependen del booleano
`AppState.modoEdicion` (ver `frontend/js/app.js`, lÃ­neas 286, 323, 411, 651, 662).

En trenza, esos cinco condicionales desaparecen. Primero se declaran los datos
(estructura pura), luego los contextos (comportamiento puro):

```
-- Capa Data: quÃ© son las cosas (sin comportamiento)
data TipoTarea:
    tipoId: Id
    nombre: Texto
    icono: Texto

data Tarea:
    tareaId: Id
    tipoId: Id

data Actividad:
    id: Id
    nombre: Texto
    color: Color

data PestaÃ±a:
    id: Id

-- Capa System: declaraciÃ³n del sistema
system CronometroPSP:
    initial: ModoNormal
    events: tap

-- Capa Context: quÃ© hacen las cosas aquÃ­ (sin estructura)

context ModoNormal:

    role tipo_tarea: TipoTarea
        on tap -> seleccionarTipo(self.tipoId)

    role tarea: Tarea
        on tap -> iniciarTarea(self.tareaId)

    role pestaÃ±a_actividad: Actividad
        on tap -> cambiarPestaÃ±a(self.id)

    role pestaÃ±a_frecuentes: PestaÃ±a
        on tap -> cambiarPestaÃ±a('frecuentes')

    transitions:
        on activarEdicion -> ModoEdicion

    effects:
        iniciarTarea -> POST /api/sesiones { tipoTareaId, comentario }
        cambiarPestaÃ±a -> actualizarUI()

context ModoEdicion:

    role tipo_tarea: TipoTarea
        on tap -> mostrarModalEditar(self.tipoId)

    role tarea: Tarea
        on tap -> mostrarModalEditar(self.tipoId)

    role pestaÃ±a_actividad: Actividad
        on tap -> mostrarModalEditarActividad(self.id)

    role pestaÃ±a_frecuentes: PestaÃ±a
        on tap -> ignorar                         -- explÃ­cito: no se puede editar

    transitions:
        on desactivarEdicion -> ModoNormal

    effects:
        mostrarModalEditar -> GET /api/tipos-tarea?id=
        mostrarModalEditarActividad -> GET /api/actividades?id=
```

### QuÃ© hace visible esta especificaciÃ³n

1. **Los cinco condicionales del cÃ³digo actual no existen.** No hay `if (modoEdicion)`
   en ningÃºn sitio. La factorÃ­a (que genera el `system`) decide quÃ© contexto estÃ¡
   activo; el resto es polimÃ³rfico.

2. **`pestaÃ±a_frecuentes` en `ModoEdicion` dice `ignorar`**, no simplemente la omite.
   Si fuera omitida, el verificador reportarÃ­a: "el rol `pestaÃ±a_frecuentes` maneja
   el evento `tap` en `ModoNormal` pero no en `ModoEdicion`". El olvido es imposible.

3. **La acciÃ³n `iniciarTarea` no aparece en `ModoEdicion`** porque ningÃºn camino
   conduce a ella desde ese contexto. No hace falta un guard defensivo
   `if (modoEdicion) return` porque la topologÃ­a lo impide.

4. **Los efectos son declarativos.** La especificaciÃ³n dice *quÃ©* se comunica con el
   exterior, no *cÃ³mo*. El cÃ³mo vive en el cÃ³digo generado o en un mÃ³dulo `external`.

### ComparaciÃ³n lado a lado

| CÃ³digo actual (app.js) | Trenza |
|---|---|
| `if (AppState.modoEdicion)` en 5 sitios | 0 condicionales; 2 contextos |
| Guard olvidado = bug silencioso | Rol sin manejador = error de verificaciÃ³n |
| Efectos mezclados con lÃ³gica de control | Efectos declarados por separado |
| Estado implÃ­cito en booleano global | Estado explÃ­cito como contexto activo |
| Diagrama: solo si alguien lo dibuja | EsquemÃ¡tico Mermaid auto-generado |

---

## QuÃ© se genera: las cuatro hebras

Cada contexto trenza genera cuatro artefactos â€” las cuatro hebras de la trenza:

### Hebra 1: ImplementaciÃ³n

Para el sistema completo, el generador produce Rust. Los contextos se
traducen a un enum, y cada combinaciÃ³n rol+evento se convierte en una
funciÃ³n con `match` exhaustivo:

```rust
pub enum Contexto {
    ModoNormal,
    ModoEdicion,
}

pub fn handle_tipo_tarea_tap(ctx: &Contexto, tipo_tarea: &TipoTarea) -> Accion {
    match ctx {
        Contexto::ModoNormal => Accion::SeleccionarTipo(tipo_tarea.tipo_id),
        Contexto::ModoEdicion => Accion::MostrarModalEditar(tipo_tarea.tipo_id),
    }
}

pub fn handle_pestaÃ±a_frecuentes_tap(ctx: &Contexto) -> Accion {
    match ctx {
        Contexto::ModoNormal => Accion::CambiarPestaÃ±a("frecuentes"),
        Contexto::ModoEdicion => Accion::Ignorar,
    }
}

// AÃ±adir un nuevo contexto al enum sin actualizar estos match
// produce un error de compilaciÃ³n. Rust impone la completitud.
```

La elecciÃ³n de Rust como destino no es casual. El `match` exhaustivo
de Rust impone la misma regla de completitud que el verificador de trenza,
pero a nivel de compilaciÃ³n del cÃ³digo generado. Es verificaciÃ³n doble:
trenza verifica la especificaciÃ³n; `rustc` verifica la implementaciÃ³n.

El cÃ³digo generado se compila a WASM para despliegue en frontend y backend,
alineÃ¡ndose con el principio de autocontenciÃ³n: un mÃ³dulo `.wasm` lleva
todo lo que necesita, sin runtime externo.

### Hebra 2: Tests

Para el mismo sistema, el reverso algebraico:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modo_edicion_tipo_tarea_tap_muestra_modal() {
        let ctx = Contexto::ModoEdicion;
        let tipo = TipoTarea { tipo_id: 42, nombre: "Test".into(), icono: "ðŸ“".into() };
        let resultado = handle_tipo_tarea_tap(&ctx, &tipo);
        assert_eq!(resultado, Accion::MostrarModalEditar(42));
    }

    #[test]
    fn modo_edicion_pestaÃ±a_frecuentes_tap_ignora() {
        let ctx = Contexto::ModoEdicion;
        let resultado = handle_pestaÃ±a_frecuentes_tap(&ctx);
        assert_eq!(resultado, Accion::Ignorar);
    }

    #[test]
    fn modo_normal_tipo_tarea_tap_selecciona() {
        let ctx = Contexto::ModoNormal;
        let tipo = TipoTarea { tipo_id: 7, nombre: "Debug".into(), icono: "ðŸ”§".into() };
        let resultado = handle_tipo_tarea_tap(&ctx, &tipo);
        assert_eq!(resultado, Accion::SeleccionarTipo(7));
    }
}
```

Cada pareja evento-acciÃ³n produce exactamente un test por contexto.
No hay tests que escribir manualmente: si la especificaciÃ³n cambia,
los tests cambian.

### Hebra 3: EsquemÃ¡tico

El diagrama Mermaid auto-generado para el sistema completo:

```mermaid
stateDiagram-v2
    [*] --> ModoNormal
    ModoNormal --> ModoEdicion : activarEdicion
    ModoEdicion --> ModoNormal : desactivarEdicion

    state ModoNormal {
        tap_tipo_tarea --> seleccionarTipo
        tap_tarea --> iniciarTarea
        tap_pestaÃ±a_actividad --> cambiarPestaÃ±a
        tap_pestaÃ±a_frecuentes --> cambiarPestaÃ±a
    }

    state ModoEdicion {
        tap_tipo_tarea --> mostrarModalEditar
        tap_tarea --> mostrarModalEditar
        tap_pestaÃ±a_actividad --> mostrarModalEditarActividad
        tap_pestaÃ±a_frecuentes --> ignorar
    }
```

Las cuatro hebras son proyecciones del mismo artefacto. Modificar una
implica regenerar las otras tres. No pueden desincronizarse.

---

## VerificaciÃ³n sin notaciÃ³n simbÃ³lica

Trenza verifica propiedades formales expresÃ¡ndolas como reglas legibles.
Cada regla puede comprobarse por inspecciÃ³n de la especificaciÃ³n, sin
ejecutar cÃ³digo.

### Regla 1: Completitud

**Enunciado**: Todo rol que maneje un evento en algÃºn contexto debe
manejar ese mismo evento en todos los contextos del sistema, aunque sea
con `ignorar` o `bloqueado`.

**Ejemplo**: Si `pestaÃ±a_frecuentes` responde a `tap` en `ModoNormal`,
debe responder a `tap` en `ModoEdicion`. Si no lo hace, el verificador
reporta:

```
ERROR [completitud]: pestaÃ±a_frecuentes.tap definido en ModoNormal
                     pero ausente en ModoEdicion
```

**QuÃ© previene**: El bug original â€” un evento sin manejador en un contexto.

### Regla 2: Determinismo

**Enunciado**: En un contexto dado, cada evento de cada rol produce
exactamente una acciÃ³n. No hay ambigÃ¼edad.

**Ejemplo**: Si alguien escribe:

```
role tarjeta:
    on tap -> mostrarModalEditar
    on tap -> seleccionarTipo          -- ERROR
```

El verificador reporta:

```
ERROR [determinismo]: tarjeta.tap tiene dos acciones en ModoEdicion
```

**QuÃ© previene**: Comportamiento impredecible por manejadores duplicados.

### Regla 3: Alcanzabilidad

**Enunciado**: Todo contexto debe poder alcanzarse desde el contexto
inicial a travÃ©s de alguna secuencia de transiciones.

**Ejemplo**: Si se define un contexto `ModoMantenimiento` pero ningÃºn
otro contexto tiene una transiciÃ³n hacia Ã©l:

```
ERROR [alcanzabilidad]: ModoMantenimiento no es alcanzable desde
                        ModoNormal (contexto inicial)
```

**QuÃ© previene**: CÃ³digo muerto â€” contextos que se especifican pero
nunca se activan.

### Regla 4: Retorno

**Enunciado**: Todo contexto no inicial debe tener al menos una
transiciÃ³n que, directa o indirectamente, regrese al contexto inicial.

**QuÃ© previene**: Estados sumidero â€” contextos de los que no se puede
salir.

### Regla 5: Exhaustividad de roles

**Enunciado**: Todo rol declarado en el bloque `system` debe aparecer
en todos los contextos del sistema.

**Ejemplo**: Si `system` declara el rol `pestaÃ±a_frecuentes` pero
`ModoEdicion` no lo menciona:

```
ERROR [exhaustividad]: rol pestaÃ±a_frecuentes declarado en el sistema
                       pero ausente del contexto ModoEdicion
```

**QuÃ© previene**: Roles olvidados â€” elementos del interfaz cuyo
comportamiento en un contexto no fue considerado.

### Regla 6: Conformidad de datos (RGPD Art. 25.1)

**Enunciado**: NingÃºn dato con `[clasificacion: X]` puede pasarse como
parÃ¡metro a una acciÃ³n `external` que no declare explÃ­citamente
`[autorizado_para: X]`.

**Ejemplo**: Si `DatosSesion` estÃ¡ marcado como `[clasificacion: personal]`
y un contexto intenta pasarlo a un mÃ³dulo sin autorizaciÃ³n:

```
ERROR [conformidad]: DatosSesion [clasificacion: personal] fluye hacia
                     modulo_analytics que no declara [autorizado_para: personal]
```

**QuÃ© previene**: Violaciones de finalidad y fugas de datos por diseÃ±o â€”
el compilador hace imposible enviar datos personales a un destino no
autorizado, incluso por error u omisiÃ³n.

### Nota sobre equivalencia formal

Estas seis reglas son equivalentes a las propiedades que se expresarÃ­an
con lÃ³gica temporal en TLA+ o con invariantes en Alloy. La diferencia es
que un ingeniero de software puede leerlas, discutirlas con su equipo, y
verificarlas con una herramienta que emite mensajes en lenguaje natural.

Las reglas 1â€“5 verifican correcciÃ³n de comportamiento. La regla 6 verifica
conformidad normativa. Ambas clases de propiedad son ciudadanos de primera
clase en el verificador.

---

## ComposiciÃ³n de contextos

La quinta pregunta abierta era: *Â¿cÃ³mo se expresa `ModoEdicion + SesiÃ³nActiva`?*

La soluciÃ³n DCI (documentada en `2026-03-04-04-influencias-dci.md`) se mantiene: los contextos
coexisten, no se combinan. Pero la composiciÃ³n necesita reglas de prioridad
cuando dos contextos activos asignan comportamientos distintos al mismo rol
para el mismo evento.

### Reglas de composiciÃ³n

```
system CronometroPSP:
    initial: ModoNormal

    -- Los contextos se declaran en orden de prioridad descendente
    contexts:
        SesionActiva        -- mayor prioridad
        ModoEdicion
        ModoNormal          -- menor prioridad (base)

    composition: prioridad  -- el contexto de mayor prioridad prevalece
```

Si `SesionActiva` y `ModoEdicion` ambos definen un manejador para
`tarea.tap`, prevalece el de `SesionActiva`. Si `SesionActiva` no define
ese manejador, se busca en `ModoEdicion`, y luego en `ModoNormal`.

Alternativa: `composition: exclusiva` â€” solo el contexto activo de mayor
prioridad tiene efecto. Los demÃ¡s se ignoran completamente. Esto es mÃ¡s
simple pero menos expresivo.

La elecciÃ³n entre prioridad y exclusiva es una decisiÃ³n del diseÃ±ador del
sistema, no del lenguaje. Trenza ofrece ambas.

---

## Efectos secundarios

Los efectos (llamadas API, modificaciones al DOM, navegaciÃ³n) se declaran
en el contexto pero no se ejecutan por el DSL. El DSL genera la interfaz;
el runtime la implementa.

```
context ModoEdicion:
    role tipo_tarea: TipoTarea
        on tap -> mostrarModalEditar(self.tipoId)

    effects:
        mostrarModalEditar -> external cargar_datos_tarea(tipo_id)
```

La palabra `external` indica que `cargar_datos_tarea` es una funciÃ³n
Rust convencional que el cÃ³digo generado invoca. Trenza no la genera â€”
espera encontrarla en el entorno destino.

Esto resuelve la segunda pregunta abierta: los efectos se declaran en
trenza pero se implementan fuera de trenza. El DSL define *quÃ©* efectos
ocurren; el cÃ³digo convencional define *cÃ³mo*.

---

## Interoperabilidad con cÃ³digo convencional

Trenza no pretende reemplazar todo el cÃ³digo de una aplicaciÃ³n. Pretende
gobernar la lÃ³gica de estados y eventos, delegando el resto.

### MÃ³dulos external

```
external module cronometro_api:
    cargar_datos_tarea(tipo_id: Id) -> TipoTarea
    guardar_edicion(tipo_id: Id, datos: DatosEdicion) -> Resultado
    iniciar_sesion(tarea_id: Id, comentario: Texto) -> Sesion
```

El bloque `external` declara funciones que existen en cÃ³digo Rust
convencional. Trenza las trata como cajas negras: conoce su firma
pero no su implementaciÃ³n. El cÃ³digo generado produce un `trait`
que el cÃ³digo convencional debe implementar:

```rust
// Generado por trenza
pub trait CronometroApi {
    fn cargar_datos_tarea(&self, tipo_id: Id) -> TipoTarea;
    fn guardar_edicion(&self, tipo_id: Id, datos: DatosEdicion) -> Resultado;
    fn iniciar_sesion(&self, tarea_id: Id, comentario: &str) -> Sesion;
}
```

Los tests generados usan mocks para este trait. Los tests de
integraciÃ³n (que verifican la implementaciÃ³n real) estÃ¡n fuera del
alcance de trenza â€” pertenecen al cÃ³digo convencional.

---

## Capa de datos: separaciÃ³n de estructura y comportamiento

La capa de datos resuelve la quinta pregunta abierta y evita el
problema clÃ¡sico de la herencia en diamante.

### El problema

En OO clÃ¡sico, la herencia mezcla dos preguntas distintas:

- "Â¿QuÃ© es esto?" â€” estructura, propiedades (herencia de datos).
- "Â¿QuÃ© hace esto aquÃ­?" â€” comportamiento en contexto (herencia funcional).

Cuando ambas viven en la misma jerarquÃ­a (`class Tarjeta extends
ElementoUI implements Editable`), la herencia mÃºltiple produce
el diamante: Â¿de quiÃ©n hereda `Tarjeta` su mÃ©todo `onClick`?

### La soluciÃ³n DCI en trenza

Trenza no tiene este problema porque las dos preguntas viven en
capas separadas que nunca se cruzan:

| Capa | Pregunta | Mecanismo | Herencia |
|------|----------|-----------|----------|
| `data` | "Â¿QuÃ© es?" | DeclaraciÃ³n de campos | No hay. Datos planos. |
| `context` + `role` | "Â¿QuÃ© hace aquÃ­?" | Manejadores de eventos | No hay. Contextos aislados. |

Un rol no *es* un TipoTarea â€” *actÃºa sobre* un TipoTarea en un
contexto dado. Fuera de ese contexto, el TipoTarea es un dato sin
comportamiento. No hay jerarquÃ­a que pueda formar un diamante.

Si dos roles necesitan las mismas propiedades, se vinculan al
mismo tipo de dato â€” no heredan de una clase base comÃºn:

```
context ModoEdicion:
    role tipo_tarea: TipoTarea        -- mismo dato, distinto rol
        on tap -> mostrarModalEditar(self.tipoId)

    role tarea: Tarea                 -- dato diferente, mismo evento
        on tap -> mostrarModalEditar(self.tipoId)
```

---

## Contextos anidados

Los contextos pueden anidarse para expresar sub-estados dentro de un
contexto padre. Esto se alinea con los statecharts jerÃ¡rquicos de Harel
(1987) que XState implementa hoy.

### Ejemplo

`ModoEdicion` tiene dos sub-estados: editando una tarea o editando
una actividad. Los sub-contextos heredan los manejadores del padre
y pueden sobreescribir los que necesiten:

```
context ModoEdicion:

    role pestaÃ±a_frecuentes: PestaÃ±a
        on tap -> ignorar

    transitions:
        on desactivarEdicion -> ModoNormal

    -- Sub-contexto: editando una tarea especÃ­fica
    context EditandoTarea:
        role campo_nombre: CampoTexto
            on cambio -> actualizarNombre(self.valor)
        role boton_guardar: Boton
            on tap -> guardarEdicion()

        transitions:
            on guardarEdicion -> ModoEdicion     -- vuelve al padre
            on cancelar -> ModoEdicion

    -- Sub-contexto: editando una actividad
    context EditandoActividad:
        role campo_nombre: CampoTexto
            on cambio -> actualizarNombreActividad(self.valor)
        role selector_color: SelectorColor
            on seleccion -> actualizarColor(self.valor)

        transitions:
            on guardarEdicionActividad -> ModoEdicion
            on cancelar -> ModoEdicion
```

### Reglas de encapsulamiento

1. **Ãmbito cerrado**: Un contexto hijo solo puede transicionar a su
   padre o a un hermano del mismo nivel. No puede saltar directamente
   a un contexto de otro padre. `EditandoTarea` no puede ir a
   `ModoNormal` â€” debe pasar por `ModoEdicion`.

2. **VerificaciÃ³n independiente**: Cada contexto anidado es verificable
   por sÃ­ mismo. Las reglas se aplican dentro de su Ã¡mbito.

3. **Profundidad limitada**: MÃ¡ximo dos niveles de anidamiento. Si se
   necesita mÃ¡s, el sistema probablemente necesita descomponerse en
   subsistemas, no en contextos mÃ¡s profundos.

### Reglas de herencia en contextos anidados (resuelto 12 marzo 2026)

> DecisiÃ³n documentada en `2026-03-12-02-decisiones-pendientes-claude.md`
> (Sonnet) y `2026-03-12-03-decisiones-pendientes-opus.md` (Opus).

**Regla H1: Herencia implÃ­cita de roles.**
Un contexto hijo hereda automÃ¡ticamente todos los roles del padre con sus
vÃ­nculos de tipo. `EditandoTarea` no dice nada sobre `pestaÃ±a_frecuentes`,
asÃ­ que hereda el `ignorar` de `ModoEdicion`. No puede cambiar el vÃ­nculo
de tipo de un rol heredado.

**Regla H2: Roles locales.**
Un contexto hijo puede declarar roles nuevos que no existen en el padre.
Estos roles son locales: solo existen en ese hijo y sus propios hijos.
No son visibles desde el padre ni desde contextos hermanos.

**Regla H3: Completitud por niveles.**
La Regla de Completitud se aplica de forma independiente en cada nivel
de anidamiento. Un rol local de `EditandoTarea` no tiene que aparecer en
`EditandoActividad` ni en `ModoEdicion`. Los roles heredados mantienen
el handler del padre salvo sobrescritura explÃ­cita.

**Regla H4: Sobrescritura explÃ­cita.**
Si un hijo quiere cambiar el handler de un rol heredado, debe re-declarar
el rol completo con sus nuevos handlers. No se puede "mencionar" un rol
sin handlers â€” serÃ­a ambiguo. El verificador emite una nota informativa
cuando detecta una sobrescritura:

```
NOTA [herencia]: EditandoTarea sobrescribe pestaÃ±a_frecuentes.tap
                 (padre: ignorar â†’ hijo: mostrarAyudaEdicion)
```

**Regla H5: ProhibiciÃ³n de eventos nuevos en roles heredados.**
Un hijo no puede aÃ±adir handlers para eventos que el padre no declarÃ³
en un rol heredado. Si `pestaÃ±a_frecuentes` responde solo a `tap` en el
padre, el hijo no puede aÃ±adirle `doble_tap`. Esto mantiene estable el
catÃ¡logo de eventos por rol en toda la jerarquÃ­a. La alternativa es
declarar un rol local con el mismo tipo de dato:

```
context EditandoTarea:
    -- pestaÃ±a_frecuentes se hereda con on tap -> ignorar

    role pestana_edicion: PestaÃ±a        -- rol local, mismo tipo
        on doble_tap -> mostrarOpciones

    role campo_nombre: CampoTexto
        on cambio -> actualizarNombre(self.valor)
```

**Independencia entre hermanos.**
Roles locales de un contexto hermano no obligan al otro. Si
`EditandoTarea` y `EditandoActividad` declaran ambos `campo_nombre:
CampoTexto`, son roles locales independientes. La Regla de Completitud
no cruza entre hermanos.

### InspecciÃ³n: vista expandida

El CLI incluye un comando de inspecciÃ³n que muestra la vista expandida
de cualquier contexto, haciendo visible la herencia sin forzar al autor
a repetirla en el cÃ³digo fuente:

```
trenza inspect contexts/ModoEdicion/EditandoTarea.trz
```

```
context EditandoTarea (hijo de ModoEdicion):

  [heredado] role pestaÃ±a_frecuentes: PestaÃ±a
      on tap -> ignorar

  [local] role campo_nombre: CampoTexto
      on cambio -> actualizarNombre(self.valor)

  [local] role boton_guardar: Boton
      on tap -> guardarEdicion()

  transitions:
      on guardarEdicion -> ModoEdicion
      on cancelar -> ModoEdicion
```

---

## Formato de archivo y paquetes

### ExtensiÃ³n

La extensiÃ³n `.hlx` estÃ¡ ocupada (HLX Deterministic Language, presets de
Line 6 Trenza, namespace de Adobe AEM). La extensiÃ³n elegida es **`.trz`**
para archivos fuente individuales.

### Estructura: un archivo por contexto

Cada contexto vive en su propio archivo `.trz`. Esto permite:

- **GeneraciÃ³n incremental**: al modificar un contexto, solo se
  regeneran sus artefactos.
- **Trabajo paralelo**: distintos desarrolladores (o LLMs) pueden
  trabajar en contextos distintos sin conflictos.
- **VerificaciÃ³n parcial**: se puede verificar un solo contexto sin
  procesar el sistema completo.

### Paquete: archivo ZIP autocontenido

Inspirado en formatos como .3mf, .epub y .docx, un sistema trenza
completo se empaqueta como un archivo ZIP con extensiÃ³n `.tzp`:

```
cronometro-psp.tzp  (ZIP)
â”‚
â”œâ”€â”€ mimetype                          -- "application/trenza-dsl" (sin comprimir)
â”œâ”€â”€ manifest.json                     -- mapa de partes, checksums, versiÃ³n
â”‚
â”œâ”€â”€ system.trz                      -- declaraciÃ³n del sistema (punto de entrada)
â”œâ”€â”€ data.trz                        -- declaraciones de datos
â”‚
â”œâ”€â”€ contexts/
â”‚   â”œâ”€â”€ ModoNormal.trz              -- un archivo por contexto
â”‚   â”œâ”€â”€ ModoEdicion.trz
â”‚   â””â”€â”€ ModoEdicion/
â”‚       â”œâ”€â”€ EditandoTarea.trz       -- contextos anidados
â”‚       â””â”€â”€ EditandoActividad.trz
â”‚
â”œâ”€â”€ external/
â”‚   â””â”€â”€ cronometro_api.trz          -- mÃ³dulos external
â”‚
â”œâ”€â”€ generated/
â”‚   â”œâ”€â”€ impl/
â”‚   â”‚   â””â”€â”€ cronometro_psp.rs         -- hebra 1: implementaciÃ³n Rust
â”‚   â”œâ”€â”€ tests/
â”‚   â”‚   â””â”€â”€ cronometro_psp_test.rs    -- hebra 2: tests Rust
â”‚   â””â”€â”€ schematics/
â”‚       â””â”€â”€ system.mermaid            -- hebra 3: esquemÃ¡tico
â”‚
â””â”€â”€ verification/
    â””â”€â”€ report.json                   -- resultado de las 5 reglas
```

El `manifest.json` contiene checksums de cada archivo. Al modificar un
contexto, la herramienta compara checksums para regenerar solo los
artefactos afectados.

La estructura de directorios refleja la jerarquÃ­a de contextos:
`contexts/ModoEdicion/EditandoTarea.trz` es hijo de
`contexts/ModoEdicion.trz`.

Este formato encarna el principio de Cohen: el paquete contiene
especificaciÃ³n, implementaciÃ³n, tests, esquemÃ¡tico y verificaciÃ³n.
Un solo archivo `.tzp` se copia, se versiona y se despliega
como una unidad.

---

## Herramienta de verificaciÃ³n: CLI

La herramienta de verificaciÃ³n es un CLI â€” la interfaz mÃ­nima sobre
la que se construye todo lo demÃ¡s:

```
trenza verify ModoEdicion.trz       -- verifica un contexto
trenza verify cronometro.tzp     -- verifica el sistema completo
trenza generate cronometro.tzp   -- genera las tres hebras
trenza check cronometro.tzp      -- verifica + genera + ejecuta tests
```

La salida del verificador usa las mismas reglas legibles documentadas
en la secciÃ³n de verificaciÃ³n:

```
$ trenza verify cronometro.tzp

  completitud ............ OK
  determinismo ........... OK
  alcanzabilidad ......... OK
  retorno ................ OK
  exhaustividad .......... OK
  conformidad de datos ... OK

  6/6 reglas cumplidas. Sistema verificado.
  Checksum del artefacto: a7f8b9...
```

Un plugin de editor invoca el CLI por debajo. Una acciÃ³n de CI/CD
es el CLI en un contenedor. Si el CLI es sÃ³lido, todo lo demÃ¡s
viene gratis.

---

## Decisiones tomadas

| # | DecisiÃ³n | ResoluciÃ³n | Razonamiento |
|---|----------|------------|--------------|
| 1 | CompilaciÃ³n destino | **Rust + WASM** | `match` exhaustivo impone completitud; WASM es autocontenido; alineado con la intenciÃ³n de la arquitectura del proyecto |
| 2 | Formato de archivo | **`.trz`** (fuente) + **`.tzp`** (paquete ZIP) | `.hlx` ocupada; un archivo por contexto; paquete autocontenido tipo .3mf |
| 3 | Herramienta | **CLI primero** (`trenza verify`, `trenza generate`) | Base sobre la que se construyen plugins y CI/CD |
| 4 | GeneraciÃ³n incremental | **Por contexto**, con checksums en `manifest.json` | Consecuencia natural de un archivo por contexto |
| 5 | Datos del rol | **Capa `data` separada**, roles vinculados por tipo | Evita herencia en diamante; separaciÃ³n DCI de estructura y comportamiento |
| 6 | Lenguaje del CLI | **Python para prototipo â†’ Rust para herramienta final** | Descubrir huecos de diseÃ±o con mÃ­nima fricciÃ³n; AST JSON como contrato de conformidad para la migraciÃ³n |
| 7 | Formato del manifest | **Schema propio JSON simple** con `trenza_version`; principios OPC adoptados (mimetype + manifiesto en raÃ­z) | OPC es XML/verboso; JSON Schema publicado solo cuando el formato estabilice |
| 8 | Herencia en contextos anidados | **ImplÃ­cita** (reglas H1â€“H5); roles locales; completitud por niveles; `trenza inspect` para vista expandida | Sub-escenas, no herencia OO; sobrescritura explÃ­cita; eventos nuevos en roles heredados prohibidos |

---

## Respuestas a las preguntas abiertas

Para referencia, las preguntas de `2026-03-04-01-concepto-inicial.md` y su estado actual:

| # | Pregunta | Estado |
|---|----------|--------|
| 1 | Â¿Unidad mÃ­nima de especificaciÃ³n? | **Respondida**: el contexto |
| 2 | Â¿Efectos secundarios? | **Respondida**: declarados con `effects`, implementados con `external` |
| 3 | Â¿Compila o interpreta? | **Respondida**: compila a Rust + WASM |
| 4 | Â¿Interop con cÃ³digo existente? | **Respondida**: mÃ³dulos `external` generan traits Rust |
| 5 | Â¿ComposiciÃ³n de estados? | **Respondida**: contextos coexistentes con reglas de prioridad + anidamiento encapsulado |

---

## Decisiones pendientes

Todas las decisiones pendientes originales fueron resueltas el 12 de marzo
de 2026. Ver decisiones 6, 7 y 8 en la tabla anterior, y los memos de
contraste:

- `docs/2026-03-12-02-decisiones-pendientes-claude.md` (posiciÃ³n Sonnet)
- `docs/2026-03-12-03-decisiones-pendientes-opus.md` (posiciÃ³n Opus + resoluciÃ³n conjunta)

