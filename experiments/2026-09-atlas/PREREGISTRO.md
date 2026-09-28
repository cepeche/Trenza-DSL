# Pre-registro: réplica del experimento de disciplina con un modelo local (ATLAS)

**Fecha de registro:** 2026-09-28, antes de ejecutar ninguna réplica contra
ATLAS. El commit que añade este archivo es el sello; los cambios posteriores
irán a *Desviaciones*.

**Diseño:** Claude, a petición de César Pérez-Chirinos, que aprobó la idea
("adelante") tras una propuesta con el protocolo descrito aquí.
**Ejecución:** `ejecutar.py` en el portátil de César; Claude no interviene
en las réplicas.

## Por qué

El primer experimento (`experiments/2026-09-disciplina/`) dio un resultado
nulo con **efecto techo**: Sonnet resolvió 20/20 tareas en JS y 19/20 en
Trenza. Su propio informe proponía repetirlo con un modelo más débil o de
otro proveedor. `gpt-oss-120b` (OpenAI, pesos abiertos), servido por Ollama
en ATLAS, cumple las dos cosas y no gasta crédito.

## Qué se conserva del primer experimento

Todo se toma del commit de su pre-registro, **2689bcb**, en un worktree
congelado (`.exp-congelado/`):
- las cuatro tareas y el texto común;
- las líneas base: el frontend original para A y `cronometro_full.trz` para B;
- `GUIA-TRENZA.md`, tal como la vieron los agentes;
- el oráculo (`oraculo/evaluar.py`, `expectativas.json`) y los dos arneses;
- `trenza-cli`, compilado desde ese commit. El verificador actual es más
  estricto (R10) y la línea base de B ya no lo pasaría;
- las hipótesis H1, H2 y H0 y el criterio de réplica correcta;
- 4 tareas × 2 condiciones × **5 réplicas**, alternando la condición inicial.

## Qué cambia

| | Primer experimento | Esta réplica |
|---|---|---|
| Modelo | Sonnet (subagente de Claude Code) | `gpt-oss-120b:latest` en Ollama, con `num_ctx` 40960 y los parámetros de muestreo y razonamiento por defecto de Ollama |
| Forma de trabajar | Agente con herramientas: lee, edita y ejecuta lo que quiera | Sin herramientas. Recibe en el prompt todo el material de su condición y responde con bloques BUSCAR/REEMPLAZAR |
| Verificación a su alcance | `trenza-cli check` (B); `node` y el runner de tests (A) | La misma, pero la ejecuta el script tras cada respuesta y le devuelve la salida; hasta **3 rondas de corrección** |
| Material de A | Todo `frontend/` y `tests/` | `index.html`, `js/app.js` y `js/api-client.js` en el prompt. El CSS no, porque ningún cambio lo requiere y consume contexto. |
| Entrega del prompt | `comun.md` + `condicion-X.md` + tarea | `comun.md` + `tareas/condicion-X.md` de esta carpeta (adaptado a "no tienes herramientas") + tarea + `tareas/formato.md` + archivos |

Una réplica termina cuando el modelo responde sin bloques (por ejemplo,
TERMINADO) o tras la tercera ronda de corrección. Los bloques que no se
pueden aplicar (texto no encontrado o repetido, o un archivo no permitido)
no se aplican, y el modelo recibe el aviso.

## Oráculo y métricas

Son las mismas del primer experimento: correcta (sí/no), expectativas
incumplidas, regresiones y si arranca. Además, por réplica:
- número de turnos;
- tokens de entrada y de salida por turno (`prompt_eval_count`, `eval_count`);
- tamaño del razonamiento;
- bloques aplicados y fallidos;
- salida de cada comprobación.

Una réplica cuyo prompt supere `num_ctx`, es decir, con
`prompt_eval_count` ≥ 40960, se anota como **truncada** y se informa aparte.
No se repite.

## Validación antes del registro

Se hizo con un servidor falso que imita a Ollama:
- **Soluciones de referencia** del primer experimento, entregadas como
  bloques (T1-A, T1-B, T3-B, T4-A): **correctas**.
- **Bloque que no aplica**: se informa al modelo y el resto de bloques se
  aplica.
- **Modelo que no cambia nada** (T2-A, T2-B): **no correcta**, sin
  regresiones.

Prompt inicial estimado: unos 18.000 tokens en A y unos 8.700 en B.

## Piloto

Se ejecutan primero **T1-B-1 y T1-A-1**. Si el protocolo no cambia tras el
piloto, cuentan como réplica 1 de T1, igual que en el primer experimento. Si
cambia, se descartan y el cambio va a *Desviaciones*.

## Amenazas a la validez

1. **Protocolo distinto.** Sin herramientas y con rondas fijas, los
   resultados **no son directamente comparables** con los de Sonnet. La
   comparación válida es **A frente a B con este modelo**.
2. **Contexto.** En A el prompt ocupa casi la mitad del contexto, y el
   razonamiento de `gpt-oss` también consume contexto. Puede perjudicar más
   a A que a B. Se medirá.
3. **Formato de edición.** Los bloques exigen copiar texto literal. Un
   modelo más débil puede fallar más en archivos grandes (A) que en uno
   mediano (B). Se contarán los bloques fallidos por condición.
4. **Mismo diseñador.** Las tareas, el oráculo y el protocolo los escribió
   Claude (ver el primer pre-registro).
5. **No determinismo.** El muestreo por defecto y el razonamiento hacen que
   dos ejecuciones difieran. Por eso hay 5 réplicas; el experimento sigue
   siendo exploratorio.

## Desviaciones

**D1 (2026-09-28, tras el piloto; aprobada por César y aplicada en `ejecutar.py` antes de repetir los pilotos).**
En los dos pilotos el modelo escribió bloques con la cabecera mal formada:
en lugar de la ruta puso en la línea `<<<<<<< BUSCAR` la primera línea del
texto que buscaba (T1-A-1, turno 1, 9 bloques) o un nombre de contexto
(`BUSCAR context ModoNormal:`, T1-B-1, turno 2). La expresión regular no los
reconoce, así que el script contó 0 bloques y dio la réplica por
TERMINADA, aunque el modelo sí quería seguir cambiando cosas. T1-A-1 acabó en
un turno sin aplicar nada. El protocolo pre-registrado solo preveía terminar
cuando el modelo respondiera sin bloques.
Cambio propuesto: toda línea que empiece por `<<<<<<< BUSCAR` y no forme un
bloque válido cuenta como **bloque fallido** («cabecera mal formada: tras
BUSCAR va solo la ruta del archivo»). Si hay alguno, la réplica sigue y
consume una ronda de corrección, como cualquier otro bloque que no se pudo
aplicar. Los dos pilotos se descartan y se repiten.

Nota, sin cambio de protocolo: en Windows, `ATLAS-A9` resuelve primero a una
IPv6 de enlace local en la que Ollama no contesta. Se usa
`ATLAS_URL=http://192.168.1.84:11434`.
