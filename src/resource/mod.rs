use bevy::prelude::*;

#[derive(Resource)]
pub struct MouseClickRes{
    pub pos:Vec2,
    pub mouse_button:MouseButton,
}

impl MouseClickRes {
    pub fn new(pos: Vec2,mouse_button: MouseButton) -> Self {
        Self {
            pos,
            mouse_button,
        }
    }
}