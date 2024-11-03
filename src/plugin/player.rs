use bevy::{
    app::prelude::*,
    asset::Assets,
    color::{palettes::css, Color},
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    utils::default,
};

use crate::comp::{control::ControlComponent, player::PlayerComponent};

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
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 50.))),
        material: materials.add(Color::Srgba(css::RED)),
        transform: Transform::from_xyz(150., 150., 0.),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 50.))),
            material: materials.add(Color::Srgba(css::ORANGE)),
            ..default()
        },
        PlayerComponent,
        ControlComponent,
    ));
}
