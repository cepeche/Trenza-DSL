## Tarea: modo pausa

Añade un **modo pausa**, que se activa y desactiva con un nuevo botón ⏸.

- En modo normal, tocar ⏸ entra en modo pausa. En modo pausa, tocar ⏸
  vuelve al modo normal.
- En modo pausa **no se puede iniciar ninguna tarea** ni entrar en modo
  edición ni crear tareas: tocar cualquier tarjeta, el botón ✏️ o el botón +
  no hace nada.
- En modo pausa **sí** se puede cambiar de pestaña (igual que en modo
  normal) y abrir el menú ⚙️ (igual que en modo normal).
- En modo edición, tocar ⏸ no hace nada.

**Nombres obligatorios** (los usa el equipo de QA):
- Condición A: el botón tiene `id="pauseButton"`; mientras dure el modo
  pausa, `<body>` tiene la clase `pause-mode`.
- Condición B: el botón es el rol `boton_pausa: Boton`, que recibe el
  evento `tap`; el modo pausa es el contexto base `ModoPausa`.
