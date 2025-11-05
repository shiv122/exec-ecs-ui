use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{command, Emitter, Window};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, Command as TokioCommand};
use tokio::sync::mpsc;

type SessionId = String;
type Writer = mpsc::UnboundedSender<Vec<u8>>;

struct TerminalState {
    writers: HashMap<SessionId, Writer>,
    processes: HashMap<SessionId, Child>,
}

impl TerminalState {
    fn new() -> Self {
        Self {
            writers: HashMap::new(),
            processes: HashMap::new(),
        }
    }
}

static TERMINAL_STATE: Mutex<Option<TerminalState>> = Mutex::new(None);

#[command]
pub async fn start_exec_session(
    window: Window,
    session_id: String,
    profile: String,
    region: String,
    cluster: String,
    task: String,
    container: String,
    shell_cmd: String,
) -> Result<(), String> {
    // Get PATH with common locations
    let path = {
        let mut paths = Vec::new();
        if let Ok(existing_path) = std::env::var("PATH") {
            paths.push(existing_path);
        }
        #[cfg(target_os = "macos")]
        {
            paths.push("/usr/local/bin".to_string());
            paths.push("/opt/homebrew/bin".to_string());
            paths.push("/opt/homebrew/opt/awscli/bin".to_string());
            paths.push("/usr/bin".to_string());
            paths.push("/bin".to_string());
        }
        #[cfg(target_os = "linux")]
        {
            paths.push("/usr/local/bin".to_string());
            paths.push("/usr/bin".to_string());
            paths.push("/bin".to_string());
        }
        #[cfg(target_os = "windows")]
        {
            if let Ok(program_files) = std::env::var("ProgramFiles") {
                paths.push(format!("{}\\Amazon\\AWSCLIV2", program_files));
            }
        }
        paths.join(if cfg!(target_os = "windows") { ";" } else { ":" })
    };
    
    let mut cmd = TokioCommand::new("aws");
    cmd.env("PATH", &path)
        .arg("ecs")
        .arg("execute-command")
        .arg("--cluster")
        .arg(&cluster)
        .arg("--task")
        .arg(&task)
        .arg("--container")
        .arg(&container)
        .arg("--command")
        .arg(&shell_cmd)
        .arg("--interactive")
        .arg("--region")
        .arg(&region)
        .arg("--profile")
        .arg(&profile);

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    let mut stdout = child.stdout.take().ok_or("Failed to get stdout")?;
    let mut stderr = child.stderr.take().ok_or("Failed to get stderr")?;
    let mut stdin = child.stdin.take().ok_or("Failed to get stdin")?;

    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

    {
        let mut state = TERMINAL_STATE.lock().unwrap();
        if state.is_none() {
            *state = Some(TerminalState::new());
        }
        let state = state.as_mut().unwrap();
        state.writers.insert(session_id.clone(), tx);
        state.processes.insert(session_id.clone(), child);
    }

    let window_clone = window.clone();
    let session_id_clone = session_id.clone();

    tokio::spawn(async move {
        let mut stdout_buf = [0u8; 8192];
        let mut stderr_buf = [0u8; 8192];
        loop {
            tokio::select! {
                result = stdout.read(&mut stdout_buf) => {
                    match result {
                        Ok(0) => break,
                        Ok(n) => {
                            let data = String::from_utf8_lossy(&stdout_buf[..n]).to_string();
                            let _ = window_clone.emit(&format!("term:data:{}", session_id_clone), data);
                        }
                        Err(_) => break,
                    }
                }
                result = stderr.read(&mut stderr_buf) => {
                    match result {
                        Ok(0) => break,
                        Ok(n) => {
                            let data = String::from_utf8_lossy(&stderr_buf[..n]).to_string();
                            let _ = window_clone.emit(&format!("term:error:{}", session_id_clone), data);
                        }
                        Err(_) => break,
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            if stdin.write_all(&data).await.is_err() {
                break;
            }
        }
    });

    let window_clone = window.clone();
    let session_id_clone = session_id.clone();

    tokio::spawn(async move {
        let process_to_wait = {
            let mut state = TERMINAL_STATE.lock().unwrap();
            if let Some(ref mut state) = *state {
                state.processes.remove(&session_id_clone)
            } else {
                None
            }
        };

        if let Some(mut process) = process_to_wait {
            let _ = process.wait().await;
        }

        let _ = window_clone.emit(&format!("term:exit:{}", session_id_clone), ());
    });

    Ok(())
}

#[command]
pub fn write_exec_stdin(session_id: String, data: String) -> Result<(), String> {
    let state = TERMINAL_STATE.lock().unwrap();
    if let Some(ref state) = *state {
        if let Some(writer) = state.writers.get(&session_id) {
            writer
                .send(data.into_bytes())
                .map_err(|e| format!("Failed to send data: {}", e))?;
            return Ok(());
        }
    }
    Err("Session not found".to_string())
}

#[command]
pub async fn close_exec_session(session_id: String) -> Result<(), String> {
    let process_to_kill = {
        let mut state = TERMINAL_STATE.lock().unwrap();
        if let Some(ref mut state) = *state {
            state.writers.remove(&session_id);
            state.processes.remove(&session_id)
        } else {
            None
        }
    };

    if let Some(mut process) = process_to_kill {
        let _ = process.kill().await;
    }

    Ok(())
}

