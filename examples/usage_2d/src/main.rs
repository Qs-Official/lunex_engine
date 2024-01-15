use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin::<NoData>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {

    cmd.spawn((
        Camera2dBundle { transform: Transform { translation: Vec3::new(0.0, 0.0, 100.0), ..default() }, ..default() },
        //UI::UINodeTree::<MyWidget, NoData>::new("UI"),
    ));

    cmd.spawn((
        //UI::Link::<MyWidget>::path("window"),
        //UI::Window::FULL,
    ));

}

pub struct MyWidget;