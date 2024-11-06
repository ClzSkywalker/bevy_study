use std::sync::OnceLock;

use bevy_rapier2d::prelude::*;

static PLAYER_GROUP: OnceLock<Group> = OnceLock::new();

static ENEMY_GROUP: OnceLock<Group> = OnceLock::new();

pub fn init_camp() {
    PLAYER_GROUP.set(Group::GROUP_1).unwrap();
    ENEMY_GROUP.set(Group::GROUP_1).unwrap();
}

pub fn get_player_group() -> Group {
    PLAYER_GROUP.get().unwrap().clone()
}

pub fn get_enemy_group() -> Group {
    ENEMY_GROUP.get().unwrap().clone()
}
