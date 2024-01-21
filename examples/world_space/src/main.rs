mod boilerplate;
use boilerplate::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_plugins(UiPlugin::<NoData, HUD>::new())
        .add_plugins(UiDebugPlugin::<NoData, HUD>::new())

        .add_systems(Startup, setup)
        .add_plugins(ShapePlugin::default())
        .add_systems(Update, render_update)

        .add_systems(Update, move_player)
        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut msh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
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
    let player = cmd.spawn((
        PbrBundle {
            mesh: msh.add(Mesh::from(shape::Cube { size: 50.0 })),
            material: mat.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        //Player
    )).id();

    // camera
    let cam = cmd.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 800.0,
            sensitivity: Vec2::splat(0.1),
        },
        HUD
    )).id();

    //cmd.entity(cam).push_children(&[light]);
    cmd.entity(player).push_children(&[cam]);



    cmd.spawn((
        UiTreeBundle::<NoData, HUD> {
            tree: UiTree::<NoData>::new("MyWidget"),
            dimension: Dimension::new((1000.0, 1000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MovableByCamera,
    )).with_children(|parent| {

        parent.spawn((
            HUD,
            UiLink::path("Root"),
            Ui::Window::FULL.with_pos( Abs::splat2(20.0) ).with_size( Prc::splat2(100.0) - Abs::splat2(40.0) ).pack(),
        ));

        parent.spawn((
            HUD,
            UiLink::path("Root/Square"),
            Ui::Solid::new().with_size(Abs((1920.0, 1080.0))).pack(),
            UiMaterialBundle::from( mat.add(StandardMaterial { base_color_texture: Some(ass.load("image.png")), unlit: true, ..default() }) ),
        ));

    });
}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct HUD;


#[derive(Component)]
struct RenderContainer {
    color: Color,
    corner_radii: Vec4
}
fn render_update (mut painter: ShapePainter, query: Query<(&Dimension, &RenderContainer)>) {
    for (dimension, color) in &query {

        //painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        painter.color = color.color;
        painter.thickness = 1.0;
        painter.corner_radii = color.corner_radii;
        painter.rect(Vec2::new(dimension.size.x, dimension.size.y));
    }
}