use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn((
        MyWidget,
        Camera2dBundle::new_with_far(100.0)
    ));

    commands.spawn((
        UiTreeBundle::<NoData, MyWidget>::from( ui() ),
        MovableByCamera,
    )).with_children(|parent| {

        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            layout::Window::FULL.pack(),
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            layout::Solid::new().with_align_x(Align::START).pack(),
        ));

    });

}

fn ui() -> Result<UiTree<NoData>, UiError> {
    let ui = UiTree::<NoData>::new("UI");
    Ok(ui)
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;