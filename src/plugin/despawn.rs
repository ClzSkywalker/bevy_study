use bevy::prelude::*;

use crate::comp::prelude::*;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_item);
    }
}

fn despawn_item(mut command: Commands, query: Query<(Entity, &CountdownTimer<DeadTimer>)>) {
    for comp in query.iter() {
        if comp.1.is_finished() {
            command.entity(comp.0).despawn();
        }
    }
}
