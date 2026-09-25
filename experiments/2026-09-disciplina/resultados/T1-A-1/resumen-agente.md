Implementado el modo pausa (index.html, js/app.js, css/styles.css).
- index.html: botón #pauseButton con onclick="togglePausa()".
- app.js: AppState.modoPausa; togglePausa() (no hace nada en modo edición; alterna pause-mode en body y active en el botón); guardas `if (AppState.modoPausa) return;` en tarjetaClick, tarjetaTouchEnd, mostrarModalCrear y toggleModoEdicion.
- styles.css: estilo .pause-button; body.pause-mode .task-card { pointer-events: none; opacity: .6 }.
Comprobación: runner.js 29/29; script temporal con stub de DOM (11 aserciones), borrado al terminar.
