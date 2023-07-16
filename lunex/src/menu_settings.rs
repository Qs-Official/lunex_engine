use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use crate::general::*;


pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    let settings = Widget::create(system, "settings", Box::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();


    //# BACKGROUND ===================================================================================================

    //# Create BACKGROUND in SETTINGS
    let background = Widget::create(system, &settings.end("background"), Box::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BACKGROUND
    let image = Widget::create(system, &background.end(""), Box::Solid {
        width: 3840,
        height: 2160,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    
    /*commands.spawn ((
        image.clone(),
        SpriteBundle {
            texture: asset_server.load("settings/background.png"),
            transform: Transform { ..default() },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));*/

    spawn_image(commands, asset_server, image.clone(), "settings/background.png");

    //# Set depth to IMAGE
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# ===============================================================================================================
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 2.0, y: 2.0 },
        relative_2: Vec2 { x: 10.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    let button_return = Widget::create(system, &boundary.end("return"), Box::Solid {
        width: 3,
        height: 1,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();

    let font = asset_server.load("Rajdhani/Rajdhani-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::rgb(204./255., 56./255., 51./255.),
    };
    spawn_text(commands, button_return, "RETURN", text_style.clone());




    let bar = Widget::create(system, &settings.end("bar"), Box::Relative {
        relative_1: Vec2 { x: 12.0, y: 2.0 },
        relative_2: Vec2 { x: 88.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    let boundary = Widget::create(system, &bar.end(""), Box::Solid {
        width: 28,
        height: 1,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();

    let map = [["sound"].to_vec(), ["controls"].to_vec(), ["video"].to_vec(), ["interface"].to_vec()].to_vec();
    
    Grid {
        width_relative: 100.0,
        height_relative: 20.0,
        width_padding_gap: true,
        gap_relative: Vec2::new(10.0, 0.0),
        ..Default::default()
    }.create_inside(system, &boundary, &map).unwrap();

    let xx = map.len();
    let yy = map[0].len();

    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::rgb(204./255., 56./255., 51./255.),
    };
    
    for x in 0..xx {
        for y in 0..yy {

            let widget = Widget::new(&boundary.end(map[x][y]));
            spawn_text(commands, widget, &map[x][y].to_uppercase(), text_style.clone());

        }
    }




    //# ===============================================================================================================
    let boundary1 = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 5.0, y: 14.0 },
        relative_2: Vec2 { x: 95.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    let boundary2 = Widget::create(system, &boundary1.end(""), Box::Solid {
        width: 105,
        height: 100,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();


    //println!("{}", &settings.add(&boundary1).add(&boundary2).end("Display"));

    let display = Widget::create(system, &settings.add(&boundary1).add(&boundary2).end("display"), Box::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.pack()).unwrap();


    let category = Widget::create(system, &display.end(""), Box::Solid {
        width: 1934,
        height: 96,
        vertical_anchor: -1.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    
    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::rgb(199./255., 186./255., 174./255.),
    };
    spawn_image_with_text(commands, asset_server, category, "settings/category.png", "Display", Vec2::new(30., -96./2.), text_style);


    let map = [["fullscreen","window_mode","resolution", "monitor", "vsync"].to_vec()].to_vec();

    let grid = Grid {
        width_relative: 96.0,
        height_relative: 11.0,
        width_padding_gap: true,
        height_padding_gap: true,
        gap_relative: Vec2::new(2.0, 2.0),
        ..Default::default()
    };
    let widget = grid.create(system, &display.end("list"), &map, Vec2::new(0.0, 16.0)).unwrap();

    let xx = map.len();
    let yy = map[0].len();

    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::rgb(204./255., 56./255., 51./255.),
    };
    
    for x in 0..xx {
        for y in 0..yy {

            let _widget = Widget::new(&widget.end(map[x][y]));
            spawn_text(commands, _widget, &map[x][y].to_uppercase(), text_style.clone());

        }
    }



}


