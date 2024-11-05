use std::time::Duration;

use bevy::{
    color::palettes::css,
    input::{
        mouse::{self, MouseButtonInput},
        ButtonState,
    },
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

use crate::{
    comp::{
        character::PlayerComponent,
        common::{BulletComponent, BulletCooling, CampBlue, CountdownTimer, DeadTimer},
        control::ControlComponent,
    },
    resource::MouseClickRes,
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (mouse_click_position, update_control, shut_bullet).chain(),
        );
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
    player: Query<&Transform, With<PlayerComponent>>,
    click_pos: Option<Res<MouseClickRes>>,
    mut shut_count_down: Query<&mut CountdownTimer<BulletCooling>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut shut_count_down = shut_count_down.single_mut();
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
    let pos2 = pos1.normalize() * 100.;
    let start_pos = play_pos + pos1.normalize() * 30.;

    command.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(5.))),
            material: materials.add(Color::Srgba(css::BLUE)),
            transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.),
            ..default()
        },
        BulletComponent::<CampBlue>::default(),
        CountdownTimer::<DeadTimer>::new(Duration::new(1, 0), Duration::new(1, 0), false),
        Collider::ball(10.),
        RigidBody::Dynamic,
        Velocity::linear(pos2),
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
    ));
    // .insert((
    //     Collider::ball(5.),
    //     CollisionGroups::new(player_group(), ITEM_GROUP),
    // ));
}

fn mouse_click_position(
    mut command: Commands,
    mut mouse_button_events: EventReader<MouseButtonInput>, // 监听按下与松开的瞬间事件
    mut cursor_moved_events: EventReader<CursorMoved>,      // 监听鼠标移动事件
    mouse_button_input: Res<ButtonInput<MouseButton>>,      // 监听鼠标按键资源
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = match window_query.get_single() {
        Ok(r) => r,
        Err(_) => {
            // 鼠标不在窗口，移除资源
            command.remove_resource::<MouseClickRes>();
            return;
        }
    };

    // 监听鼠标移动事件
    if let Some(event) = cursor_moved_events.read().last() {
        let mut cursor_position = event.position;
        cursor_position.y -= window.height() / 2.;
        cursor_position.y = -cursor_position.y;
        cursor_position.x -= window.width() / 2.;
        if mouse_button_input.pressed(MouseButton::Left) {
            command.insert_resource(MouseClickRes::new(cursor_position, MouseButton::Left));
            return;
        }
    };

    let mut cursor_position = match window.cursor_position() {
        Some(r) => r,
        None => {
            return;
        }
    };
    cursor_position.y -= window.height() / 2.;
    cursor_position.y = -cursor_position.y;
    cursor_position.x -= window.width() / 2.;
    // 监听鼠标按键事件
    if let Some(event) = mouse_button_events.read().last() {
        if event.state == ButtonState::Pressed {
            command.insert_resource(MouseClickRes::new(cursor_position, event.button));
            return;
        }
        if event.state == ButtonState::Released {
            command.remove_resource::<MouseClickRes>();
        }
    };
}
