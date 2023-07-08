use bevy::{
    prelude::*,
    render::view::*, 
};

use crate::{
    growth::*, 
    rotation::*,
};

#[derive(Component)]
pub struct Cocos {

}

impl Cocos {
    pub fn new() -> Self {
        Cocos { }
    }
}

#[derive(Bundle)]
pub struct CocosBundle {
    sprite: SpriteBundle,

    growth: Growth,
    cocos: Cocos,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl CocosBundle {
    pub fn new(asset_server: &Res<AssetServer>, transform: Transform, angle: f32) -> Self {
        CocosBundle { 
            sprite: SpriteBundle {
                texture: asset_server.load("cocos.png"),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            cocos: Cocos::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn cocos_system(mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Cocos>> ) {
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