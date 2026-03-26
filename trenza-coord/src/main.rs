use std::io::{self, BufRead};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::{Context, Result as AnyResult};
use chrono::Utc;

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
                if e.to_string().contains("UNIQUE constraint failed") {
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
        // Only allow releasing if the agent owns it (or implement force)
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
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    let server = Server::new()?;
    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }

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

        let resp = match result {
            Ok(res) => json!({
                "jsonrpc": "2.0",
                "result": res,
                "id": req.id
            }),
            Err(e) => json!({
                "jsonrpc": "2.0",
                "error": { "code": -32603, "message": e.to_string() },
                "id": req.id
            }),
        };

        println!("{}", resp);
    }

    Ok(())
}
