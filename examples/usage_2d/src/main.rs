use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {

    cmd.spawn((
        MyWidget,
        Camera2dBundle::new_with_far(100.0)
    ));

    cmd.spawn((
        UiTreeBundle::<NoData, MyWidget>::from( ui() ),
        MovableByCamera
        //UiLogic::build(), // Needs direct link at UiTree
    ));

    // This entity needs to be spawn as child
    cmd.spawn((
        MyWidget,
        UiLink::path("Root"),
        layout::Window::FULL.pack(),
    ));
}

fn ui() -> Result<UiTree<NoData>, UiError> {
    let mut ui = UiTree::<NoData>::new("UI");
    layout::Window::new().build(&mut ui, "Root")?;
    layout::Solid::new().with_align_x(Align::START).build(&mut ui, "Root/Node2")?;
    Ok(ui)
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;