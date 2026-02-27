/*!
    机器人:
    ```
    点击 world
       ↓
    world → grid （做合法性检测）
       ↓
    grid → world （得到格子中心）
       ↓
    set_target( world )
       ↓
    Rust update 用 world 计算
       ↓
    JS 直接渲染
    ```
*/

use crate::module::a::astar;
use crate::module::grid::{Grid, GridPoint, GridResultPoint};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum RobotAction {
    Idle,
    Walking,
    Running,
    Dance,
    Death,
    Sitting,
    Standing,
}

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub enum RobotEmote {
    None,
    Jump,
    Yes,
    No,
    Wave,
    Punch,
    ThumbsUp,
}

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Robot {
    current: Vec3,
    target: Vec3,
    is_moving: bool,
    speed: f32,
    rotation_y: f32,
    action: RobotAction,
    emote: RobotEmote,
    path: Vec<Vec3>,
    path_index: usize,
}

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub struct RobotState {
    pub position: Vec3,
    #[serde(rename = "isMoving")]
    pub is_moving: bool,
    #[serde(rename = "rotationY")]
    pub rotation_y: f32, // 朝向
}

impl Robot {
    pub fn new(start_x: f32, start_z: f32, speed: f32) -> Self {
        info!("Robot created!");
        Self {
            current: Vec3 { x: start_x, y: 0f32, z: start_z },
            target: Vec3 { x: start_x, y: 0f32, z: start_z },
            is_moving: false,
            rotation_y: 0f32,
            action: RobotAction::Idle,
            emote: RobotEmote::None,
            path: vec![],
            speed,
            path_index: 0,
        }
    }

    // 设置目标格子
    pub fn set_target(&mut self, grid: &Grid, x: f32, z: f32) -> Vec<Vec3> {
        let start = self.current;
        let goal = Vec3 { x, y: 0.0, z };
        if let Some(path) = astar(grid, start, goal) {
            self.path = path.into_iter().map(|p| Vec3 { x: p.x, y: 0.0, z: p.z }).collect();

            self.path_index = 0;

            if !self.path.is_empty() {
                self.target = self.path[0];
                self.is_moving = true;
            }
        } else {
            info!("A* 无法到达目标");
        }

        // self.target = Vec3 { x, z, y: 0f32 };
        // self.is_moving = true;

        self.path.clone()
    }

    // 清除路径
    pub fn clear_path(&mut self) {
        self.path.clear();
        self.path_index = 0;
        self.is_moving = false;

        // 让目标回到当前位置
        // self.target = self.current;
    }

    /*
     更新
     Three.js 默认：
       - Y 是向上
       - 角色朝向通常绕 Y 轴旋转
       - rotation.y 是朝向角

     ⚠ 注意顺序是 atan2(x, z) 还是 atan2(z, x) 取决于坐标系
    */
    pub fn update(&mut self, delta: f32) {
        if !self.is_moving {
            return;
        }

        let dx = self.target.x - self.current.x;
        let dz = self.target.z - self.current.z;

        // 计算向量长度, 公式: √(x² + y² + z²)
        let distance = (dx * dx + dz * dz).sqrt();

        // 本帧最大可移动距离
        let max_step = self.speed * delta;
        if distance <= max_step {
            // self.current = self.target;
            // self.is_moving = false;
            self.current = self.target;

            self.path_index += 1;

            if self.path_index >= self.path.len() {
                self.is_moving = false;
                return;
            }

            self.target = self.path[self.path_index];
            info!("到达目标，停止移动 ...");
            return;
        }

        let dir_x = dx / distance;
        let dir_z = dz / distance;

        let yaw = dir_x.atan2(dir_z);
        self.rotation_y = yaw;

        self.current.x += dir_x * max_step;
        self.current.z += dir_z * max_step;

        println!("current: {:?}", self.current);
    }

    // 设置动作
    pub fn set_action(&mut self, action: String) {
        self.action = match action.as_str() {
            "walking" => RobotAction::Walking,
            "running" => RobotAction::Running,
            "dance" => RobotAction::Dance,
            "death" => RobotAction::Death,
            "idle" => RobotAction::Idle,
            "sitting" => RobotAction::Sitting,
            "standing" => RobotAction::Standing,
            _ => RobotAction::Idle,
        };

        // running
        if self.action == RobotAction::Running {
            self.set_speed(5.0)
        }

        // walking
        if self.action == RobotAction::Walking {
            self.set_speed(2.5)
        }
    }

    // 设置表情
    pub fn set_emote(&mut self, emote: String) {
        self.emote = match emote.as_str() {
            "jump" => RobotEmote::Jump,
            "yes" => RobotEmote::Yes,
            "no" => RobotEmote::No,
            "wave" => RobotEmote::Wave,
            "punch" => RobotEmote::Punch,
            "thumbsup" => RobotEmote::ThumbsUp,
            _ => RobotEmote::None,
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_current(&self) -> Vec3 {
        self.current
    }

    pub fn get_point(&self) -> GridResultPoint {
        Grid::grid_to_world(&GridPoint {
            gx: self.current.x as i32,
            gz: self.current.z as i32,
        })
    }

    pub fn get_target(&self) -> Vec3 {
        self.target
    }

    pub fn get_moving(&self) -> bool {
        self.is_moving
    }

    pub fn get_rotation_y(&self) -> f32 {
        self.rotation_y
    }
}
