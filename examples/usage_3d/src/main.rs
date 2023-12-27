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

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    /*commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(15.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });*/


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


    let mut ui: UINodeTree<()> = UINodeTree::new("HUD");

    lui::Window::FULL.build(&mut ui, "Node1").unwrap();


    //ui.insert_data("Node1", Container::new()).unwrap();

    println!("{}", ui.tree("show-hidden"));
    //ShadowNodeTree::build_set(&mut commands, ui, &mut meshes, &mut materials);

}

/*pub fn build_ui() -> Result<(), LunexError> {
    Ok(())
}*/