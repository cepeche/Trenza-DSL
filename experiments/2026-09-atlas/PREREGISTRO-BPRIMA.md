# Pre-registro: B′, la condición Trenza con el material limpio

**Fecha de registro:** 2026-09-28, antes de ejecutar ninguna réplica B′
contra ATLAS. El commit que añade este archivo es el sello. Diseño: Claude.
César lo aprobó ("adelante, prepara el pre-registro de B′").

## Pregunta

En la réplica con ATLAS, B (Trenza) acertó 4 de 20 y A (JS) 19 de 20
(`RESULTADOS.md`). No se aplicó el 65 % de los bloques de B, frente al
21 % de A. En el primer turno, 21 de los 30 bloques de B apuntaban a
archivos inexistentes: el `.trz` concatenado conserva las cabeceras de sus
16 archivos de origen.

**¿Cuánto de ese resultado se debe a la presentación del material y no al
lenguaje?**

## Qué cambia: una sola variable

Solo el archivo que recibe el modelo. `material/cronometro_limpio.trz`
se genera con `limpiar_trz.py` a partir de la línea base congelada
(2689bcb). El script:
1. quita las cabeceras de origen (`-- contexts/X.trz`,
   `-- Convertido de …`);
2. repara el mojibake (`DiseÃ±o` → `Diseño`);
3. quita los espacios finales;
4. quita las líneas en blanco, salvo una antes de cada línea de nivel
   superior.

El archivo pasa de 1.570 a 732 líneas y de 33.031 a 26.634 bytes. Se
conservan los demás comentarios.

**Equivalencia comprobada antes del registro:**
- `trenza-cli` (2689bcb) lo verifica sin diagnósticos.
- El Rust generado es **idéntico byte a byte** al de la línea base original.
- El oráculo, comparándolo con la línea base original, da **0
  regresiones** y "arranca" en T1–T4.

**No cambia nada más:**
- el modelo, con el mismo `gpt-oss-120b:latest` en ATLAS;
- `num_ctx`, las 3 rondas y la desviación D1;
- el texto de las tareas, `condicion-B.md`, `formato.md` y la guía congelada;
- el oráculo.

B′ se evalúa contra su propia línea base (el archivo limpio). Es
equivalente, así que las expectativas y las regresiones significan lo mismo.

**A no se repite.** Su material no tenía este problema, y ya dio 19/20.

## Réplicas

T1–T4 × 5 = 20 réplicas, en el mismo orden (por réplica y tarea). Se
ejecutan con `ejecutar.py --bprima` y sus resultados van a
`resultados-bprima/`. No hay piloto aparte: T1-B-1 va primero y, si falla
por algo del script (no del modelo), se corrige, se anota en
*Desviaciones* y se repite.

## Predicciones, fijadas antes de ver los datos

Métrica principal: réplicas correctas de B′ sobre 20, comparadas con las
4/20 de B.
- **B′ ≥ 10/20** → la presentación era el cuello de botella principal.
  El resultado de B sobre el lenguaje no es interpretable tal cual.
- **B′ ≤ 6/20** → la presentación no explica la diferencia con A. Con
  este modelo y este protocolo, el lenguaje mismo (o su edición textual)
  es la dificultad.
- **Entre 7 y 9** → no concluyente; se informa sin forzar una lectura.

Secundarias:
- bloques no aplicados, en total y por causa (ruta inexistente, texto no
  encontrado o repetido, cabecera mal formada), comparados con B;
- reglas que fallan en la última comprobación;
- tokens y tiempo.

Se informa de todas las réplicas.

## Amenazas

1. **La limpieza hace dos cosas a la vez.** Quita las cabeceras y quita
   las líneas en blanco. Si B′ mejora, no se podrá separar qué pesó más.
   Se aceptó para no multiplicar las condiciones; los fallos por causa
   darán una pista.
2. **El archivo limpio es más corto.** El modelo gasta menos contexto, y
   eso también podría ayudar.
3. **Deriva del modelo.** Si ATLAS actualizó el modelo entre ejecuciones,
   no es el mismo. Se anotará el digest de `ollama show gpt-oss-120b`
   antes de ejecutar.
   *Anotado el 2026-09-28, antes de lanzar B′ (`GET /api/tags` en
   192.168.1.84):* digest
   `fb54b1336953877f4c31a3cb884254bda2d4c018d8d1d1a89db42be7833280b6`,
   65.369.018.106 bytes, `modified_at` 2026-09-15T20:32:41+02:00. La
   ejecución de la mañana (A/B, 13:12–15:14) no guardó el digest. Pero la
   etiqueta no se ha reescrito desde el 15-sep, que es cuando se importó
   el modelo en Ollama, así que **es casi seguro el mismo modelo**. Se
   deduce de la fecha y no se ha comparado digest contra digest.
4. **Mismo diseñador y ejecución exploratoria**, como en los dos
   pre-registros anteriores.

## Desviaciones

(ninguna todavía)
