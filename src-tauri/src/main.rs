// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod exports;
mod prepare;
mod system;

mod module;

use std::sync::Mutex;

use crate::module::grid::Grid;
use crate::module::robot::Robot;
use crate::system::tray::Tray;
use exports::{generate_pillars, generate_rocks, get_init_props, get_robot_point, grid_to_world, on_update_robot_position, set_place_flag, set_robot_action, set_robot_emote, set_robot_target, world_to_grid};

const PROJECT_NAME: &str = "n-3d";

pub const WIDTH: f32 = 200f32;
pub const HEIGHT: f32 = 200f32;

pub const CHARACTER_OCCUPY_WIDTH: f32 = 2f32;
pub const CHARACTER_OCCUPY_HEIGHT: f32 = 2f32;

// 柱子占用 2 * 2 格
pub const PILLAR_SIZE: usize = 2;

pub const SPEED: f32 = 2.0f32;

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
        .manage(Mutex::new(Robot::new(0.0, 0.0, SPEED))) // 初始在中心
        .manage(Mutex::new(Grid::new(WIDTH as usize, HEIGHT as usize))) // 初始化 Grid
        .invoke_handler(tauri::generate_handler![
            world_to_grid,
            grid_to_world,
            set_robot_target,
            on_update_robot_position,
            set_robot_action,
            set_robot_emote,
            get_robot_point,
            get_init_props,
            set_place_flag,
            generate_rocks,
            generate_pillars
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
