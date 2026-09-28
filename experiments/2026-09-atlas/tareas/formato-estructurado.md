**Cómo responder.** No copies ni reescribas texto del archivo. Expresa cada
cambio como una **operación**, una por línea, con esta forma exacta:

    CONTEXTO <base|overlay|concurrent> <Nombre>
    MANEJADOR <Contexto> <rol>: <Tipo> on <evento> -> <destino>
    TRANSICION <Contexto> on <acción> -> <destino>
    QUITAR_MANEJADOR <Contexto> <rol> <evento>
    QUITAR_TRANSICION <Contexto> <acción>

- `CONTEXTO` declara un contexto nuevo en el bloque `system` y lo crea
  vacío; después lo llenas con `MANEJADOR` y `TRANSICION`.
- `MANEJADOR` añade el rol al contexto si no lo tiene; si ya tiene
  `on <evento>`, sustituye su destino. El destino es una acción
  (`guardar`, `abrir(self.id)`), `ignored` o `forbidden`.
- `TRANSICION` añade o sustituye `on <acción> -> <destino>` en la sección
  `transitions:` del contexto (la crea si falta). El destino es un
  contexto, `[close_overlay]`, `[stay]` o `[deactivate]`.

Ejemplo, con nombres inventados:

    CONTEXTO overlay ModalAyuda
    MANEJADOR ModalAyuda boton_cerrar: Boton on tap -> cerrarAyuda
    TRANSICION ModalAyuda on cerrarAyuda -> [close_overlay]
    MANEJADOR ModoNormal boton_ayuda: Boton on tap -> abrirAyuda
    TRANSICION ModoNormal on abrirAyuda -> ModalAyuda

Puedes explicar tu razonamiento en otras líneas: solo se aplican las que
empiezan por una operación. Cuando consideres el cambio terminado y no
quieras tocar nada más, responde solo con la palabra TERMINADO.
