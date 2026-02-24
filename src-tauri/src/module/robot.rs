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
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Clone, Copy, Debug, Deserialize)]
pub struct Robot {
    current: Vec3,
    target: Vec3,
    is_moving: bool,
    speed: f32,
    rotation_y: f32,
    action: RobotAction,
    emote: RobotEmote,
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
            target: Vec3 { x: start_x, z: start_z, y: 0f32 },
            is_moving: false,
            rotation_y: 0f32,
            action: RobotAction::Idle,
            emote: RobotEmote::None,
            speed,
        }
    }

    // 设置目标格子
    pub fn set_target(&mut self, x: f32, z: f32) {
        self.target = Vec3 { x, z, y: 0f32 };
        self.is_moving = true;
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
            self.current = self.target;
            self.is_moving = false;
            info!("到达目标，停止移动 ...");
            return;
        }

        let dir_x = dx / distance;
        let dir_z = dz / distance;

        let yaw = dir_x.atan2(dir_z);
        self.rotation_y = yaw;

        self.current.x += dir_x * max_step;
        self.current.z += dir_z * max_step;

        println!("current: {:#?}", self.current);
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
