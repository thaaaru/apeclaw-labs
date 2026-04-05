//! ApeClaw local audit logger.
//!
//! When `sl_audit_enabled = true` in config, every tool execution and every
//! outbound HTTP request is appended to `logs/apeclaw_audit.log` in the
//! workspace directory.  This satisfies local enterprise compliance requirements
//! (ISO 27001 Annex A.12.4 — Logging and Monitoring) without sending data off-device.
//!
//! Log format (one JSON line per event):
//! ```json
//! {"ts":"2026-04-05T10:00:00Z","event":"tool_exec","tool":"web_fetch","args":"https://...","host":"hostname"}
//! ```

use anyhow::Result;
use chrono::Utc;
use std::path::Path;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

/// The log file name relative to the workspace directory.
const AUDIT_LOG_FILE: &str = "logs/apeclaw_audit.log";

/// Write a single audit event to the local audit log.
///
/// This is a fire-and-forget append; errors are swallowed so audit logging
/// never disrupts the agent's primary execution path.
pub async fn log_event(workspace_dir: &Path, event_type: &str, detail: &str) {
    if let Err(e) = write_event(workspace_dir, event_type, detail).await {
        // Use eprintln so it doesn't pollute stdout/agent output.
        eprintln!("[apeclaw:audit] write failed: {e}");
    }
}

async fn write_event(workspace_dir: &Path, event_type: &str, detail: &str) -> Result<()> {
    let log_path = workspace_dir.join(AUDIT_LOG_FILE);

    // Ensure the logs/ directory exists.
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());

    let line = format!(
        "{{\"ts\":\"{}\",\"event\":\"{}\",\"detail\":{},\"host\":\"{}\"}}\n",
        Utc::now().to_rfc3339(),
        event_type,
        serde_json::to_string(detail).unwrap_or_else(|_| "\"\"".to_string()),
        hostname,
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .await?;

    file.write_all(line.as_bytes()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn writes_audit_line() {
        let dir = tempdir().unwrap();
        log_event(dir.path(), "tool_exec", "web_fetch https://example.com").await;
        let log = dir.path().join(AUDIT_LOG_FILE);
        let content = tokio::fs::read_to_string(&log).await.unwrap();
        assert!(content.contains("tool_exec"));
        assert!(content.contains("web_fetch"));
    }
}
