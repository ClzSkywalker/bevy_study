use std::{ops::Sub, time::Duration};

use bevy::prelude::*;

pub trait ICamp {}

#[derive(Debug, Default)]
pub struct CampRed;

impl ICamp for CampRed {}

#[derive(Debug, Default)]
pub struct CampBlue;

impl ICamp for CampBlue {}

// 子弹
#[derive(Component, Default)]
pub struct BulletComponent<T: ICamp + Default> {
    camp: std::marker::PhantomData<T>,
}

pub trait ICountdownTimer {}

// 组件销毁计时
pub struct DeadTimer;

impl ICountdownTimer for DeadTimer {}

// 子弹发射冷却
pub struct BulletCooling;

impl ICountdownTimer for BulletCooling {}

pub struct EnemySpawn;

impl ICountdownTimer for EnemySpawn {}

#[derive(Component, Resource)]
pub struct CountdownTimer<T: ICountdownTimer> {
    pub total: Duration,
    pub current: Duration,
    pub auto_reset: bool,
    data_type: std::marker::PhantomData<T>,
}

impl<T: ICountdownTimer> CountdownTimer<T> {
    pub fn new(total: Duration, current: Duration, auto_reset: bool) -> Self {
        Self {
            auto_reset,
            total,
            current,
            data_type: std::marker::PhantomData,
        }
    }

    pub fn sub(&mut self, time: Duration) {
        if self.is_finished(){
            return;
        }
        if self.current <= time {
            self.current = Duration::ZERO;
            return;
        }
        self.current = self.current.sub(time);
    }

    pub fn is_finished(&self) -> bool {
        self.current <= Duration::ZERO
    }

    pub fn reset(&mut self) {
        self.current = self.total;
    }
}
