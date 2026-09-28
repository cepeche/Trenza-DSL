# Resultados: réplica con ATLAS (`gpt-oss-120b`)

Ejecutada el 28 sep 2026 en el portátil de César con el protocolo de
`PREREGISTRO.md` y la desviación D1. Tabla completa en `RESUMEN.md`.

## Resultado principal

| | A: JavaScript | B: Trenza |
|---|---|---|
| Réplicas correctas | **19/20** | **4/20** |
| T1 (modo pausa) | 5/5 | 0/5 |
| T2 (reordenar) | 4/5 | 0/5 |
| T3 (pestañas) | 5/5 | 1/5 |
| T4 (botón +) | 5/5 | 3/5 |
| Regresiones | 0 | 0 |
| Réplicas truncadas | 0 | 0 |

**H1 y H2 no se cumplen. El resultado va en sentido contrario:** con este
modelo y este protocolo, trabajar sobre la especificación Trenza produjo
muchos más fallos que trabajar sobre el JS. No hubo regresiones en ninguna
condición. Los fallos de B son de dos tipos: la especificación no verifica
(T1, T2) o no cumple lo pedido (T3, T4).

## Por qué fallan las réplicas B

1. **Los bloques de edición fallan mucho más en B.** No se aplicó el
   **65 %** de los bloques en B (144 de 223), frente al **21 %** en A (36 de
   174). En el primer turno de B, **21 de los 30 bloques apuntaban a
   archivos que no existen** (`contexts/ModoEdicion.trz`, `transitions:`,
   `context ModoNormal:`).
   - Una causa es el **material**: `cronometro_full.trz` es la
     concatenación de 16 archivos y conserva sus cabeceras
     (`-- contexts/ModoEdicion.trz`), así que el modelo creyó que había
     varios archivos.
   - Otra es que el modelo usa el nombre del contexto como ancla
     (`BUSCAR context ModoNormal:`).
   - La tercera son las líneas en blanco múltiples (de 2 a 4 seguidas), que
     el modelo no copia literalmente: en el primer turno, 5 bloques no se
     encontraron por eso.
   - Con tantos bloques perdidos, el cambio queda a medias.
2. **Errores reales de lenguaje.** En T1, el modelo duplicó manejadores
   (R2, determinismo) y no lo corrigió en 3 rondas. El verificador los
   detectó; el modelo no supo arreglarlos.
3. **El defecto del CLI congelado.** En T2, cuatro réplicas terminan con un
   único diagnóstico de alcanzabilidad (R3). Con el CLI de 2689bcb eso hace
   fallar `check`; con el actual sería un aviso. De todos modos, el diálogo
   era inalcanzable porque no llegó a aplicarse el bloque que lo abría, así
   que el cambio tampoco estaba completo.
4. **Diseño correcto, edición fallida.** Leyendo las conversaciones (p. ej.
   T1-B-3), el diseño en Trenza que propone el modelo es razonable: un
   contexto `ModoPausa` con cada botón decidido explícitamente, que es
   justo la disciplina que Trenza busca. Lo que falla es llevarlo al
   archivo.

## Análisis post hoc (no pre-registrado)

`posthoc_mejor_estado.py` reaplica los bloques sobre la línea base:
- **reproduce exactamente el estado final de las 40 réplicas**;
- evalúa también el último estado que pasó la comprobación.

El resultado no cambia (A 19/20, B 4/20). Los estados de B que "pasaban"
`check` en los primeros turnos eran la línea base casi intacta, porque sus
bloques no se habían aplicado.

## Interpretación

- **Lo que se puede afirmar.** Con un modelo abierto de gama media que no
  conoce Trenza y edita por bloques de texto, el coste de un lenguaje nuevo
  (formato, sintaxis, material) supera cualquier beneficio de la
  disciplina. El JS, que el modelo conoce bien, sale casi perfecto.
- **Lo que no se puede afirmar.** Que Trenza sea peor por su semántica.
  El resultado está **confundido** con el formato del material y el
  mecanismo de edición, que afectan mucho más a B. No hay forma de separar
  esos efectos con estos datos.
- **Para el paper.** Hay que informarlo. Es la otra cara del efecto techo
  de Sonnet:
  - con un modelo fuerte, Trenza no aporta en tareas pequeñas;
  - con uno más débil, estorba si el lenguaje no le es familiar y la
    edición es textual.
  El argumento a favor de Trenza tiene que pasar por **herramientas de
  edición estructurada** y material limpio, no por el texto solo.

## Siguiente paso posible (necesitaría pre-registro nuevo)

**B′**: repetir solo la condición B con el material limpio, es decir, la
especificación en sus archivos separados (`spec/reference/cronometro-psp/trenza/`)
o en un solo archivo sin cabeceras falsas ni líneas en blanco repetidas.
Si B′ sube mucho, el cuello de botella era la presentación; si no, es el
lenguaje. Corre en ATLAS y no gasta crédito.

---

# B′: la condición Trenza con el material limpio (28 sep, tarde)

Pre-registro en `PREREGISTRO-BPRIMA.md`, sin desviaciones, con el mismo
digest del modelo que por la mañana. Tabla completa en `RESUMEN-BPRIMA.md`.

| | A (mañana) | B (mañana) | **B′** |
|---|---|---|---|
| Total | 19/20 | 4/20 | **13/20** |
| T1 (modo pausa) | 5/5 | 0/5 | 0/5 |
| T2 (reordenar) | 4/5 | 0/5 | 3/5 |
| T3 (pestañas) | 5/5 | 1/5 | **5/5** |
| T4 (botón +) | 5/5 | 3/5 | **5/5** |
| Bloques no aplicados | 21 % | 65 % | 66 % (110/166) |
| … por ruta inexistente | — | 43 | 14 |
| … no encontrados o repetidos | — | 78 | 24 |
| … por cabecera mal formada | — | 55 | 72 |

**Lectura pre-registrada.** 13/20 supera el umbral de ≥10/20: **la
presentación del material era el cuello de botella principal**. El 4/20 de
la mañana no mide el lenguaje, sino un archivo que parecía dieciséis. En T3
y T4 (cambios locales), Trenza iguala a JS (5/5).

**Lo que no explica la limpieza: T1 (0/5).**
- Es la tarea de añadir un modo nuevo y transversal, la que más se parece
  al bug original.
- En T1-B-1 y T1-B-5, el verificador detecta **justo lo que Trenza promete
  detectar**: el botón nuevo `boton_pausa` no está declarado en
  `ModoNormal` ni en `ModoEdicion` (R1 y R5). El modelo no consigue
  añadirlo en 3 rondas; muchos de sus bloques no se aplican.
- En T1-B-3 y T1-B-4, la especificación verifica, pero en modo pausa ⚙️ no
  abre el menú. Es un error de requisitos: ninguna regla puede detectarlo,
  y el oráculo sí lo detecta.

**Lo que queda en pie.**
1. La edición textual sigue siendo el mayor coste. Aún se pierden dos de
   cada tres bloques, sobre todo porque el modelo escribe
   `BUSCAR context X:` como si el nombre del contexto fuera un ancla. Con
   una herramienta de edición estructurada ("añade a X el manejador…"),
   gran parte de esa pérdida desaparecería. Esto no está probado.
2. Con este modelo, JS sigue por delante: 19/20 frente a 13/20, y la
   diferencia se concentra en T1. No hay ninguna tarea en la que Trenza
   supere a JS.
3. El verificador hace su trabajo, porque ninguna especificación incorrecta
   se da por buena. Pero con un modelo débil, detectar el error no basta si
   no se sabe corregirlo.
