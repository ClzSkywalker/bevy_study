use bevy::{app::App, prelude::*, DefaultPlugins};
use plugin::{camera::CameraPlugin, control::ControlPlugin, movement::MovementPlugin, player::PlayerPlugin};

mod comp;
mod plugin;

fn main() {
    // 无头服务器插件，// 每秒运行几帧
    // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
    // .add_plugins(DefaultPlugins)
    // .add_plugins(HelloPlugin)
    // 每一帧的时间增量，默认0.25s
    // .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(2)))
    // .insert_resource(Time::<Fixed>::from_hz(2.)) // 每秒运行2次

    App::new()
        .add_plugins((DefaultPlugins)) // , WorldInspectorPlugin::default()
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
