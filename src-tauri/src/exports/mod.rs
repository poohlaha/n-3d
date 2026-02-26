//! 导出方法

use crate::module::grid::{Grid, GridPoint, GridProps, GridResultPoint, Pillar, RockData, ThreeGrid, ThreeGridResultPoint};
use crate::module::robot::{Robot, RobotState};
use std::sync::Mutex;
use tauri::State;

// 获取初始化属性
#[tauri::command]
pub fn get_init_props() -> GridProps {
    Grid::get_init_props()
}

// Three.js 坐标系 → Rust 格子坐标
#[tauri::command]
pub fn world_to_grid(grid: ThreeGrid) -> ThreeGridResultPoint {
    Grid::world_to_grid(&grid)
}

// Rust 格子坐标 → Three.js 坐标系
#[tauri::command]
pub fn grid_to_world(grid: GridPoint) -> GridResultPoint {
    Grid::grid_to_world(&grid)
}

// 获取 robot 坐标
#[tauri::command]
pub fn get_robot_point(robot: State<Mutex<Robot>>) -> GridResultPoint {
    let robot = robot.lock().unwrap();
    robot.get_point()
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
    let mut robot = robot.lock().map_err(|_| "Mutex robot poisoned")?;
    robot.set_target(x, z);

    println!("Robot target updated to: ({}, {})", x, z);
    Ok(())
}

// 设置动作
#[tauri::command]
pub fn set_robot_action(action: String, robot: State<Mutex<Robot>>) {
    let mut robot = robot.lock().unwrap();
    robot.set_action(action);
}

// 设置表情
#[tauri::command]
pub fn set_robot_emote(emote: String, robot: State<Mutex<Robot>>) {
    let mut robot = robot.lock().unwrap();
    robot.set_emote(emote);
}

// 放置小红旗
#[tauri::command]
pub fn set_place_flag(x: f32, z: f32, grid: State<Mutex<Grid>>) -> Result<bool, String> {
    let mut grid = grid.lock().map_err(|_| "Mutex grid poisoned")?;
    Ok(grid.place_flag(x, z))
}

// 随机生成石头
#[tauri::command]
pub fn generate_rocks(num_rocks: usize, grid: State<Mutex<Grid>>) -> Result<Vec<RockData>, String> {
    let mut grid = grid.lock().map_err(|_| "Mutex grid poisoned")?;
    Ok(grid.generate_rocks(num_rocks))
}

// 随机生成柱子
#[tauri::command]
pub fn generate_pillars(nums: usize, grid: State<Mutex<Grid>>) -> Result<Vec<Pillar>, String> {
    let mut grid = grid.lock().map_err(|_| "Mutex grid poisoned")?;
    Ok(grid.generate_pillars(nums))
}
