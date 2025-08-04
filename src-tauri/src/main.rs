// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show Spotlight");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                toggle_window(app);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    toggle_window(app);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // Register global shortcut Ctrl+Win+Space
            let app_handle = app.handle();
            app.global_shortcut_manager()
                .register("Ctrl+Shift+Space", move || {
                    toggle_window(&app_handle);
                })
                .unwrap();

            // Hide window on startup
            window.hide().unwrap();
            
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // Prevent window from closing, hide instead
                api.prevent_close();
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_window(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
        window.center().unwrap();
    }
}