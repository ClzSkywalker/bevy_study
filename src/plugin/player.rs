use std::time::Duration;

use bevy::{
    app::prelude::*,
    asset::Assets,
    color::{palettes::css, Color},
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    utils::default,
};
use bevy_rapier2d::prelude::*;

use crate::common::prelude::*;
use crate::comp::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_shape);
    }
}

fn player_shape(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., 20.))),
                material: materials.add(Color::Srgba(css::BLUE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            PlayerComponent,
            ControlComponent,
            HealthComponent::new(10),
            AttackComponent::new(5),
            CountdownTimer::<BulletCooling>::new(Duration::from_millis(500), Duration::ZERO, false),
        ))
        .insert((
            Collider::cuboid(10., 10.),
            RigidBody::Dynamic,
            GravityScale(0.),
            Velocity::zero(),
            CollisionGroups::new(get_player_group(), get_enemy_group()),
        ));
}
