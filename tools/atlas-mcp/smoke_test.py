# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp>=1.2", "httpx>=0.27"]
# ///
"""Prueba de humo de atlas_mcp.py sin ATLAS: levanta un servidor falso
compatible con OpenAI, lanza el servidor MCP por stdio como lo haría Claude
y llama a sus herramientas.  Uso:  uv run smoke_test.py
Como la extensión .mcpb:  uv run smoke_test.py --uv
Con ATLAS real:  ATLAS_URL=http://atlas.local:11434/v1 uv run smoke_test.py --real
"""
import asyncio, json, os, sys, tempfile, threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

HERE = Path(__file__).parent
REAL = "--real" in sys.argv


class Fake(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, obj):
        b = json.dumps(obj).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(b)

    def do_GET(self):
        self._send({"data": [{"id": "modelo-falso"}]})

    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        last = body["messages"][-1]["content"]
        self._send({"choices": [{"message": {"content": "ECO: " + last}, "finish_reason": "stop"}],
                    "usage": {"prompt_tokens": 1, "completion_tokens": 1}})


def text(res):
    err = getattr(res, "is_error", getattr(res, "isError", False))
    return err, res.content[0].text


async def main():
    env = dict(os.environ)
    root = tempfile.mkdtemp()
    Path(root, "nota.txt").write_text("contenido de prueba")
    env["ATLAS_FILES_ROOT"] = root
    if not REAL:
        httpd = HTTPServer(("127.0.0.1", 0), Fake)
        threading.Thread(target=httpd.serve_forever, daemon=True).start()
        env["ATLAS_URL"] = f"http://127.0.0.1:{httpd.server_port}/v1"
    # --uv: lanza el servidor igual que la extensión .mcpb (uv run --script).
    if "--uv" in sys.argv:
        env.setdefault("ATLAS_API_KEY", "")
        env.setdefault("ATLAS_MODEL", "")
        params = StdioServerParameters(
            command="uv", args=["run", "--script", str(HERE / "server" / "atlas_mcp.py")], env=env)
    else:
        params = StdioServerParameters(
            command=sys.executable, args=[str(HERE / "server" / "atlas_mcp.py")], env=env)
    async with stdio_client(params) as (r, w):
        async with ClientSession(r, w) as s:
            await s.initialize()
            names = sorted(t.name for t in (await s.list_tools()).tools)
            assert names == ["atlas_ask", "atlas_models"], names
            err, out = text(await s.call_tool("atlas_models", {}))
            print("atlas_models:", out)
            assert not err
            err, out = text(await s.call_tool("atlas_ask", {
                "prompt": "Resume el archivo en una frase.", "files": ["nota.txt"]}))
            print("atlas_ask:", out)
            assert not err and (REAL or "contenido de prueba" in out)
            err, out = text(await s.call_tool("atlas_ask", {"prompt": "x", "files": ["../fuera.txt"]}))
            assert err and "fuera de ATLAS_FILES_ROOT" in out, out
    print("OK")


asyncio.run(main())
