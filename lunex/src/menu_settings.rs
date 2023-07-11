use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

//use crate::general::*;


pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    let _settings = Widget::new(system, "Settings", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();


    //# BACKGROUND ===================================================================================================
    let _handle = Widget::new_in(system, &_settings, "Handle", Layout::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.wrap()).unwrap();

    let _background = Widget::new_in(system, &_handle, "Background", Layout::Solid {
        width: 3840,
        height: 2160,
        scaling: Scale::Fill,
        ..Default::default()
    }.wrap()).unwrap();
    commands.spawn ((
        _background.clone(),
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
    _background.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# ===============================================================================================================
    let _boundary = Widget::new_in(system, &_settings, "Boundary", Layout::Relative {
        relative_1: Vec2 { x: 2.0, y: 2.0 },
        relative_2: Vec2 { x: 10.0, y: 8.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let _return = Widget::new_in(system, &_boundary, "Return", Layout::Solid {
        width: 3,
        height: 1,
        scaling: Scale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.wrap()).unwrap();

    let _bar = Widget::new_in(system, &_settings, "Bar", Layout::Relative {
        relative_1: Vec2 { x: 12.0, y: 2.0 },
        relative_2: Vec2 { x: 88.0, y: 8.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let _boundary = Widget::new_in(system, &_bar, "", Layout::Solid {
        width: 28,
        height: 1,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();


    let grid = [["Sound"].to_vec(), ["Controls"].to_vec(), ["Video"].to_vec(), ["Interface"].to_vec()].to_vec();
    Widget::generate_grid_inside(system, &_boundary, "List", &grid, &WidgetListStyle {
        width_relative: 100.0,
        height_relative: 20.0,
        width_padding_gap: true,
        gap_relative: Vec2::new(2.0, 0.0),
        ..Default::default()
    }).unwrap();



    //# ===============================================================================================================
    let _boundary = Widget::new_in(system, &_settings, "", Layout::Relative {
        relative_1: Vec2 { x: 5.0, y: 14.0 },
        relative_2: Vec2 { x: 95.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();

    let _boundary2 = Widget::new_in(system, &_boundary, "", Layout::Solid {
        width: 105,
        height: 100,
        scaling: Scale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.wrap()).unwrap();

    let _display = Widget::new_in(system, &_settings, "#p0/#p0/Display", Layout::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.wrap()).unwrap();

    let _category = Widget::new_in(system, &_display, "Category", Layout::Solid {
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
        _category,
        SpriteBundle {
            texture: asset_server.load("settings/category.png"),
            transform: Transform { 
                //translation: Vec3::new(0.0, 0.0, 0.0),
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


    let grid = [["Fullscreen","Window_Mode","Resolution", "Monitor", "VSync"].to_vec()].to_vec();
    Widget::generate_grid(system, &_display, "List", &grid, Vec2::new(0.0, 16.0), &WidgetListStyle {
        width_relative: 96.0,
        height_relative: 11.0,
        width_padding_gap: true,
        height_padding_gap: true,
        gap_relative: Vec2::new(2.0, 2.0),
        ..Default::default()
    }).unwrap();





}
