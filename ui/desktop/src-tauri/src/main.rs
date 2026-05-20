mod commands;

use tauri::Manager;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::toggle_blocking,
            commands::get_stats,
            commands::update_blocklist,
            commands::add_to_allowlist
        ])
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(error) = app {
        tracing::error!(%error, "VoidBlock desktop failed");
    }
}
