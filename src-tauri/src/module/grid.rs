/*!
机器人占用格子
*/

use serde::{Deserialize, Serialize};

struct GridCell {
    walkable: bool, // 是否可以走
}

/*
 width / height → 格子数量(如果场景 200×200 单位，如果 1 格 = 1 单位 → 200×200)
 walkable → false 表示墙或障碍物，true 表示可走
*/
pub struct GridMap {
    width: usize,
    height: usize,
    cells: Vec<Vec<GridCell>>, // cells[y][x]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    gx: usize,
    gz: usize,
    character_occupy_width: f32,
    character_occupy_height: f32,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGrid {
    x: f32,
    z: f32,
    character_occupy_width: f32,
    character_occupy_height: f32,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGridResultPoint {
    x: usize,
    z: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridResultPoint {
    x: f32,
    z: f32,
}

impl Grid {
    /// 限制 value 在 min_val 和 max_val 之间
    fn clamp(value: f32, min_val: f32, max_val: f32) -> f32 {
        if value < min_val {
            min_val
        } else if value > max_val {
            max_val
        } else {
            value
        }
    }

    /*
      Three.js 坐标系 → Rust 格子坐标
      - Three.js X/Z: [-100, 100] → Rust 坐标: 0..width / 0..height

    */
    pub fn world_to_grid(grid: &ThreeGrid) -> ThreeGridResultPoint {
        let half_w = ((grid.width as f32) / grid.character_occupy_width) as f32;
        let half_h = ((grid.height as f32) / grid.character_occupy_height) as f32;

        let gx_f = (grid.x + half_w).floor() as f32;
        let gz_f = (grid.z + half_h).floor() as f32;

        let gx = Self::clamp(gx_f, 0.0, grid.width as f32 - 1.0) as usize;
        let gz = Self::clamp(gz_f, 0.0, grid.height as f32 - 1.0) as usize;

        ThreeGridResultPoint { x: gx, z: gz }
    }

    /*
      Rust 格子坐标 → Three.js 坐标系
      - Rust 坐标: 0..width / 0..height → Three.js X/Z: [-100, 100]
    */
    pub fn grid_to_world(grid: &Grid) -> GridResultPoint {
        let half_w = (grid.width as f32) / grid.character_occupy_width;
        let half_h = (grid.height as f32) / grid.character_occupy_height;

        let x = grid.gx as f32 - half_w + 0.5; // 0.5 为格子中心偏移
        let z = grid.gz as f32 - half_h + 0.5;

        GridResultPoint { x, z }
    }
}
