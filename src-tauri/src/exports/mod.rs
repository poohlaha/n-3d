//! 导出方法

use crate::module::grid::{Grid, GridResultPoint, ThreeGrid, ThreeGridResultPoint};
use crate::module::robot::{Robot, RobotState};
use std::sync::Mutex;
use tauri::State;

// Three.js 坐标系 → Rust 格子坐标
#[tauri::command]
pub fn world_to_grid(grid: ThreeGrid) -> ThreeGridResultPoint {
    Grid::world_to_grid(&grid)
}

// Rust 格子坐标 → Three.js 坐标系
#[tauri::command]
pub fn grid_to_world(grid: Grid) -> GridResultPoint {
    Grid::grid_to_world(&grid)
}

#[tauri::command]
pub fn on_update_robot_position(delta: f32, robot: State<Mutex<Robot>>) -> RobotState {
    let mut robot = robot.lock().unwrap();
    robot.update(delta);
    RobotState {
        position: robot.get_current(),
        is_moving: robot.get_moving(),
        rotation_y: robot.get_rotation_y(),
    }
}

#[tauri::command]
pub fn set_robot_target(x: f32, z: f32, robot: State<Mutex<Robot>>) -> Result<(), String> {
    let mut robot = robot.lock().map_err(|_| "Mutex poisoned")?;
    robot.set_target(x, z);

    println!("Robot target updated to: ({}, {})", x, z);
    Ok(())
}

#[tauri::command]
pub fn set_robot_action(action: String, robot: State<Mutex<Robot>>) {
    let mut robot = robot.lock().unwrap();
    robot.set_action(action);
}

#[tauri::command]
pub fn set_robot_emote(emote: String, robot: State<Mutex<Robot>>) {
    let mut robot = robot.lock().unwrap();
    robot.set_emote(emote);
}
