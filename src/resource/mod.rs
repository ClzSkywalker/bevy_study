use bevy::prelude::*;

// 鼠标点击位置资源
#[derive(Resource)]
pub struct MouseClickRes {
    pub pos: Vec2,
    pub mouse_button: MouseButton,
}

impl MouseClickRes {
    pub fn new(pos: Vec2, mouse_button: MouseButton) -> Self {
        Self { pos, mouse_button }
    }
}

// 鼠标位置资源
#[derive(Resource, Default)]
pub struct MousePositionRes {
    pub pos: Vec2,
}

impl MousePositionRes {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
