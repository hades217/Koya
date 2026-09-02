use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::{Output, Stdio},
    time::Duration,
};
use tokio::{process::Command, time::timeout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const PROBE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAT_TIMEOUT: Duration = Duration::from_secs(180);
const REMOVED_CREDENTIAL_ENV_VARS: [&str; 7] = [
    "OPENAI_API_KEY",
    "ANTHROPIC_API_KEY",
    "GOOGLE_API_KEY",
    "GEMINI_API_KEY",
    "AZURE_OPENAI_API_KEY",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiStatus {
    pub provider: String,
    pub installed: bool,
    pub authenticated: bool,
    pub available: bool,
    pub version: Option<String>,
    pub account_label: Option<String>,
    pub binding_protocol: Option<String>,
    pub auth_mode: Option<String>,
    pub plan_type: Option<String>,
    pub can_launch_login: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexGenerationAdapterReport {
    pub provider: String,
    pub project_id: String,
    pub job_id: String,
    pub installed: bool,
    pub authenticated: bool,
    pub account_label: Option<String>,
    pub approval_state: String,
    pub approval_fingerprint: String,
    pub idempotency_key: String,
    pub progress_percent: u8,
    pub capability_status: String,
    pub model_access_status: String,
    pub exact_dimensions_status: String,
    pub panorama_mode_status: String,
    pub price_status: String,
    pub quota_status: String,
    pub submission_status: String,
    pub progress_protocol: Vec<String>,
    pub credential_policy: String,
    pub detail: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct AuthStatus {
    authenticated: bool,
    account_label: Option<String>,
    binding_protocol: Option<String>,
    auth_mode: Option<String>,
    plan_type: Option<String>,
}

fn login_shell_path() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        return env::var("PATH").map_err(|error| format!("Unable to inspect PATH: {error}"));
    }
    #[cfg(not(target_os = "windows"))]
    {
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
        let output = std::process::Command::new(shell)
            .args(["-lic", "echo $PATH"])
            .output()
            .map_err(|error| format!("Unable to inspect the login-shell PATH: {error}"))?;
        if !output.status.success() {
            return Err("The login shell could not provide a PATH for Codex.".into());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return path
            .metadata()
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false);
    }
    #[cfg(not(unix))]
    true
}

fn find_codex(path_var: &str) -> Option<PathBuf> {
    for directory in env::split_paths(path_var) {
        let candidate = directory.join("codex");
        if is_executable(&candidate) {
            return Some(candidate);
        }
        #[cfg(target_os = "windows")]
        {
            let candidate = directory.join("codex.exe");
            if is_executable(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

async fn run_probe(binary: &Path, args: &[&str]) -> Result<Output, String> {
    timeout(
        PROBE_TIMEOUT,
        Command::new(binary)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .output(),
    )
    .await
    .map_err(|_| "The Codex status check timed out.".to_string())?
    .map_err(|error| format!("Unable to run Codex: {error}"))
}

fn parse_app_server_account(value: &serde_json::Value) -> Result<AuthStatus, String> {
    if let Some(error) = value.get("error") {
        let message = error
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("Codex app-server returned an account error.");
        return Err(message.to_string());
    }
    let account = value
        .get("result")
        .and_then(|result| result.get("account"));
    let Some(account) = account.filter(|account| !account.is_null()) else {
        return Ok(AuthStatus {
            binding_protocol: Some("app-server".into()),
            ..AuthStatus::default()
        });
    };
    let auth_mode = account
        .get("type")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let plan_type = account
        .get("planType")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let account_label = match (auth_mode.as_deref(), plan_type.as_deref()) {
        (Some("chatgpt"), Some(plan)) => Some(format!("ChatGPT {plan}")),
        (Some("chatgpt"), None) => Some("ChatGPT account".into()),
        (Some("apiKey"), _) => Some("OpenAI API account".into()),
        (Some(mode), _) => Some(format!("Codex {mode}")),
        _ => Some("Codex account".into()),
    };
    Ok(AuthStatus {
        authenticated: true,
        account_label,
        binding_protocol: Some("app-server".into()),
        auth_mode,
        plan_type,
    })
}

async fn probe_app_server_account(binary: &Path, path_var: &str) -> Result<AuthStatus, String> {
    timeout(PROBE_TIMEOUT, async {
        let mut command = Command::new(binary);
        command
            .args(["app-server", "--stdio"])
            .env("PATH", path_var)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        for variable in REMOVED_CREDENTIAL_ENV_VARS {
            command.env_remove(variable);
        }
        let mut child = command
            .spawn()
            .map_err(|error| format!("Unable to start Codex app-server: {error}"))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "Codex app-server input is unavailable.".to_string())?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Codex app-server output is unavailable.".to_string())?;
        for message in [
            serde_json::json!({
                "method": "initialize",
                "id": 1,
                "params": { "clientInfo": { "name": "estate_studio", "title": "Estate Studio", "version": env!("CARGO_PKG_VERSION") } }
            }),
            serde_json::json!({ "method": "initialized", "params": {} }),
            serde_json::json!({ "method": "account/read", "id": 2, "params": { "refreshToken": false } }),
        ] {
            let mut encoded = serde_json::to_vec(&message)
                .map_err(|error| format!("Unable to encode Codex app-server request: {error}"))?;
            encoded.push(b'\n');
            stdin
                .write_all(&encoded)
                .await
                .map_err(|error| format!("Unable to query Codex app-server: {error}"))?;
        }
        stdin
            .flush()
            .await
            .map_err(|error| format!("Unable to flush Codex app-server request: {error}"))?;
        let mut lines = BufReader::new(stdout).lines();
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|error| format!("Unable to read Codex app-server: {error}"))?
        {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) else {
                continue;
            };
            if value.get("id").and_then(serde_json::Value::as_u64) == Some(2) {
                let result = parse_app_server_account(&value);
                let _ = child.kill().await;
                return result;
            }
        }
        let _ = child.kill().await;
        Err("Codex app-server ended before returning account state.".into())
    })
    .await
    .map_err(|_| "The Codex app-server account check timed out.".to_string())?
}

fn parse_codex_auth(output: &Output) -> AuthStatus {
    if !output.status.success() {
        return AuthStatus::default();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout
        .lines()
        .chain(stderr.lines())
        .find(|line| line.to_ascii_lowercase().contains("logged in"));
    AuthStatus {
        authenticated: line.is_some(),
        account_label: line.map(|value| {
            value
                .split_once("using ")
                .map(|(_, method)| format!("{method} account"))
                .unwrap_or_else(|| "Signed in".to_string())
        }),
        binding_protocol: Some("cli-status-fallback".into()),
        auth_mode: None,
        plan_type: None,
    }
}

pub async fn check_status() -> Result<AiStatus, String> {
    let path_var = login_shell_path()?;
    let Some(binary) = find_codex(&path_var) else {
        return Ok(AiStatus {
            provider: "codex".into(),
            installed: false,
            authenticated: false,
            available: false,
            version: None,
            account_label: None,
            binding_protocol: None,
            auth_mode: None,
            plan_type: None,
            can_launch_login: false,
            detail: "Codex is not installed or is not visible on the login-shell PATH.".into(),
        });
    };
    let version = run_probe(&binary, &["--version"])
        .await
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty());
    let auth = match probe_app_server_account(&binary, &path_var).await {
        Ok(status) => status,
        Err(_) => run_probe(&binary, &["login", "status"])
            .await
            .map(|output| parse_codex_auth(&output))
            .unwrap_or_default(),
    };
    let binding_protocol = auth.binding_protocol.clone();
    let auth_mode = auth.auth_mode.clone();
    let plan_type = auth.plan_type.clone();
    Ok(AiStatus {
        provider: "codex".into(),
        installed: true,
        authenticated: auth.authenticated,
        available: auth.authenticated,
        version,
        account_label: auth.account_label,
        binding_protocol: binding_protocol.clone(),
        auth_mode,
        plan_type,
        can_launch_login: cfg!(any(target_os = "macos", target_os = "windows")),
        detail: if auth.authenticated {
            if binding_protocol.as_deref() == Some("app-server") {
                "Bound through the official Codex app-server account interface. Ready for project-aware chat and human-reviewed project-profile drafts; image capability remains a separate check.".into()
            } else {
                "Connected through the Codex CLI compatibility check. Project-aware chat is ready; refresh after upgrading Codex to enable app-server account binding.".into()
            }
        } else {
            "Codex is installed, but its official login session is not active.".into()
        },
    })
}

pub async fn generation_adapter_report(
    project_id: &str,
    job_id: &str,
    approval_state: &str,
    approval_fingerprint: &str,
    idempotency_key: &str,
    progress_percent: u8,
) -> Result<CodexGenerationAdapterReport, String> {
    let status = check_status().await?;
    Ok(CodexGenerationAdapterReport {
        provider: "codex".into(),
        project_id: project_id.into(),
        job_id: job_id.into(),
        installed: status.installed,
        authenticated: status.authenticated,
        account_label: status.account_label,
        approval_state: approval_state.into(),
        approval_fingerprint: approval_fingerprint.into(),
        idempotency_key: idempotency_key.into(),
        progress_percent,
        capability_status: "unavailable".into(),
        model_access_status: "unavailable".into(),
        exact_dimensions_status: "unavailable".into(),
        panorama_mode_status: "unavailable".into(),
        price_status: "unavailable".into(),
        quota_status: "unavailable".into(),
        submission_status: "blocked_capability".into(),
        progress_protocol: vec![
            "queued".into(),
            "submitted".into(),
            "processing".into(),
            "completed".into(),
            "failed".into(),
            "timed_out".into(),
        ],
        credential_policy:
            "official_codex_app_server_account_only; no vendor key in project, payload, argv, or audit"
                .into(),
        detail: if status.authenticated {
            "Codex account binding is active for chat, but app-server account state does not prove an exact image-generation model, 3840 × 1920 panorama support, current price, or image quota. Submission remains blocked until those are reported by an image-capable provider.".into()
        } else {
            "Codex official login is unavailable and no image-generation capability evidence exists. Submission remains blocked.".into()
        },
    })
}

pub fn open_login() -> Result<(), String> {
    let path_var = login_shell_path()?;
    let binary = find_codex(&path_var)
        .ok_or_else(|| "Install Codex before binding Property AI.".to_string())?;
    #[cfg(target_os = "macos")]
    {
        let command = format!(
            "'{}' login",
            binary.to_string_lossy().replace('\'', "'\\''")
        );
        let script = format!("tell application \"Terminal\" to do script \"{}\"\ntell application \"Terminal\" to activate", command.replace('\\', "\\\\").replace('"', "\\\""));
        std::process::Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .map_err(|error| format!("Unable to open the Codex sign-in window: {error}"))?;
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        let binary = binary.to_string_lossy().replace('\'', "''");
        std::process::Command::new("powershell.exe")
            .args([
                "-NoLogo",
                "-NoExit",
                "-NoProfile",
                "-Command",
                &format!("& '{binary}' login"),
            ])
            .env("PATH", path_var)
            .spawn()
            .map_err(|error| format!("Unable to open the Codex sign-in window: {error}"))?;
        return Ok(());
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    Err("Open a terminal and run `codex login`.".into())
}

pub fn parse_chat_output(stdout: &str) -> Result<String, String> {
    let mut content = String::new();
    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("type").and_then(serde_json::Value::as_str) == Some("item.completed")
            && value
                .get("item")
                .and_then(|item| item.get("type"))
                .and_then(serde_json::Value::as_str)
                == Some("agent_message")
        {
            if let Some(text) = value
                .get("item")
                .and_then(|item| item.get("text"))
                .and_then(serde_json::Value::as_str)
            {
                content = text.trim().to_string();
            }
        }
    }
    if content.is_empty() {
        Err("Property AI returned an incomplete response.".into())
    } else {
        Ok(content)
    }
}

pub async fn chat(working_dir: &Path, prompt: &str) -> Result<String, String> {
    let path_var = login_shell_path()?;
    let binary = find_codex(&path_var)
        .ok_or_else(|| "Codex is unavailable. Bind it in Settings.".to_string())?;
    let auth = run_probe(&binary, &["login", "status"])
        .await
        .map(|output| parse_codex_auth(&output))?;
    if !auth.authenticated {
        return Err("Codex is not signed in. Bind it in Settings before chatting.".into());
    }
    let mut command = Command::new(binary);
    command
        .args([
            "exec",
            "--sandbox",
            "read-only",
            "--skip-git-repo-check",
            "--json",
            "-",
        ])
        .current_dir(working_dir)
        .env("PATH", path_var)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    for variable in REMOVED_CREDENTIAL_ENV_VARS {
        command.env_remove(variable);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("Unable to start Property AI: {error}"))?;
    use tokio::io::AsyncWriteExt;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Property AI input is unavailable.".to_string())?;
    stdin
        .write_all(prompt.as_bytes())
        .await
        .map_err(|error| format!("Unable to send the project context: {error}"))?;
    drop(stdin);
    let output = timeout(CHAT_TIMEOUT, child.wait_with_output())
        .await
        .map_err(|_| "Property AI timed out after three minutes.".to_string())?
        .map_err(|error| format!("Property AI stopped unexpectedly: {error}"))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr)
            .lines()
            .last()
            .unwrap_or("Codex returned an error.")
            .to_string();
        return Err(format!(
            "Property AI could not complete the request: {detail}"
        ));
    }
    parse_chat_output(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_agent_message_from_codex_jsonl() {
        let output = "{\"type\":\"thread.started\",\"thread_id\":\"abc\"}\n{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"Koya has four unit types.\"}}\n";
        assert_eq!(
            parse_chat_output(output).unwrap(),
            "Koya has four unit types."
        );
    }
    #[test]
    fn rejects_jsonl_without_an_agent_answer() {
        assert!(parse_chat_output("{\"type\":\"thread.started\"}\n").is_err());
    }
    #[test]
    fn parses_app_server_chatgpt_account_without_exposing_email() {
        let response = serde_json::json!({
            "id": 2,
            "result": {
                "account": { "type": "chatgpt", "email": "private@example.com", "planType": "team" },
                "requiresOpenaiAuth": true
            }
        });
        let auth = parse_app_server_account(&response).unwrap();
        assert!(auth.authenticated);
        assert_eq!(auth.binding_protocol.as_deref(), Some("app-server"));
        assert_eq!(auth.auth_mode.as_deref(), Some("chatgpt"));
        assert_eq!(auth.plan_type.as_deref(), Some("team"));
        assert_eq!(auth.account_label.as_deref(), Some("ChatGPT team"));
        assert!(!format!("{auth:?}").contains("private@example.com"));
    }
    #[test]
    fn app_server_missing_account_is_not_authenticated() {
        let auth = parse_app_server_account(&serde_json::json!({
            "id": 2,
            "result": { "account": null, "requiresOpenaiAuth": true }
        }))
        .unwrap();
        assert!(!auth.authenticated);
        assert_eq!(auth.binding_protocol.as_deref(), Some("app-server"));
    }
    #[test]
    fn codex_adapter_contract_excludes_raw_credential_fields() {
        assert!(REMOVED_CREDENTIAL_ENV_VARS.contains(&"OPENAI_API_KEY"));
        assert!(REMOVED_CREDENTIAL_ENV_VARS.contains(&"AWS_SECRET_ACCESS_KEY"));
        let report = CodexGenerationAdapterReport {
            provider: "codex".into(),
            project_id: "project-test".into(),
            job_id: "job-test".into(),
            installed: true,
            authenticated: true,
            account_label: Some("Signed in".into()),
            approval_state: "not_approved".into(),
            approval_fingerprint: "fingerprint".into(),
            idempotency_key: "attempt".into(),
            progress_percent: 0,
            capability_status: "unavailable".into(),
            model_access_status: "unavailable".into(),
            exact_dimensions_status: "unavailable".into(),
            panorama_mode_status: "unavailable".into(),
            price_status: "unavailable".into(),
            quota_status: "unavailable".into(),
            submission_status: "blocked_capability".into(),
            progress_protocol: vec!["queued".into(), "processing".into()],
            credential_policy: "official_login_session_only".into(),
            detail: "Generation capability unavailable.".into(),
        };
        let encoded = serde_json::to_string(&report).unwrap();
        for forbidden in ["apiKey", "accessToken", "refreshToken", "secretValue"] {
            assert!(!encoded.contains(forbidden));
        }
    }
}
