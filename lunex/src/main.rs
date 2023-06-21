mod library;
mod style;

use crate::library::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use style::*;

use bevy::core_pipeline::bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)

        .add_system(image_update)
        .add_system(cursor_update)
        .add_system(button_animation)
        .add_system(smooth_movement)
        .add_system(smooth_bg_movement)

        .add_system(effect_update)

        .add_system(Ui::hiearchy_update)
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
    let system = get_hiearchy();
    commands.spawn ((
        system,
    ));

    //SPAWN CURSOR
    commands.spawn ((
        CursorInfo {
            offset: 10.,
            ..default()
        },
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
        Ui::Widget {
            path: "App/Handle".to_string()
        },
        SmoothSlider {..Default::default()},
    ));

    //SPAWN BACKGROUND IMAGE
    commands.spawn ((
        Ui::Widget {
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
        Ui::Widget {
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
        Ui::Widget {
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
        Ui::Widget {
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
            Ui::Widget {
                path: "App/Board/ButtonList/".to_string() + button_list[i]
            },
            ImageInfo {
                width: 532.,
                height: 75.,
            },
            MainMenuButton{},
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


fn image_update(mut systems: Query<&mut Ui::Hiearchy>, mut query: Query<(&mut Ui::Widget, &ImageInfo, &mut Transform)>) {

    let mut system = systems.get_single_mut().unwrap();     //Unwrap the hiearchy struct

    for (widget, imageinfo, mut transform) in &mut query {

        let dimensions = (system.width, system.height);
        let pos = widget.position(&mut system).unwrap();
        transform.translation.x = pos.point_1[0] - dimensions.0/2.;
        transform.translation.y = pos.point_2[1] - dimensions.1/2.;

        transform.scale.x = pos.width/imageinfo.width;
        transform.scale.y = pos.height/imageinfo.height;
    }
}

#[derive(Component, Default)]
struct CursorInfo {
    depth: f32,
    offset: f32,
    cursor_world: Vec2,
    cursor_screen: Vec2,
}
fn cursor_update(mut windows: Query<&mut Window>, mut query: Query<(&mut CursorInfo, &mut Transform)>) {
    for (mut cursorinfo, mut transform) in &mut query {
        let mut window = windows.get_single_mut().unwrap();

        match window.cursor_position() {
            Some (cursor) => {
                window.cursor.visible = false;

                let offset_x = window.resolution.width()/2. + cursorinfo.offset*transform.scale.x;
                let offset_y = window.resolution.height()/2. - cursorinfo.offset*transform.scale.y;

                cursorinfo.cursor_screen = Vec2 {x: cursor.x, y: cursor.y};
                cursorinfo.cursor_world = Vec2 {x: cursor.x - offset_x, y: cursor.y - offset_y};

                transform.translation.x = cursorinfo.cursor_world.x;
                transform.translation.y = cursorinfo.cursor_world.y;

            },
            None => {
                transform.translation.x = -window.resolution.width();
                transform.translation.y = -window.resolution.height();
            }
        }
    }
}

#[derive(Component)]
struct MainMenuButton {}
fn button_animation(systems: Query<&Ui::Hiearchy>, mut query: Query<(&mut Ui::Widget, &mut Sprite, &mut Transform, &MainMenuButton)>, cursor_query: Query<&CursorInfo>) {
    let system = systems.get_single().unwrap();
    for cursor in &cursor_query {
        for (widget, mut sprite, mut transform, _) in &mut query {

            if widget.is_within(&system, &cursor.cursor_screen).unwrap(){
                
                sprite.color.set_a(0.4);
                transform.translation.x += 5.

            } else {

                let alpha = sprite.color.a();
                if alpha > 0.0 {sprite.color.set_a(alpha - 0.01);} else {sprite.color.set_a(0.0);}
                
            }
        }
        break;
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
fn smooth_bg_movement (mut query: Query<(&mut SmoothSlider, &mut Ui::Widget)>, mut systems: Query<&mut Ui::Hiearchy>) {
    let mut system = systems.get_single_mut().unwrap();
    for (mut smoothslider, mut widget) in &mut query {
        smoothslider.x += 0.01;
        smoothslider.y += 0.005;
        
        let pos = widget.position_borrow_window_mut(&mut system, "default", "").unwrap();
        pos.absolute.x = smoothslider.x.sin()*25.;
        pos.absolute.y = smoothslider.y.sin()*16.;
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
