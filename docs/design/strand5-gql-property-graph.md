# Strand 5: Representación GQL como grafo de propiedades

*Análisis elaborado 2026-03-30. Basado en diálogo sobre arquitectura de Cimbra.*
*Análisis técnico de Opus con acceso al código del compilador.*

---

## Motivación

Los cuatro strands actuales del compilador representan el sistema Trenza de formas distintas:

| Strand | Formato | Captura |
|---|---|---|
| 1 | TypeScript/WASM | Comportamiento ejecutable |
| 2 | Tests algebraicos | Verificación formal |
| 3 | Mermaid | Topología de contextos (subgrafo visual) |
| 4 | Audit trail Markdown | Roles y transiciones (tabular) |

Todos son serializaciones parciales del mismo grafo subyacente. **Un sistema Trenza es estructuralmente un grafo dirigido de propiedades** — el AST ya lo es. El Strand 5 es la serialización ejecutable y consultable de ese grafo completo.

El caso de uso inmediato es Cimbra: necesita un repositorio de componentes con ciclo de vida, grafo de dependencias y catálogo. Pero el Strand 5 es útil para cualquier sistema Trenza que necesite introspección o consultas sobre su propia estructura.

---

## Corte estático/dinámico

### Estático — derivable del `.trz` en tiempo de compilación

Prácticamente todo el AST actual. El compilador ya conoce toda esta información:

| Nodo GQL | Fuente en AST | Ejemplo |
|---|---|---|
| `(:Sistema {nombre, initial})` | `SystemDef` | `Cimbra`, initial: `Inicio` |
| `(:Contexto {nombre, tipo})` | `ContextDef` + `SystemSection` | `Inicio` (context), `NuevoDataducto` (overlay) |
| `(:TipoDato {nombre})` | `DataDef` | `Peticion`, `Respuesta` |
| `(:Campo {nombre, tipo})` | `DataField` | `texto: Texto`, `exitoso: Booleano` |
| `(:Rol {nombre})` | `RoleDef` | `autor`, `modelo`, `sistema` |
| `(:Evento {nombre})` | `RoleAction.event` | `crear`, `enviar`, `aceptar` |
| `(:Efecto {trigger, funcion, args})` | `EffectRule` | `cargarDataducto -> leer_proyecto(ruta)` |

Aristas estáticas:

```
(:Contexto)-[:TRANSICION {evento, decorador}]->(:Contexto)
(:Contexto)-[:TIENE_ROL]->(:Rol)
(:Rol)-[:TIPADO_CON]->(:TipoDato)
(:Rol)-[:MANEJA {accion}]->(:Evento)
(:TipoDato)-[:TIENE_CAMPO]->(:Campo)
(:Efecto)-[:INVOCA {args}]->(STRING)
(:Rol)-[:PROHIBIDO_EN]->(:Contexto)   -- para acciones forbidden
```

Nota: el Strand 3 (Mermaid) solo captura las aristas `TRANSICION` entre contextos. El Strand 5 captura el grafo completo del AST.

### Dinámico — solo conocido en runtime

Estos nodos y aristas los puebla el runtime de Cimbra (u otro sistema Trenza), no el compilador:

- `(:ResultadoCompilacion {exitoso, timestamp, errores})` → creado cuando se compila
- `(:Componente)-[:VERSION_ANTERIOR]->(:Componente)` → historial de ediciones
- `(:Catalogo)-[:INCLUYE]->(:Componente)` → membresía por acción del usuario
- `(:Componente)-[:PROMOVIDO_DESDE]->(:Dataducto)` → trazabilidad de origen
- Telemetría de ejecución

### Zona gris: dependencias entre componentes

Las relaciones `DEPENDE_DE` y `COMPUESTO_DE` entre componentes son **dinámicas hoy** porque Trenza-DSL no tiene aún primitivas para imports entre `.trz`. Son estáticas cuando el lenguaje soporte referencias inter-componente. El Strand 5 debe dejar esta extensión prevista en el `dynamic_schema`.

---

## Formato de output: JSON-Graph + esquema GQL

**No emitir Cypher/GQL textual como formato primario.** Si el compilador emite Cypher, el consumidor necesita un parser GQL para leer el output de su propio compilador — circular.

### Archivo 1: `{Sistema}_graph.json` — datos ejecutables

```json
{
  "schema_version": "0.1.0",
  "system": "Cimbra",
  "nodes": [
    {"id": "ctx:Inicio", "label": "Contexto", "properties": {"nombre": "Inicio", "tipo": "context"}},
    {"id": "ctx:Especificando", "label": "Contexto", "properties": {"nombre": "Especificando", "tipo": "context"}},
    {"id": "data:Peticion", "label": "TipoDato", "properties": {"nombre": "Peticion"}},
    {"id": "rol:Inicio:autor", "label": "Rol", "properties": {"nombre": "autor"}},
    {"id": "evt:crear", "label": "Evento", "properties": {"nombre": "crear"}}
  ],
  "edges": [
    {"source": "ctx:Inicio", "target": "ctx:NuevoDataducto", "label": "TRANSICION",
     "properties": {"evento": "abrirNuevoDataducto"}},
    {"source": "ctx:Inicio", "target": "rol:Inicio:autor", "label": "TIENE_ROL"},
    {"source": "rol:Inicio:autor", "target": "data:Peticion", "label": "TIPADO_CON"},
    {"source": "rol:Inicio:autor", "target": "evt:crear", "label": "MANEJA",
     "properties": {"accion": "abrirNuevoDataducto"}}
  ],
  "dynamic_schema": {
    "node_types": [
      {"label": "Componente", "properties": {"nombre": "STRING", "version": "STRING"}},
      {"label": "ResultadoCompilacion", "properties": {"exitoso": "BOOLEAN", "timestamp": "DATETIME"}},
      {"label": "Catalogo", "properties": {"nombre": "STRING"}}
    ],
    "edge_types": [
      {"label": "DEPENDE_DE", "properties": {"requerida": "BOOLEAN"}},
      {"label": "VERSION_ANTERIOR"},
      {"label": "RESULTADO", "properties": {"orden": "INT"}},
      {"label": "PROMOVIDO_DESDE"},
      {"label": "INCLUYE"},
      {"label": "GENERA"}
    ]
  }
}
```

### Archivo 2: `{Sistema}_graph.gql` — esquema formal (documentación)

Esquema ISO/IEC 39075 que sirve como contrato tipado. No se ejecuta en Cimbra directamente — es la especificación formal del grafo para herramientas externas (Neo4j, documentación).

### Flag del compilador propuesto

```
trenza-cli generate --lang=ts --strand5=json spec/cimbra.trz
```

Produce `{Sistema}_graph.json` + `{Sistema}_graph.gql`.

---

## Implementación en el compilador

- **Ubicación**: nueva función en `generator.rs` — `generate_property_graph(&program_ast) -> (String, String)`
- **Complejidad estimada**: ~200-300 líneas en Rust
- **Cambios necesarios**: ninguno en parser ni validator. Función pura sobre el AST existente.
- **No hacer aún**: no convertir Strand 5 en representación intermedia del compilador. Strand 3 y Strand 5 deben seguir generándose independientemente desde el AST.

---

## Política de identidad de nodos — decidir antes de implementar

Los nodos estáticos tienen identidad natural (nombre del contexto, del tipo, etc.). Los nodos dinámicos necesitan identidad sintética.

**Recomendación: URIs deterministas**

```
cimbra://proyecto/{dataducto}/componente/{nombre}@{version}
cimbra://compilacion/{hash-sha256-del-trz-input}
cimbra://catalogo/global
```

Ventajas: reproducibles entre sesiones, trazables al origen, no dependen de UUIDs aleatorios.

---

## Invariantes arquitectónicas críticas

### 1. El grafo es un índice, no el sustrato computacional

El estado del sistema lo gobierna el Strand 1 (`System.handleEvent`). El grafo GQL es una vista consultable. Si la lógica de Cimbra migra hacia "consultar y mutar el grafo" directamente, se pierde la verificación formal de los Strands 1-2.

### 2. Toda mutación del grafo pasa por un handler del `.trz`

Cada escritura en el grafo dinámico debe ser efecto de un evento procesado por el Strand 1 y declarado como `EffectRule` en el `.trz`. Trazabilidad completa.

En `cimbra.trz`:
- `actualizarInventario -> promover_componente_validado(componente)` → crea `[:RESULTADO]`
- `registrar_en_catalogo(nombre, version)` → crea arista `[:INCLUYE]`

### 3. Persistencia del grafo dinámico vía event sourcing

La telemetría que el Strand 1 ya genera:
```
[telemetry] context=Especificando, role=autor, event=aceptar
```

**Es el event source para reconstruir el grafo dinámico.** Si se persiste en un log, el arranque es:

```
compilar .trz → grafo estático
  +
replay del log de telemetría → grafo dinámico
  =
grafo completo en memoria
```

Coherente con la arquitectura de máquina de estados de Trenza. No requiere una base de datos permanente para el modo single-user.

---

## Conexión con las brechas de expresividad de Trenza-DSL

El Strand 5 hace visibles dos limitaciones actuales del DSL que Cimbra necesita:

1. **Tipos suma**: `estado: EnProposicion | Validado` no es expresable. El compilador debería poder emitir constraints de enumeración en el `dynamic_schema`. Esto requiere tipos suma en Trenza-DSL.

2. **Listas tipadas**: `nodos: Lista<Componente>` no es expresable. El compilador no puede tipar las aristas de lista sin genéricos. Esto requiere polimorfismo paramétrico en Trenza-DSL.

Estos gaps se añaden al lenguaje cuando Cimbra los necesite con demanda probada, no antes.
