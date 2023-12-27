use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub struct Player;
pub fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform), Without<PlayerCam>>, camera: Query<&Transform, With<PlayerCam>>) {
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
    pub orbit: Vec3,
    pub distance: f32,
    pub sensitivity: Vec2,
}
pub fn rotate_playercam(mut mouse_motion_events: EventReader<MouseMotion>, mut query: Query<(&PlayerCam, &mut Transform)>) {
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
pub fn zoom_playercam(mut mouse_wheel_events: EventReader<MouseWheel>, mut query: Query<&mut PlayerCam>) {
    let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
    for mut camera in &mut query {
        camera.distance += -delta;
    }
}
