// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, MenuItem, PhysicalSize, Size, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let home = CustomMenuItem::new("home".to_string(), "首页");
    let bookshelf = CustomMenuItem::new("bookshelf".to_string(), "我的书架");
    let refresh = CustomMenuItem::new("refresh".to_string(), "刷新");
    // 大小调整菜单
    let smallsize = CustomMenuItem::new("smallsize".to_string(), "较小");
    let normalsize = CustomMenuItem::new("normalsize".to_string(), "正常");
    let resizemenu = Submenu::new(
        "调整窗口",
        Menu::new().add_item(smallsize).add_item(normalsize),
    );
    let _optmenu = Submenu::new(
        "操作",
        Menu::new()
            .add_item(refresh)
            .add_submenu(resizemenu)
            .add_native_item(MenuItem::Separator)
            .add_item(home)
            .add_item(bookshelf),
    );
    let about = CustomMenuItem::new("about".to_string(), "关于");
    let restmenu = Submenu::new("其他", Menu::new().add_item(about));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(_optmenu)
        .add_submenu(restmenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "refresh" => {
                let _ = event.window().eval("window.location.reload()");
            }
            "smallsize" => event
                .window()
                .set_size(Size::Physical(PhysicalSize {
                    width: 450,
                    height: 700,
                }))
                .unwrap(),
            "normalsize" => event
                .window()
                .set_size(Size::Physical(PhysicalSize {
                    width: 800,
                    height: 800,
                }))
                .unwrap(),
            "home" => {
                let _ = event
                    .window()
                    .eval("window.location.href='https://weread.qq.com'");
            }
            "bookshelf" => {
                let _ = event
                    .window()
                    .eval("window.location.href='https://weread.qq.com/web/shelf'");
            }
            "about" => {}
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
