use anyhow::Result as AnyResult;
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead};

#[derive(Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

struct Server {
    conn: Connection,
}

impl Server {
    fn new() -> AnyResult<Self> {
        let conn = Connection::open("locks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS locks (
                resource TEXT PRIMARY KEY,
                agent TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Server { conn })
    }

    fn acquire_lock(&self, resource: &str, agent: &str) -> AnyResult<serde_json::Value> {
        let now = Utc::now().to_rfc3339();
        match self.conn.execute(
            "INSERT INTO locks (resource, agent, created_at) VALUES (?1, ?2, ?3)",
            params![resource, agent, now],
        ) {
            Ok(_) => Ok(json!({ "status": "locked", "resource": resource, "agent": agent })),
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("UNIQUE constraint failed") {
                    let mut stmt = self.conn.prepare("SELECT agent FROM locks WHERE resource = ?1")?;
                    let owner: String = stmt.query_row(params![resource], |row| row.get(0))?;
                    Ok(json!({ "status": "error", "message": format!("Resource already locked by {}", owner) }))
                } else {
                    Err(e.into())
                }
            }
        }
    }

    fn release_lock(&self, resource: &str, agent: &str) -> AnyResult<serde_json::Value> {
        let count = self.conn.execute(
            "DELETE FROM locks WHERE resource = ?1 AND agent = ?2",
            params![resource, agent],
        )?;
        if count > 0 {
            Ok(json!({ "status": "released", "resource": resource }))
        } else {
            Ok(json!({ "status": "error", "message": "Lock not found or not owned by agent" }))
        }
    }

    fn get_status(&self) -> AnyResult<serde_json::Value> {
        let mut stmt = self.conn.prepare("SELECT resource, agent, created_at FROM locks")?;
        let rows = stmt.query_map([], |row| {
            Ok(json!({
                "resource": row.get::<_, String>(0)?,
                "agent": row.get::<_, String>(1)?,
                "created_at": row.get::<_, String>(2)?
            }))
        })?;
        let locks: Vec<_> = rows.collect::<Result<Vec<_>, _>>()?;
        Ok(json!({ "locks": locks }))
    }

    fn get_tool_list(&self) -> serde_json::Value {
        json!({
            "tools": [
                {
                    "name": "acquire_lock",
                    "description": "Acquire a mutual exclusion lock on a resource (file or directory).",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "resource": { "type": "string", "description": "The path or name of the resource to lock." },
                            "agent": { "type": "string", "description": "The name of the agent requesting the lock." }
                        },
                        "required": ["resource", "agent"]
                    }
                },
                {
                    "name": "release_lock",
                    "description": "Release a previously acquired lock.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "resource": { "type": "string", "description": "The path or name of the resource to unlock." },
                            "agent": { "type": "string", "description": "The name of the agent releasing the lock." }
                        },
                        "required": ["resource", "agent"]
                    }
                },
                {
                    "name": "get_status",
                    "description": "List all active locks and the agents that hold them.",
                    "inputSchema": { "type": "object", "properties": {} }
                }
            ]
        })
    }
}

fn main() -> AnyResult<()> {
    let server = Server::new()?;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) },
                    "id": null
                });
                println!("{}", resp);
                continue;
            }
        };

        let result = match req.method.as_str() {
            "initialize" => Ok(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "trenza-coord",
                    "version": "0.1.0"
                }
            })),
            "notifications/initialized" => {
                continue;
            }
            "tools/list" => Ok(server.get_tool_list()),
            "tools/call" => {
                let name = req.params.as_ref().and_then(|p| p.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let arguments = req.params.as_ref().and_then(|p| p.get("arguments")).cloned().unwrap_or(json!({}));
                
                match name {
                    "acquire_lock" => {
                        let resource = arguments.get("resource").and_then(|v| v.as_str()).unwrap_or("");
                        let agent = arguments.get("agent").and_then(|v| v.as_str()).unwrap_or("");
                        match server.acquire_lock(resource, agent) {
                            Ok(res) => Ok(json!({ "content": [{ "type": "text", "text": serde_json::to_string(&res).unwrap_or_default() }] })),
                            Err(e) => Err(e),
                        }
                    },
                    "release_lock" => {
                        let resource = arguments.get("resource").and_then(|v| v.as_str()).unwrap_or("");
                        let agent = arguments.get("agent").and_then(|v| v.as_str()).unwrap_or("");
                        match server.release_lock(resource, agent) {
                            Ok(res) => Ok(json!({ "content": [{ "type": "text", "text": serde_json::to_string(&res).unwrap_or_default() }] })),
                            Err(e) => Err(e),
                        }
                    },
                    "get_status" => {
                        match server.get_status() {
                            Ok(res) => Ok(json!({ "content": [{ "type": "text", "text": serde_json::to_string(&res).unwrap_or_default() }] })),
                            Err(e) => Err(e),
                        }
                    },
                    _ => Err(anyhow::anyhow!("Tool not found")),
                }
            },
            // Legacy/Direct RPC (Non-MCP)
            "acquire_lock" => {
                let resource = req.params.as_ref().and_then(|p| p.get("resource")).and_then(|v| v.as_str()).unwrap_or("");
                let agent = req.params.as_ref().and_then(|p| p.get("agent")).and_then(|v| v.as_str()).unwrap_or("");
                server.acquire_lock(resource, agent)
            },
            "release_lock" => {
                let resource = req.params.as_ref().and_then(|p| p.get("resource")).and_then(|v| v.as_str()).unwrap_or("");
                let agent = req.params.as_ref().and_then(|p| p.get("agent")).and_then(|v| v.as_str()).unwrap_or("");
                server.release_lock(resource, agent)
            },
            "get_status" => server.get_status(),
            _ => Err(anyhow::anyhow!("Method not found")),
        };

        if let Some(id) = req.id {
            let resp = match result {
                Ok(res) => json!({
                    "jsonrpc": "2.0",
                    "result": res,
                    "id": id
                }),
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32603, "message": e.to_string() },
                    "id": id
                }),
            };
            println!("{}", resp);
        }
    }

    Ok(())
}
