//! Integration tests for wrap/proxy path (Wave B / S-B3).
//!
//! Covers:
//! - scaffold status/router (existing)
//! - real OS child process speaking newline-delimited JSON-RPC (mock MCP)
//! - binary-level wrap: security-mcp --stdio --wrap over a mock child
//!
//! Bulletin `docs/bulletins/security-mcp-wrap.md` stays DRAFT until the full
//! STABLE checklist (consumer acks, human sign-off) is complete; these tests
//! close the S-B3 "integration tests with mock child server" exit criterion.

use std::process::Stdio;
use std::time::Duration;

use security_mcp::pipeline::ScreeningConfig;
use security_mcp::screeners::ScreeningPolicy;
use security_mcp::server::{SecurityServer, ServerConfig};
use security_mcp::wrap::{WrapConfig, WrapController};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

/// Minimal newline-delimited JSON-RPC MCP mock (Python).
/// Echoes `tools/list` / `initialize` / other methods with a well-formed result.
const MOCK_MCP_PY: &str = r#"
import json, sys
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    try:
        req = json.loads(line)
    except Exception:
        print(json.dumps({"jsonrpc":"2.0","id":None,"error":{"code":-32700,"message":"parse error"}}), flush=True)
        continue
    rid = req.get("id")
    method = req.get("method") or ""
    if method == "initialize":
        result = {
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {"listChanged": False}},
            "serverInfo": {"name": "mock-child-mcp", "version": "0.0.0"},
        }
    elif method == "tools/list":
        result = {
            "tools": [
                {
                    "name": "child_echo",
                    "description": "mock child tool",
                    "inputSchema": {"type": "object", "properties": {}},
                }
            ]
        }
    elif method == "tools/call":
        result = {
            "content": [{"type": "text", "text": "child-ok"}],
            "isError": False,
        }
    else:
        result = {"ok": True, "method": method}
    print(json.dumps({"jsonrpc": "2.0", "id": rid, "result": result}), flush=True)
"#;

fn python3_available() -> bool {
    std::process::Command::new("python3")
        .arg("-c")
        .arg("import sys; sys.exit(0)")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn write_mock_script() -> tempfile::TempPath {
    use std::io::Write;
    let mut f = tempfile::NamedTempFile::new().expect("temp mock script");
    f.write_all(MOCK_MCP_PY.as_bytes()).expect("write mock");
    f.into_temp_path()
}

#[tokio::test]
async fn wrap_controller_status_when_disabled() {
    let wrap = WrapController::new(None);
    let status = wrap.status().await;
    assert_eq!(status["wrap_enabled"], false);
}

#[tokio::test]
async fn server_router_includes_optional_wrap_routes() {
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        screening: ScreeningConfig::default(),
        policy: ScreeningPolicy::default(),
        wrap: Some(WrapConfig {
            command: "echo".to_string(),
            args: vec!["wrap-test".to_string()],
        }),
        enable_websocket: true,
        enable_sse: true,
        ..Default::default()
    };
    let server = SecurityServer::new(config);
    let _router = server.router();
}

/// S-B3: spawn a real OS child that speaks JSON-RPC over stdio and assert
/// `WrapController::forward_request` returns a parsed result for `tools/list`.
#[tokio::test]
async fn real_child_mcp_stdio_roundtrip() {
    if !python3_available() {
        // sh one-shot fallback: single response, still a real child process.
        let wrap = WrapController::new(Some(WrapConfig {
            command: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "read -r line; printf '%s\\n' \"{\\\"jsonrpc\\\":\\\"2.0\\\",\\\"id\\\":1,\\\"result\\\":{\\\"tools\\\":[{\\\"name\\\":\\\"child_echo\\\"}]}}\"".to_string(),
            ],
        }));
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {}
        });
        let response = timeout(Duration::from_secs(5), wrap.forward_request(&request))
            .await
            .expect("forward timed out")
            .expect("forward request succeeds");
        assert_eq!(response["result"]["tools"][0]["name"], "child_echo");
        let status = wrap.status().await;
        assert_eq!(status["wrap_enabled"], true);
        return;
    }

    let script = write_mock_script();
    let wrap = WrapController::new(Some(WrapConfig {
        command: "python3".to_string(),
        args: vec![script.to_string_lossy().into_owned()],
    }));

    let list_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });
    let list_resp = timeout(Duration::from_secs(5), wrap.forward_request(&list_req))
        .await
        .expect("tools/list timed out")
        .expect("tools/list forward");
    assert_eq!(list_resp["id"], 1);
    assert_eq!(list_resp["result"]["tools"][0]["name"], "child_echo");

    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "s-b3-test", "version": "0.0.0"}
        }
    });
    let init_resp = timeout(Duration::from_secs(5), wrap.forward_request(&init_req))
        .await
        .expect("initialize timed out")
        .expect("initialize forward");
    assert_eq!(init_resp["id"], 2);
    assert_eq!(init_resp["result"]["serverInfo"]["name"], "mock-child-mcp");

    let status = wrap.status().await;
    assert_eq!(status["wrap_enabled"], true);
    assert_eq!(status["child_running"], true);
    assert_eq!(status["command"], "python3");
}

/// Binary-level wrap: outer `security-mcp --stdio --wrap` screens/forwards to a mock child MCP.
#[tokio::test]
async fn wrap_binary_stdio_forwards_tools_list_to_mock_child() {
    if !python3_available() {
        eprintln!("skip wrap_binary_stdio_forwards_tools_list_to_mock_child: python3 missing");
        return;
    }

    let script = write_mock_script();
    let bin = env!("CARGO_BIN_EXE_security-mcp");

    let mut child = Command::new(bin)
        .args([
            "--stdio",
            "--wrap",
            "--wrap-command",
            "python3",
            "--wrap-arg",
            script.to_str().expect("utf8 path"),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn security-mcp --stdio --wrap");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);

    // Non-local methods forward to the child when wrap is on.
    let list_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/list",
        "params": {}
    });
    let payload = list_req.to_string() + "\n";
    stdin.write_all(payload.as_bytes()).await.expect("write");
    stdin.flush().await.expect("flush");

    let mut line = String::new();
    timeout(Duration::from_secs(8), reader.read_line(&mut line))
        .await
        .expect("read timed out")
        .expect("read tools/list response");
    assert!(!line.trim().is_empty(), "empty tools/list response");

    let resp: serde_json::Value = serde_json::from_str(line.trim()).expect("json");
    assert_eq!(resp["id"], 10);
    assert_eq!(
        resp["result"]["tools"][0]["name"], "child_echo",
        "expected mock child tools/list, got: {resp}"
    );

    // Local screening tool must not be forwarded (handled by outer process).
    let call_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "tools/call",
        "params": {
            "name": "proxy_status",
            "arguments": {}
        }
    });
    let payload = call_req.to_string() + "\n";
    stdin.write_all(payload.as_bytes()).await.expect("write");
    stdin.flush().await.expect("flush");

    line.clear();
    timeout(Duration::from_secs(8), reader.read_line(&mut line))
        .await
        .expect("proxy_status read timed out")
        .expect("read proxy_status");
    let status_resp: serde_json::Value = serde_json::from_str(line.trim()).expect("json");
    assert_eq!(status_resp["id"], 11);
    assert!(
        status_resp.get("error").is_none(),
        "proxy_status error: {status_resp}"
    );
    // CallToolResult content is a text blob containing JSON status.
    let text = status_resp["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or("");
    assert!(
        text.contains("wrap_enabled") || text.contains("\"wrap_enabled\""),
        "proxy_status body missing wrap_enabled: {text}"
    );

    let _ = child.kill().await;
    let _ = child.wait().await;
}
