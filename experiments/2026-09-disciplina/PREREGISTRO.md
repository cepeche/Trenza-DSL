# Pre-registro: ¿la disciplina estructural de Trenza reduce los defectos de un LLM?

**Fecha de registro:** 2026-09-25, antes de ejecutar ninguna réplica. El
commit que añade este archivo hace de sello temporal: cualquier cambio
posterior al protocolo quedará en el historial de git y se justificará en
la sección *Desviaciones*.

**Diseño y ejecución:** Claude, en una sesión de Claude Code, a petición de
César Pérez-Chirinos.
**Aprobación del diseño:** César aprobó el diseño general (tareas de cambio
funcional sobre el cronómetro, dos condiciones, 5 réplicas y un presupuesto
de 20–50 $). Las tareas concretas y el oráculo los redactó Claude; ver
*Amenazas*.

## Pregunta

La tesis del paper es que Trenza convierte en error de compilación la
indisciplina de quien especifica, sea humano o modelo. Lo que se mide aquí
es: cuando un agente LLM hace un cambio funcional sensible al modo de la
aplicación, **¿introduce menos defectos si trabaja sobre la especificación
Trenza que si trabaja sobre el código JavaScript original?**

## Hipótesis

- **H1.** La proporción de réplicas **correctas** es mayor en B (Trenza) que
  en A (JavaScript). Una réplica es correcta si cumple todos los cambios
  pedidos y no introduce ninguna regresión.
- **H2.** El número medio de **regresiones** por réplica es menor en B.
- **H0 (resultado nulo, también publicable).** No hay diferencia apreciable.
  Es plausible, porque las tareas son pequeñas y los modelos actuales son
  buenos. Si ocurre, se informará tal cual.

Con 5 réplicas por celda el experimento es **exploratorio**. No se harán
contrastes de significación; se informarán las proporciones por celda y
todas las réplicas individuales.

## Condiciones

| | A: JavaScript | B: Trenza |
|---|---|---|
| Material | `frontend/` y `tests/` del CronometroPSP original (`history/inspirations/cronometro-psp-original`) | `cronometro.trz` (copia de `examples/cronometro-wasm/src/cronometro_full.trz` en el commit del registro), `GUIA-TRENZA.md` y el binario `trenza-cli` |
| Qué modifica el agente | `index.html`, `js/app.js` | `cronometro.trz` |
| Verificación a su alcance | `node`, el runner de tests existente | `trenza-cli check` |

Las dos líneas base **no tienen el mismo comportamiento**: el `.trz` es una
especificación reconstruida y difiere del JS en detalles. Por ejemplo, en el
`.trz` tocar una tarea inicia la sesión directamente, y en el JS abre antes
el diálogo de comentario. Por eso cada condición se compara **con su propia
línea base** (ver *Oráculo*).

## Tareas

Están en `tareas/`, con su texto literal. El prompt de cada réplica es:
`comun.md` + el bloque de su condición + la tarea. De la sección "Nombres
obligatorios" se eliminan las líneas de la otra condición.

| Id | Cambio | Por qué es sensible al modo |
|---|---|---|
| T1 | Nuevo **modo pausa** con botón ⏸ | Es un modo nuevo y transversal: cada elemento de la pantalla necesita un comportamiento en ese modo. Es la forma del bug original (`modoEdicion` comprobado en 5 sitios). |
| T2 | En modo edición, Frecuentes abre un diálogo nuevo | Añade un overlay y cambia un comportamiento solo en un modo. |
| T3 | En modo edición, las pestañas de actividad cambian de pestaña | Cambia un comportamiento en un modo y debe dejar el otro intacto. |
| T4 | En modo edición, **+** abre "crear actividad" | Igual que T3, pero con reencaminamiento a otro diálogo. |

## Réplicas

- 4 tareas × 2 condiciones × **5 réplicas** = 40 ejecuciones.
- **Agente:** subagente de Claude Code (`general-purpose`, modelo `sonnet`),
  con un directorio de trabajo propio y recién copiado para cada réplica.
- El orden de ejecución alterna las condiciones.
- **Tope de gasto:** 60 $. Si se alcanza, se para y se informa de lo hecho.

## Oráculo

El oráculo es automático y aplica la misma lógica a las dos condiciones.

**Vocabulario de acciones abstractas** (`tap` sobre):
- `tarjeta_tipo`, `tarjeta_tarea`, `pestana_actividad`, `pestana_frecuentes`,
  `boton_edicion`, `boton_nuevo`, `boton_configuracion`;
- según la tarea, `boton_pausa` (T1) y `boton_cerrar` (T2).

**Cómo se ejecuta cada acción:**
- En A: clic sobre el elemento DOM correspondiente, con jsdom, una API
  simulada y datos fijos (`harness-a/`).
- En B: se genera el Rust a partir del `.trz` del agente, se compila y se
  llama a `System::dispatch_<rol>_tap` con datos por defecto (`harness-b/`).

**Observación normalizada tras cada acción:**
- En A:
  - modo (`edit-mode` / `pause-mode` en `<body>`);
  - overlays abiertos (`.modal-overlay.active` y `#settingsMenu.active`);
  - pestaña activa;
  - número de peticiones no-GET.
- En B:
  - contexto base;
  - pila de overlays;
  - conjunto concurrente;
  - acción producida;
  - si hubo `forbidden`.

**Escenarios:** desde el arranque, cada prefijo de estado × cada acción. Los
prefijos son `[]` (normal), `[boton_edicion]` (edición) y, en T1,
`[boton_pausa]` (pausa).

**Una réplica es correcta si se cumplen las tres condiciones:**
1. Compila y arranca. En A: sin errores de JS al arrancar. En B:
   `trenza-cli check` pasa y el Rust generado compila.
2. Se cumplen todas las **expectativas** de la tarea
   (`oraculo/expectativas.json`).
3. Todas las demás observaciones de los escenarios comparables con la línea
   base son **idénticas a las de la línea base**, es decir, no hay
   regresiones.

**Defecto** es toda expectativa incumplida o toda regresión. En B, recibir
un `forbidden` en un escenario donde la tarea pide que "no haga nada"
cuenta como "no hace nada", pero se anota aparte.

## Métricas

Por réplica:
- correcta (sí/no);
- número de expectativas incumplidas;
- número de regresiones;
- si compila o arranca;
- tokens y duración, según los reporte la herramienta de subagentes;
- en B, cuántas veces ejecutó el agente `trenza-cli check` y cuántas falló,
  si se puede extraer del resumen del agente.

## Amenazas a la validez

1. **Sesgo del diseñador.** Claude diseña las tareas, el oráculo y los dos
   arneses, y además defiende la tesis en el paper. Mitigación: este
   registro previo, el oráculo automático y la publicación de todas las
   réplicas y sus diffs. César debería revisar tareas y oráculo; si lo hace
   después del registro, cualquier cambio irá a *Desviaciones*.
2. **Un solo proveedor.** Solo modelos Claude; Gemini no está disponible en
   este entorno.
3. **Contaminación.** Los agentes trabajan en un directorio temporal fuera
   del repositorio y tienen instrucción de no salir de él, pero el
   repositorio (con este oráculo) es accesible en la misma máquina. No
   podemos garantizar que no lo lean.
4. **Líneas base distintas** (ver *Condiciones*). Cada condición se mide
   contra la suya, pero la dificultad de una misma tarea puede no ser la
   misma en las dos.
5. **Tamaño.** Hay 5 réplicas por celda y 4 tareas pequeñas, así que el
   experimento es exploratorio y no confirmatorio.
6. **Mismo modelo, dos papeles.** El modelo que diseña (Claude) pertenece a
   la misma familia que el que ejecuta las réplicas.

## Desviaciones

*(Vacío en el momento del registro.)*

## Validación del oráculo (antes del registro)

- **Línea base contra sí misma** (T1 y T3, en A y B): 0 regresiones. Todas
  las expectativas fallan, como debe ser, porque el cambio no está hecho.
- **Soluciones de referencia** (`referencia/A/T*/frontend`,
  `referencia/B/T*.trz`), escritas por Claude: las 8 son **correctas**
  según el oráculo.
- **Control negativo:** a la referencia T1 se le quita la protección del
  botón + en modo pausa, en A y en B. El oráculo informa exactamente del
  defecto `(pausa, boton_nuevo)` en las dos condiciones, sin regresiones
  espurias.
