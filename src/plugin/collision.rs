use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::comp::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(PostUpdate, collision);
    }
}

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    attack: Query<&AttackComponent>,
    mut health: Query<&mut HealthComponent>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let attack_entity = match attack.get(entity1.clone()) {
                    Ok(r) => r,
                    Err(_) => {
                        return;
                    }
                };
                let mut health_entity = match health.get_mut(entity2.clone()) {
                    Ok(r) => r,
                    Err(_) => {
                        return;
                    }
                };
                health_entity.damage(attack_entity.attack());
            }
            _ => {}
        }
    }
}
