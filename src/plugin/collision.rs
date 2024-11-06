use bevy::{ecs::entity, prelude::*};
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

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
                let attack_entity =
                    match find_eneity(&attack, vec![entity1.clone(), entity2.clone()]) {
                        Some(r) => r,
                        None => {
                            println!("attach none entity");
                            return;
                        }
                    };

                let mut health_entitys =
                    match health.get_many_mut([entity1.clone(), entity2.clone()]) {
                        Ok(r) => r,
                        Err(e) => {
                            println!("health1 none entity {:?}", e);
                            return;
                        }
                    };

                let health_entity = match health_entitys.first_mut() {
                    Some(r) => r,
                    None => {
                        println!("health2 none entity");
                        return;
                    }
                };

                health_entity.damage(attack_entity.attack());
            }
            _ => {}
        }
    }
}

fn find_eneity<'a, T>(attack: &'a Query<&T>, entity: Vec<Entity>) -> Option<&'a T>
where
    T: Component,
{
    for entity in entity {
        if let Ok(r) = attack.get(entity) {
            return Some(r);
        }
    }
    None
}

// fn find_entity_mut<'a, T, const N: usize>(
//     mut attack: &'a mut Query<&mut T>,
//     entity: [Entity; N],
// ) -> Option<&'a mut T>
// where
//     T: Component,
// {
//     if let Ok(r) = attack.get_many_mut(entity) {
//         if let Some(r) = r.first() {
//             return Some(r.);
//         }
//     }
//     None
// }
