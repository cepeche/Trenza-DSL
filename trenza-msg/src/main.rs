use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::env;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
    id: serde_json::Value,
}

struct Client {
    stream: TcpStream,
    next_id: i64,
}

impl Client {
    fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).context("Failed to connect to trenza-coord")?;
        let mut client = Client { stream, next_id: 1 };
        client.handshake()?;
        Ok(client)
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        let id = self.next_id;
        self.next_id += 1;

        let req = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        let req_str = serde_json::to_string(&req)? + "\n";
        self.stream.write_all(req_str.as_bytes())?;

        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let resp: JsonRpcResponse = serde_json::from_str(&line)
            .context(format!("Failed to parse response: {}", line))?;

        if let Some(err) = resp.error {
            anyhow::bail!("RPC Error: {}", err);
        }

        Ok(resp.result.unwrap_or(json!({})))
    }

    fn handshake(&mut self) -> Result<()> {
        self.call("initialize", json!({
            "protocolVersion": "2024-11-05",
            "clientInfo": { "name": "trenza-msg-cli", "version": "0.1.0" }
        }))?;
        Ok(())
    }

    fn tool_call(&mut self, name: &str, arguments: serde_json::Value) -> Result<serde_json::Value> {
        self.call("tools/call", json!({
            "name": name,
            "arguments": arguments
        }))
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: trenza-msg <command> [args...]");
        eprintln!("Commands: register, send, poll, status");
        std::process::exit(1);
    }

    let addr = "127.0.0.1:7878";
    let client_res = Client::connect(addr);
    
    let mut client = match client_res {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Coordination server not found. Attempting auto-start...");
            // Try to find trenza-coord in typical locations
            let exe_name = if cfg!(windows) { "trenza-coord.exe" } else { "trenza-coord" };
            let potential_paths = [
                format!("./target/debug/{}", exe_name),
                format!("../target/debug/{}", exe_name),
                format!("./trenza-coord/target/debug/{}", exe_name),
            ];
            
            let mut started = false;
            for path in potential_paths {
                if std::path::Path::new(&path).exists() {
                    eprintln!("Launching {}...", path);
                    std::process::Command::new(&path)
                        .spawn()
                        .context("Failed to spawn server")?;
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    started = true;
                    break;
                }
            }
            
            if !started {
                anyhow::bail!("Could not find trenza-coord binary. Please run 'cargo build' first.");
            }
            
            Client::connect(addr).context("Failed to connect after auto-start")?
        }
    };

    let cmd = &args[1];

    match cmd.as_str() {
        "register" => {
            if args.len() < 4 { anyhow::bail!("Usage: register <id> <name>"); }
            let res = client.tool_call("register_agent", json!({ "agent_id": args[2], "display_name": args[3] }))?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        "send" => {
            if args.len() < 5 { anyhow::bail!("Usage: send <to> <subject> <body>"); }
            let res = client.tool_call("send_message", json!({ "to": args[2], "subject": args[3], "body": args[4] }))?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        "poll" => {
            if args.len() < 3 { anyhow::bail!("Usage: poll <agent_id>"); }
            let res = client.tool_call("poll_messages", json!({ "agent_id": args[2] }))?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        "status" => {
            let res = client.tool_call("get_status", json!({}))?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        _ => anyhow::bail!("Unknown command: {}", cmd),
    }

    Ok(())
}
