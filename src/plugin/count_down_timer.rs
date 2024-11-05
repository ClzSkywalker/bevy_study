use bevy::prelude::*;

use crate::comp::common::{BulletCooling, CountdownTimer, DeadTimer, EnemySpawn};

pub struct CountdownTimerPlugin;

impl Plugin for CountdownTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, count_down_timer);
    }
}

fn count_down_timer(
    time: Res<Time>,
    mut bullet_timer: Query<&mut CountdownTimer<BulletCooling>>, // 玩家子弹冷却时间
    mut enemy_spawn_timer: ResMut<CountdownTimer<EnemySpawn>>, // 敌人生成器
    mut despawn_timer: Query<&mut CountdownTimer<DeadTimer>>, // 控制 entity 的生命时限
) {
    if !enemy_spawn_timer.is_finished() {
        enemy_spawn_timer.sub(time.delta());
    } else {
        enemy_spawn_timer.reset();
    }

    for mut ele in despawn_timer.iter_mut() {
        if ele.is_finished() {
            continue;
        }
        ele.sub(time.delta());
    }

    for mut ele in bullet_timer.iter_mut() {
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
