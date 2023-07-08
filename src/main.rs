use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, window::PresentMode,
};

mod seed;
use seed::*;

mod growth;
use growth::*;

mod branch;
use branch::*;

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

    // Circle
    commands.spawn(SeedBundle::new(&mut meshes,&mut materials, Transform::from_translation(Vec3::splat(0.0))));
    
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
        RenderLayers::layer(1),
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
