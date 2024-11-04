use bevy::prelude::*;

use crate::comp::movement::{Acceleration, Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position).chain());
    }
}

fn update_velocity(mut movecomp: Query<(&mut Velocity, &Acceleration)>) {
    for mut movecomp in movecomp.iter_mut() {
        movecomp.0.value += movecomp.1.value;
    }
}

fn update_position(mut movecomp: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for mut movecomp in movecomp.iter_mut() {
        movecomp.0.translation += movecomp.1.value.extend(0.) * time.delta_seconds();
    }
}
