use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;
use tauri::command;
use tokio::process::{Child, Command as TokioCommand};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

struct SsoLoginState {
    processes: HashMap<String, Child>,
}

impl SsoLoginState {
    fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }
}

static SSO_LOGIN_STATE: OnceLock<Mutex<Option<SsoLoginState>>> = OnceLock::new();

fn get_sso_state() -> &'static Mutex<Option<SsoLoginState>> {
    SSO_LOGIN_STATE.get_or_init(|| Mutex::new(Some(SsoLoginState::new())))
}

async fn check_aws_cli() -> Result<(), String> {
    let output = TokioCommand::new("aws")
        .arg("--version")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => Err("AWS CLI is installed but not working properly. Please check your installation.".to_string()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err("AWS CLI is not installed or not found in PATH. Please install AWS CLI v2 from https://aws.amazon.com/cli/".to_string())
        },
        Err(e) => Err(format!("Failed to check AWS CLI: {}", e))
    }
}

async fn run_json_async(cmd: &str, args: &[&str]) -> Result<Value, String> {
    // Check if AWS CLI is available before running commands
    if cmd == "aws" {
        check_aws_cli().await?;
    }

    let full_command = format!("{} {}", cmd, args.join(" "));
    eprintln!("[DEBUG] Executing command: {}", full_command);
    
    let process = TokioCommand::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                format!("Command '{}' not found. Please ensure it is installed and in your PATH.", cmd)
            } else {
                format!("Failed to execute command: {}", e)
            }
        })?;

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        process.wait_with_output()
    ).await
    .map_err(|_| "Command timeout after 30 seconds".to_string())?
    .map_err(|e| {
        let err_msg = format!("Failed to wait for command: {}", e);
        eprintln!("[DEBUG] Command wait error: {}", err_msg);
        err_msg
    })?;

    eprintln!("[DEBUG] Command exit status: {}", output.status);
    eprintln!("[DEBUG] Command stdout length: {} bytes", output.stdout.len());
    eprintln!("[DEBUG] Command stderr length: {} bytes", output.stderr.len());
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("[DEBUG] Command failed with stderr: {}", stderr);
        
        // Parse common AWS CLI errors and provide user-friendly messages
        let error_msg = stderr.to_string();
        if error_msg.contains("Unable to locate credentials") || error_msg.contains("InvalidClientTokenId") {
            return Err("AWS credentials not found or invalid. Please sign in with SSO first.".to_string());
        } else if error_msg.contains("No such file or directory") || error_msg.contains("config") {
            return Err("AWS configuration file not found. Please ensure ~/.aws/config exists and contains your profiles.".to_string());
        } else if error_msg.contains("profile") && error_msg.contains("not found") {
            return Err("AWS profile not found. Please check your ~/.aws/config file.".to_string());
        } else if error_msg.contains("AccessDenied") || error_msg.contains("UnauthorizedOperation") {
            return Err("Access denied. Your credentials may not have permission for this operation.".to_string());
        } else if error_msg.contains("timeout") || error_msg.contains("Could not connect") {
            return Err("Network timeout or connection error. Please check your internet connection and try again.".to_string());
        }
        
        return Err(format!("AWS CLI error: {}", error_msg.trim()));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    eprintln!("[DEBUG] Command stdout: {}", stdout_str);
    
    match serde_json::from_slice::<Value>(&output.stdout) {
        Ok(value) => {
            eprintln!("[DEBUG] Parsed JSON successfully");
            Ok(value)
        }
        Err(e) => {
            let err_msg = format!("Failed to parse JSON: {}", e);
            eprintln!("[DEBUG] JSON parse error: {}", err_msg);
            Err(err_msg)
        }
    }
}

#[command]
pub async fn sso_login(profile: String) -> Result<String, String> {
    let login_id = format!("sso-{}", profile.clone());
    
    // Cancel any existing login for this profile
    let existing_process = {
        let mut state = get_sso_state().lock().await;
        if let Some(ref mut state) = *state {
            state.processes.remove(&login_id)
        } else {
            None
        }
    };

    if let Some(mut existing) = existing_process {
        let _ = existing.kill().await;
    }

    // Check AWS CLI before attempting login
    check_aws_cli().await?;

    let mut cmd = TokioCommand::new("aws");
    cmd.arg("sso")
        .arg("login")
        .arg("--profile")
        .arg(&profile);

    let child = cmd
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "AWS CLI is not installed or not found in PATH. Please install AWS CLI v2 from https://aws.amazon.com/cli/".to_string()
            } else {
                format!("Failed to execute SSO login: {}", e)
            }
        })?;

    // Store the child process for potential cancellation
    {
        let mut state = get_sso_state().lock().await;
        if let Some(ref mut state) = *state {
            state.processes.insert(login_id.clone(), child);
        }
    }

    // Wait for the process - get it back from state to wait on it
    let process_to_wait = {
        let mut state = get_sso_state().lock().await;
        if let Some(ref mut state) = *state {
            state.processes.remove(&login_id)
        } else {
            None
        }
    };

    let result = if let Some(mut process) = process_to_wait {
        process.wait().await
    } else {
        return Err("Process not found in state".to_string());
    };

    match result {
        Ok(status) if status.success() => {
            Ok("SSO login successful".to_string())
        }
        Ok(status) => {
            let exit_code = status.code().unwrap_or(-1);
            Err(format!("SSO login failed (exit code: {}). Please check your profile configuration in ~/.aws/config and try again.", exit_code))
        }
        Err(e) => {
            Err(format!("SSO login process error: {}. Please ensure AWS CLI is properly installed.", e))
        }
    }
}

#[command]
pub async fn cancel_sso_login(profile: String) -> Result<(), String> {
    let login_id = format!("sso-{}", profile);
    
    let process_to_kill = {
        let mut state = get_sso_state().lock().await;
        if let Some(ref mut state) = *state {
            state.processes.remove(&login_id)
        } else {
            None
        }
    };

    if let Some(mut process) = process_to_kill {
        process.kill().await.map_err(|e| format!("Failed to kill process: {}", e))?;
    }

    Ok(())
}

#[command]
pub fn list_aws_profiles() -> Result<Vec<String>, String> {
    // First check if AWS CLI is available
    let aws_check = std::process::Command::new("aws")
        .arg("--version")
        .output();
    
    match aws_check {
        Ok(output) if !output.status.success() => {
            return Err("AWS CLI is installed but not working properly. Please check your installation.".to_string());
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err("AWS CLI is not installed or not found in PATH. Please install AWS CLI v2 from https://aws.amazon.com/cli/".to_string());
        },
        Err(e) => {
            return Err(format!("Failed to check AWS CLI: {}", e));
        },
        _ => {}
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Could not find home directory")?;

    let config_path = format!("{}/.aws/config", home);
    
    // Check if config file exists
    if !std::path::Path::new(&config_path).exists() {
        return Err(format!(
            "AWS configuration file not found at: {}\n\nPlease create ~/.aws/config with your AWS SSO profiles. Example:\n\n[profile my-profile]\nsso_start_url = https://your-sso-portal.awsapps.com/start\nsso_region = us-east-1\nsso_account_id = 123456789012\nsso_role_name = YourRole\nregion = eu-north-1",
            config_path
        ));
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read AWS config file at {}: {}\n\nPlease ensure the file exists and is readable.", config_path, e))?;

    let mut profiles = Vec::new();
    for line in content.lines() {
        if line.starts_with("[profile ") {
            if let Some(profile) = line.strip_prefix("[profile ").and_then(|s| s.strip_suffix("]")) {
                profiles.push(profile.to_string());
            }
        } else if line.starts_with("[default]") {
            profiles.push("default".to_string());
        }
    }

    if profiles.is_empty() {
        profiles.push("default".to_string());
    }

    Ok(profiles)
}

#[command]
pub async fn ecs_list_clusters(profile: String, region: String) -> Result<Vec<String>, String> {
    eprintln!("[DEBUG] ecs_list_clusters called with profile: {}, region: {}", profile, region);
    
    let v = run_json_async(
        "aws",
        &[
            "ecs",
            "list-clusters",
            "--region",
            &region,
            "--profile",
            &profile,
            "--output",
            "json",
        ],
    ).await?;

    eprintln!("[DEBUG] JSON response: {}", serde_json::to_string(&v).unwrap_or_default());
    
    let cluster_arns_value = &v["clusterArns"];
    eprintln!("[DEBUG] clusterArns field type: {:?}", cluster_arns_value);
    
    let arns = if let Some(array) = cluster_arns_value.as_array() {
        eprintln!("[DEBUG] Found {} clusters in response", array.len());
        array
            .iter()
            .filter_map(|x| {
                if let Some(s) = x.as_str() {
                    eprintln!("[DEBUG] Found cluster ARN: {}", s);
                    Some(s.to_string())
                } else {
                    eprintln!("[DEBUG] Cluster ARN is not a string: {:?}", x);
                    None
                }
            })
            .collect()
    } else {
        eprintln!("[DEBUG] clusterArns is not an array or is missing");
        vec![]
    };

    eprintln!("[DEBUG] Returning {} cluster ARNs", arns.len());
    Ok(arns)
}

#[command]
pub async fn ecs_list_services(
    profile: String,
    region: String,
    cluster: String,
) -> Result<Vec<String>, String> {
    let v = run_json_async(
        "aws",
        &[
            "ecs",
            "list-services",
            "--cluster",
            &cluster,
            "--region",
            &region,
            "--profile",
            &profile,
            "--output",
            "json",
        ],
    ).await?;

    let arns = v["serviceArns"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|x| x.as_str().map(|s| s.to_string()))
        .collect();

    Ok(arns)
}

#[command]
pub async fn ecs_list_tasks(
    profile: String,
    region: String,
    cluster: String,
    service: Option<String>,
) -> Result<Vec<String>, String> {
    let mut args = vec![
        "ecs",
        "list-tasks",
        "--cluster",
        &cluster,
        "--region",
        &region,
        "--profile",
        &profile,
        "--output",
        "json",
    ];

    if let Some(s) = service {
        args.push("--service-name");
        args.push(Box::leak(s.into_boxed_str()));
    }

    let v = run_json_async("aws", &args).await?;

    let arns = v["taskArns"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|x| x.as_str().map(|s| s.to_string()))
        .collect();

    Ok(arns)
}

#[command]
pub async fn ecs_describe_tasks(
    profile: String,
    region: String,
    cluster: String,
    task_arn: String,
) -> Result<Value, String> {
    run_json_async(
        "aws",
        &[
            "ecs",
            "describe-tasks",
            "--cluster",
            &cluster,
            "--tasks",
            &task_arn,
            "--region",
            &region,
            "--profile",
            &profile,
            "--output",
            "json",
        ],
    ).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolStatus {
    name: String,
    installed: bool,
    version: Option<String>,
}

#[command]
pub async fn check_required_tools() -> Result<Vec<ToolStatus>, String> {
    let mut tools = Vec::new();

    // Check AWS CLI
    let aws_cli_status = check_tool("aws", &["--version"]).await;
    tools.push(ToolStatus {
        name: "AWS CLI".to_string(),
        installed: aws_cli_status.is_ok(),
        version: aws_cli_status.ok(),
    });

    // Check Session Manager Plugin
    let ssm_status = check_tool("session-manager-plugin", &["--version"]).await;
    tools.push(ToolStatus {
        name: "Session Manager Plugin".to_string(),
        installed: ssm_status.is_ok(),
        version: ssm_status.ok(),
    });

    Ok(tools)
}

async fn check_tool(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = TokioCommand::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Extract version info (first line usually)
            let version = stdout.lines().next().unwrap_or("").trim().to_string();
            Ok(version)
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Command failed: {}", stderr))
        }
        Err(e) => Err(format!("Command not found: {}", e)),
    }
}

