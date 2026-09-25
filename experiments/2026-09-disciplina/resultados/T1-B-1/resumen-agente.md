Añadido el modo pausa en la especificación Trenza.

Archivo modificado: `cronometro.trz`

Cambios:
1. `system CronometroPSP`: añadido `ModoPausa` a la lista de `contexts:` (contextos base), junto a `ModoEdicion` y `ModoNormal`.
2. `context ModoNormal`: añadido `role boton_pausa: Boton` con `on tap -> activarPausa`, y la transición `on activarPausa -> ModoPausa`.
3. `context ModoEdicion`: añadido `role boton_pausa: Boton` con `on tap -> ignored`.
4. Nuevo `context ModoPausa`: tarjetas, boton_edicion y boton_nuevo `ignored`; pestañas `cambiarPestana`; boton_configuracion `abrirMenuConfiguracion`; boton_pausa `desactivarPausa` → ModoNormal; `effects: cambiarPestana -> actualizarGridVisible()`.
5. Actualizado el comentario de `MenuConfiguracion`.

Verificación: `trenza-cli check` → "Verificación Semántica: Superada".
