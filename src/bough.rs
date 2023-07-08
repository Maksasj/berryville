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
    pub done_with_branches: bool,
    pub done_with_sticks: bool,
}

impl Bough {
    pub fn new() -> Self {
        Bough { 
            done_with_branches: false,
            done_with_sticks: false,
        }
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

pub fn bough_system(mut commands: Commands, mut targets: Query<(&mut Growth, &mut Transform, &Rotation, &mut Bough)>, branches: Query<&Branch>, sticks: Query<&Stick>) {
    let mut active_branch_count = branches.iter().count();
    let mut active_stick_count = sticks.iter().count();
    
    for (growth, mut transform, rotation, mut bough) in targets.iter_mut() {
        // Still growing
        if growth.timer < growth.max_time {
            transform.scale = Vec3::splat(growth.growth_value);
            continue;
        }

        // Nice we growth up !
        if bough.done_with_branches && bough.done_with_sticks {
            continue; // We have growth up, last iteration i guess
        }

        let mut rng = rand::thread_rng();
        
        if !bough.done_with_branches {
            let branch_count = rng.gen_range(1..3); // [x0, x1)
    
            for _ in 0..branch_count {
                if active_branch_count > 100 {
                    continue;
                }
    
                let new_angle = rng.gen_range((-0.572665)..(0.572665));
                let mut new_transform: Transform = transform.clone();
                
                new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);
                new_transform.scale = Vec3::splat(0.0);
                
                commands.spawn(BranchBundle::new(new_transform, rotation.angle + new_angle));
                bough.done_with_branches = true;
                active_branch_count += 1;
            }
        }

        if !bough.done_with_sticks {
            let stick_count = rng.gen_range(1..5); // [x0, x1)

            for _ in 0..stick_count {
                if active_stick_count > 100 {
                    continue;
                }

                let new_angle = rng.gen_range((-3.14159)..(3.14159));
                let mut new_transform: Transform = transform.clone();

                new_transform.rotation = Quat::from_rotation_z(rotation.angle + new_angle);
                new_transform.scale = Vec3::splat(0.0);

                commands.spawn(StickBundle::new(new_transform, rotation.angle + new_angle));
                bough.done_with_sticks = true;
                active_stick_count += 1;
            }
        }
    }
}