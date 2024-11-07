use bevy::prelude::*;

use crate::comp::prelude::*;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, despawn_item);
    }
}

fn despawn_item(
    mut command: Commands,
    dead_timer: Query<(Entity, &CountdownTimer<DeadTimer>)>,
    health_check: Query<(Entity, &HealthComponent)>,
) {
    for comp in dead_timer.iter() {
        if comp.1.is_finished() {
            command.entity(comp.0).despawn();
        }
    }

    for (entity, comp) in health_check.iter() {
        if comp.is_dead() {
            println!("dead,{:?}", entity);
            command.entity(entity).despawn();
        }
    }
}
