use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use bevy_lunex::{prelude::*, Container};

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

    let goo = Rect {
        pos : Vec3::new(0.0, 0.0, 0.0),
        size: Vec2::new(5.0, 5.0),
    
        ..default()
    };

    commands.spawn(goo.into_bundle(&mut meshes, &mut materials));

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
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
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

    let _ = ui.add_data(5.0);
    //let dd = ui.obtain_data().unwrap();

    let _ = ui.insert_data("Node 3", 10.0).unwrap();
    let dd = ui.borrow_data("Node 3").unwrap().unwrap();

    println!("{:?}", dd);

    println!("{}", ui.tree_node("show-hidden"));

}

#[derive(Component)]
pub struct Player;
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform), Without<PlayerCam>>,
    camera: Query<&Transform, With<PlayerCam>>
) {
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
fn rotate_playercam(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&PlayerCam, &mut Transform)>,
) {
    let delta: Vec2 = mouse_motion_events.iter().map(|e| e.delta).sum();
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
fn zoom_playercam(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut PlayerCam>,
) {
    let delta: f32 = mouse_wheel_events.iter().map(|e| e.y).sum();
    for mut camera in &mut query {
        camera.distance += -delta;
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pos : Vec3,
    size: Vec2,

    pitch: f32, //Rotate around Y
    yaw: f32,   //Rotate around X
    roll: f32,  //Rotate around center
}
impl Rect {
    pub fn into_bundle(self, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> PbrBundle {
        let mut pp = self.pos;
        pp.x += self.size.x*0.5;
        pp.y += self.size.y*0.5*-1.0;
        PbrBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::default().with_translation(pp),
            mesh: meshes.add(shape::Quad {
                size: self.size,
                flip: false,
            }.into()),
            ..default()
        }
    }
}


/*
pub struct Interface {
    width: f32,
    height: f32,
    nested: InterfaceBox,
}

struct InterfaceBox {
    rect: Rect,
    modifier: (),   //Z, Tilt, Yaw, Roll, etc...
    layout: lui::Window,

    nested: Vec<InterfaceBox>
}
impl InterfaceBox {
    fn compute(&mut self, parent: &Rect) {

        //let pos = self.layout.pos.evaluate(40.0, parent.size);

    }
}*/

