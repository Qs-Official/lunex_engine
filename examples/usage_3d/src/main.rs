mod boilerplate;
use boilerplate::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData>::new())
        .add_systems(Startup, setup)
        .add_systems(Update, ui_compute::<NoData>)

        .add_systems(Update, move_player)
        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}
fn setup(
    mut cmd: Commands,
    mut msh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
) {
    // light
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 10.0).with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 30_f32.to_radians(), 0.0)),
        ..default()
    });

    // cube
    let player = cmd.spawn((
        PbrBundle {
            mesh: msh.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: mat.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player
    )).id();

    // camera
    let cam = cmd.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 300.0,
            sensitivity: Vec2::splat(0.1),
        }
    )).id();

    cmd.entity(player).push_children(&[cam]);


    // Spawn the DOM
    cmd.spawn((
        MyWidget,
        Transform::from_xyz(0.0, 50.0, 0.0),
        build_ui().unwrap(),
    ));

}

fn build_ui() -> Result<UiTree<NoData>, UiError> {

    // Create new DOM
    let mut ui = UiTree::<NoData>::new("UI_Widget");

    // Create the layout
    layout::Window::new().build(&mut ui, "Node1")?;
    layout::Solid::new().with_align_y(Align::LEFT).build(&mut ui, "Node1/Node2")?;

    // Print layout tree
    println!("\n{}\n", ui.tree(""));

    Ok(ui)
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;

fn ui_compute<T: Component + Default>(mut query: Query<&mut UiTree<T>>, time: Res<Time>) {
    for mut ui in &mut query {
        ui.compute(Rect2D::new().with_size((100.0 + time.elapsed_seconds().cos() * 30.0, 100.0)).into());
    }
}