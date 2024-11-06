use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct EnemyComponent;

#[derive(Component, Debug)]
pub struct HealthComponent(pub u64);

impl HealthComponent {
    pub fn new(health: u64) -> Self {
        Self(health)
    }

    pub fn damage(&mut self, damage: u64) {
        if self.0 < damage {
            self.0 = 0;
            return;
        }
        self.0 -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Component, Debug)]
pub struct AttackComponent(u64);

impl AttackComponent {
    pub fn new(attack: u64) -> Self {
        Self(attack)
    }

    pub fn attack(&self) -> u64 {
        self.0
    }
}
