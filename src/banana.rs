use bevy::{
    prelude::*,
    render::view::*, 
};

use crate::{
    growth::*, 
    rotation::*,
};

#[derive(Component)]
pub struct Banana {

}

impl Banana {
    pub fn new() -> Self {
        Banana { }
    }
}

#[derive(Bundle)]
pub struct BananaBundle {
    sprite: SpriteBundle,

    growth: Growth,
    banana: Banana,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl BananaBundle {
    pub fn new(asset_server: &Res<AssetServer>, transform: Transform, angle: f32) -> Self {
        BananaBundle { 
            sprite: SpriteBundle {
                texture: asset_server.load("textures/banana.png"),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            banana: Banana::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn banana_system(mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Banana>> ) {
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