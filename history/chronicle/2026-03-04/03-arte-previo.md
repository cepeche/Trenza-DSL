# Arte previo — helix-dsl-verified

**Fecha**: 4 de marzo de 2026
**Objetivo**: mapear iniciativas existentes antes de diseñar el DSL,
para no reinventar la rueda y para identificar qué hueco real queda por cubrir.

---

## Resumen ejecutivo

Las cuatro hipótesis de diseño de helix-dsl-verified tienen precedentes sólidos
por separado. Lo que **no existe** es su combinación en un único artefacto
diseñado explícitamente para la co-generación humano/LLM. Ese es el hueco.

---

## 1. Máquinas de estado con transiciones explícitas

### XState / Stately (JavaScript/TypeScript)
La implementación más madura del espacio. Basado en los *statecharts* de
David Harel (1987), XState permite declarar estados, transiciones y guards
de forma completamente explícita. Puntos de contacto directos con helix:

- Los eventos no manejados en un estado no producen transición silenciosa.
- Soporte para subestados jerárquicos y estados paralelos.
- `@xstate/test` genera automáticamente caminos de test a partir de la
  máquina, recorriendo el grafo de transiciones. Esto es la cara funcional
  de la "doble hélice": la especificación genera los tests.
- Visualizador gráfico integrado (Stately Studio).

**Limitación crítica**: XState no genera *implementación* a partir de la
especificación. El programador escribe la máquina y luego escribe el código
que la usa. La hélice solo tiene una hebra (tests), no dos (tests + impl).

### Elm Architecture (TEA)
El modelo `(Msg, Model) → Model` garantiza que toda transición de estado es
una función pura y explícita. Ningún componente puede mutar el estado
directamente; todo pasa por `update`. Esto es estructuralmente equivalente
a "condicionales solo en factorías": el `update` central es la única fábrica
de estados.

**Limitación**: TEA no genera tests automáticamente, y la arquitectura
no escala bien a aplicaciones grandes sin fragmentarla en múltiples apps.
Tampoco es un DSL externo; es un patrón arquitectónico dentro de Elm.

---

## 2. Co-generación implementación/tests desde una especificación

### Diseño por Contrato — Eiffel (Bertrand Meyer, 1988)
El referente más próximo conceptualmente. En Eiffel, cada rutina declara
`require` (precondición) y `ensure` (postcondición). La postcondición es
exactamente el "reverso algebraico" que helix propone: si la implementación
dice `A → B`, el contrato dice `dado(A) verificar(B)`.

Diferencias con helix:
- Los contratos en Eiffel son parte del código de producción, no artefactos
  separados. Son verificados en runtime (o en compilación con herramientas
  adicionales), no generados como suite de tests independiente.
- No hay generación automática de tests a partir de los contratos; el
  desarrollador escribe la implementación y los contratos manualmente.
- La relación entre contrato e implementación no es bidireccional generativa;
  es una aserción sobre la implementación existente.

El debate DbC vs TDD es relevante: los contratos son especificaciones
*intensionales* (definen el espacio válido); los tests son *extensionales*
(listan puntos concretos). helix busca algo intermedio: tests derivados
mecánicamente de una especificación intensional.

### `@xstate/test` y model-based testing
Como se menciona en §1, XState puede generar caminos de test a partir del
grafo. Esto se acerca más a la doble hélice. La diferencia es que en XState
el programador escribe primero la implementación (la máquina), y los tests
son derivados de ella. En helix la idea es que *ambos* se deriven de una
especificación de nivel superior.

### react-automata
Wrapper de XState para React que incluye `testStateMachine()`, una función
que genera snapshots automáticos para cada estado de la máquina. Es el
ejemplo más concreto de co-generación en el espacio frontend, pero limitado
a snapshots visuales y acoplado a React.

---

## 3. Verificabilidad formal

### TLA+ / PlusCal (Leslie Lamport)
El estándar industrial para especificación formal de sistemas concurrentes.
TLA+ permite expresar flujos de estado como ciudadanos de primera clase y
verificar propiedades de seguridad (*safety*) y vivacidad (*liveness*)
mediante model checking exhaustivo.

Relevante para helix porque:
- Un desarrollador ha documentado (Medium, enero 2026) el uso de TLA+ como
  especificación previa a la generación de código Rust con LLMs, con
  resultados positivos: la especificación formal ancla al LLM y reduce las
  regresiones.
- La investigación reciente (SYSMOBENCH, 2025) muestra que los LLMs dominan
  TLA+ mejor que Alloy o PAT, lo que lo convierte en candidato natural para
  el nivel formal de helix.

**Limitación**: TLA+ no compila a código ejecutable. Es un lenguaje de
especificación puro. El salto especificación → implementación sigue siendo
manual (o delegado al LLM, con los riesgos asociados).

### Dafny (Microsoft Research)
Lenguaje con pre/postcondiciones y pruebas verificadas por SMT. Los estudios
muestran ~77% pass@1 en HumanEval con Dafny como lenguaje intermedio.
Se acerca más que TLA+ a la co-generación porque el código Dafny *es*
ejecutable y verificado. Sin embargo, la curva de aprendizaje es alta y está
diseñado para algoritmos, no para flujos de UI/eventos.

### Idris / tipos dependientes
En Idris, los tipos son pruebas: una función con el tipo correcto *es* su
verificación. Esto es el extremo más radical de la doble hélice: no hay
distinción entre implementación y test porque el sistema de tipos garantiza
la corrección. Trabajo reciente (TyDe 2024) explora *property-based testing*
a nivel de tipos.

**Limitación práctica**: Idris es investigación académica, no herramienta
de producción. La curva de aprendizaje es prohibitiva para el caso de uso
de helix (desarrollo web cotidiano con LLMs).

---

## 4. DSLs diseñados para LLMs

### Self-Spec (OpenReview, 2025)
Propuesta reciente que experimenta con LLMs generando su propio DSL
de especificación (model-invented DSL) como paso intermedio entre lenguaje
natural y código. Es conceptualmente el más cercano a helix en espíritu:
un DSL compacto y "on-distribution" para el LLM mejora la calidad de la
generación. Los resultados muestran mejora sobre CoT directo.

**Diferencia clave**: Self-Spec es efímero (el DSL se genera por sesión);
helix propone un DSL estable y reutilizable, diseñado deliberadamente por
humanos con el LLM como colaborador.

### Alloy + LLMs para validación (Cunha & Macedo, 2025)
Estudio que usa GPT-5 para generar casos de test positivos y negativos a
partir de especificaciones en Alloy. Alta tasa de éxito sintáctico y
semántico. Es la cadena `especificación → tests` ya funcionando en la
práctica, aunque limitada a modelos estructurales (no flujos de eventos).

---

## 5. Lo que no existe

Ninguna iniciativa encontrada combina simultáneamente:

1. **Flujos de estado explícitos** como ciudadanos de primera clase del
   lenguaje (XState lo hace, Elm parcialmente).
2. **Co-generación de implementación + tests** como acto generativo único
   a partir de una especificación de nivel superior (nadie lo hace
   completamente; XState genera tests pero no implementación).
3. **Semántica restringida para razonamiento LLM** — diseño deliberado del
   DSL para que sea fácil de razonar por un modelo de lenguaje, no solo por
   un humano (Self-Spec lo explora pero de forma ad-hoc).
4. **Orientación al desarrollo frontend cotidiano** — TLA+ y Dafny son
   demasiado académicos; XState es demasiado cercano al código destino.

---

## Conclusiones para el diseño de helix

**Aprovechar sin reinventar:**

- La semántica de máquina de estados de XState es madura y correcta. Helix
  puede adoptar su modelo (estados, eventos, guards, transiciones) como
  capa semántica, evitando rediseñar esa parte.
- El patrón de `@xstate/test` confirma que la generación de tests a partir
  del grafo de estados es viable y útil. Helix extiende esto generando
  también la implementación.
- DbC (Eiffel) confirma que el "reverso algebraico" es un patrón sólido.
  La postcondición como espejo de la implementación es la idea correcta.

**Decisiones de diseño que quedan abiertas:**

- ¿Compila helix a XState como IR, aprovechando su ecosistema (visualizador,
  `@xstate/test`)? Esto reduciría el trabajo pero acoplaría helix a JS.
- ¿Hasta qué punto adoptar la semántica de TLA+ para el nivel formal?
  PlusCal es más legible que TLA+ y los LLMs lo conocen relativamente bien.
- El gap entre especificación formal y código ejecutable sigue siendo el
  problema no resuelto por ningún referente. Helix probablemente lo aborde
  con el LLM como compilador dirigido por la especificación.

---

## Referencias consultadas

- XState: https://github.com/statelyai/xstate
- Elm Architecture: https://guide.elm-lang.org/architecture/
- Eiffel DbC: https://www.eiffel.com/values/design-by-contract/introduction/
- TLA+ en soporte de generación con LLMs: https://medium.com/@polyglot_factotum/tla-in-support-of-ai-code-generation-9086fc9715c4
- Self-Spec: https://openreview.net/pdf?id=6pr7BUGkLp
- Alloy + LLMs: https://www.arxiv.org/pdf/2510.23350
- SYSMOBENCH (LLMs y TLA+): https://arxiv.org/pdf/2509.23130
- Idris tipos dependientes: https://www.idris-lang.org/pages/papers.html
