use ahash::AHashMap;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use bevy_lunex::{prelude::*, Container, Node};

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


    let mut ui = Interface::new("HUD");
    ui.create_node("Node 1").unwrap();
    ui.create_node("Node 1/.Node 2").unwrap();
    ui.create_node("Node 3").unwrap();


    let _ = ui.insert_data("Node 3", Container::new()).unwrap();
    let _dd = ui.borrow_data("Node 3").unwrap().unwrap();

    println!("{}", ui.tree("show-hidden"));
    ShadowNodeMap::build_set(&mut commands, ui, &mut meshes, &mut materials);

}


#[derive(Component)]
pub struct Player;
fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform), Without<PlayerCam>>, camera: Query<&Transform, With<PlayerCam>>) {
    let camera = camera.get_single().unwrap();
    let vector = camera.rotation.to_euler(EulerRot::YXZ);

    let xx = 0.05 * vector.0.sin();
    let yy = 0.05 * vector.0.cos();

    if keyboard_input.pressed(KeyCode::W) {
        for (_, mut transform) in &mut query {
            transform.translation.x += -xx;
            transform.translation.z += -yy;
        }
    }
    if keyboard_input.pressed(KeyCode::A) {
        for (_, mut transform) in &mut query {
            transform.translation.x += -yy;
            transform.translation.z += xx;
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for (_, mut transform) in &mut query {
            transform.translation.x += xx;
            transform.translation.z += yy;
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for (_, mut transform) in &mut query {
            transform.translation.x += yy;
            transform.translation.z += -xx;
        }
    }

    //transform.rotation = Quat::from_euler(EulerRot::YXZ, vector.0, 0.0, 0.0);
}

#[derive(Component)]
pub struct PlayerCam {
    orbit: Vec3,
    distance: f32,
    sensitivity: Vec2,
}
fn rotate_playercam(mut mouse_motion_events: EventReader<MouseMotion>, mut query: Query<(&PlayerCam, &mut Transform)>) {
    let delta: Vec2 = mouse_motion_events.read().map(|e| e.delta).sum();
    for (camera, mut transform) in &mut query {

        // ROTATION 
        let (mut rx, mut ry, rz) = transform.rotation.to_euler(EulerRot::YXZ);
        rx += (-delta.x * camera.sensitivity.x).to_radians();
        ry += (-delta.y * camera.sensitivity.x).to_radians();
        ry = ry.clamp(-90_f32.to_radians(), 90_f32.to_radians());
        transform.rotation = Quat::from_euler(EulerRot::YXZ, rx, ry, rz);


        // ORBIT TRANSFORM
        let tx = camera.distance * rx.sin();
        let ty = camera.distance * rx.cos();
        let tz = camera.distance * ry.sin();

        let diff = camera.distance * ry.cos();
        let plane_ratio_decrease = (camera.distance - diff)/camera.distance;

        transform.translation = camera.orbit;
        transform.translation.x += tx * (1.0 - plane_ratio_decrease);
        transform.translation.z += ty * (1.0 - plane_ratio_decrease);
        transform.translation.y += -tz;
    }
}
fn zoom_playercam(mut mouse_wheel_events: EventReader<MouseWheel>, mut query: Query<&mut PlayerCam>) {
    let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
    for mut camera in &mut query {
        camera.distance += -delta;
    }
}



#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension(pub Vec2);



#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNodeMap {
    id_map: AHashMap<String, Entity>
}
impl ShadowNodeMap {
    pub fn build_set(cmd: &mut Commands, ui: Interface, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
        let shadownode = cmd.spawn((

            msh.add(shape::Quad { size: Vec2::splat(4.0), flip: false }.into()),
            mat.add(Color::rgb(0.5, 1.0, 0.5).into()),

            ShadowNodeMap::default(),
            ShadowNode::default(),
            Dimension::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),

        )).id();
        for (_, node) in &ui.node.nodes {
            ShadowNode::build(cmd, node, shadownode, msh, mat);
        }
        //ShadowNode::default(); // Needs to be inserted as component to the realnode
    }
}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNode {}
impl ShadowNode {
    fn build(cmd: &mut Commands, ui: &Node<Container>, parent_id: Entity, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
        let shadownode = cmd.spawn((

            msh.add(shape::Quad { size: Vec2::splat(4.0), flip: false }.into()),
            mat.add(Color::rgb(0.5, 0.5, 0.5).into()),

            ShadowNode::default(),
            Dimension::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),

        )).id();
        cmd.entity(parent_id).push_children(&[shadownode]);
        for (_, node) in &ui.nodes {
            ShadowNode::build(cmd, node, shadownode, msh, mat);
        }
    }
}
