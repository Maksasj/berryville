use bevy::{prelude::*, transform};

use crate::game_camera::*;

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

pub fn boundery_growth_limit_system(mut commands: Commands, targets: Query<(Entity, &Transform), With<Growth>>, cameras: Query<(&Transform, &GameCamera), Without<Growth>>) {
    let mut max_camera_height = 0.0;

    for (transform, _) in cameras.iter() {
        if transform.translation.y > max_camera_height {
            max_camera_height = transform.translation.y;
        }
    }

    for (entity, transform) in targets.iter() {
        if transform.translation.x > 100.0 || transform.translation.x < -100.0{
            commands.entity(entity).despawn();
        }

        if transform.translation.y < (max_camera_height - 100.0) {
            commands.entity(entity).despawn();
        }
    }
}