use bevy::{
    color::palettes::css,
    input::mouse::MouseMotion,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::comp::{
    control::ControlComponent,
    movement::{Acceleration, MovementBundle, Velocity},
    player::PlayerComponent,
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_control, shut_bullet).chain());
    }
}

fn update_control(
    mut query: Query<&mut Transform, With<ControlComponent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 100.0 * time.delta_seconds();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.;
    }
    for mut transform in query.iter_mut() {
        transform.translation += direction * speed;
    }
}

fn shut_bullet(
    mut command: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: EventReader<MouseMotion>,
    player: Query<&Transform, With<PlayerComponent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // mouse_input.get_just_pressed().last().unwrap().
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    println!("click");
    let palyer = match player.get_single() {
        Ok(r) => r,
        Err(_) => {
            return;
        }
    };
    println!("palyer");
    if let Some(pos) = print_mouse_click_position(mouse_motion) {
        let pos1 = palyer.translation - Vec3::new(pos.x, pos.y, 0.);
        let pos2 = Transform::from_scale(pos1).forward() * 100.;
        println!("pos:{:?}", pos2);
        command.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 50.))),
                material: materials.add(Color::Srgba(css::RED)),
                transform: Transform::from_xyz(palyer.translation.x, palyer.translation.y, 0.),
                ..default()
            },
            MovementBundle::new(Velocity::new(pos2), Acceleration::new(Vec3::ZERO)),
        ));
    }
}

fn print_mouse_click_position(mut mouse_motion: EventReader<MouseMotion>) -> Option<Vec2> {
    match mouse_motion.read().last() {
        Some(r) => Some(r.delta),
        None => None,
    }
}
