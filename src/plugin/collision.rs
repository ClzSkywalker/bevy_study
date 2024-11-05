use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostUpdate, collision);
    }
}

fn collision(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.read() {
        println!("Collision event: {:?}", event);
    }
}

        // match event {
        //     CollisionEvent::Started(entity1, entity2, _) => {
        //         println!("{:?} and {:?} started colliding", entity1, entity2);
        //     }
        //     CollisionEvent::Stopped(entity1, entity2, _) => {
        //         println!("{:?} and {:?} stopped colliding", entity1, entity2);
        //     }
        // }