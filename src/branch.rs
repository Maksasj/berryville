use bevy::{
    prelude::*,
    render::view::*, sprite::Anchor,
};

use crate::{
    growth::*, 
    seed::SeedBundle
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

    rendering_layer: RenderLayers,
}

impl BranchBundle {
    pub fn new(transform: Transform) -> Self {
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

            growth: Growth::new(5.0, 1.0),
            branch: Branch::new(),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn branch_system(
        mut commands: Commands,  
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut targets: Query<(&mut Growth, &mut Transform), With<Branch>>
    ) {
        
    for (mut growth, mut transform) in targets.iter_mut() {
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

        // Circle
        let mut new_transform: Transform = transform.clone();
        new_transform.translation.y += 25.0 * transform.scale.y;

        commands.spawn(SeedBundle::new(&mut meshes, &mut materials, new_transform));
    }   
}