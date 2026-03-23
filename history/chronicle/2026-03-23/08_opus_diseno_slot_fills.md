---
date: 2026-03-23
from: Claude Opus 4.6
to: Gemini (implementador)
subject: "Diseno tecnico de slot/fills — gramatica PEG, AST, verificacion"
---

# Diseno tecnico: `slot` / `fills`

**Objetivo**: Este documento contiene todo lo necesario para que Gemini
implemente `slot`/`fills` en el compilador Rust (`trenza-cli/`) en una sola
sesion. No requiere decisiones de diseno adicionales.

**Documentos de referencia**:
- `history/chronicle/2026-03-20/03-resolucion-gap4-definitiva.md` (semantica)
- `spec/reference/cronometro-psp/trenza/contexts/ModalComentario.trz` (ejemplo slot)
- `spec/reference/cronometro-psp/trenza/contexts/SesionActiva.trz` (ejemplo fills)

---

## 1. Sintaxis PEG (reglas pest)

### Cambios en la gramatica existente

La regla `context_clause` se amplia para aceptar `slot_def` y `fills_def`:

```pest
context_clause = { input_def | role_def | transitions_def | effects_def | slot_def | fills_def }
```

### Nuevas reglas

```pest
slot_def = { "slot" ~ ident }

fills_def = { "fills" ~ slot_ref ~ ":" ~ fills_clause* }
slot_ref = { ident ~ "." ~ ident }
fills_clause = { role_def | effects_def }
```

### Explicacion de las reglas

**`slot_def`**: Declara un punto de extension. Sintaxis minima: solo un nombre.
No tiene tipo explicito (ver seccion 4 para la justificacion). No tiene bloque
subordinado — un slot es una declaracion atomica.

**`fills_def`**: Declara la contribucion de un concurrent a un slot. Se compone
de una referencia al slot (`ContextName.slot_name`) seguida de `:` y un bloque
que puede contener roles y effects. La referencia usa `slot_ref` (dos idents
separados por punto).

**`fills_clause`**: Reusa las reglas existentes `role_def` y `effects_def` sin
modificacion. Esto es deliberado: los roles y effects dentro de un `fills` son
exactamente iguales a los de un contexto normal. No hay sintaxis nueva que
aprender.

**`slot_ref`**: Usa `ident ~ "." ~ ident` en lugar de reusar `field_access`
porque `field_access` ya existe dentro de `arg` y tiene semantica diferente
(acceso a campo de dato). La nueva regla es estructuralmente identica pero
semanticamente distinta, lo que permite al parser y al AST distinguirlas sin
ambiguedad.

### Nota sobre `definition_kw`

`slot` y `fills` no necesitan anadirse a `definition_kw` porque solo aparecen
dentro de `context_clause`, no como definiciones top-level. La regla
`section_ident` (que usa `!definition_kw`) no se ve afectada.

### Ejemplo concreto de `.trz` que debe parsear

```trenza
context ModalComentario:

    role campo_comentario: CampoTexto
        on cambio -> actualizarComentario(self.valor)

    role boton_confirmar: Boton
        on tap -> confirmarInicio

    slot sesion_opts

    transitions:
        on confirmarInicio -> [cerrar_overlay]
        on cancelar -> [cerrar_overlay]
```

```trenza
context SesionActiva:

    role display_timer: Boton
        on tap -> ignored

    fills ModalComentario.sesion_opts:
        role checkbox_sustituir: Checkbox
            on cambio -> marcarSustituir(self.marcado)

        effects:
            [on_entry] -> cargar_opciones_sesion()

    transitions:
        on sesionFinalizada -> [deactivate]
```

---

## 2. Structs Rust para el AST

### Nuevos tipos

```rust
#[derive(Debug, Clone)]
pub struct SlotDef {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FillsDef {
    pub target_context: String,
    pub target_slot: String,
    pub roles: Vec<RoleDef>,
    pub effects: Vec<EffectRule>,
}
```

### Cambios en `ContextDef`

```rust
#[derive(Debug, Clone)]
pub struct ContextDef {
    pub name: String,
    pub inputs: Vec<InputField>,
    pub roles: Vec<RoleDef>,
    pub transitions: Vec<TransitionRule>,
    pub effects: Vec<EffectRule>,
    pub slots: Vec<SlotDef>,       // NUEVO
    pub fills: Vec<FillsDef>,      // NUEVO
}
```

### Justificacion de los campos

**`SlotDef`**: Solo tiene `name`. No tiene tipo porque `slot` es un punto de
extension sin contrato de tipo (ver seccion 4). No tiene anotaciones en esta
primera version. La struct es intencionalmente minima para permitir extension
futura sin romper la API.

**`FillsDef`**: Tiene `target_context` y `target_slot` como campos separados
(no un string concatenado con punto). Esto facilita la verificacion sin
necesidad de parsear strings en el validator. Contiene `roles` y `effects`
que reusan los tipos existentes `RoleDef` y `EffectRule`.

**Por que `Vec<SlotDef>` y no `Option<SlotDef>`**: Un overlay puede tener
multiples slots (aunque en CronometroPSP solo tiene uno). Usar `Vec` es
consistente con el patron del resto del AST y no cuesta nada.

**Por que `Vec<FillsDef>` y no un solo `fills`**: Un concurrent puede llenar
slots de multiples overlays. Ejemplo hipotetico: `SesionActiva` podria llenar
`ModalComentario.sesion_opts` y `ModalResumen.sesion_info`. Usar `Vec` lo
permite sin cambios futuros.

---

## 3. Cambios en el parser (`parser.rs`)

### Dentro de `parse_context`

Anadir la inicializacion de `slots` y `fills`:

```rust
fn parse_context(pair: pest::iterators::Pair<Rule>) -> ContextDef {
    let mut name = String::new();
    let mut inputs = Vec::new();
    let mut roles = Vec::new();
    let mut transitions = Vec::new();
    let mut effects = Vec::new();
    let mut slots = Vec::new();       // NUEVO
    let mut fills = Vec::new();       // NUEVO

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::context_clause => {
                let clause = inner.into_inner().next().unwrap();
                match clause.as_rule() {
                    Rule::input_def => { /* existente */ },
                    Rule::role_def => { /* existente */ },
                    Rule::transitions_def => { /* existente */ },
                    Rule::effects_def => { /* existente */ },
                    Rule::slot_def => {                          // NUEVO
                        let slot_name = clause.into_inner()
                            .next().unwrap().as_str().to_string();
                        slots.push(SlotDef { name: slot_name });
                    },
                    Rule::fills_def => {                         // NUEVO
                        fills.push(parse_fills(clause));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    ContextDef { name, inputs, roles, transitions, effects, slots, fills }
}
```

### Nueva funcion `parse_fills`

```rust
fn parse_fills(pair: pest::iterators::Pair<Rule>) -> FillsDef {
    let mut target_context = String::new();
    let mut target_slot = String::new();
    let mut roles = Vec::new();
    let mut effects = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::slot_ref => {
                let mut ref_iter = inner.into_inner();
                target_context = ref_iter.next().unwrap().as_str().to_string();
                target_slot = ref_iter.next().unwrap().as_str().to_string();
            },
            Rule::fills_clause => {
                let clause = inner.into_inner().next().unwrap();
                match clause.as_rule() {
                    Rule::role_def => roles.push(parse_role(clause)),
                    Rule::effects_def => {
                        for eff in clause.into_inner() {
                            let mut e_iter = eff.into_inner();
                            let trigger_pair = e_iter.next().unwrap();
                            let trigger = if trigger_pair.as_rule() == Rule::lifecycle_hook {
                                EffectTrigger::Lifecycle(
                                    trigger_pair.as_str()
                                        .replace("[", "").replace("]", "")
                                )
                            } else {
                                EffectTrigger::Event(trigger_pair.as_str().to_string())
                            };
                            let call = parse_action_call(e_iter.next().unwrap());
                            effects.push(EffectRule { trigger, call });
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    FillsDef { target_context, target_slot, roles, effects }
}
```

---

## 4. Decision: el `fills` NO declara tipo explicito

### La pregunta

El briefing plantea: cuando `SesionActiva` escribe `fills ModalComentario.sesion_opts`,
debe declarar un tipo (como `fills ModalComentario.sesion_opts: OpcionesSesion`)?

### La decision: NO. El tipo se omite.

### Justificacion

1. **El slot no tiene tipo.** En la semantica de GAP-4, un slot es un punto de
   extension sin contrato de datos. No es un puerto tipado como un generic de
   Rust o un type parameter de TypeScript. Es un hueco donde se inyectan roles
   completos con su propia definicion de tipo. Cada rol dentro del `fills` ya
   declara su propio tipo (`role checkbox_sustituir: Checkbox`). El tipo del
   slot seria redundante con el tipo de los roles que lo llenan.

2. **No hay contrato de conformidad que verificar.** Si el slot tuviera un
   tipo `OpcionesSesion`, habria que verificar que los roles dentro del `fills`
   "conforman" a ese tipo. Pero conformidad de roles es una nocion compleja:
   un rol no es un valor de un tipo, es un participante con eventos y acciones.
   No existe un sistema de tipos natural para expresar "este conjunto de roles
   satisface este contrato". Inventar uno introduciria complejidad sin beneficio
   claro en la primera version.

3. **La verificacion real es estructural, no tipada.** Lo que el compilador
   verifica es:
   - Que el slot referenciado exista (Rule S1).
   - Que no haya conflictos de fills (Rule S3).
   - Que los roles inyectados cumplan completitud y determinismo (Rule S4).
   Ninguna de estas verificaciones requiere un tipo en el slot.

4. **Consistencia con el `slot` existente en las specs.** Los archivos
   `ModalComentario.trz` y `SesionActiva.trz` en `spec/reference/` usan
   `slot sesion_opts` sin tipo. El diseno sigue lo que ya esta establecido.

### Trade-off

La desventaja es que un slot no documenta que "espera" recibir. Si en el futuro
se necesita un contrato (por ejemplo, "el fills debe proveer al menos un rol de
tipo Checkbox"), se podria anadir una anotacion opcional:

```trenza
slot sesion_opts [expects: Checkbox]   -- futuro, no implementar ahora
```

Esto seria retrocompatible: los slots sin anotacion seguirian siendo validos.

---

## 5. Algoritmo de verificacion

### Es una regla nueva (Rule 7), no extension de Rule 5

Rule 5 (Role Exhaustiveness) verifica que todo rol aparezca en todos los
contextos. Slot/fills opera en una dimension diferente: verifica relaciones
entre overlays y concurrents. Mezclar ambas en un solo pass oscureceria la
logica. Se crea un **Pass 7** independiente.

### Pseudocodigo

```
FUNCTION verify_slots_fills(program):
    errors = []

    -- Paso 1: Construir indice de slots
    slot_index = {}   -- Map<(context_name, slot_name), true>
    FOR EACH context IN program.contexts:
        FOR EACH slot IN context.slots:
            slot_index[(context.name, slot.name)] = true

    -- Paso 2: Construir indice de fills agrupados por slot
    fills_index = {}  -- Map<(target_context, target_slot), Vec<(source_context, FillsDef)>>
    FOR EACH context IN program.contexts:
        FOR EACH fills IN context.fills:
            key = (fills.target_context, fills.target_slot)

            -- Rule S1: Referencia valida
            IF key NOT IN slot_index:
                errors.push(
                    "ERROR [slot]: {context.name} declares fills {fills.target_context}.{fills.target_slot} "
                    "but {fills.target_context} does not declare that slot"
                )
                CONTINUE

            fills_index[key].push((context.name, fills))

    -- Paso 3: Rule S3 — Conflicto de fills
    FOR EACH (key, sources) IN fills_index:
        IF sources.length > 1:
            names = sources.map(s => s.0).join(", ")
            errors.push(
                "ERROR [slot-conflict]: contexts {names} both declare fills for "
                "{key.0}.{key.1}. Declare priority in the system block"
            )

    -- Paso 4: Rule S4 — Completitud y determinismo dentro de cada fills
    FOR EACH (key, sources) IN fills_index:
        FOR EACH (source_name, fills_def) IN sources:
            -- Verificar determinismo: no hay role+event duplicados dentro del fills
            seen_role_events = {}
            FOR EACH role IN fills_def.roles:
                FOR EACH action IN role.actions:
                    re = (role.name, action.event)
                    IF re IN seen_role_events:
                        errors.push(
                            "ERROR [determinism]: role '{role.name}' has duplicate handlers "
                            "for event '{action.event}' in fills {key.0}.{key.1} "
                            "of context '{source_name}'"
                        )
                    seen_role_events.insert(re)

    -- Nota: Rule S2 (slot vacio es valido) no genera errores — es la ausencia
    -- de un fills, que simplemente se ignora.

    RETURN errors
```

### Donde insertar el nuevo pass en `validator.rs`

Despues del Pass 6 (Rule 4 — Return/No Sinks), anadir:

```rust
// Pass 7: Rule 7 (Slot/Fills Integrity)
// ... implementacion del pseudocodigo arriba
```

El pass necesita acceso a todos los `ContextDef` del programa, que ya estan
disponibles en el bucle principal. No requiere datos de passes anteriores.

### Interaccion con las reglas existentes

| Regla existente | Impacto de slot/fills |
|-----------------|----------------------|
| Rule 1 (Completeness) | Los roles dentro de un `fills` NO generan obligaciones en otros contextos. `checkbox_sustituir` solo existe en la interseccion `SesionActiva x ModalComentario`. `ModoNormal` no necesita declararlo. |
| Rule 2 (Determinism) | Se aplica dentro de cada `fills` individualmente (cubierto por paso 4 del pseudocodigo). |
| Rule 3 (Reachability) | No aplica a roles de slot. Se aplica normalmente a los contextos overlay y concurrent. |
| Rule 4 (Return) | No afectado. Los slots no son contextos y no participan en el grafo de transiciones. |
| Rule 5 (Exhaustiveness) | Los roles dentro de un `fills` NO se anaden al conjunto global `all_roles`. Son roles locales del scope del fills. |
| Rule 6 (Data Conformance) | Se aplica a los roles dentro de `fills` igual que a cualquier otro rol. Si un rol del fills accede a datos GDPR, necesita la anotacion `[access: gdpr]`. |

### Cambio critico en Rules 1 y 5

Para que Rules 1 y 5 no consideren los roles de `fills`, el validator debe
**excluir los roles de fills del conjunto global**. Actualmente, el validator
recorre `ctx.roles` para construir `all_roles` y `role_events`. Los roles
dentro de `ctx.fills[*].roles` deben quedar fuera de esos conjuntos.

Dado que `fills` es un campo nuevo de `ContextDef` (no parte de `roles`),
esto ocurre naturalmente sin cambios en los passes 1-6. Los fills viven en
`ctx.fills`, no en `ctx.roles`. El codigo existente que itera sobre
`ctx.roles` simplemente no los ve.

---

## 6. Cambios en los generadores (fuera de scope, pero documentados)

Estos cambios son necesarios eventualmente pero NO son parte de esta sesion
de implementacion. Se documentan para completitud.

### Generador Rust (Strand 1)

Los roles dentro de un `fills` deben generar funciones handler adicionales,
pero solo activas cuando ambos contextos (overlay + concurrent) estan activos.
Esto requiere un modelo de composicion en runtime que excede lo que el
generador actual produce.

### Generador Mermaid (Strand 3)

Los slots deben visualizarse como nodos especiales en el diagrama del overlay,
y los fills como aristas desde el concurrent al slot.

### Generador Auditoria (Strand 4)

Los roles dentro de fills deben aparecer en la narrativa de auditoria,
indicando su condicionalidad.

---

## 7. Resumen de cambios por archivo

| Archivo | Cambio |
|---------|--------|
| `trenza.pest` | Anadir `slot_def`, `fills_def`, `slot_ref`, `fills_clause`. Ampliar `context_clause`. |
| `ast.rs` | Anadir `SlotDef`, `FillsDef`. Ampliar `ContextDef` con `slots` y `fills`. |
| `parser.rs` | Anadir ramas para `Rule::slot_def` y `Rule::fills_def` en `parse_context`. Anadir funcion `parse_fills`. |
| `validator.rs` | Anadir Pass 7 con verificacion S1, S3, S4. Sin cambios en passes 1-6. |
| `main.rs` | Sin cambios (el validator ya se invoca sobre el programa completo). |

### Test minimo de aceptacion

Un archivo `.trz` con un overlay que declara `slot sesion_opts` y un concurrent
que declara `fills ModalComentario.sesion_opts:` con un rol y un effect debe:

1. Parsear sin errores.
2. Producir un AST con `slots.len() == 1` en ModalComentario y `fills.len() == 1` en SesionActiva.
3. Pasar las 7 reglas de verificacion.

Un archivo donde el `fills` referencia un slot inexistente debe producir:
```
ERROR [slot]: SesionActiva declares fills ModalComentario.opciones_inexistentes
              but ModalComentario does not declare that slot
```

---

## 8. Ejemplo completo para test de integracion

```trenza
system SistemaSlotTest:
    initial: Base
    contexts:
        Base
    concurrent:
        Auxiliar
    overlays:
        Popup

data Elemento:
    id: Id

context Base:
    role boton: Elemento
        on tap -> abrir()
    role helper: Elemento
        on tap -> ignored
    transitions:
        on abrir -> Popup
        on activar -> Auxiliar

context Popup:
    role boton: Elemento
        on tap -> cerrar()
    role helper: Elemento
        on tap -> ignored
    slot extras
    transitions:
        on cerrar -> Base

context Auxiliar:
    role boton: Elemento
        on tap -> ignored
    role helper: Elemento
        on tap -> ayudar()
    fills Popup.extras:
        role indicador: Elemento
            on tap -> mostrarInfo()
        effects:
            [on_entry] -> cargar_datos()
    transitions:
        on desactivar -> Base
```

Este ejemplo debe compilar y pasar las 7 reglas. El rol `indicador` solo
existe en la interseccion Auxiliar x Popup, y no genera obligaciones en `Base`.
