# Propuesta: `@intent` — Anotación de trazabilidad de intención

**De**: Claude Opus 4.6
**Para**: Desarrollador Principal y Gemini 3.1 Pro
**Fecha**: 20 de marzo de 2026
**Estado**: Propuesto (ADR-019)
**Versión target**: v0.0.1 (como extensión de la gramática)
**Contexto**: La Cuarta Hebra (requisitos como proyección de la
especificación) necesita un mecanismo para distinguir qué elementos
de un `.trz` representan intención confirmada del usuario y cuáles
son andamiaje técnico derivado por el modelo.

---

## 1. El problema

Un archivo `.trz` es semánticamente opaco respecto a la **procedencia**
de cada elemento. Considérese el cronómetro PSP:

```trz
state Activo {
    on tap_pausa -> Pausado
    on tick -> Activo
}

state Validando {
    on resultado.ok -> Guardado
    on resultado.error -> ErrorRed
}
```

El usuario expresó: *"quiero poder pausar el cronómetro"*. Eso se
materializó en `Activo`, `tap_pausa`, `Pausado`. Pero `Validando`,
`ErrorRed`, la transición `.error` — eso lo añadió el modelo porque
sin ello el sistema no es robusto.

Ambas cosas son necesarias. Pero **no son lo mismo**:

- **Intención**: lo que el usuario pidió conscientemente.
- **Derivación**: lo que el modelo añadió para hacer viable la intención.

Sin distinguirlas, la Cuarta Hebra (generación de requisitos) produce
documentos que mezclan lo que el negocio pidió con lo que la ingeniería
infirió. Esto invalida la trazabilidad y reduce los requisitos generados
a una paráfrasis técnica del código — que es exactamente lo que Trenza
pretende superar.

---

## 2. La propuesta: `@intent`

Un único modificador de anotación. Binario. Lo que no está marcado es
**derivado por defecto**.

### 2.1. Sintaxis

```trz
@intent("El usuario puede pausar y reanudar el cronómetro")
state Activo {
    @intent
    on tap_pausa -> Pausado

    on tick -> Activo              -- derivado: mecánica interna
}

@intent("El usuario puede pausar y reanudar el cronómetro")
state Pausado {
    @intent
    on tap_reanudar -> Activo
}

state Validando {                  -- derivado: robustez técnica
    on resultado.ok -> Guardado
    on resultado.error -> ErrorRed
}
```

### 2.2. Formas válidas

| Forma | Significado |
|-------|-------------|
| `@intent` | El elemento es intención confirmada (sin descripción textual) |
| `@intent("texto libre")` | Intención confirmada con requisito en lenguaje natural |
| `@intent("texto", source: "ruta")` | Intención con referencia a la crónica de origen |
| *(sin anotación)* | Elemento derivado (valor por defecto) |

### 2.3. Elementos anotables

`@intent` puede aplicarse a:

| Elemento | Ejemplo |
|----------|---------|
| `state` | `@intent state Activo { ... }` |
| `on` (transición) | `@intent on tap_pausa -> Pausado` |
| `data` (campo) | `@intent segundos: Int` |
| `effects` (efecto) | `@intent [on_entry] iniciarTimer()` |
| `when` (guarda) | `@intent when tiempoRestante > 0` |
| `external` (servicio) | `@intent external ApiPersistencia { ... }` |

No puede aplicarse a:

| Elemento | Razón |
|----------|-------|
| `system` | El sistema en sí no es un requisito individual |
| `context` (declaración) | Muy grueso; la intención opera a nivel de estado/transición |
| `slot` / `fills` | Son mecanismo de composición, no intención de negocio |

---

## 3. Reglas de diseño

### Regla 1: `@intent` es opcional

Un `.trz` sin ningún `@intent` es completamente válido. El verificador
no lo exige. Pero el CLI puede emitir un aviso informativo:

```
⚠ Intent coverage: 0/12 states, 0/18 transitions
  Run `trenza intent-coverage` for details
```

Esto es consistente con el principio de Trenza de que las anotaciones
enriquecen pero no condicionan la validez de la especificación.

### Regla 2: `@intent` no es heredable

Si se marca un `state` como `@intent`, sus transiciones **no** heredan
la marca automáticamente. Cada elemento se marca individualmente.

**Razón**: "el usuario pidió este estado" no significa "el usuario pidió
cada transición dentro de él". La granularidad fuerza al diseñador
(humano o IA) a ser preciso sobre qué es intención y qué es derivación.

```trz
@intent("El usuario gestiona tareas")
state GestionTareas {
    @intent
    on crear_tarea -> Creando         -- el usuario pidió esto

    on validar_duplicado -> Validando  -- el modelo añadió esto
}
```

### Regla 3: texto libre = semilla de la Cuarta Hebra

El texto entre comillas de `@intent("...")` es la formulación del
requisito en lenguaje natural. Es la semilla a partir de la cual el
CLI genera el documento de requisitos.

Cuando dos o más elementos comparten el mismo texto de `@intent`,
el CLI los agrupa bajo un mismo requisito:

```trz
@intent("El usuario puede pausar y reanudar el cronómetro")
state Activo { ... }

@intent("El usuario puede pausar y reanudar el cronómetro")
state Pausado { ... }
```

Genera:

```markdown
### REQ-001: El usuario puede pausar y reanudar el cronómetro

**Implementado por:**
- Estado `Activo` (contexto `base/cronometro`)
- Estado `Pausado` (contexto `base/cronometro`)
- Transición `tap_pausa`: Activo → Pausado
- Transición `tap_reanudar`: Pausado → Activo
```

### Regla 4: `@intent` es inmutable post-confirmación

Una vez que el usuario confirma un `@intent`, el modelo no debería
eliminarlo ni reformular su texto sin confirmación explícita. Esta es
una guarda de **proceso**, no del lenguaje — el verificador no la
aplica. Pero el CLI puede implementarla:

```bash
trenza intent-diff v0.0.0..v0.0.1
# → Requisito eliminado: "El usuario puede exportar datos" ⚠
# → Requisito reformulado: "pausar y reanudar" → "pausar, reanudar y reiniciar"
# → 3 elementos derivados añadidos (sin impacto en requisitos)
```

La advertencia sobre eliminaciones y reformulaciones protege contra
la erosión silenciosa de requisitos — un problema clásico en desarrollo
iterativo.

### Regla 5: referencia opcional a la crónica

`@intent` puede opcionalmente incluir un `source:` que enlaza con la
conversación donde se originó la intención:

```trz
@intent("El usuario puede pausar y reanudar",
        source: "history/chronicle/2026-03-04/01-concepto-inicial.md")
state Activo { ... }
```

Esto cierra el ciclo completo de trazabilidad:

```
intención (conversación) → crónica → .trz (@intent) → requisito generado
         ↑                                                      │
         └──────────────────────────────────────────────────────┘
```

La referencia no es validada por el verificador (el archivo puede no
existir aún), pero el CLI puede verificarla:

```bash
trenza intent-sources cronometro.tzp
# → 7 @intent con source válido ✓
# → 2 @intent con source no encontrado ⚠
# → 4 @intent sin source (ok, es opcional)
```

---

## 4. Comandos del CLI

`@intent` habilita una familia de comandos nuevos en la herramienta:

```bash
# Genera documento de requisitos desde los @intent
trenza requirements cronometro.tzp --format markdown
trenza requirements cronometro.tzp --format html

# Cobertura de intención (porcentaje de elementos con @intent)
trenza intent-coverage cronometro.tzp
# → States:      7/12 (58%) with @intent
# → Transitions: 4/18 (22%) with @intent
# → Data fields: 3/8  (38%) with @intent
# → Warning: 5 states without intent traceability

# Diff de intención entre versiones
trenza intent-diff v0.0.0..v0.0.1
# → Added:    "El usuario puede exportar datos"
# → Modified: "pausar y reanudar" (text refined)
# → Removed:  none
# → Derived elements added: 3 (no requirement impact)

# Verificar que las referencias source: existen
trenza intent-sources cronometro.tzp

# Mapa de trazabilidad completo (intent → spec → generated code)
trenza trace cronometro.tzp --from-intent "pausar y reanudar"
```

---

## 5. Impacto en la gramática

`@intent` es **aditivo**: no modifica ninguna construcción existente.
La gramática se extiende con:

```ebnf
annotation     = "@intent" [ "(" intent_args ")" ]
intent_args    = STRING [ "," "source" ":" STRING ]

state_decl     = { annotation } "state" IDENTIFIER "{" state_body "}"
transition     = { annotation } "on" event_ref "->" state_ref
data_field     = { annotation } IDENTIFIER ":" type_ref
effect_entry   = { annotation } "[" hook "]" action
guard           = { annotation } "when" expression
external_decl  = { annotation } "external" IDENTIFIER "{" external_body "}"
```

La posición del `@` antes de la keyword es consistente con la convención
de anotaciones en Java, Kotlin, Python (decoradores), TypeScript y Rust
(`#[...]`, aunque con sintaxis distinta). El `@` es legible tanto para
humanos como para LLMs.

---

## 6. Relación con la Cuarta Hebra (ADR-017)

Sin `@intent`, la Cuarta Hebra genera requisitos a partir de **todo**
el AST — una paráfrasis técnica del sistema completo. Útil, pero no
es trazabilidad real.

Con `@intent`, la Cuarta Hebra genera requisitos **solo** a partir de
los elementos marcados como intención. Lo demás se relega a un apéndice
de "decisiones de implementación". Esto produce dos documentos
complementarios:

1. **Requisitos de negocio** — proyectados desde los `@intent`.
   Lenguaje de negocio. Legible por stakeholders no técnicos.

2. **Decisiones técnicas** — los elementos derivados, con justificación
   de por qué existen. Legible por ingenieros.

Ambos generados automáticamente. Ambos siempre consistentes con el
código. Ambos trazables hasta la conversación original.

---

## 7. Por qué va en v0.0.1, no en v0.1

`@intent` no es una feature de uso — es una feature de **identidad**.
Define qué hace a Trenza distinto de cualquier otro DSL de máquinas
de estados. Sin `@intent`, la Cuarta Hebra es una idea. Con `@intent`,
es un mecanismo verificable.

Además:

- **Es aditivo**: no rompe nada existente. Un `.trz` sin `@intent`
  sigue siendo válido.
- **No requiere implementación inmediata**: puede documentarse en
  `spec/language/` como parte de la gramática y no implementarse en
  el parser Python hasta v0.1. Lo importante es que la especificación
  lo contemple.
- **Afecta a la presentación pública**: si v0.0.1 es la versión que
  se publica en repositorio abierto, `@intent` es lo que diferencia
  a Trenza de XState con más pasos. Es el argumento de venta.

**Propuesta concreta**: ADR-019 se acepta en v0.0.1 como parte de la
especificación (`spec/language/02-grammar.md`). La implementación en
el parser Python se planifica para v0.1. El CLI solo emite un warning
de "anotación reconocida pero no procesada" en v0.0.1.

---

## 8. Preguntas abiertas

1. **¿Debe `@intent` permitir prioridad?** Por ejemplo,
   `@intent("...", priority: must)` siguiendo MoSCoW. Mi inclinación:
   no en v0.0.1. La priorización es una capa de gestión de proyecto,
   no de especificación. Pero vale la pena dejarlo como extensión
   futura.

2. **¿Puede un elemento ser `@intent` en un contexto y derivado en
   otro?** Sí, si el mismo estado aparece en dos contextos (via
   herencia), cada aparición se anota independientemente. La intención
   es contextual.

3. **¿Quién pone los `@intent`?** En el flujo ideal, el modelo propone
   y el humano confirma. El CLI podría tener un modo interactivo:
   ```
   trenza annotate cronometro.tzp
   # → State "Activo": ¿es intención del usuario? [y/N]
   # → Transition "tap_pausa -> Pausado": ¿es intención? [Y/n]
   ```
   Pero esto es UX de herramientas, no especificación del lenguaje.

---

## 9. Impacto en el proyecto

| Artefacto | Cambio |
|-----------|--------|
| `spec/language/02-grammar.md` | Añadir sección de anotaciones con `@intent` |
| `spec/language/05-cli.md` | Añadir comandos `requirements`, `intent-coverage`, `intent-diff`, `intent-sources`, `trace` |
| `history/decisions/ADR-019-intent-annotation.md` | Nuevo ADR |
| `src/trenza/parser.py` | Reconocer `@intent` como token (v0.1) |
| `src/trenza/ast.py` | Nodo `IntentAnnotation` en el AST (v0.1) |
| `src/trenza/reqgen.py` | Nuevo generador de requisitos (v0.1) |
| `docs/manual/trenza-manual.md` | Sección sobre anotaciones de intención |

---

**Conclusión**: `@intent` es la pieza que faltaba para que la Cuarta
Hebra sea operativa. Convierte una idea filosófica (los requisitos son
una proyección) en un mecanismo concreto, verificable y diferenciador.
Propongo incluirlo en la especificación de v0.0.1 como gramática
documentada, con implementación planificada para v0.1.
