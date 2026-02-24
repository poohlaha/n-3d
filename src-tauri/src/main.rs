// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod exports;
mod prepare;
mod system;

mod module;

use std::sync::Mutex;

use crate::module::robot::Robot;
use crate::system::tray::Tray;
use exports::{grid_to_world, on_update_robot_position, set_robot_action, set_robot_emote, set_robot_target, world_to_grid};

const PROJECT_NAME: &str = "n-3d";

// 日志目录: /Users/xxx/Library/Logs/n-3d
// 程序配置目录: /Users/xxx/Library/Application Support/n-3d

fn main() {
    // tauri
    tauri::Builder::default()
        // .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(move |app| {
            // 创建系统托盘
            Tray::builder(app);

            let app_handle = app.handle();

            Ok(())
        })
        .manage(Mutex::new(Robot::new(0.0, 0.0, 2.0))) // 初始在中心
        .invoke_handler(tauri::generate_handler![world_to_grid, grid_to_world, set_robot_target, on_update_robot_position, set_robot_action, set_robot_emote])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
