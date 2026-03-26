use anyhow::Result as AnyResult;
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
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
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agents (
                agent_id TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                last_seen TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sender TEXT NOT NULL,
                recipient TEXT NOT NULL,
                subject TEXT NOT NULL,
                body TEXT NOT NULL,
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

    fn register_agent(&self, agent_id: &str, display_name: &str) -> AnyResult<serde_json::Value> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (agent_id, display_name, last_seen) VALUES (?1, ?2, ?3)",
            params![agent_id, display_name, now],
        )?;
        Ok(json!({ "status": "registered", "agent_id": agent_id }))
    }

    fn send_message(&self, sender: &str, to: &str, subject: &str, body: &str) -> AnyResult<serde_json::Value> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO messages (sender, recipient, subject, body, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![sender, to, subject, body, now],
        )?;
        Ok(json!({ "status": "sent", "to": to }))
    }

    fn poll_messages(&self, agent_id: &str) -> AnyResult<serde_json::Value> {
        let mut stmt = self.conn.prepare("SELECT sender, subject, body, created_at FROM messages WHERE recipient = ?1")?;
        let rows = stmt.query_map(params![agent_id], |row| {
            Ok(json!({
                "from": row.get::<_, String>(0)?,
                "subject": row.get::<_, String>(1)?,
                "body": row.get::<_, String>(2)?,
                "timestamp": row.get::<_, String>(3)?
            }))
        })?;
        let messages: Vec<_> = rows.collect::<Result<Vec<_>, _>>()?;
        
        // Clear queue after polling
        self.conn.execute("DELETE FROM messages WHERE recipient = ?1", params![agent_id])?;
        
        Ok(json!({ "messages": messages }))
    }

    fn get_tool_list(&self) -> serde_json::Value {
        json!({
            "tools": [
                {
                    "name": "acquire_lock",
                    "description": "Acquire a mutual exclusion lock on a resource.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "resource": { "type": "string" },
                            "agent": { "type": "string" }
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
                            "resource": { "type": "string" },
                            "agent": { "type": "string" }
                        },
                        "required": ["resource", "agent"]
                    }
                },
                {
                    "name": "get_status",
                    "description": "List all active locks.",
                    "inputSchema": { "type": "object", "properties": {} }
                },
                {
                    "name": "register_agent",
                    "description": "Register an agent in the network coordinator.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "agent_id": { "type": "string" },
                            "display_name": { "type": "string" }
                        },
                        "required": ["agent_id", "display_name"]
                    }
                },
                {
                    "name": "send_message",
                    "description": "Send a message to another agent.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "to": { "type": "string" },
                            "subject": { "type": "string" },
                            "body": { "type": "string" }
                        },
                        "required": ["to", "subject", "body"]
                    }
                },
                {
                    "name": "poll_messages",
                    "description": "Retrieve pending messages for the caller agent.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "agent_id": { "type": "string" }
                        },
                        "required": ["agent_id"]
                    }
                }
            ]
        })
    }
}

fn handle_client(stream: TcpStream, server: Arc<Mutex<Server>>) -> AnyResult<()> {
    let reader = BufReader::new(&stream);
    let mut writer = stream.try_clone()?;
    let mut current_agent: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = json!({ "jsonrpc": "2.0", "error": { "code": -32700, "message": e.to_string() }, "id": null });
                writer.write_all(format!("{}\n", resp).as_bytes())?;
                continue;
            }
        };

        let result = {
            let server = server.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            match req.method.as_str() {
                "initialize" => {
                    // Extract client name for the session if available
                    if let Some(client_info) = req.params.as_ref().and_then(|p| p.get("clientInfo")) {
                        if let Some(name) = client_info.get("name").and_then(|v| v.as_str()) {
                            current_agent = Some(name.to_string());
                        }
                    }
                    Ok(json!({
                        "protocolVersion": "2024-11-05",
                        "capabilities": { "tools": {} },
                        "serverInfo": { "name": "trenza-coord", "version": "0.1.0" }
                    }))
                },
                "tools/list" => Ok(server.get_tool_list()),
                "tools/call" => {
                    let name = req.params.as_ref().and_then(|p| p.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                    let args = req.params.as_ref().and_then(|p| p.get("arguments")).cloned().unwrap_or(json!({}));
                    
                    match name {
                        "acquire_lock" => server.acquire_lock(args.get("resource").and_then(|v| v.as_str()).unwrap_or(""), args.get("agent").and_then(|v| v.as_str()).unwrap_or("")),
                        "release_lock" => server.release_lock(args.get("resource").and_then(|v| v.as_str()).unwrap_or(""), args.get("agent").and_then(|v| v.as_str()).unwrap_or("")),
                        "get_status" => server.get_status(),
                        "register_agent" => server.register_agent(args.get("agent_id").and_then(|v| v.as_str()).unwrap_or(""), args.get("display_name").and_then(|v| v.as_str()).unwrap_or("")),
                        "send_message" => {
                            let sender = current_agent.as_deref().unwrap_or("unknown");
                            server.send_message(sender, args.get("to").and_then(|v| v.as_str()).unwrap_or(""), args.get("subject").and_then(|v| v.as_str()).unwrap_or(""), args.get("body").and_then(|v| v.as_str()).unwrap_or(""))
                        },
                        "poll_messages" => server.poll_messages(args.get("agent_id").and_then(|v| v.as_str()).unwrap_or("")),
                        _ => Err(anyhow::anyhow!("Tool not found")),
                    }
                },
                _ => Err(anyhow::anyhow!("Method not found")),
            }
        };

        if let Some(id) = req.id {
            let resp = match result {
                Ok(res) => json!({ "jsonrpc": "2.0", "result": res, "id": id }),
                Err(e) => json!({ "jsonrpc": "2.0", "error": { "code": -32603, "message": e.to_string() }, "id": id }),
            };
            writer.write_all(format!("{}\n", resp).as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> AnyResult<()> {
    let server = Arc::new(Mutex::new(Server::new()?));
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Trenza-Coord listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream?;
        let server = Arc::clone(&server);
        thread::spawn(move || {
            if let Err(e) = handle_client(stream, server) {
                eprintln!("Client error: {}", e);
            }
        });
    }
    Ok(())
}
