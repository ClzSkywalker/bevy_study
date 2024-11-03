use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    // pub collider: Collider,
    // pub collider: SphereCollider,
}

impl MovementBundle {
    pub fn new(velocity: Velocity, acceleration: Acceleration) -> Self {
        Self {
            velocity: velocity,
            acceleration: acceleration,
        }
    }
}
