use bevy::{
    prelude::*,
    render::view::*, sprite::Anchor,
};
use rand::*;

use crate::{
    growth::*, 
    apple::*,
    rotation::*, grape::GrapeBundle, banana::BananaBundle, cocos::CocosBundle, 
};

#[derive(Component)]
pub struct Stick {
}

impl Stick {
    pub fn new() -> Self {
        Stick { }
    }
}

#[derive(Bundle)]
pub struct StickBundle {
    sprite: SpriteBundle,
    
    growth: Growth,
    stick: Stick,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl StickBundle {
    pub fn new(mut transform: Transform, angle: f32) -> Self {
        transform.scale = Vec3::splat(0.0);

        StickBundle { 
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.85, 0.25),
                    custom_size: Some(Vec2::new(2.0, 10.0)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: transform,
                ..default()
            },

            growth: Growth::new(1.0, 1.0),
            stick: Stick::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn stick_system(
        mut commands: Commands,  
        asset_server: Res<AssetServer>,
        mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Stick>>
    ) {
        
    for (mut growth, mut transform, rotation) in targets.iter_mut() {
        // Still growing
        if growth.timer < growth.max_time {
            transform.scale.y = growth.growth_value;
            transform.scale.x = growth.timer / growth.max_time;
            continue;
        }

        // Nice we growth up !
        if growth.done {
            continue; // We have growth up, last iteration i guess
        }
        
        growth.done = true;

        let mut rng = rand::thread_rng();
 
        let random_number = rng.gen_range(1..5); // [x0, x1) 1 2 3 4

        let mut new_transform: Transform = transform.clone();
            
        new_transform.translation.x -= 10.0 * rotation.angle.sin();
        new_transform.translation.y += 10.0 * rotation.angle.cos();

        new_transform.scale = Vec3::splat(0.0);

        match random_number {
            1 => { commands.spawn(AppleBundle::new(&asset_server, new_transform, 0.0)); },
            2 => { commands.spawn(GrapeBundle::new(&asset_server, new_transform, 0.0)); },
            3 => { commands.spawn(BananaBundle::new(&asset_server, new_transform, 0.0)); },
            4 => { commands.spawn(CocosBundle::new(&asset_server, new_transform, 0.0)); },
            _ => {}
        }
    }   
}