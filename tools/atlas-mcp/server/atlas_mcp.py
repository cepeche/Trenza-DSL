# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp>=1.2", "httpx>=0.27"]
# ///
"""atlas-mcp — servidor MCP (stdio) que delega tareas en un modelo local.

Habla con cualquier API compatible con OpenAI (Ollama, llama.cpp server,
LM Studio, vLLM) que corra en ATLAS u otra máquina de la red local. Se
ejecuta en el ordenador del usuario, lanzado por Claude Desktop / Cowork o
por Claude Code; no expone nada a internet.

Configuración (variables de entorno, nunca en el código ni en el chat):
  ATLAS_URL         base de la API OpenAI, p. ej. http://atlas.local:11434/v1
  ATLAS_API_KEY     opcional; se envía como "Authorization: Bearer ..."
  ATLAS_MODEL       modelo por defecto (si falta, el primero de /models)
  ATLAS_TIMEOUT     segundos por petición (defecto 600: los modelos
                    locales pueden ser lentos)
  ATLAS_FILES_ROOT  opcional; si se define, `atlas_ask` puede leer
                    archivos bajo ese directorio y mandárselos al modelo
                    sin que Claude tenga que leerlos (ahorra tokens).
  ATLAS_MAX_FILE_CHARS  límite por archivo (defecto 200000)

Uso:  uv run atlas_mcp.py            (stdio)
"""

from __future__ import annotations

import os
import time
from datetime import datetime
from pathlib import Path

import httpx

try:  # mcp >= 2
    from mcp.server.mcpserver import MCPServer as _Server
    from mcp.server.mcpserver.exceptions import ToolError
except ImportError:  # mcp 1.x
    from mcp.server.fastmcp import FastMCP as _Server
    from mcp.server.fastmcp.exceptions import ToolError

def _env(name: str, default: str = "") -> str:
    # Un campo opcional de la extensión .mcpb que el usuario deja vacío puede
    # llegar vacío o como el marcador sin sustituir: ambos cuentan como "no hay".
    v = os.environ.get(name, "").strip()
    return default if not v or v.startswith("${") else v


URL = _env("ATLAS_URL", "http://localhost:11434/v1").rstrip("/")
API_KEY = _env("ATLAS_API_KEY")
DEFAULT_MODEL = _env("ATLAS_MODEL")
TIMEOUT = float(_env("ATLAS_TIMEOUT", "600"))
FILES_ROOT = _env("ATLAS_FILES_ROOT")
MAX_FILE_CHARS = int(_env("ATLAS_MAX_FILE_CHARS", "200000"))

server = _Server("atlas")

_DIAS = ["lunes", "martes", "miércoles", "jueves", "viernes", "sábado", "domingo"]


def _ahora() -> str:
    # El modelo local no tiene reloj: se le da la fecha en cada petición.
    t = datetime.now().astimezone()
    return f"Fecha y hora actuales: {_DIAS[t.weekday()]} {t.isoformat(timespec='minutes')}."


def _http_error(e: Exception) -> ToolError:
    if isinstance(e, httpx.TimeoutException):
        return ToolError(f"ATLAS no respondió en {TIMEOUT:.0f}s (ATLAS_TIMEOUT).")
    if isinstance(e, httpx.ConnectError):
        return ToolError(f"No se puede conectar con ATLAS en {URL}: ¿está encendido y escuchando en la red?")
    if isinstance(e, httpx.HTTPStatusError):
        return ToolError(f"ATLAS devolvió {e.response.status_code}: {e.response.text[:300]}")
    return ToolError(f"Error hablando con ATLAS: {e}")


def _headers() -> dict[str, str]:
    return {"Authorization": f"Bearer {API_KEY}"} if API_KEY else {}


async def _models(client: httpx.AsyncClient) -> list[str]:
    r = await client.get(f"{URL}/models", headers=_headers())
    r.raise_for_status()
    return [m["id"] for m in r.json().get("data", [])]


def _read_files(paths: list[str]) -> str:
    if not paths:
        return ""
    if not FILES_ROOT:
        raise ToolError("Leer archivos requiere definir ATLAS_FILES_ROOT.")
    root = Path(FILES_ROOT).expanduser().resolve()
    parts = []
    for p in paths:
        f = (root / p).resolve() if not Path(p).is_absolute() else Path(p).resolve()
        if not f.is_relative_to(root):
            raise ToolError(f"{p}: fuera de ATLAS_FILES_ROOT")
        if not f.is_file():
            raise ToolError(f"{p}: no existe o no es un archivo")
        text = f.read_text(encoding="utf-8", errors="replace")
        if len(text) > MAX_FILE_CHARS:
            text = text[:MAX_FILE_CHARS] + "\n[... truncado ...]"
        parts.append(f"<file path=\"{f.relative_to(root)}\">\n{text}\n</file>")
    return "\n\n".join(parts)


@server.tool()
async def atlas_models() -> str:
    """Lista los modelos que sirve ATLAS y cuál se usa por defecto."""
    try:
        async with httpx.AsyncClient(timeout=httpx.Timeout(30.0, connect=5.0)) as client:
            models = await _models(client)
    except httpx.HTTPError as e:
        raise _http_error(e) from e
    default = DEFAULT_MODEL or (models[0] if models else "(ninguno)")
    return f"Por defecto: {default}\nDisponibles:\n" + "\n".join(f"- {m}" for m in models)


async def _complete(messages, model, max_tokens, temperature):
    async with httpx.AsyncClient(timeout=httpx.Timeout(TIMEOUT, connect=5.0)) as client:
        chosen = model or DEFAULT_MODEL
        if not chosen:
            models = await _models(client)
            if not models:
                raise ToolError("ATLAS no anuncia ningún modelo en /models")
            chosen = models[0]
        r = await client.post(
            f"{URL}/chat/completions",
            headers=_headers(),
            json={
                "model": chosen,
                "messages": messages,
                "max_tokens": max_tokens,
                "temperature": temperature,
                "stream": False,
            },
        )
        r.raise_for_status()
        data = r.json()
    return data, chosen


@server.tool()
async def atlas_ask(
    prompt: str,
    system: str = "",
    model: str = "",
    files: list[str] | None = None,
    max_tokens: int = 2048,
    temperature: float = 0.2,
) -> str:
    """Delega una tarea en el modelo local de ATLAS y devuelve su respuesta.

    Úsalo para trabajo acotado y verificable (resumir, clasificar, redactar
    un primer borrador, revisar un archivo largo) cuando no haga falta el
    razonamiento de Claude. El modelo local es más débil: comprueba lo que
    devuelva. `files` son rutas relativas a ATLAS_FILES_ROOT; el servidor
    las lee y las añade al prompt sin pasar por el contexto de Claude.
    El servidor antepone la fecha y hora actuales al mensaje de sistema.
    """
    content = prompt
    attached = _read_files(files or [])
    if attached:
        content = f"{attached}\n\n{prompt}"
    system = f"{_ahora()}\n\n{system}" if system else _ahora()
    messages = [{"role": "system", "content": system}] + [
        {"role": "user", "content": content}
    ]
    t0 = time.monotonic()
    try:
        data, chosen = await _complete(messages, model, max_tokens, temperature)
    except httpx.HTTPError as e:
        raise _http_error(e) from e
    text = data["choices"][0]["message"].get("content") or ""
    u = data.get("usage") or {}
    meta = (
        f"[atlas: modelo={chosen}, {time.monotonic() - t0:.1f}s, "
        f"tokens entrada={u.get('prompt_tokens', '?')}, salida={u.get('completion_tokens', '?')}, "
        f"fin={data['choices'][0].get('finish_reason', '?')}]"
    )
    return f"{text}\n\n{meta}"


if __name__ == "__main__":
    server.run()
