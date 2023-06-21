mod library;
mod style;

use crate::library::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use style::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)

        .add_system(image_update)
        .add_system(cursor_update)
        .add_system(button_animation)

        .add_system(Ui::hiearchy_update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //SPAWN 2D CAMERA
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 { x: -200., y: -200., z: 1000. },
            ..default()
        },
        ..default()
    });


    //SPAWN UI HIEARCHY
    let system = get_hiearchy();
    commands.spawn ((
        system,
    ));

    //SPAWN CURSOR
    commands.spawn ((
        CursorInfo {
            offset: 10.,
            camera_offset_x: 0.,
            camera_offset_y: 0.,
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

    //SPAWN BACKGROUND IMAGE
    commands.spawn ((
        Ui::Widget {
            path: "App/Background".to_string()
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

    //SPAWN LOGO IMAGE
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
    let mut system = systems.get_single_mut().unwrap();
    for (widget, imageinfo, mut transform) in &mut query {
        let dimensions = (system.width, system.height);
        let pos = widget.position(&mut system).unwrap();
        transform.translation.x = pos.point_1[0] - dimensions.0/2.;
        transform.translation.y = pos.point_2[1] - dimensions.1/2.;

        transform.scale.x = pos.width/imageinfo.width;
        transform.scale.y = pos.height/imageinfo.height;
    }
}

#[derive(Component)]
struct CursorInfo {
    offset: f32,
    camera_offset_x:f32,
    camera_offset_y: f32,
}
fn cursor_update(mut windows: Query<&mut Window>, mut query: Query<(&mut CursorInfo, &mut Transform)>) {
    for (mut cursorinfo, mut transform) in &mut query {
        let mut window = windows.get_single_mut().unwrap();

        match window.cursor_position() {
            Some (cursor) => {
                window.cursor.visible = false;
                cursorinfo.camera_offset_x = window.resolution.width()/2. + cursorinfo.offset*transform.scale.x;
                cursorinfo.camera_offset_y = window.resolution.height()/2. - cursorinfo.offset*transform.scale.y;

                transform.translation.x = cursor.x - cursorinfo.camera_offset_x;
                transform.translation.y = cursor.y - cursorinfo.camera_offset_y;
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
fn button_animation(mut systems: Query<&mut Ui::Hiearchy>, mut query: Query<(&mut Ui::Widget, &mut Sprite, &ImageInfo, &MainMenuButton)>, cursor: Query<(&CursorInfo, &Transform)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (cursorinfo, transform) in &cursor {
        let cursor_x = transform.translation.x + cursorinfo.camera_offset_x;
        let cursor_y = transform.translation.y + cursorinfo.camera_offset_y;

        for (widget, mut sprite, _, _) in &mut query {
            let pos = widget.position(&mut system).unwrap();
            if (cursor_x > pos.point_1[0] && cursor_x < pos.point_2[0]) && (cursor_y > pos.point_1[1] && cursor_y < pos.point_2[1]){
                sprite.color.set_a(0.4);
            } else {
                let alpha = sprite.color.a();
                if alpha > 0.0 {
                    sprite.color.set_a(alpha - 0.01);
                }
            }
        }
        
        break;
    }
}
