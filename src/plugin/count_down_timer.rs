use bevy::prelude::*;

use crate::comp::common::{BulletCooling, CountdownTimer, EnemySpawn};

pub struct CountdownTimerPlugin;

impl Plugin for CountdownTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, count_down_timer);
    }
}

fn count_down_timer(
    time: Res<Time>,
    mut query: Query<&mut CountdownTimer<BulletCooling>>,
    mut enemy_spawn: ResMut<CountdownTimer<EnemySpawn>>,
) {
    if !enemy_spawn.is_finished() {
        enemy_spawn.sub(time.delta());
    }

    for mut ele in query.iter_mut() {
        if ele.is_finished() {
            if ele.auto_reset {
                ele.reset();
                continue;
            }
            continue;
        }
        ele.sub(time.delta());
    }
}
