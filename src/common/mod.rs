use bevy::math::Vec2;
use bevy_rapier2d::prelude::Group;
use rand::{thread_rng, Rng};

pub const WINDOW_WIDTH: f32 = 300.0;
pub const WINDOW_HEIGHT: f32 = 300.0;

pub const ITEM_GROUP: Group = Group::GROUP_3;

pub fn generate_random_excluding_range(radio1: f32, radio2: f32) -> Vec2 {
    if radio1.abs() <= radio2.abs() {
        panic!("radio1 must be greater than radio2");
    }
    loop {
        let x = thread_rng().gen_range(-radio1..=radio1);
        let y = thread_rng().gen_range(-radio1..=radio1);
        if radio2 <= (x.powf(2.) + y.powf(2.)).sqrt() {
            return Vec2::new(x, y);
        }
    }
}

pub fn player_group() -> Group {
    Group::GROUP_1 | Group::GROUP_2
}

pub fn enemy_group() -> Group {
    Group::GROUP_1 | Group::GROUP_2
}
