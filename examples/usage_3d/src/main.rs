mod boilerplate;
use boilerplate::*;

use bevy::prelude::*;
use bevy_lunex::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 10.0).with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 30_f32.to_radians(), 0.0)),
        ..default()
    });

    // cube
    let player = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player
    )).id();

    // camera
    let cam = commands.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 7.5,
            sensitivity: Vec2::splat(0.1),
        }
    )).id();

    commands.entity(player).push_children(&[cam]);

    build_ui().unwrap();
}

fn build_ui() -> Result<(), LunexError> {

    let mut ui: UiTree<()> = UiTree::new("HUD");

    layout::Window::FULL.build(&mut ui, "Node1")?;

    layout::Window::FULL.with_x( Abs::SM + Rem::XL * 2.0 ).build(&mut ui, "Node2")?;

    layout::Window::EMPTY
        .with_size(Abs::splat2(15.) + Rem::splat2(5.))
        .build(&mut ui, "Node1/Node3")?;

    ui.compute(Rect2D::new().with_size((100.0, 100.0)).into());

    println!("\n{}\n", ui.tree("show-hidden"));

    Ok(())
}