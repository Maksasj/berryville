//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, window::PresentMode, sprite::{MaterialMesh2dBundle, Material2d, Mesh2dHandle},
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(
        ImagePlugin::default_nearest(),
        ).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Berryville".into(),
                resolution: (800.0, 600.0).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: false,
                canvas: Some("#game".to_string()),
                resizable: false,
                prevent_default_event_handling: false,
                resize_constraints: WindowResizeConstraints{
                    min_width: 800.0,
                    min_height: 600.0,
                    max_width: 800.0,
                    max_height: 600.0,
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::Off)
        .add_startup_system(setup)
        .add_systems((growth_system, seed_system, branch_system).chain())
        .run();
}

#[derive(Component)]
pub struct Growth {
    pub timer: f32,
    pub max_time: f32,
    pub done: bool
}

impl Growth {
    pub fn new(max_growth_time: f32) -> Self {
        Growth { 
            timer: 0.0,
            max_time: max_growth_time,
            done: false
        }
    }
}

#[derive(Component)]
pub struct Seed {
}

impl Seed {
    pub fn new() -> Self {
        Seed { }
    }
}

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

#[derive(Bundle)]
pub struct SeedBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    
    growth: Growth,
    seed: Seed,
}

impl SeedBundle {
    pub fn new(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        SeedBundle { 
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            growth: Growth::new(2.0),
            seed: Seed::new()
        }
    }

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

pub fn growth_system(mut commands: Commands, mut targets: Query<(&mut Growth)>) {
    for mut growth in targets.iter_mut() {
        growth.timer += 0.01;
        if growth.timer > growth.max_time {
            growth.timer = growth.max_time;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 160,
        height: 120,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    let first_pass_layer = RenderLayers::layer(1);

    // Circle
    commands.spawn((SeedBundle::new(meshes, materials), first_pass_layer));

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        first_pass_layer,
    ));

    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        transform: Transform {
            scale: Vec3::splat(5.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
}
