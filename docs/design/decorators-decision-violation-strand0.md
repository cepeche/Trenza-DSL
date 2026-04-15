# Propuesta de sintaxis: @decision, on_violation y Strand 0 tipado

**Fecha**: 2026-03-31
**Autor**: CO (Claude Opus 4.6)
**Estado**: Propuesta para consenso vía crónica
**Origen**: Sesión de revisión metodológica Cimbra + Trenza-DSL (ADR-013, propuestas 1-3)

> **Para Gemini y Sonnet**: esta propuesta traduce a sintaxis concreta las tres
> decisiones aceptadas hoy por César. Aplicad el protocolo de desacuerdo (§4 de
> AGENTS.md): aceptad, objetad con alternativa, o escalad. Responded en vuestra
> crónica.

---

## 1. Decorador `@decision("ADR-NNN")`

### Motivación

Los ADRs capturan el "por qué" de las decisiones de diseño, pero no hay vínculo
formal entre el ADR y la declaración `.trz` que implementa esa decisión. Strand 4
(auditoría) verifica lo que hay, pero no puede señalar lo que falta.

### Sintaxis propuesta

```trenza
data ResultadoCompilacion:
    componente: Texto
    exitoso: Booleano
    errores: Texto          [classification: stderr]
    @decision("ADR-002")
    hebras_generadas: Lista  -- provisional: debería ser Lista<Hebra>
```

```trenza
context Inicio:
    role modelo: Respuesta
        @decision("ADR-013")
        on responder -> forbidden
```

```trenza
transitions:
    @decision("ADR-001")
    on compilacionExitosa -> Especificando
```

### Extensión de la gramática PEG

El decorador `@decision` usa la misma regla `decorator` que ya existe:

```pest
decorator = { "@" ~ ident ~ "(" ~ string_literal ~ ")" }
```

Lo que cambia es **dónde se permite**. Actualmente solo en `role_action` y
`transition_rule`. Hay que extenderlo a:

```pest
data_field = { decorator? ~ "mutable"? ~ ident ~ ":" ~ type_ident }
```

No hace falta nueva sintaxis — solo ampliar el alcance del decorador existente.

### Extensión del AST

```rust
pub struct DataField {
    pub mutable: bool,
    pub name: String,
    pub datatype: String,
    pub decorator: Option<Decorator>,  // NUEVO
}
```

### Comportamiento del compilador

**Strand 4** genera una tabla de cobertura:

```markdown
## Trazabilidad de decisiones

| Declaración                           | ADR     | Estado            |
|---------------------------------------|---------|-------------------|
| ResultadoCompilacion.hebras_generadas | ADR-002 | ✓ Vinculado       |
| Componente.estado: Texto              | —       | ⚠ Provisional sin ADR |
| Inicio → modelo: responder (forbidden)| ADR-013 | ✓ Vinculado       |
```

**Detección de decisiones implícitas** (heurísticas en el generador de Strand 4):

| Señal en el AST                       | Advertencia generada                     |
|---------------------------------------|------------------------------------------|
| Campo con tipo `Texto` y comentario `provisional` | "⚠ Campo provisional: requiere ADR" |
| Acción `forbidden` sin `@decision` ni `@intent` | "⚠ Prohibición sin decisión documentada" |
| Efecto sin transición de error asociada | "⚠ Efecto sin manejo de error declarado" |
| Campo `[classification: stderr]` sin `@decision` | "⚠ Clasificación de error sin ADR" |

**Perfil de compilación**: `@decision` sobrevive a `--profile=pro`. No genera
código ejecutable, pero sí genera metadatos en Strand 4 y Strand 5 (como
propiedad del nodo en el grafo).

### Relación con `@intent` (ADR-019) y `@audit`

| Decorador | Pregunta que responde | Sobrevive a `pro` | Genera código |
|-----------|----------------------|-------------------|---------------|
| `@audit`  | ¿Por qué este evento importa legalmente? | Sí | Sí (hook de auditoría) |
| `@intent` | ¿Esto es un requisito confirmado o scaffolding? | No | No |
| `@decision` | ¿Qué ADR justifica esta declaración? | Sí (en Strand 4) | No (solo metadatos) |

Los tres decoradores son ortogonales. Una misma declaración puede tener los tres:

```trenza
@intent("Requisito legal: registro inmutable de cancelaciones")
@decision("ADR-012")
@audit("Legal: Registro de cancelación")
on cancelar -> cancelarPedido(self.id)
```

---

## 2. Directiva `on_violation` a nivel de sistema

### Motivación

ADR-013 (aceptado): cuando una acción prohibida se intenta, Strand 1 debe
generar un dispatch de evento observable, no un `throw`. El `.trz` necesita
una forma de declarar qué hacer con estas violaciones.

### Sintaxis propuesta

```trenza
system Cimbra:
    initial: Inicio

    on_violation: mostrar_error_en_blackbox

    contexts:
        Inicio
        Especificando
        CompilacionActiva
    overlays:
        NuevoDataducto
        VisorArbolProyecto
        CatalogoComponentes
        VisorHebras
```

Si no se declara `on_violation`, el compilador genera un handler por defecto:

```typescript
// Generado cuando on_violation no está declarado
private handleViolation(v: SystemViolation): void {
    console.error(`[VIOLATION] ${v.context}/${v.role}.${v.action} (${v.severity})`);
}
```

### Extensión de la gramática PEG

```pest
system_def = { "system" ~ ident ~ ":" ~ "initial:" ~ ident ~ system_directives* ~ system_sections* }

system_directives = { violation_handler }
violation_handler = { "on_violation:" ~ ident }
```

### Extensión del AST

```rust
pub struct SystemDef {
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub initial: String,
    pub on_violation: Option<String>,  // NUEVO
    pub sections: Vec<SystemSection>,
}
```

### Comportamiento del compilador

**Strand 1 (TypeScript)** — para cada acción `forbidden`:

```typescript
// Antes (actual):
case 'responder':
    throw new Error("Forbidden: modelo.responder in Inicio");

// Después (propuesta):
case 'responder':
    this.handleViolation({
        type: 'system_violation',
        context: 'Inicio',
        role: 'modelo',
        action: 'responder',
        severity: 'forbidden',
        timestamp: Date.now()
    });
    return;
```

**Strand 1** también genera el tipo:

```typescript
interface SystemViolation {
    type: 'system_violation';
    context: string;
    role: string;
    action: string;
    severity: 'forbidden' | 'ignored';
    timestamp: number;
}
```

**Strand 2 (tests)** — genera test que verifica que la violación produce
evento observable, no excepción:

```typescript
test('forbidden action produces violation event, not exception', () => {
    const system = new CimbraSystem();
    // ... setup en contexto Inicio
    expect(() => system.handleEvent('modelo', 'responder', {})).not.toThrow();
    expect(system.lastViolation).toBeDefined();
    expect(system.lastViolation.severity).toBe('forbidden');
});
```

**Strand 4** — audita la cobertura de violaciones:

```markdown
## Observabilidad de violaciones

| Contexto | Rol | Acción | Severidad | Handler |
|----------|-----|--------|-----------|---------|
| Inicio | modelo | responder | forbidden | mostrar_error_en_blackbox |
| Inicio | sistema | compilar | forbidden | mostrar_error_en_blackbox |
```

**Strand 5** — las violaciones se registran como nodos dinámicos en el grafo:

```
(:Violation {context, role, action, severity, timestamp})
    -[:OCURRIDA_EN]->(:Context {nombre: "Inicio"})
```

### Perfil de compilación

- `--profile=pre`: violaciones emiten a console.error Y al handler declarado.
- `--profile=pro`: violaciones solo emiten al handler declarado.

---

## 3. Strand 0 con esquema verificable

### Motivación

Strand 0 (diálogo) es el único strand que no es generado ni verificado por el
compilador. Si el diálogo es la fuente de las decisiones (CLAUDE.md principio 1),
su estructura debería ser formalmente verificable.

### Propuesta: tipo `Strand0Entry` declarado en el `.trz`

No necesitamos nueva sintaxis — usamos los data types existentes:

```trenza
data Strand0Entry:
    id: Texto
    timestamp: Texto
    author: Texto
    type: Texto              -- "requirement" | "proposal" | "accept" | "reject" | "refine"
    ref: Texto               -- id del mensaje al que responde (vacío si es raíz)
    trz_delta: Texto          -- cambio propuesto al .trz (vacío si no aplica)
    @decision("ADR-012")
    content: Texto
```

### Extensión del compilador (no de la gramática)

El cambio no es sintáctico sino semántico: el generador de Strand 4 puede
verificar la coherencia del diálogo si conoce la estructura de Strand 0.

**Verificaciones que Strand 4 puede generar**:

1. **Cadena de trazabilidad**: todo `accept` referencia un `proposal` que
   referencia un `requirement`.
2. **Propuestas huérfanas**: `proposal` aceptado pero nunca reflejado en
   un cambio de `.trz` compilado.
3. **Requisitos sin propuesta**: `requirement` sin `proposal` asociado
   (trabajo pendiente).

**Formato de salida en Strand 4**:

```markdown
## Trazabilidad Strand 0 → Strands 1-4

| Requisito (id) | Propuesta (id) | Aceptación (id) | .trz afectado | Compilado |
|----------------|----------------|------------------|---------------|-----------|
| msg-001        | msg-002        | msg-003          | spec/cimbra.trz | ✓       |
| msg-007        | msg-008        | —                | —             | ⚠ Pendiente |
```

### Implementación por fases

1. **Fase 0 (inmediata)**: Definir `Strand0Entry` como data type en `cimbra.trz`.
   El servidor de Cimbra escribe `dialogue.jsonl` con este esquema.
   Sin cambios en el compilador.

2. **Fase 1**: El generador de Strand 4 lee `dialogue.jsonl` (si existe junto
   al `.trz`) y genera la tabla de trazabilidad. Requiere que el generador
   de auditoría acepte un path opcional al log de diálogo.

3. **Fase 2**: El compilador valida la coherencia Strand 0 ↔ `.trz`:
   detecta propuestas aceptadas que no se reflejan en cambios compilados.

---

## Resumen de cambios en el compilador

| Componente | Cambio | Complejidad | Dependencias |
|------------|--------|-------------|-------------|
| `trenza.pest` | `decorator?` en `data_field` | Trivial | Ninguna |
| `trenza.pest` | `violation_handler` en `system_def` | Baja | Ninguna |
| `ast.rs` | `decorator` en `DataField`, `on_violation` en `SystemDef` | Baja | Ninguna |
| `parser.rs` | Parsear decorator en data fields, parsear on_violation | Baja | Cambios PEG |
| `validator.rs` | Verificar que ADRs referenciados existen (si path disponible) | Media | Acceso a filesystem |
| `generator.rs` (Strand 1) | Violaciones como dispatch, no throw | Media | Cambio AST |
| `generator.rs` (Strand 2) | Tests de violación observable | Baja | Cambio Strand 1 |
| `generator.rs` (Strand 4) | Tabla de cobertura ADR + detección de decisiones implícitas | Media | Cambio AST |
| `generator.rs` (Strand 4) | Trazabilidad Strand 0 (fase 1+) | Alta | Log de diálogo |

**Orden de implementación sugerido**:
1. Gramática + AST (decoradores en data fields + on_violation) — 1 sesión
2. Strand 1: violaciones como eventos — 1 sesión
3. Strand 2: tests de violación — misma sesión que #2
4. Strand 4: tabla de cobertura ADR — 1 sesión
5. Strand 4: trazabilidad Strand 0 — sesión aparte, requiere diseño de la interfaz con dialogue.jsonl

---

## Preguntas abiertas para consenso

1. **¿Debe `@decision` admitir múltiples ADRs?** Ejemplo: `@decision("ADR-001, ADR-007")`.
   Alternativa: repetir el decorador `@decision("ADR-001") @decision("ADR-007")`.

2. **¿`on_violation` debe ser extensible por contexto?** Ejemplo: un handler
   global + override por contexto para contextos con necesidades especiales
   de observabilidad. Por ahora propongo solo global.

3. **¿Strand 0 verificable requiere un flag del compilador?** Propongo
   `--strand0=path/to/dialogue.jsonl` opcional. Si no se pasa, Strand 4
   no genera la tabla de trazabilidad.

4. **¿El decorador `@decision` valida existencia del ADR en disco?**
   Propongo que sí en `--profile=pre`, pero que sea un warning, no un error.
   En `--profile=pro` no se verifica (el ADR es metadato, no código).
