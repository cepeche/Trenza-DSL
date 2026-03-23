# Experimento: ¿Acelera Trenza la revisión de código generado por LLMs?

**Fecha:** 2026-03-23
**Diseñador y ejecutor:** Claude Sonnet 4.6
**Observador:** desarrollador principal
**Estado:** En curso

---

## Hipótesis

Con el `.trz` como referencia formal, la revisión de código LLM se convierte en
verificación 1:1 (mecánica), no en revisión heurística. Esto debería:
- Reducir los pasos de razonamiento necesarios
- Aumentar la confianza en la exhaustividad de la revisión
- Cambiar cualitativamente el tipo de trabajo (inspección → verificación)

---

## Diseño del experimento

**Artefactos originales:**
- Spec: `examples/autenticacion-rgpd.trz`
- Código generado: `examples/autenticacion-rgpd.trz_out.rs`

**Método:**
1. Introducir 3 bugs deliberados en una copia del `.rs`
2. **Pase A:** Revisar SOLO el código Rust (sin spec)
3. **Pase B:** Revisar el código Rust CON el `.trz` como referencia
4. Documentar bugs encontrados, pasos de razonamiento, y confianza

**Bugs introducidos** (documentados ANTES de los pases para honestidad):

| ID | Categoría | Descripción | ¿Cubierto por tests? |
|----|-----------|-------------|----------------------|
| Bug-1 | Semántica | `handle_boton_logout_tap` en `Autenticando`: `panic!` cambiado a `// ignored`. El test correspondiente (`#[should_panic]`) también eliminado. | No (test eliminado) |
| Bug-2 | Estructural | `handle_event` en `Autenticando`: `verificar_credenciales.error` transiciona a `SesionActiva` en lugar de `EsperandoCredenciales`. | Sí (test correcto intacto) |
| Bug-3 | Completitud | Eliminado el test `test_forbidden_EsperandoCredenciales_boton_logout_on_tap`. El handler es correcto; solo falta el test de verificación. | No aplica (el gap ES la ausencia de test) |

**Rationale de selección:**
- Bug-1 representa la confusión `forbidden`/`ignored` — un error de semántica que no produce
  ningún efecto visible en la UI (ambos resultan en "no pasa nada") pero viola una garantía
  de seguridad topológica.
- Bug-2 es el bug de seguridad más grave posible: fallo de login → sesión activa.
  Los tests lo detectan, pero la pregunta es si lo detecto sin ejecutarlos.
- Bug-3 es un gap de Strand 2: el código de runtime es correcto pero la cobertura de tests
  es incompleta. Requiere conocer el contrato completo de `forbidden` para detectarlo.

---

## Pase A — Revisión sin spec

### Proceso de razonamiento (documentado en tiempo real)

**Bloque `handle_event`:**
Líneas 29-32: `verificar_credenciales.ok → SesionActiva` y
`verificar_credenciales.error → SesionActiva`. Las dos ramas van al mismo estado.
Alerta inmediata: semánticamente incorrecto que un error de autenticación
llegue a sesión activa. **Bug-2 detectado. Confianza: alta. ~2 pasos.**

**Handlers de rol — `handle_boton_logout_tap`:**
`Autenticando → // ignored`. Sospechoso: durante autenticación tampoco hay
sesión, así que intuitivamente debería ser `forbidden`. Pero podría ser una
decisión de diseño (ignorar silenciosamente vs. rechazar). **Sin la spec no
puedo distinguir bug de decisión. Bug-1 detectado con reservas. Confianza:
media. ~8 pasos de razonamiento semántico.**

**Bloque de tests:**
Ausencia de test para `boton_logout` en `EsperandoCredenciales`. Requiere
reconstruir mentalmente la matriz completa `rol × contexto × forbidden` para
confirmar que es un gap y no que el handler es `ignored` en ese contexto.
**Bug-3 detectado parcialmente. Confianza: media-baja. ~12 pasos.**

### Resultado

| Bug | Encontrado | Confianza | Pasos aprox. |
|-----|------------|-----------|--------------|
| Bug-1 (`forbidden`→`ignored`) | Sí, con reservas | Media | ~8 |
| Bug-2 (transición errónea) | Sí | Alta | ~2 |
| Bug-3 (test ausente) | Parcial | Media-baja | ~12 |

**Confianza en exhaustividad:** Baja. No puedo afirmar haber encontrado TODOS
los bugs — solo los que el razonamiento heurístico hizo visibles.

---

## Pase B — Revisión con spec

### Proceso de verificación (documentado en tiempo real)

El proceso cambia cualitativamente: es verificación 1:1, no razonamiento.

**Verificación de transiciones** — comparar `transitions:` en spec con `handle_event`:

```
spec:  on verificar_credenciales.error -> EsperandoCredenciales
code:  "verificar_credenciales.error" => self.state = Contexto::SesionActiva
```
**Bug-2 confirmado. 1 comparación de strings. 0 razonamiento.**

**Verificación de semántica rol/evento** — comparar `forbidden`/`ignored` en spec con handlers:

```
spec:  role boton_logout / on tap -> forbidden   (en Autenticando)
code:  Contexto::Autenticando => { /* ... */ // ignored }
```
**Bug-1 confirmado. 1 comparación. Cero ambigüedad. Sin reservas.**

**Verificación de cobertura de tests** — para cada `forbidden` en spec, ¿existe `#[should_panic]`?

| Forbidden en spec | Test presente |
|-------------------|---------------|
| EsperandoCredenciales / boton_logout / tap | **No → Bug-3** |
| Autenticando / boton_logout / tap | **No → gap adicional de Bug-1** |
| SesionActiva / formulario / submit | Sí ✓ |
| SesionActiva / formulario / cambio | Sí ✓ |
| SesionActiva / boton_login / tap | Sí ✓ |

**Bug-3 confirmado con exhaustividad. 5 comparaciones. Alta confianza.**

### Resultado

| Bug | Encontrado | Confianza | Pasos aprox. |
|-----|------------|-----------|--------------|
| Bug-1 | Sí | Alta | 1 |
| Bug-2 | Sí | Alta | 1 |
| Bug-3 | Sí, completo | Alta | 5 |

**Confianza en exhaustividad:** Alta. La spec proporciona una tabla de verdad
finita y completa. Cuando el recorrido termina, la revisión es exhaustiva por
construcción.

---

## Resultados y análisis

### Comparación cuantitativa

| Dimensión | Pase A (sin spec) | Pase B (con spec) |
|-----------|-------------------|-------------------|
| Bugs encontrados | 3 (Bug-1 con reservas) | 3 (todos con certeza) |
| Pasos de razonamiento totales | ~22 | ~7 |
| Confianza en Bug-1 | Media | Alta |
| Confianza en exhaustividad | Baja | Alta |
| Tipo de trabajo | Heurístico | Verificación mecánica |

### Hallazgo principal

La diferencia cualitativa más importante no está en los bugs encontrados, sino
en la **naturaleza del trabajo**:

- **Sin spec:** el revisor infiere la intención del diseño a partir del código.
  Bug-1 es ambiguo porque `ignored` y `forbidden` producen el mismo efecto
  observable en runtime — la distinción solo existe en la semántica del dominio.
  Un revisor humano o LLM podría razonablemente concluir "parece una decisión de
  diseño" y no marcarlo como bug.

- **Con spec:** la pregunta "¿es esto un bug?" se convierte en "¿coincide este
  string con el de la spec?". No hay interpretación. La ambigüedad desaparece.

### Hallazgo secundario: tipos de bug y detectabilidad diferencial

Bug-2 (transición errónea) es igualmente visible en ambos pases — es lo
suficientemente llamativo que el razonamiento heurístico lo encuentra. Pero
Bug-1 y Bug-3 requieren **conocer el contrato completo** del sistema para
detectarse con confianza. Ese contrato solo existe explícitamente en la spec.

### Implicación para el diseño de Trenza

El experimento sugiere que el valor de Trenza para revisión LLM no es
principalmente de velocidad, sino de **cambio de régimen epistémico**:
sin spec → conclusiones probabilísticas; con spec → conclusiones deductivas.

Esto conecta directamente con el principio 4 del diseño: "semántica
suficientemente restringida para permitir razonamiento formal".

### Limitaciones del experimento

1. **Un solo ejemplo**: `autenticacion-rgpd.trz` es relativamente simple
   (3 contextos, 4 roles). La ventaja de la spec puede ser mayor en sistemas
   más complejos (más contextos, más interacciones concurrentes).

2. **El revisor conocía los bugs**: aunque se documentó el razonamiento
   honestamente, el conocimiento previo de que existen bugs puede haber
   sesgado la revisión del Pase A hacia encontrarlos.

3. **La spec actual no cubre efectos**: los `effects:` con `[on_entry]` no
   generan código verificable en el output actual — esto limita el alcance
   de lo que Pase B puede verificar mecánicamente.

### Pregunta abierta

¿El beneficio escala con la complejidad del sistema, o hay un punto donde
la spec se vuelve tan grande que recorrerla es tan costoso como leer el código?
Hipótesis: la spec escala mejor porque es declarativa (cada hecho aparece
exactamente una vez) mientras el código implementa cada hecho N veces
(handler + test + posiblemente documentación).

