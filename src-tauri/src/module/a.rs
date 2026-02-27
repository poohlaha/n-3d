/*!
  A* 算法, 查找最短路径
*/

use crate::module::grid::{Grid, GridPoint, ThreeGrid};
use crate::module::robot::Vec3;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
struct Node {
    point: GridPoint,
    g: f64, // 从起点到当前点的真实代价
    h: f64, // 启发式估计代价
    f: f64, // 总代价 f = g + h
}

/// 让 BinaryHeap 变成“最小堆”
/// 默认是最大堆
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

/// 欧几里得距离启发函数
fn heuristic(a: &GridPoint, b: &GridPoint) -> f64 {
    let dx = (a.gx - b.gx) as f64;
    let dz = (a.gz - b.gz) as f64;
    (dx * dx + dz * dz).sqrt()
}

/// 获取 8 方向邻居(含障碍检测 + 防止对角穿墙)
fn get_neighbors(grid: &Grid, p: GridPoint) -> Vec<(GridPoint, f64)> {
    // (dx, dy, move_cost), 上下左右为 1 格, 对角线为 2 格
    // 对角线: √(1² + 1²) = √2
    let directions = vec![
        (1, 0, 1.0),           // 右
        (-1, 0, 1.0),          // 左
        (0, 1, 1.0),           // 下
        (0, -1, 1.0),          // 上
        (1, 1, 2f64.sqrt()),   // 右下
        (-1, -1, 2f64.sqrt()), // 左上
        (1, -1, 2f64.sqrt()),  // 右上
        (-1, 1, 2f64.sqrt()),  // 左下
    ];

    let mut neighbors = Vec::new();
    for (dx, dz, cost) in directions {
        let nx = p.gx + dx;
        let nz = p.gz + dz;

        // 边界检查
        if nx < 0 || nz < 0 {
            continue;
        }

        if nx >= grid.width() as i32 || nz >= grid.height() as i32 {
            continue;
        }

        // 目标格子是否 blocked
        let cell = grid.get_cell(nx as usize, nz as usize);
        if cell.blocked {
            continue;
        }

        // 对角线防止穿墙
        if dx != 0 && dz != 0 {
            let cell1 = grid.get_cell((p.gx + dx) as usize, p.gz as usize);
            let cell2 = grid.get_cell(p.gx as usize, (p.gz + dz) as usize);

            if cell1.blocked || cell2.blocked {
                continue;
            }
        }

        neighbors.push((GridPoint { gx: nx, gz: nz }, cost));
    }

    neighbors
}

/// A* 主函数
pub fn astar(grid: &Grid, start_world: Vec3, goal_world: Vec3) -> Option<Vec<ThreeGrid>> {
    // 世界坐标 → 格子
    let start = grid.point_to_cell(start_world.x, start_world.z);
    let goal = grid.point_to_cell(goal_world.x, goal_world.z);

    if start.is_none() {
        return None;
    }

    if goal.is_none() {
        return None;
    }

    let start = start.unwrap();
    let goal = goal.unwrap();

    // 如果终点是障碍，直接返回 None
    if grid.get_cell(goal.gx as usize, goal.gz as usize).blocked {
        return None;
    }

    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<GridPoint, GridPoint> = HashMap::new();
    let mut g_score: HashMap<GridPoint, f64> = HashMap::new();
    let mut closed: HashSet<GridPoint> = HashSet::new();

    let h = heuristic(&start, &goal);

    g_score.insert(start.clone(), 0.0f64);

    open.push(Node { point: start.clone(), g: 0.0, h, f: h });

    while let Some(current) = open.pop() {
        // 过滤已经处理过的节点
        if closed.contains(&current.point) {
            continue;
        }

        // 如果到达终点
        if current.point == goal {
            // 回溯路径
            let mut grid_path = vec![goal];
            let mut p = goal;

            while let Some(prev) = came_from.get(&p) {
                grid_path.push(*prev);
                p = *prev;
            }

            grid_path.reverse();
            // 转换为世界坐标路径
            let mut world_path = Vec::new();

            for cell in grid_path {
                let world = grid.cell_to_point(cell.gx as usize, cell.gz as usize);
                world_path.push(world);
            }

            return Some(world_path);
        }

        // 加入到 closed
        closed.insert(current.point);

        // 当前最优 g
        let current_g = *g_score.get(&current.point).unwrap();

        // 扩展邻居
        for (neighbor, move_cost) in get_neighbors(grid, current.point) {
            // 判断是否在 closed 中
            if closed.contains(&neighbor) {
                continue;
            }

            let tentative_g = current_g + move_cost;

            // 查找 g_score 中是否存在节点
            let best_g = g_score.get(&neighbor).cloned().unwrap_or(f64::INFINITY);

            // 小于则更新
            if tentative_g < best_g {
                came_from.insert(neighbor, current.point);
                g_score.insert(neighbor, tentative_g);

                let h = heuristic(&neighbor, &goal);
                let f = tentative_g + h;

                open.push(Node { point: neighbor, g: tentative_g, h, f });
            }
        }
    }

    None
}
