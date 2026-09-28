# Pre-registro: B″, edición estructurada en T1

**Fecha de registro:** 2026-09-28, antes de ejecutar ninguna réplica B″
contra ATLAS. El commit que añade este archivo es el sello. Diseño: Claude.
César lo aprobó ("adelante con el experimento de edición estructurada en
T1; primero, entendamos si Trenza aporta algo").

## Pregunta

Con el material limpio (B′), T3 y T4 igualaron a JS (5/5), pero **T1 (modo
pausa) quedó en 0/5** frente a 5/5 en JS. En T1, el verificador detectó lo
que Trenza promete detectar (el botón nuevo sin declarar en los otros
modos), pero el modelo no supo corregirlo. Además, dos de cada tres bloques
BUSCAR/REEMPLAZAR no se aplicaron.

**Si el modelo edita la especificación con operaciones del propio lenguaje
en vez de copiar texto, ¿resuelve T1?**

## Qué cambia respecto a B′: una sola variable

El formato de edición. `tareas/formato-estructurado.md` sustituye a
`tareas/formato.md`. El modelo escribe operaciones, una por línea:

    CONTEXTO <base|overlay|concurrent> <Nombre>
    MANEJADOR <Contexto> <rol>: <Tipo> on <evento> -> <destino>
    TRANSICION <Contexto> on <acción> -> <destino>
    QUITAR_MANEJADOR <Contexto> <rol> <evento>
    QUITAR_TRANSICION <Contexto> <acción>

`ops_trz.py` las aplica sobre el archivo. Cada operación que no se puede
aplicar se le comunica al modelo con su motivo (contexto inexistente, tipo
de rol distinto, forma incorrecta…) y consume la ronda, igual que un
bloque fallido. Los bloques BUSCAR que aparezcan se ignoran y se avisa.

Todo lo demás es igual que en B′:
- el material limpio, la guía congelada y el texto de T1;
- el modelo (el mismo digest si no ha cambiado; se anotará);
- `num_ctx`, las 3 rondas de corrección y el oráculo.

El ejemplo de `formato-estructurado.md` usa nombres inventados
(`ModalAyuda`) y no contiene ninguna pista de la solución de T1.

**Validación antes del registro** (servidor falso):
- La solución de referencia de T1, escrita como 14 operaciones, sale
  **correcta** según el oráculo.
- Una respuesta sin operaciones sale **no correcta**.
- `ops_trz.py` rechaza con un mensaje claro: un tipo de rol distinto, un
  contexto inexistente, un contexto duplicado, una transición inexistente
  y una forma incorrecta.
- `ops_trz.py` edita bien los sub-contextos con sangría de 8 espacios y
  distingue los roles directos de los que están dentro de `fills`.

## Réplicas

Solo **T1-B**, **10 réplicas**: `ejecutar.py --estructurada` (10 por
defecto). Los resultados van a `resultados-estructurada/`.

## Predicciones, fijadas antes de ver los datos

Métrica principal: réplicas correctas de T1 sobre 10. Referencias: B′ T1
0/5, B T1 0/5, A T1 5/5.
- **≥ 6/10** → en T1, la barrera era la edición textual. Con herramientas
  adecuadas, un modelo débil sí aprovecha la disciplina de Trenza en el
  cambio transversal.
- **≤ 2/10** → la dificultad es la tarea en Trenza: pensar un modo nuevo
  en todos los contextos hermanos, no el formato.
- **3 a 5** → no concluyente.

Secundarias:
- operaciones no aplicadas y su motivo;
- reglas que fallan en la última comprobación;
- errores de requisitos que verifican pero el oráculo rechaza (como ⚙️ en
  pausa en B′);
- turnos y tokens.

## Amenazas

1. **Comparación desigual con A.** JS no tiene un equivalente natural de
   estas operaciones, que son precisamente la ventaja estructural que se
   quiere medir. Aun así, el paper no puede presentar B″ frente a A como
   "a igualdad de herramientas".
2. **El conjunto de operaciones lo diseñó Claude sabiendo cómo es la
   solución de T1.** Son las operaciones mínimas del lenguaje (contexto,
   manejador, transición), no atajos para T1, pero el riesgo existe.
3. **Diez réplicas de una sola tarea.** Es exploratorio.
4. **Mismo diseñador**, como en los pre-registros anteriores.
5. **Deriva del modelo.** *Anotado el 2026-09-28, antes de lanzar B″*
   (`GET /api/tags` en 192.168.1.84): digest
   `fb54b1336953877f4c31a3cb884254bda2d4c018d8d1d1a89db42be7833280b6`,
   `modified_at` 2026-09-15T20:32:41+02:00. Es **idéntico** al anotado
   antes de B′ en `PREREGISTRO-BPRIMA.md`.

## Desviaciones

(ninguna todavía)
