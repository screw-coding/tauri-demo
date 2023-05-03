// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, Menu, MenuEntry, MenuItem, RunEvent,
    Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowMenuEvent,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        // 系统窗口相关事件
        .on_window_event(window_event)
        // .menu(menu)
        .menu(window_menu())
        // menu 事件
        .on_menu_event(menu_event)
        .system_tray(system_tray())
        .on_system_tray_event(system_tray_event)
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen])
        .build(context)
        .expect("error while running tauri application");

    app.run(run_event);
}

fn run_event(app: &AppHandle, event: RunEvent) {
    match event {
        tauri::RunEvent::Updater(updater_event) => {
            match updater_event {
                tauri::UpdaterEvent::UpdateAvailable {
                    body,
                    date,
                    version,
                } => {
                    println!("update available {} {:?} {}", body, date, version);
                }
                // Emitted when the download is about to be started.
                tauri::UpdaterEvent::Pending => {
                    println!("update is pending!");
                }
                tauri::UpdaterEvent::DownloadProgress {
                    chunk_length,
                    content_length,
                } => {
                    println!("downloaded {} of {:?}", chunk_length, content_length);
                }
                // Emitted when the download has finished and the update is about to be installed.
                tauri::UpdaterEvent::Downloaded => {
                    println!("update has been downloaded!");
                }
                // Emitted when the update was installed. You can then ask to restart the app.
                tauri::UpdaterEvent::Updated => {
                    println!("app has been updated");
                    app.restart();
                }
                // Emitted when the app already has the latest version installed and an update is not needed.
                tauri::UpdaterEvent::AlreadyUpToDate => {
                    println!("app is already up to date");
                }
                // Emitted when there is an error with the updater. We suggest to listen to this event even if the default dialog is enabled.
                tauri::UpdaterEvent::Error(error) => {
                    println!("failed to update: {}", error);
                }
            }
        }
        _ => {}
    }
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
        println!("close splashscreen");
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn system_tray() -> SystemTray {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}

fn system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

fn window_menu() -> Menu {
    // 菜单相关功能 https://tauri.app/zh-cn/v1/guides/features/menu
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let js_playground = CustomMenuItem::new("js_playground".to_string(), "js_playground");
    let baidu = CustomMenuItem::new("baidu".to_string(), "baidu");
    let submenu: Submenu = Submenu::new(
        "工具",
        Menu::new()
            .add_item(js_playground)
            .add_item(baidu)
            .add_native_item(MenuItem::CloseWindow),
    );
    Menu::with_items([
        MenuEntry::Submenu(Submenu::new(
            "File",
            Menu::with_items([
                MenuItem::CloseWindow.into(),
                MenuItem::Hide.into(),
                MenuItem::Quit.into(),
                #[cfg(target_os = "macos")]
                CustomMenuItem::new("hello", "Hello").into(),
            ]),
        )),
        MenuEntry::Submenu(submenu),
    ])
}

fn menu_event(event: WindowMenuEvent) {
    // 多窗口 https://tauri.app/zh-cn/v1/guides/features/multiwindow
    match event.menu_item_id() {
        "js_playground" => {
            let _docs_window = tauri::WindowBuilder::new(
                &event.window().app_handle(),
                "docs_window", /* the unique window label */
                tauri::WindowUrl::App(PathBuf::from("/#/jsruntime")),
            )
            .build();
        }
        "baidu" => {
            let _baidu_window = tauri::WindowBuilder::new(
                &event.window().app_handle(),
                "baidu_window", /* the unique window label */
                tauri::WindowUrl::External("https://www.baidu.com/".parse().unwrap()),
            )
            .build();
        }
        _ => {}
    }
}

fn window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            // 组织 window 的默认关闭事件
            api.prevent_close();
            let window = event.window().clone();
            tauri::api::dialog::confirm(
                Some(&event.window().clone()),
                "close window",
                "Are you sure you want to close this window?",
                move |answer| {
                    if answer {
                        _ = window.close();
                    }
                },
            );
        }
        _ => {
            println!("other event");
        }
    }
}
