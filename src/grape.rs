use bevy::{
    prelude::*,
    render::view::*, 
};

use crate::{
    growth::*, 
    rotation::*,
};

#[derive(Component)]
pub struct Grape {

}

impl Grape {
    pub fn new() -> Self {
        Grape { }
    }
}

#[derive(Bundle)]
pub struct GrapeBundle {
    sprite: SpriteBundle,

    growth: Growth,
    grape: Grape,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl GrapeBundle {
    pub fn new(asset_server: &Res<AssetServer>, transform: Transform, angle: f32) -> Self {
        GrapeBundle { 
            sprite: SpriteBundle {
                texture: asset_server.load("textures/grape.png"),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            grape: Grape::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn grape_system( mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Grape>> ) {
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