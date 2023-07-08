use bevy::prelude::*;

#[derive(Component)]
pub struct Growth {
    pub timer: f32,
    pub max_time: f32,
    pub done: bool
}

impl Growth {
    pub fn new(max_growth_time: f32) -> Self {
        Growth { 
            timer: 0.0,
            max_time: max_growth_time,
            done: false
        }
    }
}

pub fn growth_system(mut commands: Commands, mut targets: Query<(&mut Growth)>) {
    for mut growth in targets.iter_mut() {
        growth.timer += 0.01;
        if growth.timer > growth.max_time {
            growth.timer = growth.max_time;
        }
    }
}