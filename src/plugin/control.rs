use std::time::Duration;

use bevy::{
    color::palettes::css,
    input::{
        mouse::{self, MouseButtonInput},
        touch::touch_screen_input_system,
        ButtonState,
    },
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{common::prelude::*, resource::MouseClickRes};
use crate::{comp::prelude::*, resource::MousePositionRes};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePositionRes>().add_systems(
            Update,
            (
                touch_screen_input_system,
                mouse_click_position,
                update_control,
                shut_bullet,
            )
                .chain(),
        );
    }
}

fn update_control(
    mut query: Query<(&mut Velocity, &SpeedComponent), With<PlayerComponent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::default();
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
    for (mut velocity, speed) in query.iter_mut() {
        velocity.linvel = direction * speed.speed();
    }
}

fn shut_bullet(
    mut command: Commands,
    player: Query<&Transform, With<PlayerComponent>>,
    click_pos: Option<Res<MouseClickRes>>,
    mut shut_count_down: Query<&mut CountdownTimer<BulletCooling>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut shut_count_down = match shut_count_down.get_single_mut() {
        Ok(r) => r,
        Err(_) => {
            return;
        }
    };
    if !shut_count_down.is_finished() {
        return;
    }

    let mouse_input = match click_pos {
        Some(r) => r,
        None => {
            return;
        }
    };

    if mouse_input.mouse_button != mouse::MouseButton::Left {
        return;
    }
    let palyer = match player.get_single() {
        Ok(r) => r,
        Err(_) => {
            return;
        }
    };

    shut_count_down.reset();
    let play_pos = Vec2::new(palyer.translation.x, palyer.translation.y);
    let pos1 = Vec2::new(mouse_input.pos.x, mouse_input.pos.y) - play_pos;
    let start_pos = play_pos + pos1.normalize() * 30.;

    command
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(5.))),
                material: materials.add(Color::Srgba(css::BLUE)),
                transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.),
                ..default()
            },
            BulletComponent::<CampBlue>::default(),
            AttackComponent::new(5),
            SpeedComponent::new(100.),
            CountdownTimer::<DeadTimer>::new(Duration::new(1, 0), Duration::new(1, 0), false),
        ))
        .insert((
            Collider::ball(5.),
            RigidBody::Dynamic,
            // 设置重力
            GravityScale(0.),
            Velocity::linear(pos1.normalize()),
            CollidingEntities::default(),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(get_player_group(), get_enemy_group()),
        ));
}

fn mouse_click_position(
    mut command: Commands,
    mut mouse_button_events: EventReader<MouseButtonInput>, // 监听按下与松开的瞬间事件
    mut cursor_moved_events: EventReader<CursorMoved>,      // 监听鼠标移动事件
    mouse_button_input: Res<ButtonInput<MouseButton>>,      // 鼠标按键资源
    mouse_pos_res: Res<MousePositionRes>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera.single();

    // 监听鼠标移动事件
    for event in cursor_moved_events.read() {
        command.insert_resource(MousePositionRes::new(event.position));
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, event.position)
        {
            if mouse_button_input.pressed(MouseButton::Left) {
                command.insert_resource(MouseClickRes::new(touch_position, MouseButton::Left));
                return;
            }
        }
    }

    // 监听鼠标按键事件
    if let Some(event) = mouse_button_events.read().next() {
        if event.state == ButtonState::Pressed {
            if let Some(touch_position) =
                camera.viewport_to_world_2d(camera_transform, mouse_pos_res.pos)
            {
                command.insert_resource(MouseClickRes::new(touch_position, event.button));
                return;
            }
        }
        if event.state == ButtonState::Released {
            command.remove_resource::<MouseClickRes>();
        }
    }
}
