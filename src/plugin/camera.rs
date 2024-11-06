use bevy::prelude::*;

use crate::comp::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera);
    }
}

#[derive(Component, Debug)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerComponent>)>,
    player: Query<&Transform, (With<PlayerComponent>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    let player = match player.get_single() {
        Ok(r) => r,
        Err(_) => {
            return;
        }
    };
    let Vec3 { x, y, .. } = player.translation;

    if let Ok(mut r) = camera.get_single_mut() {
        let direction = Vec3::new(x, y, 0.);
        r.translation = r.translation.lerp(direction, 0.5 * time.delta_seconds());
    }
}
