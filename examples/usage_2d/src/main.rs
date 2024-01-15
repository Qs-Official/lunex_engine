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
        MyUiWidget,
        UiTree::<NoData>::new("UI"),
    ));

    cmd.spawn((
        MyUiWidget,
        UiLink::path("window"),
        UI::Window::FULL,
    ));

}

pub struct MyUiWidget;