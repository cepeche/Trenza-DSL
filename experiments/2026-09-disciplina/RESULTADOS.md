# Resultados: ¿la disciplina estructural de Trenza reduce los defectos de un LLM?

**Fecha:** 2026-09-25 · **Protocolo:** `PREREGISTRO.md`, sellado en `2689bcb`
antes de la primera réplica · **Datos:** `resultados/<id>/`, con el diff, la
evaluación del oráculo, los metadatos y el resumen del agente de cada
réplica · **Tabla:** generada con `analizar.py`.

## Resultado principal

| Tarea | Cond. | Correctas | Tokens (mediana) | Duración s (mediana) | Llamadas a herramientas (mediana) | Líneas de diff (mediana) |
|---|---|---:|---:|---:|---:|---:|
| T1 | A | 5/5 | 102.578 | 101 | 25 | 67 |
| T1 | B | 5/5 | 75.394 | 60 | 14 | 94 |
| T2 | A | 5/5 | 94.315 | 71 | 18 | 35 |
| T2 | B | 5/5 | 71.813 | 33 | 10 | 30 |
| T3 | A | 5/5 | 59.647 | 46 | 12 | 6 |
| T3 | B | 4/5 | 77.126 | 73 | 15 | 23 |
| T4 | A | 5/5 | 74.379 | 55 | 17 | 4 |
| T4 | B | 5/5 | 70.733 | 26 | 7 | 4 |
| **Total** | A | 20/20 | 81.032 | 64 | 18 | 20 |
| **Total** | B | 19/20 | 71.844 | 35 | 10 | 30 |

Réplicas no correctas:
- T3-B-2: arranca=False fallos=0 regresiones=0

Tokens totales de los subagentes: 3.105.624

A = el agente modifica el JavaScript original. B = el agente modifica la
especificación Trenza. En las dos condiciones el agente fue Sonnet, como
subagente de Claude Code.

- **H1 (más réplicas correctas en B): no se sostiene.** A: 20/20. B: 19/20.
- **H2 (menos regresiones en B): no se puede evaluar.** No hubo ninguna
  regresión en ninguna de las dos condiciones.
- **Resultado nulo, con efecto techo.** Las cuatro tareas fueron demasiado
  fáciles para el modelo en ambas condiciones: el oráculo no encontró ni un
  solo defecto de comportamiento. Este diseño no distingue entre las
  condiciones.

## La única réplica incorrecta está en B

**T3-B-2** entregó un `.trz` que no pasa `trenza-cli check`. Su
comportamiento era correcto, pero el protocolo exige que la especificación
verifique.

- **Qué pasó.** Al redirigir la pestaña en modo edición, el overlay
  `ModalEditarActividad` quedaba inalcanzable (R3). El agente decidió no
  borrarlo, porque la tarea decía que se editaría "por otra vía más
  adelante", y dejó el fallo tal cual.
- **Causa contribuyente, que es error del experimentador.** La guía
  `GUIA-TRENZA.md` describe R3 como "aviso", pero el CLI falla también con
  avisos. Está anotado como desviación 2 del pre-registro.

## Observaciones secundarias

Son exploratorias; no estaban pre-registradas como hipótesis.

1. **B fue más barato en 3 de las 4 tareas.** Medianas globales:
   - tokens: unos 72.000 en B frente a 81.000 en A (−11 %);
   - duración: 35 s frente a 64 s;
   - llamadas a herramientas: 10 frente a 18.

   La excepción es T3, donde B tuvo que lidiar con R3. Una interpretación
   plausible es que el agente B no necesita buscar los sitios afectados:
   el verificador se los señala. Hay una alternativa igual de válida: el
   agente A escribió tests y scripts de comprobación que el agente B no
   necesitó.
2. **Rodeos del verificador (T3-B).** Cuatro de las cinco réplicas de T3 en
   B (T3-B-1, 3, 4 y 5) conservaron una transición muerta,
   `on abrirEditarActividad -> ModalEditarActividad`, que ninguna acción
   produce, solo para satisfacer R3. Es el hueco que ya señalaba la
   auditoría: R3 y R4 miran las transiciones declaradas, no si alguna
   acción las dispara. **El verificador empujó a los agentes a introducir
   un olor en la especificación**, y todos lo hicieron conscientemente y
   lo documentaron con un comentario.
3. **Verificación en A.** Ningún agente A tenía un DOM real. Casi todos
   escribieron un DOM simulado o tests de funciones con espías, y varios
   añadieron tests al repositorio. Ninguno de esos tests habría detectado
   una regresión en otra parte de la interfaz; aquí no hizo falta porque no
   hubo ninguna.
4. **Instrucción "no salgas del directorio".** Cinco agentes A (T3-A-1,
   T1-A-3, T2-A-4, T4-A-4 y T1-A-5) escribieron scripts temporales en el
   directorio scratchpad de la sesión, fuera de su directorio de trabajo.
   No consta que ninguno leyera el oráculo.
5. **`forbidden` frente a `ignored`.** En los casos de "no hace nada", los
   20 agentes B eligieron siempre `ignored`, nunca `forbidden`.

## Qué se puede y qué no se puede afirmar en el paper

**Se puede afirmar:**
- Un modelo actual modifica correctamente una especificación Trenza real
  de unas 1.550 líneas con una guía de una página. Lo hizo en 19 de 20
  casos; el fallo restante verificaba el comportamiento pero no pasaba el
  verificador. Esto responde en parte al escepticismo del revisor C.
- En estas tareas, trabajar sobre la especificación **no costó más** que
  trabajar sobre el código, y tendió a costar menos.
- Hay un caso documentado en el que el verificador induce un rodeo
  (transiciones muertas), útil como lección de diseño.

**No se puede afirmar:**
- Que Trenza reduzca los defectos de un LLM. Con estas tareas y este
  modelo no hubo defectos que reducir.

## Siguientes pasos propuestos

Cada uno requiere un pre-registro nuevo.

- **Tareas más difíciles.** Varios cambios encadenados; un modo nuevo que
  interactúe con overlays y con la sesión concurrente; o cambios sobre una
  base que el agente no puede leer entera.
- **Un modelo más débil** (Haiku) o **de otro proveedor** (el de ATLAS, si
  se puede conectar), donde el efecto techo sea menos probable.
- **Corregir la guía o el CLI** para que R3 sea realmente un aviso, y
  **cerrar el hueco de las transiciones muertas** antes de repetir.
