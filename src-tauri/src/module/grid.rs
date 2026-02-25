/*!
机器人占用格子
*/

use crate::{CHARACTER_OCCUPY_HEIGHT, CHARACTER_OCCUPY_WIDTH, HEIGHT, WIDTH};
use serde::{Deserialize, Serialize};

struct GridCell {
    pub occupied: bool, // 机器人占用
    pub has_flag: bool, // 是否有红旗
    pub blocked: bool,  // 是否是障碍物（墙等）
}

pub struct Grid {
    width: f32,
    height: f32,
    cells: Vec<GridCell>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridPoint {
    pub gx: f32,
    pub gz: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGrid {
    x: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeGridResultPoint {
    x: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridResultPoint {
    x: f32,
    z: f32,
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
    pub fn new(width: f32, height: f32) -> Self {
        let total = (width * height) as usize;
        let mut cells = Vec::with_capacity(total);
        for _ in 0..total {
            cells.push(GridCell {
                occupied: false,
                has_flag: false,
                blocked: false,
            });
        }

        Self { width, height, cells }
    }

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
        let center_x = (WIDTH / 2.0).floor() as f32;
        let center_z = (HEIGHT / 2.0).floor() as f32;

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
    fn point_to_cell(&self, x: f32, z:f32) -> Option<GridPoint> {
        let gx = ((x + self.width / 2.0) / self.width * self.width).floor() as f32;
        let gz = ((z + self.height / 2.0) / self.height * self.height).floor() as f32;

        // 越界检查
        if gx < 0.0 || gx >= self.width|| gz < 0.0 || gz >= self.height {
            return None;
        }

        Some(GridPoint {
            gx,
            gz
        })
    }

    // 添加红旗
    pub fn place_flag(&mut self, x: f32, z: f32) -> bool {
        if let Some(point) = self.point_to_cell(x, z) {
            // 清除旧红旗
            self.clear_flag();

            let index = self.index(point.gx, point.gz);
            let cell = &mut self.cells[index as usize];
            if cell.has_flag {
                return false;
            }

            cell.has_flag = true;
            true
        } else {
            false
        }
    }

    // 清除红旗
    pub fn clear_flag(&mut self) {
        for cell in &mut self.cells {
            cell.has_flag = false;
        }
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
    fn index(&self, x: f32, z: f32) -> f32 {
        z * self.width + x
    }

    pub fn get_cell(&self, x: f32, z: f32) -> &GridCell {
        let i = self.index(x, z);
        &self.cells[i as usize]
    }

    pub fn get_cell_mut(&mut self, x: f32, z: f32) -> &mut GridCell {
        let i = self.index(x, z);
        &mut self.cells[i as usize]
    }
}
