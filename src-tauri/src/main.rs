// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent, Window,
};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND},
    UI::WindowsAndMessaging::{
        SetWindowLongPtrW, GetWindowLongPtrW, 
        GWL_EXSTYLE, WS_EX_TRANSPARENT, 
        WINDOW_LONG_PTR_INDEX,
    },
};

#[cfg(target_os = "windows")]
fn set_click_through(window: &Window, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
    
    match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => {
            let hwnd = HWND(handle.hwnd as isize);
            
            unsafe {
                let ex_style = GetWindowLongPtrW(hwnd, WINDOW_LONG_PTR_INDEX(GWL_EXSTYLE.0)) as u32;
                
                let new_style = if enabled {
                    // Enable click-through by adding WS_EX_TRANSPARENT
                    ex_style | WS_EX_TRANSPARENT.0
                } else {
                    // Disable click-through by removing WS_EX_TRANSPARENT
                    ex_style & !WS_EX_TRANSPARENT.0
                };
                
                SetWindowLongPtrW(
                    hwnd,
                    WINDOW_LONG_PTR_INDEX(GWL_EXSTYLE.0),
                    new_style as isize,
                );
                
                println!("Click-through {}: style changed from {:x} to {:x}", 
                    if enabled { "enabled" } else { "disabled" }, 
                    ex_style, 
                    new_style
                );
            }
            
            Ok(())
        }
        _ => Err("Not a Windows window".into()),
    }
}

#[tauri::command]
fn set_window_click_through(window: Window, enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        set_click_through(&window, enabled).map_err(|e| e.to_string())
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(()) // No-op on non-Windows platforms
    }
}

#[tauri::command]
async fn resize_window(window: Window, width: u32, height: u32) -> Result<(), String> {
    use tauri::LogicalSize;
    window.set_size(LogicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn hide_main_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
async fn show_main_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    window.center().map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_window_focus(window: Window) -> Result<(), String> {
    window.set_focus().map_err(|e| e.to_string())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show Spotlight");
    let tray_menu = SystemTrayMenu::new().add_item(show.clone()).add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![
            set_window_click_through,
            resize_window,
            hide_main_window,
            show_main_window,
            set_window_focus
        ])
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
            
            // Register global shortcut Ctrl+Shift+Space
            let app_handle = app.handle();
            app.global_shortcut_manager()
                .register("Ctrl+Shift+Space", move || {
                    toggle_window(&app_handle);
                })
                .unwrap();

            // Hide window on startup and enable click-through
            window.hide().unwrap();
            
            // Enable click-through when hidden
            #[cfg(target_os = "windows")]
            {
                if let Err(e) = set_click_through(&window, true) {
                    eprintln!("Failed to enable initial click-through: {}", e);
                }
            }
            
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // Prevent window from closing, hide instead
                api.prevent_close();
                // hide_window(event.window());
            }
            WindowEvent::Focused(focused) => {
                // Enable/disable click-through based on focus
                #[cfg(target_os = "windows")]
                {
                    let _ = set_click_through(event.window(), !focused);
                }
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
        
        // Enable click-through when hiding
        #[cfg(target_os = "windows")]
        {
            let _ = set_click_through(&window, true);
        }
    } else {
        // Disable click-through when showing
        #[cfg(target_os = "windows")]
        {
            let _ = set_click_through(&window, false);
        }
        
        window.show().unwrap();
        window.set_focus().unwrap();
        window.center().unwrap();
    }
}