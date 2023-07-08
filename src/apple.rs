use bevy::{
    prelude::*,
    sprite::*,
    render::view::*, 
};
use rand::Rng;

use crate::{
    growth::*, 
    branch::*,
    rotation::*,
};

#[derive(Component)]
pub struct Apple {

}

impl Apple {
    pub fn new() -> Self {
        Apple { }
    }
}

#[derive(Bundle)]
pub struct AppleBundle {
    sprite: SpriteBundle,

    growth: Growth,
    apple: Apple,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl AppleBundle {
    pub fn new(asset_server: &Res<AssetServer>, transform: Transform, angle: f32) -> Self {
        AppleBundle { 
            sprite: SpriteBundle {
                texture: asset_server.load("apple.png"),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            apple: Apple::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn apple_system(mut commands: Commands, mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Apple>> ) {
    for (mut growth, mut transform, rotation) in targets.iter_mut() {
        // Still growing
        if growth.timer < growth.max_time {
            transform.scale = Vec3::splat(growth.growth_value);
            continue;
        }

        // Nice we growth up !
        if growth.done {
            continue; // We have growth up, last iteration i guess
        }
        
        growth.done = true;

        /*
        let mut rng = rand::thread_rng();
 
        let branch_count = rng.gen_range(1..3); // [x0, x1)

        for _ in 0..branch_count {
            let new_angle = rng.gen_range((-0.872665)..(0.872665));
            let mut new_transform: Transform = transform.clone();

            new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);
            new_transform.scale = Vec3::splat(0.0);

            commands.spawn(BranchBundle::new(new_transform, rotation.angle + new_angle));
        }
        */
    }
}