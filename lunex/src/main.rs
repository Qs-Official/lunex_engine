//# bevy_lunex is located here
mod library;

//# Importing the main crates
use crate::library::prelude::*;                                            //Will be replaced with "use bevy_lunex::prelude::*" when the crate is released
use bevy::{prelude::*, sprite::Anchor};

//# This is where Main Menu is styled
mod style;
use style::*;

//# For visual effects only
use bevy::core_pipeline::bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_plugin(ButtonPlugin)

        .add_startup_system(setup)

        .add_system(image_update)
        .add_system(smooth_movement)
        .add_system(smooth_bg_movement)

        .add_system(effect_update)

        .add_system(hiearchy_update)
        .add_system(cursor_update)

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {

    let music = asset_server.load("main_menu.ogg");
    audio.play_with_settings(music, PlaybackSettings { repeat: true, volume: 1., speed: 1. });

    //SPAWN 2D CAMERA
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1000. },
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings {
            intensity: 0.25,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 0.7,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.3,
                threshold_softness: 0.5,
            },
            composite_mode: BloomCompositeMode::Additive,
        },
        SmoothSlider {..Default::default()},
    ));


    //SPAWN UI HIEARCHY
    let system = create_main_menu();
    commands.spawn ((
        system,
    ));

    //SPAWN CURSOR
    commands.spawn ((
        Cursor::new(10.0),
        SpriteBundle {
            texture: asset_server.load("cursor_mouse.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 200. } , scale: Vec3 { x: 0.4, y: 0.4, z: 1. }, ..default() },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));


    //SPAWN BACKGROUND HANGLE MOVEMENT
    commands.spawn ((
        Widget {
            path: "App/Handle".to_string()
        },
        SmoothSlider {..Default::default()},
    ));

    //SPAWN BACKGROUND IMAGE
    commands.spawn ((
        Widget {
            path: "App/Handle/Background".to_string()
        },
        ImageInfo {
            width: 2560.,
            height: 1440.,
        },
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform { ..default() },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    //SPAWN BOARD IMAGE
    commands.spawn ((
        Widget {
            path: "App/Board".to_string()
        },
        ImageInfo {
            width: 807.,
            height: 1432.,
        },
        SpriteBundle {
            texture: asset_server.load("board.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 10. }, ..default() },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    //SPAWN LOGO IMAGE
    commands.spawn ((
        Widget {
            path: "App/Board/Logo".to_string()
        },
        ImageInfo {
            width: 681.,
            height: 166.,
        },
        SpriteBundle {
            texture: asset_server.load("logo.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 15. }, ..default() },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    //SPAWN LOGO SHADOW IMAGE
    commands.spawn ((
        Widget {
            path: "App/Board/Logo/LogoShadow".to_string()
        },
        ImageInfo {
            width: 858.,
            height: 209.,
        },
        SpriteBundle {
            texture: asset_server.load("logo_shadow.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 12. }, ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.7),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));


    //SPAWN BUTTON IMAGE
    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::rgb(204./255., 56./255., 51./255.),
    };

    let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    let button_name_list = ["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"];

    for i in 0..button_list.len() {
        commands.spawn ((
            Widget {
                path: "App/Board/ButtonList/".to_string() + button_list[i]
            },
            MainMenuButton {}
        ));

        commands.spawn ((
            Widget {
                path: "App/Board/ButtonList/".to_string() + button_list[i] + "/#p0"
            },
            MainMenuButtonDecoration {alpha: 0.0},
            ImageInfo {
                width: 532.,
                height: 75.,
            },
            SpriteBundle {
                texture: asset_server.load("button.png"),
                transform: Transform { translation: Vec3 { x: 0., y: 0., z: 15. }, ..default() },
                sprite: Sprite {
                    color: Color::rgba(1., 1., 1., 0.0),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            }
        )).with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(button_name_list[i], text_style.clone()).with_alignment(TextAlignment::Left),
                transform: Transform { translation: Vec3 { x: 30., y: -75./2., z: 15. }, ..default() },
                text_anchor: Anchor::CenterLeft,
                ..default()
            });
        });
    }
    
}


#[derive(Component)]
struct ImageInfo {
    width: f32,
    height: f32,
}
fn image_update(mut systems: Query<&mut Hierarchy>, mut query: Query<(&mut Widget, &ImageInfo, &mut Transform)>) {

    let mut system = systems.get_single_mut().unwrap();     //Unwrap the hiearchy struct

    for (widget, imageinfo, mut transform) in &mut query {

        let dimensions = (system.width, system.height);
        let pos = widget.fetch_position(&mut system, "").unwrap();
        transform.translation.x = pos.point_1[0] - dimensions.0/2.;
        transform.translation.y = pos.point_2[1] - dimensions.1/2.;

        transform.scale.x = pos.width/imageinfo.width;
        transform.scale.y = pos.height/imageinfo.height;
    }
}


//#SMOOTH MENU EFFECTS
#[derive(Component, Default)]
struct SmoothSlider {
    x: f32,
    y: f32,
}
fn smooth_movement (mut query: Query<(&mut SmoothSlider, &mut Transform)>) {
    for (mut smoothslider, mut transform) in &mut query {
        smoothslider.x += 0.005;
        smoothslider.y += 0.003;
        transform.translation.x = smoothslider.x.sin()*9.;
        transform.translation.y = smoothslider.y.sin()*3.;
    }
}
fn smooth_bg_movement (mut query: Query<(&mut SmoothSlider, &Widget)>, mut systems: Query<&mut Hierarchy>) {
    let mut system = systems.get_single_mut().unwrap();
    for (mut smoothslider, widget) in &mut query {
        
        let pos = widget.fetch_layout_mut(&mut system, "").unwrap().expect_window_mut();
        smoothslider.x += 0.007;
        smoothslider.y += 0.002;

        pos.relative.x = -5.0 + smoothslider.x.sin()*1.3*2.;  //25
        pos.relative.y = -5.0 + smoothslider.y.sin()*1.0*2.;   //16
    }
}
fn effect_update (mut query: Query<&mut BloomSettings>) {
    for mut bloom in &mut query {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) > 20 {break;}

        bloom.intensity += (rng.gen_range(0.25..0.32)-bloom.intensity)/10.;
        bloom.prefilter_settings.threshold += (rng.gen_range(0.2..0.35)-bloom.prefilter_settings.threshold)/10.;
    }
}
