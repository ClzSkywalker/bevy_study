use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

fn collision(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.read() {
        println!("Collision event: {:?}", event);
    }
}