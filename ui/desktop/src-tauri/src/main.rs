mod commands;

use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn build_tray() -> SystemTray {
    SystemTray::new().with_menu(SystemTrayMenu::new())
}

fn main() {
    let tray = build_tray();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::toggle_blocking,
            commands::get_stats,
            commands::update_blocklist,
            commands::add_to_allowlist
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
            }
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(error) = app {
        eprintln!("VoidBlock desktop failed: {error}");
    }
}
