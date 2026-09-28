## Tarea: reordenar Frecuentes

En **modo edición**, tocar la pestaña **Frecuentes** debe abrir un nuevo
diálogo "Reordenar frecuentes" (hoy no hace nada). El diálogo tiene un botón
**Cerrar** que lo cierra y deja la aplicación en modo edición, como estaba.
Por ahora el diálogo no necesita más contenido.

En modo normal, la pestaña Frecuentes sigue funcionando como hasta ahora.

**Nombres obligatorios** (los usa el equipo de QA):
- Condición A: el diálogo es un elemento con `id="reorderModal"` y clase
  `modal-overlay`, que se muestra añadiéndole la clase `active` (como los
  demás diálogos); el botón Cerrar tiene `id="btnReorderClose"`.
- Condición B: el diálogo es el overlay `ModalReordenarFrecuentes`; el botón
  Cerrar es el rol `boton_cerrar: Boton`, que recibe el evento `tap`.
