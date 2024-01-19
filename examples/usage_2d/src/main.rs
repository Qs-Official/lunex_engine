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
        Camera2dBundle { transform: Transform { translation: Vec3::new(0.0, 0.0, 100.0), ..default() }, ..default() }
    ));

    cmd.spawn((
        MyWidget,
        init_ui().unwrap(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        //UiLogic::build(), // Needs direct link at UiTree
    ));

    // This entity needs to be spawn as child
    /*cmd.spawn((
        MyWidget,
        UiLink::path("window"),
        //UI::Window::FULL,
    ));*/
}

fn init_ui() -> Result<UiTree<NoData>, UiError> {

    // Create new DOM
    let mut ui = UiTree::<NoData>::new("UI");

    // Create the layout
    layout::Window::new().build(&mut ui, "Root")?;
    layout::Solid::new().with_align_x(Align::START).build(&mut ui, "Root/Node2")?;
    
    Ok(ui)
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;