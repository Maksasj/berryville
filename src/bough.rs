use bevy::{
    prelude::*,
    sprite::*,
    render::view::*, 
};
use rand::Rng;

use crate::{
    growth::*, 
    branch::*,
    stick::*,
    rotation::*,
};

#[derive(Component)]
pub struct Bough {

}

impl Bough {
    pub fn new() -> Self {
        Bough { }
    }
}

#[derive(Bundle)]
pub struct BoughBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    
    growth: Growth,
    bough: Bough,

    rotation: Rotation,

    rendering_layer: RenderLayers,
}

impl BoughBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, transform: Transform, angle: f32) -> Self {
        BoughBundle { 
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0.25, 0.65, 0.25))),
                transform: transform,
                ..default()
            },
            growth: Growth::new(1.0, 0.5),
            bough: Bough::new(),

            rotation: Rotation::new(angle),

            rendering_layer: RenderLayers::layer(1)
        }
    }
}

pub fn bough_system(mut commands: Commands, mut targets: Query<(&mut Growth, &mut Transform, &Rotation), With<Bough>> ) {
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
            let new_angle = rng.gen_range((-0.572665)..(0.572665));
            let mut new_transform: Transform = transform.clone();
            
            new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);
            new_transform.scale = Vec3::splat(0.0);
            
            commands.spawn(BranchBundle::new(new_transform, rotation.angle + new_angle));
        }

        let stick_count = rng.gen_range(1..5); // [x0, x1)

        for _ in 0..stick_count {
            let new_angle = rng.gen_range((-3.14159)..(3.14159));
            let mut new_transform: Transform = transform.clone();

            new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);
            new_transform.scale = Vec3::splat(0.0);

            commands.spawn(StickBundle::new(new_transform, rotation.angle + new_angle));
        }
    }
}