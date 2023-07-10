#![windows_subsystem = "windows"]

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
    let mut app = App::new();
    
    #[allow(unused_mut)]
    let mut default_plugin = DefaultPlugins.set(
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
    });

    #[cfg(target = "wasm32-unknown-unknown")]
    {
        default_plugin = default_plugin.set(AssetPlugin{watch_for_changes: true, asset_folder: "berryville/assets".into(), ..Default::default()});
    }

    app.add_plugins(default_plugin);
    app.insert_resource(Msaa::Off);
    app.add_state::<AppState>();
    app.add_startup_system(setup);

    app.add_systems((
        main_menu_scene_enter_system,
        ).in_schedule(OnEnter(AppState::MainMenu)));
    // .add_systems((
    //     ).in_schedule(OnExit(AppState::MainMenu)))
    app.add_systems((
        main_menu_scene_update_system,
        ).in_set(OnUpdate(AppState::MainMenu)));

    app.add_systems((
        games_scene_on_enter,
        ).in_schedule(OnEnter(AppState::InGame)));
    // .add_systems((
    //     game_scene_exit_system,
    //     ).in_schedule(OnExit(AppState::InGame)))
    app.add_systems((
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

        game_scene_update_system,
        ).in_set(OnUpdate(AppState::InGame)));

    app.add_systems((
        restart_scene_on_enter_system,
        ).in_schedule(OnEnter(AppState::Restart)));
    app.add_systems((
        repeat_scene_exit_system,
        ).in_schedule(OnExit(AppState::Restart)));
    // .add_systems((
    // ).in_set(OnUpdate(AppState::Restart)))
    
    app.add_systems((
        camera_system, 
        wavy_update_system,
        transparency_update_system,
        curtain_system,
        game_scene_background_sky_update_system,
    ));
    
    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Restart,
}

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct RestartCurtain {
    progress: f32,
    direction: Direction,
    forward: bool,
    wait: f32
}

impl RestartCurtain {
    pub fn new(direction: Direction) -> Self {
        RestartCurtain {
            progress: 0.0,
            direction: direction,
            forward: false,
            wait: 0.0,
        }
    } 
}

fn restart_scene_on_enter_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/right_curtain.png"),
            transform: Transform {
                translation: Vec3::new(-450.0, 0.0, 12.0),
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        },
        RestartEntity{},
        RestartCurtain::new(Direction::Right),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/left_curtain.png"),
            transform: Transform {
                translation: Vec3::new(450.0, 0.0, 12.0),
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        },
        RestartEntity{},
        RestartCurtain::new(Direction::Left),
    ));
}

fn curtain_system(        
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut targets: Query<(&mut Transform, &mut RestartCurtain)>, 
        time: Res<Time>
    ) {

    for (mut transform, mut curtain) in targets.iter_mut() {
        if curtain.forward == false {
            curtain.progress += time.delta_seconds();
            
            if curtain.progress > 1.0 {
                curtain.progress = 1.0;
                curtain.forward = true;
            }

            if curtain.direction == Direction::Left {
                transform.translation.x = (1.0 - curtain.progress) * 450.0;
            } else {
                transform.translation.x = (1.0 - curtain.progress) * -450.0;
            }

            continue;
        }
        
        curtain.wait += time.delta_seconds();
        if curtain.wait < 2.0 {
            curtain.progress = 0.0;
            
            if app_state.0 != AppState::MainMenu {
                app_state_next_state.set(AppState::MainMenu);
            }

            continue;
        }
        curtain.wait = 2.0;

        if curtain.forward == true {
            curtain.progress += time.delta_seconds();
            
            if curtain.progress > 1.0 {
                curtain.progress = 1.0;
            }

            if curtain.direction == Direction::Left {
                transform.translation.x = (curtain.progress) * 450.0;
            } else {
                transform.translation.x = (curtain.progress) * -450.0;
            }
        }
    }
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

fn main_menu_scene_update_system(
        input: Res<Input<KeyCode>>,
        buttons: Res<Input<MouseButton>>,
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>,

        mut menu_transparent: Query<(&mut Transparency, &MainMenuEntity), Without<GameEntity>>, 
        mut game_transparent: Query<(&mut Transparency, &GameEntity), Without<MainMenuEntity>>,
        time: Res<Time>
    ) {

    if app_state.0 == AppState::InGame {
        return;
    }

    if input.just_pressed(KeyCode::Space) {
        app_state_next_state.set(AppState::InGame);
    }

    if  buttons.just_pressed(MouseButton::Left) || 
        buttons.just_pressed(MouseButton::Right) ||
        buttons.just_pressed(MouseButton::Middle) {
        app_state_next_state.set(AppState::InGame);
    }

    for (mut transparency, _) in menu_transparent.iter_mut() {
        if transparency.value < 1.0 {
            transparency.value += 0.6 * time.delta_seconds()
        } else {
            transparency.value = 1.0;
        }
    }

    for (mut transparency, _) in game_transparent.iter_mut() {
        if transparency.value > 0.0 {
            transparency.value -= 0.6 * time.delta_seconds()
        } else {
            transparency.value = 0.0;
        }
    }
}

fn game_scene_update_system(
        input: Res<Input<KeyCode>>,
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>,

        mut menu_transparent: Query<(&mut Transparency, &MainMenuEntity), Without<GameEntity>>, 
        mut game_transparent: Query<(&mut Transparency, &GameEntity), Without<MainMenuEntity>>, 
        time: Res<Time>
    ) {

    if input.just_pressed(KeyCode::R) {
        if app_state.0 != AppState::Restart {
            app_state_next_state.set(AppState::Restart);
        }
    }

    for (mut transparency, _) in menu_transparent.iter_mut() {
        if transparency.value > 0.0 {
            transparency.value -= 0.6 * time.delta_seconds()
        } else {
            transparency.value = 0.0;
        }
    }

    for (mut transparency, _) in game_transparent.iter_mut() {
        if transparency.value < 1.0 {
            transparency.value += 0.6 * time.delta_seconds()
        } else {
            transparency.value = 1.0;
        }
    }
}

#[derive(Component)]
pub struct Wavy {
    amplitude: f32,
    timer: f32,
    speed: f32,
}

impl Wavy {
    pub fn new(amplitude: f32, speed: f32) -> Self {
        Wavy { 
            amplitude: amplitude, 
            timer: 0.0,
            speed: speed,
        }
    }
}

#[derive(Component)]
pub struct Transparency {
    value: f32,
}

impl Transparency {
    pub fn new(value: f32) -> Self {
        Transparency { 
            value: value
        }
    }

    pub fn default() -> Self {
        Transparency { 
            value: 1.0
        }
    }
}

#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct RestartEntity;

fn main_menu_scene_enter_system(        
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>
    ) {

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/background_grass.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        BackGroundEntity{},
        RenderLayers::layer(1)
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/background_sky.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        BackGroundSky{},
        BackGroundEntity{},
        RenderLayers::layer(1)
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/background_sky.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 120.0, 1.0)),
            ..default()
        },
        BackGroundSky{},
        BackGroundEntity{},
        RenderLayers::layer(1)
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/background_sky.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 240.0, 1.0)),
            ..default()
        },
        BackGroundSky{},
        BackGroundEntity{},
        RenderLayers::layer(1)
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/background_sky.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 360.0, 1.0)),
            ..default()
        },
        BackGroundSky{},
        BackGroundEntity{},
        RenderLayers::layer(1)
    ));

    // Circle
    commands.spawn(SeedBundle::new(&mut meshes,&mut materials, Transform::from_translation(Vec3::new(0.0, -40.0, 100.0)), 0.2));
}

pub fn transparency_update_system(
        mut sprite_targets: Query<(&Transparency, &mut Sprite), Without<Text>>,
        mut text_targets: Query<(&Transparency, &mut Text), Without<Sprite>>
    ) {
        
    for (transparency, mut sprite) in sprite_targets.iter_mut() {
        sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), transparency.value); 
    }

    for (transparency, mut text) in text_targets.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.color = Color::rgba(section.style.color.r(), section.style.color.g(), section.style.color.b(), transparency.value); 
        }
    }
}

pub fn wavy_update_system(mut targets: Query<(&mut Wavy, &mut Transform)>, time: Res<Time>) {
    for (mut wavy, mut transform) in targets.iter_mut() {
        wavy.timer += time.delta_seconds();
        transform.translation.y -= ((wavy.timer * wavy.speed).sin() / 600.0) * wavy.amplitude;
    }
}

fn games_scene_on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut text_bundle = TextBundle::from_section(
        "0",
        TextStyle {
            font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
            font_size: 85.0,
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
        },
    );
    text_bundle.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

    commands.spawn((
        text_bundle
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
        ScoreText{},
        GameEntity{},
        Transparency::new(0.0)
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            texture: asset_server.load("textures/repeat_button.png"),
            transform: Transform {
                translation: Vec3::new(-350.0, 250.0, 11.0),
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        },
        Transparency::new(0.0),
        GameEntity{},
    ));

}

fn repeat_scene_exit_system(
        mut commands: Commands, 
        growing_things: Query<Entity, With<Growth>>,
        backgrounds: Query<Entity, With<BackGroundEntity>>,
        mut cameras: Query<(&mut Transform, &GameCamera), Without<Growth>>

        
    ) {
    
    for entity in backgrounds.iter() {
        commands.entity(entity).despawn();
    }

    for entity in growing_things.iter() {
        commands.entity(entity).despawn();
    }

    for (mut transform, _) in cameras.iter_mut() {
        transform.translation.y = 0.0;
    }
}

fn game_scene_background_sky_update_system(
        mut commands: Commands, 
        targets: Query<(Entity, &Transform, &BackGroundSky)>,
        cameras: Query<(&Transform, &GameCamera), Without<Growth>>,
        asset_server: Res<AssetServer>
    ) {
    
    let mut max_camera_height = 0.0;

    for (transform, _) in cameras.iter() {
        if transform.translation.y > max_camera_height {
            max_camera_height = transform.translation.y;
        }
    }

    let mut max_background_height = 0.0;

    for (_, transform, _) in targets.iter() {
        if transform.translation.y > max_background_height {
            max_background_height = transform.translation.y;
        }
    }

    for (background, transform, _) in targets.iter() {
        if transform.translation.y > (max_camera_height - 150.0) {
            continue;
        } 

        commands.entity(background).despawn();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/background_sky.png"),
                transform: Transform::from_translation(Vec3::new(0.0, max_background_height + 120.0, 1.0)),
                ..default()
            },
            BackGroundSky{},
            BackGroundEntity{},
            RenderLayers::layer(1)
        ));
    }
}


#[derive(Component)]
struct BackGroundSky;

#[derive(Component)]
struct BackGroundEntity;

#[derive(Component)]
struct ScoreText;

fn setup(
        mut commands: Commands,
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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            texture: asset_server.load("textures/background_logo.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 130.0, 10.0),
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        },
        Transparency::new(0.0),
        MainMenuEntity{}
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            texture: asset_server.load("textures/press_any_button_to_start.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -150.0, 10.0),
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        },
        Wavy::new(8.0, 5.0),
        Transparency::new(0.0),
        MainMenuEntity{}
    ));

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

    commands.spawn((Camera2dBundle::default(),));
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