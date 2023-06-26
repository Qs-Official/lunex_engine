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
        .add_startup_system(setup)
        .add_startup_system(setup_main_menu)

        .add_plugin(ButtonPlugin)
        .add_plugin(WigglePlugin)
        .add_system(vfx_bloom_update)

        .add_system(hierarchy_update)
        .add_system(cursor_update)
        .add_system(image_update)

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {

    //# Start playing the main menu music
    let music = asset_server.load("main_menu.ogg");
    audio.play_with_settings(music, PlaybackSettings { repeat: true, volume: 1., speed: 1. });

    //# Spawn the camera
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
        SmoothWiggle {..Default::default()},
    ));

    //# Spawn cursor
    commands.spawn ((
        Cursor::new(10.0),
        SpriteBundle {
            texture: asset_server.load("cursor_mouse.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 200. } , scale: Vec3 { x: 0.4, y: 0.4, z: 1. }, ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 2.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));
    
}



//################################################################################
//# == Bloom Update ==
//# Just a quick system to randomly change bloom threshold (smoothly)
//# It adds another dynamic layer to static camera
fn vfx_bloom_update (mut query: Query<&mut BloomSettings>) {
    for mut bloom in &mut query {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) > 20 {break;}

        bloom.intensity += (rng.gen_range(0.25..0.32)-bloom.intensity)/10.;
        bloom.prefilter_settings.threshold += (rng.gen_range(0.2..0.35)-bloom.prefilter_settings.threshold)/10.;
    }
}