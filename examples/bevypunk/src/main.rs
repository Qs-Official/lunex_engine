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
        .add_systems(PreStartup, prestartup)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetCache>, mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn((
        MyWidget,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    commands.spawn((
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
            
            //UiImage2dBundle::from(assets.main_background.clone()),
            UiMaterial3dBundle::from( materials.add(StandardMaterial { base_color_texture: Some(assets.main_background.clone()), unlit: true, ..default() }) ),
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;



#[derive(Resource)]
pub struct AssetCache {
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub button: Handle<Image>,

    pub switch_base: Handle<Image>,
    pub switch_head: Handle<Image>,

    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,
    pub settings_background: Handle<Image>,
}
fn prestartup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetCache {
        font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),
        button: asset_server.load("images/main_menu/button.png"),

        switch_base: asset_server.load("images/settings/switch_base.png"),
        switch_head: asset_server.load("images/settings/switch_head.png"),

        main_background: asset_server.load("images/main_menu/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),
        settings_background: asset_server.load("images/settings/background.png"),
    });
}
