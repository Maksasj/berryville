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

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl SeedBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, transform: Transform, angle: f32) -> Self {
        SeedBundle { 
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            seed: Seed::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn seed_system(mut commands: Commands, mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Seed>> ) {
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

        let mut rng = rand::thread_rng();
 
        let branch_count = rng.gen_range(1..3); // [x0, x1)

        for _ in 0..branch_count {
            let new_angle = rng.gen_range((-0.872665)..(0.872665));
            let mut new_transform: Transform = transform.clone();

            new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);

            commands.spawn(BranchBundle::new(new_transform, rotation.angle + new_angle));
        }
    }
}