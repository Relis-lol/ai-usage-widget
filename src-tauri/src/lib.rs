mod config;
mod model;
mod providers;
use config::AppConfig;
use model::{ProviderId, ProviderSnapshot, ProviderStatus, SourceType};
use providers::{UsageProvider, claude::ClaudeProvider, codex::CodexProvider};
use std::sync::Mutex;
use tauri::{
    AppHandle, Manager, PhysicalPosition, State, WindowEvent,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
struct AppState(Mutex<AppConfig>);
fn config_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|p| p.join("config.json"))
        .map_err(|_| "App config path unavailable".into())
}
#[tauri::command]
fn get_config(state: State<'_, AppState>) -> AppConfig {
    state.0.lock().expect("config mutex poisoned").clone()
}
#[tauri::command]
fn save_config(
    app: AppHandle,
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window unavailable")?;
    window
        .set_always_on_top(config.always_on_top)
        .map_err(|_| "Could not update always-on-top")?;
    set_startup(&app, config.start_with_windows)?;
    config.save(&config_path(&app)?)?;
    *state.0.lock().expect("config mutex poisoned") = config;
    Ok(())
}
#[tauri::command]
fn refresh_usage(state: State<'_, AppState>) -> Vec<ProviderSnapshot> {
    let c = state.0.lock().expect("config mutex poisoned").clone();
    let mut r = Vec::new();
    if c.codex_enabled {
        r.push(
            CodexProvider
                .read()
                .unwrap_or_else(|e| unavailable(ProviderId::Codex, e)),
        );
    }
    if c.claude_enabled {
        r.push(
            ClaudeProvider
                .read()
                .unwrap_or_else(|e| unavailable(ProviderId::Claude, e)),
        );
    }
    r
}
fn unavailable(provider: ProviderId, message: String) -> ProviderSnapshot {
    ProviderSnapshot {
        provider,
        status: ProviderStatus::Unavailable,
        captured_at: chrono::Utc::now(),
        source_type: SourceType::Unavailable,
        windows: vec![],
        message: Some(message),
    }
}
fn set_startup(app: &AppHandle, enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::{os::windows::process::CommandExt, process::Command};
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let exe = std::env::current_exe().map_err(|_| "Executable path unavailable")?;
        let status = if enabled {
            Command::new("reg")
                .args([
                    "add",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    "AI Usage Widget",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\"", exe.display()),
                    "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .status()
        } else {
            Command::new("reg")
                .args([
                    "delete",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    "AI Usage Widget",
                    "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .status()
        };
        if enabled && !status.map(|s| s.success()).unwrap_or(false) {
            return Err("Could not update Windows startup".into());
        }
    }
    let _ = app;
    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_config_dir()?.join("config.json");
            let config = AppConfig::load(&path);
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.set_always_on_top(config.always_on_top);
                if let (Some(x), Some(y)) = (config.window_x, config.window_y) {
                    let _ = w.set_position(PhysicalPosition::new(x, y));
                }
            }
            app.manage(AppState(Mutex::new(config)));
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let refresh = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &refresh, &quit])?;
            let mut tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("AI Usage Widget")
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "refresh" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.eval("window.refreshUsage&&window.refreshUsage()");
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                        && let Some(window) = tray.app_handle().get_webview_window("main")
                    {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            tray.build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let close = window
                    .state::<AppState>()
                    .0
                    .lock()
                    .map(|c| c.close_to_tray)
                    .unwrap_or(true);
                if close {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
            WindowEvent::Moved(pos) => {
                let state = window.state::<AppState>();
                if let Ok(mut c) = state.0.lock() {
                    c.window_x = Some(pos.x);
                    c.window_y = Some(pos.y);
                    if let Ok(p) = config_path(window.app_handle()) {
                        let _ = c.save(&p);
                    }
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            refresh_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running AI Usage Widget");
}
