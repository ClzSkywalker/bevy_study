use bevy::{app::App, prelude::*, DefaultPlugins};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use common::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugin::{
    camera::CameraPlugin, collision::CollisionPlugin, control::ControlPlugin,
    count_down_timer::CountdownTimerPlugin, despawn::DespawnPlugin, enemy::EnemyPlugin,
    movement::MovementPlugin, player::PlayerPlugin,
};

mod common;
mod comp;
mod plugin;
mod resource;

fn main() {
    // 无头服务器插件，// 每秒运行几帧
    // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
    // .add_plugins(DefaultPlugins)
    // .add_plugins(HelloPlugin)
    // 每一帧的时间增量，默认0.25s
    // .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(2)))
    // .insert_resource(Time::<Fixed>::from_hz(2.)) // 每秒运行2次

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Game".to_string(),
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                    max_width: WINDOW_WIDTH,
                    max_height: WINDOW_HEIGHT,
                },
                ..default()
            }),
            ..default()
        })) // , WorldInspectorPlugin::default()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(CollisionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CountdownTimerPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
