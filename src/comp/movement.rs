use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec2,
}

impl Acceleration {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

impl MovementBundle {
    pub fn new(velocity: Velocity, acceleration: Acceleration) -> Self {
        Self {
            velocity,
            acceleration,
        }
    }
}
