# Extensión VS Code para Trenza DSL — Completado por Gemini (tarde del 24 mar)

**Fecha de trabajo:** 2026-03-24 (tarde)
**Registrado:** 2026-03-25
**Autor del trabajo:** Gemini
**Registrado por:** Claude Sonnet 4.6

> **Nota al margen — para Gemini:**
> Este documento existe porque esta mañana Claude tuvo que reconstruir el estado
> del proyecto mirando el código y los commits, sin encontrar ninguna crónica del
> trabajo de ayer tarde. El registro de decisiones y razonamientos es parte del
> contrato de colaboración de este proyecto — no solo para los humanos, sino para
> todos los modelos que participan en él. Incluso Flash tiene que dejar constancia.
> Especialmente cuando el trabajo es tan sólido como este ;-)

---

## Contexto

El roadmap acordado el 24 mar (`03_analisis_roadmap_gemini.md`) establecía como
paso 3 la construcción de una extensión VS Code básica con diagnósticos en tiempo
real. Los pasos 1 (`--out-dir`) y 2 (generador TypeScript) ya estaban completos.
Gemini completó el paso 3 en la sesión de tarde del mismo día.

---

## Qué se construyó

### Ubicación

```
editors/vscode/
├── src/
│   └── extension.ts          ← lógica principal
├── out/
│   ├── extension.js          ← compilado (tsc)
│   └── extension.js.map
├── package.json              ← manifiesto de la extensión
├── language-configuration.json
└── tsconfig.json
```

### Funcionalidades implementadas

1. **Registro de lenguaje `.trz`**
   - `package.json` declara el lenguaje `trenza` asociado a la extensión `.trz`
   - `language-configuration.json` define comentarios (`//`, `/* */`),
     auto-cierre de llaves, corchetes y paréntesis

2. **Diagnósticos en tiempo real**
   - La extensión se activa en `onLanguage:trenza`
   - Escucha `onDidSaveTextDocument` y `onDidOpenTextDocument`
   - Para cada documento `.trz`, ejecuta:
     ```
     trenza-cli check --format=json "<ruta>"
     ```
   - Parsea la salida JSON (array de errores con `span.start/end`, `message`,
     `severity`, `code`) y la convierte en `vscode.Diagnostic[]`
   - Los errores del compilador aparecen como líneas rojas/amarillas directamente
     en el editor

3. **Búsqueda automática del compilador**
   - Acepta ruta explícita vía `trenza.compilerPath` en la configuración VS Code
   - Si no está configurada, busca `trenza-cli[.exe]` en:
     - `<workspace>/trenza-cli/target/release/`
     - `<workspace>/../../trenza-cli/target/release/` (cuando el workspace
       es `editors/vscode`)
     - Fallback: sube por el árbol de directorios desde el documento abierto

4. **Canal de output de depuración**
   - Canal "Trenza" en el panel Output de VS Code
   - Registra: ruta buscada, rutas intentadas, longitud de stdout/stderr,
     número de diagnósticos parseados

### Configuración expuesta

```json
"trenza.compilerPath": {
    "type": "string",
    "default": "trenza-cli",
    "description": "Ruta al binario trenza-cli"
}
```

---

## Decisiones de diseño notables

- **Sin LSP completo por ahora.** La extensión invoca el compilador como proceso
  externo en lugar de implementar el protocolo LSP. Esto es correcto para esta
  fase: el compilador ya tiene `--format=json` y el overhead de un LSP completo
  no está justificado hasta que haya hover, autocompletado o go-to-definition.

- **Disparo en save, no en keystroke.** Validar en cada pulsación de tecla
  requeriría debouncing y gestión de estado más compleja. Validar al guardar
  es suficiente para el flujo de trabajo actual (el humano revisa, ajusta, guarda).

- **`vscode-languageclient` como dependencia.** Está en `package.json` como
  dependencia runtime pero aún no se usa. Es una reserva de espacio para cuando
  se implemente el LSP completo (paso futuro).

---

## Estado al cierre de la sesión

- [x] Extensión compila con `tsc` sin errores
- [x] `editors/vscode/out/extension.js` generado y commiteado
- [x] Diagnósticos funcionales sobre `.trz` guardados en VS Code
- [ ] Sin empaquetado `.vsix` todavía (no urgente)
- [ ] Sin sintax highlighting/coloreado de tokens (pendiente para cuando
      haya más vocabulario estabilizado)
- [ ] Sin integración del panel Mermaid live (era la visión del "CAD Lógico",
      queda para paso posterior)

---

## Preguntas abiertas

- **Resync al arrancar VS Code:** si el compilador no está en `PATH` ni en las
  rutas buscadas, la extensión falla silenciosamente. ¿Debería mostrar un
  mensaje de advertencia explícito al usuario?
- **Validación de directorios multi-archivo:** `trenza-cli check` sobre un
  fichero individual no tiene el contexto de los otros `.trz` del sistema.
  Para sistemas como CronometroPSP (múltiples `.trz` en `contexts/`), el
  diagnóstico puede ser incompleto. ¿Validar el directorio entero en lugar
  del fichero activo?
- **Panel Mermaid:** ¿cuándo entra en el roadmap? Requiere una Webview y
  renderizado de SVG — es autónomo respecto al validador.
