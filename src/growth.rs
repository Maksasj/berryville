use bevy::prelude::*;

#[derive(Component)]
pub struct Growth {
    pub timer: f32,
    pub max_time: f32,

    pub growth_value: f32,
    pub max_growth: f32,

    pub done: bool,
}

impl Growth {
    pub fn new(max_growth_time: f32, max_growth_value: f32) -> Self {
        Growth { 
            timer: 0.0,
            max_time: max_growth_time,
            
            growth_value: 0.0,
            max_growth: max_growth_value, 

            done: false,
        }
    }
}

pub fn growth_system(mut targets: Query<&mut Growth>, time: Res<Time>) {
    for mut growth in targets.iter_mut() {
        growth.timer += time.delta_seconds();

        if growth.timer > growth.max_time {
            growth.timer = growth.max_time;
        }

        growth.growth_value = growth.max_growth * (growth.timer / growth.max_time); 
    }
}