mod boilerplate;
use boilerplate::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        //.add_plugins(UiDebugPlugin::<NoData, MyWidget>::new())

        .add_systems(Startup, setup)
        .add_systems(Update, ui_compute::<NoData>)

        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut msh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // light
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500000.0,
            range: 200000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 250.0, 250.0).with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 30_f32.to_radians(), 0.0)),
        ..default()
    });

    // cube
    let player = cmd.spawn(
        PbrBundle {
            //mesh: msh.add(Mesh::from(shape::Cube { size: 50.0 })),
            //material: mat.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        }
    ).id();

    // camera
    let cam = cmd.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 800.0,
            sensitivity: Vec2::splat(0.1),
        }
    )).id();

    //cmd.entity(cam).push_children(&[light]);
    cmd.entity(player).push_children(&[cam]);


    cmd.spawn((
        UiTreeBundle::<NoData, MyWidget> {
            transform: Transform::from_xyz(0.0, 300.0, 0.0),
            dimension: Dimension::new((400.0, 400.0)),
            tree: UiTree::<NoData>::new("MyWidget"),
            ..default()
        },

        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            Ui::Window::FULL.with_size(Abs((818.0, 965.0))).pack(),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

    });

    cmd.spawn((
        UiTreeBundle::<NoData, MyWidget> {
            transform: Transform::from_xyz(0.0, 300.0, 200.0),
            dimension: Dimension::new((400.0, 400.0)),
            tree: UiTree::<NoData>::new("MyWidget"),
            ..default()
        },

        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            Ui::Window::FULL.pack(),
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            Ui::Solid::new().with_size(Abs((818.0, 965.0))).pack(),
            //UiImage2dBundle::from(assets.load("board.png")),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

    });

    cmd.spawn((
        UiTreeBundle::<NoData, MyWidget> {
            transform: Transform::from_xyz(0.0, 300.0, 400.0),
            dimension: Dimension::new((400.0, 400.0)),
            tree: UiTree::<NoData>::new("MyWidget"),
            ..default()
        },

        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            Ui::Window::FULL.pack(),
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            Ui::Solid::new().with_size(Abs((818.0, 965.0))).pack(),
            //UiImage2dBundle::from(assets.load("board.png")),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

    });
}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;

fn ui_compute<T: Component + Default>(mut query: Query<&mut Dimension, (With<MyWidget>, With<UiTree<T>>)>, time: Res<Time>) {
    for mut dimension in &mut query {
        //dimension.size = (200.0 + time.elapsed_seconds().cos() * 60.0, 300.0 + time.elapsed_seconds().sin() * 50.0).into();
    }
}