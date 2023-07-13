use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

//use crate::general::*;


pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    let settings = Widget::create(system, "settings", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();


    //# BACKGROUND ===================================================================================================

    //# Create BACKGROUND in SETTINGS
    let background = Widget::create(system, &settings.end("background"), Layout::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create 'nameless' widget in BACKGROUND
    let image = Widget::create(system, &background.end(""), Layout::Solid {
        width: 3840,
        height: 2160,
        scaling: Scale::Fill,
        ..Default::default()
    }.wrap()).unwrap();
    
    commands.spawn ((
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
    ));

    //# Set depth to IMAGE
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# ===============================================================================================================
    let boundary = Widget::create(system, &settings.end(""), Layout::Relative {
        relative_1: Vec2 { x: 2.0, y: 2.0 },
        relative_2: Vec2 { x: 10.0, y: 8.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let button_return = Widget::create(system, &boundary.end("return"), Layout::Solid {
        width: 3,
        height: 1,
        scaling: Scale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.wrap()).unwrap();




    let bar = Widget::create(system, &settings.end("bar"), Layout::Relative {
        relative_1: Vec2 { x: 12.0, y: 2.0 },
        relative_2: Vec2 { x: 88.0, y: 8.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let boundary = Widget::create(system, &bar.end(""), Layout::Solid {
        width: 28,
        height: 1,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    let grid = [["sound"].to_vec(), ["controls"].to_vec(), ["video"].to_vec(), ["interface"].to_vec()].to_vec();
    Widget::generate_grid_inside(system, &boundary, &grid, &WidgetListStyle {
        width_relative: 100.0,
        height_relative: 20.0,
        width_padding_gap: true,
        gap_relative: Vec2::new(10.0, 0.0),
        ..Default::default()
    }).unwrap();

    let xx = grid.len();
    let yy = grid[0].len();

    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::rgb(204./255., 56./255., 51./255.),
    };
    
    for x in 0..xx {
        for y in 0..yy{
            let _container = Widget::new(&boundary.end(grid[x][y]));
            commands.spawn (
                ElementBundle {
                    widget: _container,
                    element: Element {
                        relative: Vec2::new(50.0, 50.0),
                        size: Vec2::new(50.0, 50.0),
                        ..default()
                    },
                    ..Default::default()
                }
            ).with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text::from_section(grid[x][y].to_uppercase(), text_style.clone()).with_alignment(TextAlignment::Center),
                    transform: Transform { translation: Vec3 { x: 0., y: 0., z: 15. }, ..default() },
                    ..default()
                });
            });
        }
    }




    //# ===============================================================================================================
    let boundary1 = Widget::create(system, &settings.end(""), Layout::Relative {
        relative_1: Vec2 { x: 5.0, y: 14.0 },
        relative_2: Vec2 { x: 95.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let boundary2 = Widget::create(system, &boundary1.end(""), Layout::Solid {
        width: 105,
        height: 100,
        scaling: Scale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.wrap()).unwrap();


    //println!("{}", &settings.add(&boundary1).add(&boundary2).end("Display"));

    let display = Widget::create(system, &settings.add(&boundary1).add(&boundary2).end("display"), Layout::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.wrap()).unwrap();

    let category = Widget::create(system, &display.end(""), Layout::Solid {
        width: 1934,
        height: 96,
        vertical_anchor: -1.0,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();
    
    let font = asset_server.load("Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::rgb(199./255., 186./255., 174./255.),
    };
    commands.spawn ((
        category,
        SpriteBundle {
            texture: asset_server.load("settings/category.png"),
            transform: Transform {
                ..default()
            },
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    )).with_children(|builder| {
        builder.spawn(Text2dBundle {
            text: Text::from_section("Display", text_style.clone()).with_alignment(TextAlignment::Left),
            transform: Transform { translation: Vec3 { x: 30., y: -96./2., z: 15. }, ..default() },
            text_anchor: Anchor::CenterLeft,
            ..default()
        });
    });


    let grid = [["fullscreen","window_mode","resolution", "monitor", "vsync"].to_vec()].to_vec();
    Widget::generate_grid(system, &display.end("List"), &grid, Vec2::new(0.0, 16.0), &WidgetListStyle {
        width_relative: 96.0,
        height_relative: 11.0,
        width_padding_gap: true,
        height_padding_gap: true,
        gap_relative: Vec2::new(2.0, 2.0),
        ..Default::default()
    }).unwrap();





}
