use bevy::{
    prelude::*,
    render::view::*, sprite::Anchor,
};
use rand::*;

use crate::{
    growth::*, 
    seed::*, 
    rotation::*,
};

#[derive(Component)]
pub struct Branch {
}

impl Branch {
    pub fn new() -> Self {
        Branch { }
    }
}

#[derive(Bundle)]
pub struct BranchBundle {
    sprite: SpriteBundle,
    
    growth: Growth,
    branch: Branch,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl BranchBundle {
    pub fn new(transform: Transform, angle: f32) -> Self {
        BranchBundle { 
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 25.0)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: transform,
                ..default()
            },

            growth: Growth::new(1.0, 1.0),
            branch: Branch::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn branch_system(
        mut commands: Commands,  
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Branch>>
    ) {
        
    for (mut growth, mut transform, rotation) in targets.iter_mut() {
        // Still growing
        if growth.timer < growth.max_time {
            transform.scale.y = growth.growth_value;
            continue;
        }

        // Nice we growth up !
        if growth.done {
            continue; // We have growth up, last iteration i guess
        }
        
        growth.done = true;

        let mut rng = rand::thread_rng();

        {
            let new_angle = rng.gen_range(-0.2..0.2);

            let mut new_transform: Transform = transform.clone();
            
            new_transform.translation.x -= 25.0 * rotation.angle.sin();
            new_transform.translation.y += 25.0 * rotation.angle.cos();

            new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);

            commands.spawn(SeedBundle::new(&mut meshes, &mut materials, new_transform, rotation.angle + new_angle));
        }
    }   
}