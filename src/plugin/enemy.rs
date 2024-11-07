use std::time::Duration;

use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;

use crate::common::prelude::*;
use crate::comp::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CountdownTimer::<EnemySpawn>::new(
            Duration::new(2, 0),
            Duration::new(1, 0),
            true,
        ))
        .add_systems(PostUpdate, enemy_spawn);
    }
}

fn enemy_spawn(
    mut commands: Commands,
    count_down: ResMut<CountdownTimer<EnemySpawn>>,
    player: Query<&Transform, With<PlayerComponent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !count_down.is_finished() {
        return;
    }

    let player = match player.get_single() {
        Ok(r) => r,
        Err(_) => {
            return;
        }
    };

    let pos = generate_random_excluding_range(150., 50.);
    let pos = player.translation + Vec3::new(pos.x, pos.y, 0.);

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(10.))),
                material: materials.add(Color::Srgba(css::RED)),
                transform: Transform::from_translation(pos),
                ..default()
            },
            EnemyComponent,
            HealthComponent::new(10),
            SpeedComponent::new(50.)
        ))
        .insert((
            Collider::ball(10.),
            RigidBody::KinematicVelocityBased,
            GravityScale(0.),
            Velocity::zero(),
            CollidingEntities::default(),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(get_enemy_group(), get_player_group()),
        ));
}
