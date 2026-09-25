# Auditoría: afirmaciones del paper Onward! 2026 frente al código

**Fecha:** 2026-09-25
**Autor:** Claude (sesión en la nube, Claude Code)
**Base:** rama `paper` @ `3f5a336` (22 may 2026), que es la más reciente
(`main` está 57 commits por detrás).
**Motivo:** las tres revisiones de Onward! 2026 (rechazo, 23 jun 2026)
coinciden en que el paper no muestra Trenza. El revisor A añade que
esperaba "vaporware, o algo que técnicamente cumple lo prometido si se lee
la letra pequeña". Antes de escribir la sección técnica para 2027 había que
saber qué letra pequeña hay. Este documento lo registra.

**Método:** lectura de `trenza-core/src/{trenza.pest,validator.rs,generator.rs}`,
ejecución de `trenza-cli check` sobre todos los `.trz` del repo y
experimentos mínimos. Las citas `G:NNN` son líneas de
`trenza-core/src/generator.rs` en `3f5a336`.

---

## 1. Resumen

El núcleo de la tesis **se sostiene en un ejemplo mínimo**: si
`ModoEdicion` olvida el manejador de `pestana_frecuentes.tap`, el verificador
emite exactamente un error `[completeness]`
(`paper/onward2027/listings/modos_olvido.trz`, comprobado por
`trenza-core/tests/paper_listings.rs`).

Pero varias afirmaciones del paper de 2026 **no corresponden al código**, y
una de ellas afecta al caso de estudio principal:

> Los 18 contextos de `cronometro_full.trz` declaran `role *: ignored`.
> Ese comodín exime al contexto de las Reglas 1 y 5. Al quitarlo aparecen
> 1.062 errores (531 de completitud y 531 de exhaustividad). La
> verificación del caso de estudio "en <100 ms con las ocho reglas" es, para
> esas dos reglas, vacía.

La causa no es pereza del autor de la especificación, sino que las Reglas 1
y 5 son **globales** (todo par rol·evento de cualquier contexto debe
aparecer en todos los contextos, incluidos overlays y modales). Con 18
contextos heterogéneos eso es inviable, y el comodín es la única salida.
Esto es una **decisión de diseño pendiente**, no un bug (ver §4).

## 2. Tabla de afirmaciones

| # | Afirmación (paper 2026) | Lo que hace el código | Acción propuesta |
|---|---|---|---|
| A1 | "Eight formal verification rules" | Hay 8 reglas numeradas y además `name-reserved`, `initial-*` e `import-mismatch` (Regla 9, ADR-022). La spec `spec/language/03-verification.md` habla de **6** reglas. | Unificar el recuento en spec, paper y código. |
| A2 | R1 hace del bug original "a compile-time error" | Cierto en el ejemplo mínimo. **Falso en el cronómetro real** por `role *: ignored` en todos los contextos. | Decidir el alcance de R1 y R5 (§4) y volver a verificar el cronómetro sin comodines. |
| A3 | R3 (alcanzabilidad) es una regla de error | Es un **warning** (`severity: "warning"`). | Decirlo o cambiarlo. |
| A4 | R6: los datos clasificados solo fluyen a `external` autorizados (`[clasificacion: personal]` / `[autorizado_para: …]`) | **No existe.** R6 comprueba que un **rol** sin `[access: gdpr]` no pase como argumento un campo `x.f` de un `data` anotado `[privacy: gdpr]`. `External` se ignora en el validador. | Describir la R6 real o implementar la prometida. |
| A5 | R7: todo slot tiene al menos un `fills`, todo `fills` apunta a un slot real y el tipo encaja | Solo S1 (el `fills` apunta a un slot existente), S3 (dos contextos llenan el mismo slot) y S4 (manejadores duplicados dentro de un `fills`). **No** se comprueban "al menos un fills" ni la compatibilidad de tipos. | Ajustar el texto o implementar S2 y la comprobación de tipos. |
| A6 | R8: un rol tiene la misma "superficie de eventos" en todos los contextos | Compara el **tipo de dato** del rol (`role x: T`), no sus eventos. | Ajustar el texto. |
| A7 | Las reglas son "formally equivalent" a invariantes TLA+/Alloy | Sin demostración. R3 y R4 son propiedades de grafo sobre la abstracción contexto→contexto. R1, R2, R5 y R8 son restricciones de buena formación, no propiedades temporales. | Retirar la afirmación de equivalencia y reformular (§3). |
| A8 | Strand 1: "exhaustive `match`… verified by `rustc`", "double verification" | Todos los `match` generados terminan en `_ => {}` (G:1107, G:1210, etc.). `rustc` **no** detectaría un contexto olvidado. | Quitar el comodín del generador (se puede, porque el enum `Contexto` es cerrado) o retirar la afirmación. |
| A9 | Strand 2: tests como "algebraic inverse", "algebraically tested runtime" | Tests por ejemplo, uno por transición o por (contexto, rol, evento). El test de exhaustividad es tautológico (G:1600-1617). El test de `[close_overlay]` es casi vacío (G:1541). El test `on_entry` del contexto inicial falla por diseño (G:1649). | Retirar "algebraic". Añadir tests basados en propiedades reales (tarea #3). |
| A10 | Strand 4: el informe de auditoría mapea cada elemento a un resultado de verificación | Las líneas `- [x] Rule N` del informe son **cadenas fijas** (G:1796-1801), no resultados del validador. | Generarlas a partir de los diagnósticos reales. |
| A11 | Multi-target: Rust y TS "cannot diverge" | **Semánticas distintas.** En TS, `on X` en `transitions:` se compara con el nombre de la **acción** devuelta por el manejador (G:581-583, G:619). En Rust, con la cadena pasada a `dispatch`, y los manejadores nunca provocan transiciones. En TS, `[deactivate]` genera código inválido y no hay sub-contextos `initial:` ni despacho concurrente. El intérprete WASM trata `[close_overlay]` como un estado literal (ya documentado en `tests/interpreter_smoke.rs`). | Fijar una semántica (§3) y hacer que ambos generadores la respeten. Mientras tanto, no afirmar la equivalencia. |
| A12 | Paquete `.tzp`: "a signed ZIP" | No hay código `.tzp` ni firma. Solo SHA-256 para nombrar contextos anónimos (ADR-021). | Presentarlo como trabajo futuro. |
| A13 | Verificación en "<100 ms" | **Cierto**, con margen: el cronómetro se parsea en ~0,9 ms y se verifica en ~0,13 ms (`paper/onward2027/bench-2026-09-25.md`). Pero no había código de medición, y el benchmark reveló que el **parseo era cuadrático** (3,1 s para 9.000 líneas). Corregido en `5c4eb01`: ahora 7,6 ms. | Citar la tabla y el script en el paper. |
| A14 | MonitoreoRed "remains without a `.trz`" | `examples/MonitoreoRed.trz` existe (21 abr) y verifica, pero sus tres contextos usan `role *: ignored`, y la sección `effects:` final se adjunta sintácticamente a `Alerting`. | Rehacerlo sin comodines como segundo caso de estudio (tarea #5). |
| A15 | El ejemplo de la spec (`02-grammar.md`, "Complete Example") | No parsea: `effects: iniciarTarea -> POST /api/…` no es una `action_call`. | Sustituirlo por un listado verificado. |

Hallazgos al escribir `trenza-core/tests/rules.rs` (22 tests, uno o más por regla):
- **R6 no ve `self.campo`.** Sólo reconoce `rol.campo` y `binding.campo`. El
  mismo acceso a un dato `[privacy: gdpr]` escrito como `self.nombre` pasa
  sin diagnóstico. Como `self.` es la forma habitual en los ejemplos, el
  hueco es serio. Queda documentado con un test marcado LIMITACIÓN.
- **Transiciones muertas.** R4 mira las transiciones declaradas, pero no
  comprueba que algún manejador del contexto produzca la acción que las
  dispara. Un contexto cuyo único `on back -> A` nunca se activa (porque
  ningún rol produce `back`) se acepta como "capaz de volver".
- **Slot sin `fills`**: no se detecta (confirma A5).

Otros hallazgos menores:
- `trenza-cli --help` entra en pánico porque intenta leer `--help` como archivo.
- `examples/autenticacion-rgpd.trz` y `carrito-checkout.trz` no parsean: usan `action …:` dentro de `external`, algo que la gramática no admite.
- `cronometro_full.trz` tiene los acentos con doble codificación (mojibake, `Ã³`).
- `serializer::tests::test_parse_cimbra_spec` depende de un `cimbra.trz` que no está en el repo (tarea #1).

## 3. Semántica que la sección técnica puede afirmar honestamente

Se describe un **modelo abstracto** `S = (D, C, c₀, R, E, A, H, T, W)`:
- `C` son los contextos; `c₀ ∈ C` es el inicial.
- `H : C × R × E ⇀ A ∪ {ignored, forbidden}` son los manejadores.
- `T : C × A ⇀ C ∪ {stay, close, deactivate}` son las transiciones, disparadas por **acciones** (semántica TS; la del Rust generado debería alinearse con ella).
- `W ⊆ C` son los contextos con comodín.

Sobre ese modelo, cada regla es un predicado decidible en tiempo polinómico:
- **R1, R2, R5 y R8** son restricciones de buena formación, del mismo tipo que la comprobación de exhaustividad de patrones.
- **R3 y R4** son alcanzabilidad directa e inversa en el grafo de contextos. Con la abstracción `close ↦ c₀`, R4 equivale a `AG EF c₀` en CTL sobre ese grafo. Esta es la única conexión con lógica temporal que se puede sostener.

La sección está en `paper/onward2027/sec-language.tex`.

## 4. Pregunta abierta de diseño: alcance de las Reglas 1 y 5

**Opciones:**
1. **Alcance por grupo.** R1 y R5 se aplican entre contextos **hermanos**: los contextos base entre sí y los sub-contextos de un mismo overlay entre sí. Un overlay no hereda la obligación de manejar los roles del modo base, porque los suspende por definición. Es la lectura que coincide con el bug original (`ModoNormal` frente a `ModoEdicion`).
2. **Declaración explícita de la interfaz.** El bloque `system` declara qué roles·eventos forman la "interfaz común" de un grupo de contextos, y R1 se aplica a esa interfaz.
3. **Mantener la regla global y el comodín**, pero hacer que el comodín sea selectivo (`role * except tarjeta: ignored`) y que el informe de auditoría enumere lo que el comodín silencia.

**Recomendación:** la opción 1. Es la más cercana al modelo mental de DCI y no añade sintaxis. Además convierte el comodín en algo raro en lugar de universal. **Decide César.**

Sea cual sea la decisión, el paper de 2027 no debe presentar el cronómetro como verificado por R1 y R5 mientras sus contextos lleven `role *: ignored`.
