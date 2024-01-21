use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        .add_plugins(UiDebugPlugin::<NoData, MyWidget>::new())

        //.add_plugins(Shape2dPlugin::default())
        //.add_systems(Update, render_update)
        //.add_systems(PreStartup, presetup)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, mut mat: ResMut<Assets<StandardMaterial>>, ast: Res<AssetServer>,) {

    cmd.spawn((
        MyWidget,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    cmd.spawn((
        UiTreeBundle::<NoData, MyWidget> {
            tree: UiTree::<NoData>::new("MyWidget"),
            dimension: Dimension::new((1000.0, 1000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MovableByCamera,
    )).with_children(|parent| {

        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            Ui::Window::FULL.with_pos( Abs::splat2(20.0) ).with_size( Prc::splat2(100.0) - Abs::splat2(40.0) ).pack(),
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            Ui::Solid::new().with_size(Abs((1920.0, 1080.0))).pack(),
            
            UiImageBundle::from(ast.load("images/main_menu/background.png")),
            //UiMaterialBundle::from( mat.add(StandardMaterial { base_color_texture: Some(ast.load("images/main_menu/background.png")), unlit: true, ..default() }) ),
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;
