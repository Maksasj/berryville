use bevy::{
    prelude::*,
    render::view::*, 
};

use crate::{
    growth::*, 
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
                texture: asset_server.load("textures/apple.png"),
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

pub fn apple_system(mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Apple>> ) {
    for (mut growth, mut transform, _) in targets.iter_mut() {
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
    }
}