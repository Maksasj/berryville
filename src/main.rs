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

mod bough;
use bough::*;

mod apple;
use apple::*;

mod grape;
use grape::*;

mod banana;
use banana::*;

mod cocos;
use cocos::*;

mod stick;
use stick::*;

mod rotation;

mod game_camera;
use game_camera::*;

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
        .add_systems((
            growth_system, 
            
            seed_system, 
            branch_system, 
            bough_system, 
            stick_system, 
            apple_system,
            grape_system,
            banana_system,
            cocos_system,

            score_text_update_system,
            
            boundery_growth_limit_system, 
            camera_system).chain())
        .run();
}

fn camera_system(targets: Query<&Transform, With<Growth>>, mut cameras: Query<(&mut Transform, &GameCamera), Without<Growth>>, time: Res<Time>) {
    let mut max_height = 0.0;
    for transform in targets.iter() {
        if transform.translation.y > max_height  {
            max_height = transform.translation.y;
        }
    }

    for (mut transform, _) in cameras.iter_mut() {
        transform.translation.y += (max_height - transform.translation.y) * 0.3 * time.delta_seconds();
    }
}

#[derive(Component)]
struct ScoreText;

fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut images: ResMut<Assets<Image>>,
        asset_server: Res<AssetServer>
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
    commands.spawn(SeedBundle::new(&mut meshes,&mut materials, Transform::from_translation(Vec3::splat(0.0)), 0.2));
    
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.45, 0.45, 0.45)),
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
        GameCamera{},
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

    commands.spawn((
        TextBundle::from_section(
            "000",
            TextStyle {
                font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                font_size: 75.0,
                color: Color::WHITE,
            },
        ) 
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText,
    ));
}

fn score_text_update_system(mut query: Query<&mut Text, With<ScoreText>>, cameras: Query<(&Transform, &GameCamera), Without<Growth>>) {
    let mut max_camera_height = 0.0;

    for (transform, _) in cameras.iter() {
        if transform.translation.y > max_camera_height {
            max_camera_height = transform.translation.y;
        }
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.0}", max_camera_height);
    }
}