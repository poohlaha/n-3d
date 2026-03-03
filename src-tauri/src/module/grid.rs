/*!
机器人占用格子
*/

use crate::{CHARACTER_OCCUPY_HEIGHT, CHARACTER_OCCUPY_WIDTH, HEIGHT, WIDTH};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct GridCell {
    pub occupied: bool,       // 机器人占用
    pub has_flag: bool,       // 是否有红旗
    pub blocked: bool,        // 是否是障碍物（墙等）
    pub blocked_type: String, // 障碍物类型, 'pillar'
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<GridCell>,
    obstacles: Vec<Obstacle>,
}

// 障碍物
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Obstacle {
    pub x: f32,       // 柱子在 grid 中左上角格子坐标 X
    pub z: f32,       // 柱子在 grid 中左上角格子坐标 Z
    pub width: usize, // 占用格子尺寸宽度
    pub depth: usize, // 占用格子尺寸高度
    pub kind: ObstacleType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ObstacleType {
    Pillar,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPoint {
    pub gx: i32,
    pub gz: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGrid {
    pub x: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGridResultPoint {
    x: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridResultPoint {
    x: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridProps {
    width: usize,
    height: usize,
    #[serde(rename = "characterOccupyWidth")]
    character_occupy_width: usize,
    #[serde(rename = "characterOccupyHeight")]
    character_occupy_height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let total = width * height;
        let mut cells = Vec::with_capacity(total);
        for _ in 0..total {
            cells.push(GridCell {
                occupied: false,
                has_flag: false,
                blocked: false,
                blocked_type: String::new(),
            });
        }

        Self { width, height, cells, obstacles: Vec::new() }
    }

    /**
    设:
    ```
     width = 4
     height = 3
    ```

    格子长这样 `(x, z)`:
    ```
     (0, 0) (1, 0) (2, 0) (3, 0)
     (0, 1) (1, 1) (2, 1) (3, 1)
     (0, 2) (1, 2) (2, 2) (3, 2)
    ```

    转成二维数组:
    ```
     (x, z):
     - x: 水平方向的坐标（列），从左到右 0 → width-1
     - z: (写作 z，其实也可以理解成 y 或 row)：竖直方向的坐标(行)，从上到下 0 → height-1

     index 0  -> (0, 0)
     index 1  -> (1, 0)
     index 2  -> (2, 0)
     index 3  -> (3, 0)

     index 4  -> (0, 1)
     index 5  -> (1, 1)
     index 6  -> (2, 1)
     index 7  -> (3, 1)

     index 8  -> (0, 2)
     index 9  -> (1, 2)
     index 10 -> (2, 2)
     index 11 -> (3, 2)
    ```

     发现: `每一行有 width 个元素`
     所以: `第 z 行的起始 index = z * width`, 再加上 `x`
    */
    fn index(&self, x: usize, z: usize) -> usize {
        z * self.width + x
    }

    pub fn get_cell(&self, x: usize, z: usize) -> &GridCell {
        let i = self.index(x, z);
        &self.cells[i]
    }

    pub fn get_cell_mut(&mut self, x: usize, z: usize) -> &mut GridCell {
        let i = self.index(x, z);
        &mut self.cells[i]
    }

    // 限制 value 在 min_val 和 max_val 之间
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
      - Three.js X/Z: [-100, 100] → Rust 坐标: [-100, 100]
    */
    pub fn world_to_grid(grid: &ThreeGrid) -> ThreeGridResultPoint {
        let gx = grid.x.round();
        let gz = grid.z.round();

        ThreeGridResultPoint { x: gx, z: gz }
    }

    /*
      Rust 格子坐标 → Three.js 坐标系
      - Rust 坐标: 0..width / 0..height → Three.js X/Z: [-100, 100]
    */
    pub fn grid_to_world(grid: &GridPoint) -> GridResultPoint {
        // 转换坐标，并将中心平移
        let x = grid.gx;
        let z = grid.gz;

        GridResultPoint { x, z }
    }

    // 获取初始化坐标
    pub fn get_init_point() -> (f32, f32) {
        let center_x = (WIDTH / 2.0).floor();
        let center_z = (HEIGHT / 2.0).floor();

        (center_x, center_z)
    }

    pub fn get_init_props() -> GridProps {
        GridProps {
            width: WIDTH as usize,
            height: HEIGHT as usize,
            character_occupy_width: CHARACTER_OCCUPY_WIDTH as usize,
            character_occupy_height: CHARACTER_OCCUPY_HEIGHT as usize,
        }
    }

    // 映射 -100~100 → 0..width-1/0..height-1
    pub fn point_to_cell(&self, x: f32, z: f32) -> Option<GridPoint> {
        let gx = (x + self.width as f32 / 2.0).floor() as i32;
        let gz = (z + self.height as f32 / 2.0).floor() as i32;

        // 越界检查
        if gx < 0 || gz < 0 {
            return None;
        }

        if gx >= self.width as i32 || gz >= self.height as i32 {
            return None;
        }

        Some(GridPoint { gx, gz })
    }

    // 映射 0..width-1/0..height-1 → -100~100
    // 映射到世界坐标 -width/2..width/2, -height/2..height/2
    pub fn cell_to_point(&self, gx: f32, gz: f32) -> ThreeGrid {
        let x = gx - self.width as f32 / 2.0;
        let z = gz - self.height as f32 / 2.0;

        ThreeGrid { x, z }
    }

    // 添加红旗
    pub fn place_flag(&mut self, x: f32, z: f32) -> bool {
        if let Some(point) = self.point_to_cell(x, z) {
            // 清除旧红旗
            self.clear_flag();

            let cell = &mut self.get_cell_mut(point.gx as usize, point.gz as usize);
            if cell.has_flag || cell.blocked || cell.occupied {
                return false;
            }

            cell.has_flag = true;
            return true;
        }

        false
    }

    // 清除红旗
    pub fn clear_flag(&mut self) {
        for cell in &mut self.cells {
            cell.has_flag = false;
        }
    }

    // 清除石头|柱子
    pub fn clear_obstacle(&mut self, kind: ObstacleType) {
        // 删除 obstacles 里对应类型
        self.obstacles.retain(|o| o.kind != kind);

        let kind_str = match kind {
            ObstacleType::Pillar => "pillar",
            ObstacleType::Rock => "rock",
        };

        // 清理格子被柱子占用的标记
        for cell in &mut self.cells {
            if cell.blocked_type == kind_str {
                cell.blocked = false; // 柱子占用的格子恢复空闲
                cell.blocked_type.clear();
            }
        }
    }

    // 清除所有障碍物
    pub fn clear_obstacles(&mut self) {
        self.obstacles.clear();

        for cell in &mut self.cells {
            if !cell.blocked_type.is_empty() {
                cell.blocked = false;
                cell.blocked_type.clear();
            }
        }
    }

    // 生成障碍物
    pub fn generate_obstacle(&mut self, nums: usize, width: usize, depth: usize, kind: ObstacleType) -> Vec<Obstacle> {
        let mut rng = rand::rng();
        self.clear_obstacle(kind);

        for _ in 0..nums {
            let mut placed = false;

            while !placed {
                // 随机选择柱子左上角格子
                let x = rng.random_range(0..self.width - width);
                let z = rng.random_range(0..self.height - depth);

                // 检查 2 * 2 区域是否空闲
                let mut can_place = true;
                for dx in 0..width {
                    for dz in 0..depth {
                        let cell = self.get_cell_mut(x + dx, z + dz);
                        if cell.blocked || cell.occupied || cell.has_flag {
                            can_place = false;
                            break;
                        }
                    }
                }

                if can_place {
                    // 标记柱子占用的格子
                    for dx in 0..width {
                        for dz in 0..depth {
                            let cell = self.get_cell_mut(x + dx, z + dz);
                            cell.blocked = true;
                            cell.blocked_type = match kind {
                                ObstacleType::Pillar => "pillar".to_string(),
                                ObstacleType::Rock => "rock".to_string(),
                            };
                        }
                    }

                    let center_gx = x as f32 + width as f32 / 2.0;
                    let center_gz = z as f32 + depth as f32 / 2.0;

                    let point = self.cell_to_point(center_gx, center_gz);

                    self.obstacles.push(Obstacle { x: point.x, z: point.z, width, depth, kind });

                    placed = true;
                }
            }
        }

        self.obstacles.clone()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
