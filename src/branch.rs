use bevy::{
    prelude::*,
    render::view::*,
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
}

impl BranchBundle {
    pub fn new_from_transform(mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<ColorMaterial>>, transform: Transform) -> Self {
        BranchBundle { 
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                transform: transform,
                ..default()
            },

            growth: Growth::new(15.0),
            branch: Branch::new()
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
            transform.scale.y = 0.05 * growth.timer;
            continue;
        }

        // Nice we growth up !
        if growth.done {
            continue; // We have growth up, last iteration i guess
        }
        
        growth.done = true;

        let first_pass_layer = RenderLayers::layer(1);

        // Circle
        let mut new_transform: Transform = transform.clone();
        new_transform.translation.y += 25.0 * transform.scale.y;

        commands.spawn((SeedBundle::new_from_transform(&mut meshes, &mut materials, new_transform), first_pass_layer));
    }   
}