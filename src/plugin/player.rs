use std::time::Duration;

use bevy::{
    app::prelude::*,
    asset::Assets,
    color::{palettes::css, Color},
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    utils::default,
};

use crate::comp::{
    common::{CountdownTimer, BulletCooling},
    control::ControlComponent,
    character::PlayerComponent,
};

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
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., 20.))),
            material: materials.add(Color::Srgba(css::ORANGE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        PlayerComponent,
        ControlComponent,
        CountdownTimer::<BulletCooling>::new(Duration::new(1, 0), Duration::ZERO, false),
    ));
}
