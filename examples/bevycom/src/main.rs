mod boilerplate;
use boilerplate::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, MyWidget>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())

        .add_systems(Startup, setup)

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
        UiTreeBundle::<NoData, NoData, MyWidget> {
            transform: Transform::from_xyz(0.0, 300.0, 0.0),
            tree: UiTree::new("MyWidget"),
            ..default()
        },
        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            Ui::Window::FULL.size(Abs((818.0, 965.0))).pack(),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

        let head = root.add("Head");
        parent.spawn((
            MyWidget,
            head.clone(),
            Ui::Window::new().width(Prc::FULL).height(Abs(168.0)).pack(),
        ));

        parent.spawn((
            MyWidget,
            head.add("Div"),
            Ui::Div::new().size(Abs::MD).pad(Abs(10.0)).mar(Abs(5.0)).pack(),
        ));

    });

    cmd.spawn((
        UiTreeBundle::<NoData, NoData, MyWidget> {
            transform: Transform::from_xyz(-300.0, 300.0, 200.0),
            tree: UiTree::new("MyWidget"),
            ..default()
        },
        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            Ui::Window::FULL.size(Abs((818.0, 965.0))).pack(),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

        let head = root.add("Head");
        parent.spawn((
            MyWidget,
            head.clone(),
            Ui::Window::new().width(Prc::FULL).height(Abs(168.0)).pack(),
        ));

        parent.spawn((
            MyWidget,
            head.add("Div"),
            Ui::Div::new().size(Abs::MD).pad(Abs(10.0)).mar(Abs(5.0)).pack(),
        ));

    });

    cmd.spawn((
        UiTreeBundle::<NoData, NoData, MyWidget> {
            transform: Transform::from_xyz(-600.0, 300.0, 400.0),
            tree: UiTree::new("MyWidget"),
            ..default()
        },
        msh.add(Mesh::from(shape::Cube { size: 15.0 })),
        mat.add(Color::rgb(1.0, 0.0, 1.0).into()),
        Visibility::default(),
        ViewVisibility::default(),

    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            Ui::Window::FULL.size(Abs((818.0, 965.0))).pack(),
            UiMaterial3dBundle::from( mat.add(StandardMaterial { base_color_texture: Some(assets.load("bevycom.png")), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }) ),
        ));

        let head = root.add("Head");
        parent.spawn((
            MyWidget,
            head.clone(),
            Ui::Window::new().width(Prc::FULL).height(Abs(168.0)).pack(),
        ));

        parent.spawn((
            MyWidget,
            head.add("Div"),
            Ui::Div::new().size(Abs::MD).pad(Abs(10.0)).mar(Abs(5.0)).pack(),
        ));

    });
}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;
