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
        .add_system(growth)
        .run();
}

#[derive(Component)]
pub struct Seed {
    pub timer: f32
}

impl Seed {
    pub fn new() -> Self {
        Seed { 
            timer: 0.0 
        }
    }
}

#[derive(Bundle)]
pub struct SeedBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    seed: Seed
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
            seed: Seed::new()
        }
    }
}

#[derive(Component)]
pub struct Branch {
    pub timer: f32
}

impl Branch {
    pub fn new() -> Self {
        Branch { 
            timer: 0.0 
        }
    }
}

pub fn growth(mut commands: Commands, mut targets: Query<(&mut Seed, &mut Transform)>) {
    for (mut seed, mut transform) in targets.iter_mut() {
        seed.timer += 0.001;

        transform.scale = Vec3::splat(seed.timer);
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
