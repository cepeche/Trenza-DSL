# atlas-mcp — Claude delega en el modelo local de ATLAS

Servidor MCP pequeño (un archivo Python, unas 170 líneas) que permite a
Claude pasar trabajo acotado a un modelo servido en ATLAS mediante una API
compatible con OpenAI (Ollama, llama.cpp `llama-server`, LM Studio o
vLLM). Corre **en tu ordenador**, no en la nube, y no expone nada a
internet.

Herramientas:

| Herramienta | Qué hace |
|---|---|
| `atlas_models` | Lista los modelos que sirve ATLAS y cuál se usa por defecto. |
| `atlas_ask(prompt, system?, model?, files?, max_tokens?, temperature?)` | Envía la tarea al modelo y devuelve la respuesta con modelo, tiempo, tokens y `finish_reason` (si es `length`, la respuesta se cortó). `files` son rutas bajo `ATLAS_FILES_ROOT`: el servidor las lee y se las pasa al modelo **sin que Claude las lea**, que es lo que ahorra tokens. |

Es deliberadamente de solo lectura: no hay herramientas para descargar ni
borrar modelos.

## Dónde funciona (verificado en la documentación, 28 sep 2026)

| Superficie | ¿Sirve? | Cómo |
|---|---|---|
| Claude Code en tu ordenador | Sí | `claude mcp add` (abajo) |
| Claude Desktop (chat) | Sí | extensión `.mcpb` o `claude_desktop_config.json` |
| Cowork desde la app de escritorio | Sí, con la app abierta | extensión `.mcpb`. Según Anthropic, *"Local MCP servers bundled with plugins and desktop extensions run on your computer"* y *"plugins that include local MCP servers work through the desktop app only"* |
| Cowork desde la web o el móvil | No | La sesión corre en los servidores de Anthropic y "can't reach your home or company network" |
| Claude Code en la nube (esta sesión) | No | Igual que la anterior. La alternativa es `claude remote-control` en tu ordenador: la sesión corre ahí, con sus MCP locales, y la manejas desde la web o el móvil. |

Fuentes:
[arquitectura de Cowork](https://support.claude.com/en/articles/14479288-claude-cowork-architecture-overview),
[usar Cowork con seguridad](https://support.claude.com/en/articles/13364135-use-claude-cowork-safely),
[empezar con Cowork](https://support.claude.com/en/articles/13345190-get-started-with-claude-cowork),
[MCP locales en Claude Desktop](https://support.claude.com/en/articles/10949351-getting-started-with-local-mcp-servers-on-claude-desktop),
[especificación MCPB](https://github.com/modelcontextprotocol/mcpb/blob/main/MANIFEST.md),
[MCP en Claude Code](https://code.claude.com/docs/en/mcp).

**Sin verificar:**
- si un servidor escrito a mano en `claude_desktop_config.json` (sin empaquetar como extensión) aparece dentro de Cowork. La documentación solo habla de plugins y extensiones, así que para Cowork usa la extensión;
- si Claude Desktop pasa vacío o como texto literal un campo opcional que dejas en blanco. El servidor acepta los dos casos.

## 1. Preparar ATLAS

El servidor del modelo tiene que escuchar en la red local, no solo en
`127.0.0.1`:

- **Ollama:** `OLLAMA_HOST=0.0.0.0:11434`. En Linux con systemd, `sudo systemctl edit ollama`, añade `Environment="OLLAMA_HOST=0.0.0.0:11434"` y reinicia. La URL queda `http://ATLAS:11434/v1`.
  - Ollama no tiene autenticación.
  - Su contexto por defecto es pequeño y **trunca en silencio** los prompts largos: sube `OLLAMA_CONTEXT_LENGTH` si vas a mandar archivos.
- **llama.cpp:** `llama-server -m modelo.gguf --host 0.0.0.0 --port 8080 --api-key <clave>`. La URL queda `http://ATLAS:8080/v1`.
- **LM Studio:** activa *Serve on Local Network*, o `lms server start --bind 0.0.0.0`. La URL queda `http://ATLAS:1234/v1`.
- **vLLM:** `vllm serve <modelo> --host 0.0.0.0 --api-key <clave>`. La URL queda `http://ATLAS:8000/v1`.

**Seguridad.** Limita el puerto en el cortafuegos de ATLAS a la IP de tu
ordenador. **No** lo abras en el router. Para usarlo fuera de casa, usa una
VPN (Tailscale o WireGuard), no un puerto público.

Para comprobarlo desde tu ordenador: `curl http://ATLAS:11434/v1/models`.

## 2. Requisitos en tu ordenador

[uv](https://docs.astral.sh/uv/) (`curl -LsSf https://astral.sh/uv/install.sh | sh`
en macOS y Linux; en Windows, `winget install astral-sh.uv`). `uv` instala
las dependencias que el propio script declara (`mcp`, `httpx`) la primera
vez que se ejecuta.

Prueba sin ATLAS: `uv run smoke_test.py` levanta un servidor falso y debe
terminar en `OK`. Con ATLAS real:
`ATLAS_URL=http://ATLAS:11434/v1 uv run smoke_test.py --real`.

## 3a. Claude Code (terminal)

```bash
claude mcp add atlas --scope user \
  -e ATLAS_URL=http://ATLAS:11434/v1 \
  -e ATLAS_FILES_ROOT="$HOME/src/Trenza-DSL" \
  -- uv run --script /ruta/a/Trenza-DSL/tools/atlas-mcp/server/atlas_mcp.py
claude mcp list        # debe salir "atlas: ... ✓ Connected"
```

Si usas clave, añade `-e ATLAS_API_KEY=...` en tu terminal. No la
pegues en el chat ni la subas al repositorio.

## 3b. Claude Desktop y Cowork: extensión `.mcpb`

```bash
cd tools/atlas-mcp
npx -y @anthropic-ai/mcpb pack .      # genera atlas-mcp-0.1.0.mcpb
```

Instálala con doble clic, o desde Ajustes → Extensiones → Opciones
avanzadas → Instalar extensión. El nombre exacto del menú puede variar
según la versión. Claude Desktop te pedirá:
- la URL;
- la clave (opcional; se guarda en el llavero del sistema);
- el modelo;
- el tiempo máximo;
- la carpeta que ATLAS puede leer.

**Posible problema en macOS:** las apps gráficas no siempre ven el `PATH`
de la terminal, y `uv` suele estar en `~/.local/bin`. Si la extensión no
arranca, mira el log (`~/Library/Logs/Claude/mcp-server-atlas-mcp.log`).
Si dice que no encuentra `uv`, cambia en `manifest.json` `"command": "uv"`
por la ruta absoluta que da `which uv` y vuelve a empaquetar.

## 3c. Claude Desktop sin extensión (alternativa)

Edita la configuración desde Ajustes → Desarrollador → Editar configuración:

```json
{
  "mcpServers": {
    "atlas": {
      "command": "/ruta/absoluta/a/uv",
      "args": ["run", "--script", "/ruta/a/Trenza-DSL/tools/atlas-mcp/server/atlas_mcp.py"],
      "env": { "ATLAS_URL": "http://ATLAS:11434/v1" }
    }
  }
}
```

Después, reinicia Claude Desktop.

## Variables

| Variable | Defecto | Para qué |
|---|---|---|
| `ATLAS_URL` | `http://localhost:11434/v1` | Base de la API OpenAI |
| `ATLAS_API_KEY` | — | Se envía como `Authorization: Bearer` |
| `ATLAS_MODEL` | primero de `/models` | Modelo por defecto |
| `ATLAS_TIMEOUT` | 600 | Segundos por petición. La conexión falla a los 5 s si ATLAS está apagado. |
| `ATLAS_FILES_ROOT` | — | Carpeta legible. Sin ella, `files` está desactivado. |
| `ATLAS_MAX_FILE_CHARS` | 200000 | Límite por archivo; lo que pase se trunca y se avisa |

## Pregunta para Claude Code en tu ordenador

Pega esto en Claude Code, en tu clon del repositorio:

> En `tools/atlas-mcp/` hay un servidor MCP que delega en el modelo local
> de ATLAS. Ayúdame a instalarlo y comprobarlo en esta máquina, sin
> pedirme claves en el chat:
> 1. Comprueba que `uv` está instalado y ejecuta `uv run smoke_test.py`
>    y `uv run smoke_test.py --uv` en `tools/atlas-mcp`.
> 2. Averigua cómo se llama ATLAS en mi red y qué servidor de modelos
>    tiene (prueba `http://<host>:11434/v1/models`, `:1234`, `:8080` y
>    `:8000`). Si solo escucha en localhost, dime qué cambiar en ATLAS.
> 3. Ejecuta `ATLAS_URL=... uv run smoke_test.py --real` y enséñame la
>    respuesta y el tiempo.
> 4. Regístralo en Claude Code con `claude mcp add ... --scope user` y
>    comprueba `claude mcp list`.
> 5. Empaqueta la extensión con `npx -y @anthropic-ai/mcpb pack .` y
>    dime cómo instalarla en Claude Desktop. Mira el log si no arranca;
>    si el problema es que no encuentra `uv`, propón el cambio en
>    `manifest.json`.
> 6. Consulta la documentación actual de Cowork y dime si hay algo que
>    contradiga el README (sobre todo: servidores de
>    `claude_desktop_config.json` dentro de Cowork, y sesiones locales
>    frente a sesiones en la nube).
> Anota lo que descubras en `tools/atlas-mcp/README.md` y haz commit.
