use bevy::{
    prelude::*,
    sprite::*,
    render::view::*, 
};

use crate::{
    growth::*, 
    branch::*,
};

#[derive(Component)]
pub struct Seed {

}

impl Seed {
    pub fn new() -> Self {
        Seed { }
    }
}

#[derive(Bundle)]
pub struct SeedBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    
    growth: Growth,
    seed: Seed,
}

impl SeedBundle {
    pub fn new_from_transform(mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<ColorMaterial>>, transform: Transform) -> Self {
        SeedBundle { 
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: transform,
                ..default()
            },
            growth: Growth::new(2.0),
            seed: Seed::new()
        }
    }
}

pub fn seed_system(
        mut commands: Commands,  
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut targets: Query<(&mut Growth, &mut Transform), With<Seed>>
    ) {

    for (mut growth, mut transform) in targets.iter_mut() {
        // Still growing
        if growth.timer < growth.max_time {
            transform.scale = 0.2 * Vec3::splat(growth.timer);
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
        new_transform.translation.y += 2.0;

        commands.spawn((BranchBundle::new_from_transform(&mut meshes, &mut materials, new_transform), first_pass_layer));
}   
}