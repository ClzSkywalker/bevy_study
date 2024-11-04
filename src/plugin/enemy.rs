use std::time::Duration;

use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use rand::thread_rng;
use rand::Rng;

use crate::comp::character::EnemyComponent;
use crate::comp::character::PlayerComponent;
use crate::comp::common::{CountdownTimer, EnemySpawn};
use crate::comp::movement::Acceleration;
use crate::comp::movement::MovementBundle;
use crate::comp::movement::Velocity;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CountdownTimer::<EnemySpawn>::new(
            Duration::new(2, 0),
            Duration::new(2, 0),
            true,
        ))
        .add_systems(Update, enemy_spawn);
        // .add_system(enemy_spawn)
        // .add_system(enemy_despawn);
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

    let x = generate_random_excluding_range(200, 80) as f32;
    let y = generate_random_excluding_range(200, 80) as f32;

    let pos = player.translation + Vec3::new(x, y, 0.);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., 20.))),
            material: materials.add(Color::Srgba(css::ORANGE)),
            transform: Transform::from_translation(pos),
            ..default()
        },
        EnemyComponent,
        MovementBundle::new(
            Velocity::new(Vec2::default()),
            Acceleration::new(Vec2::default()),
        ),
    ));
}

fn generate_random_excluding_range(radio1: i32, radio2: i32) -> i32 {
    loop {
        let num = thread_rng().gen_range(-radio1..=radio1);
        if num < -radio2 || num > radio2 {
            return num;
        }
    }
}
