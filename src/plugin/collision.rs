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
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let attack_entitys = find_eneity(&attack, [*entity1, *entity2]);
            if attack_entitys.is_empty() {
                println!("attach none entity");
                return;
            }
            for (source, attack_entity, target) in attack_entitys {
                if let Ok(mut health_entity) = health.get_mut(target) {
                    println!("attack entity: {:?}, target entity: {:?}", source, target);
                    if health_entity.is_dead() {
                        continue;
                    }
                    health_entity.damage(attack_entity.attack());
                }
            }
        }
    }
}

/// 查找相互作用的 entity
/// 触发者, 组件, 被触发者
fn find_eneity<'a, T>(query: &'a Query<&T>, entitys: [Entity; 2]) -> Vec<(Entity, &'a T, Entity)>
where
    T: Component,
{
    let mut res = Vec::new();
    for (index, entity) in entitys.iter().enumerate() {
        let target = if index == 0 { entitys[1] } else { entitys[0] };
        if let Ok(r) = query.get(*entity) {
            res.push((*entity, r, target));
        }
    }
    res
}
