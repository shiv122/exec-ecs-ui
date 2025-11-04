#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod aws;
mod terminal;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            aws::sso_login,
            aws::cancel_sso_login,
            aws::list_aws_profiles,
            aws::ecs_list_clusters,
            aws::ecs_list_services,
            aws::ecs_list_tasks,
            aws::ecs_describe_tasks,
            aws::check_required_tools,
            terminal::start_exec_session,
            terminal::write_exec_stdin,
            terminal::close_exec_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
