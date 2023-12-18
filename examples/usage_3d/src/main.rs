use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use lunex_core::Amount;

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

pub struct Interface {
    width: f32,
    height: f32,
    nested: InterfaceBox,
}

struct InterfaceBox {
    rect: Rect,
    modifier: (),   //Z, Tilt, Yaw, Roll, etc...
    layout: declarative::Window,

    nested: Vec<InterfaceBox>
}
impl InterfaceBox {
    fn compute(&mut self, parent: &Rect) {

        let pos = self.layout.pos.evaluate(40.0, parent.size);

    }
}





pub mod interactive {
    pub struct Button;
    pub struct Toggle;
    pub struct Selector;
    pub struct RangeSlider;
    pub struct Radio;
}

pub mod predefined {
    pub struct Badge;
    pub struct Chip;
    pub struct Avatar;
    pub struct ProgressBar;
    pub struct ProgressRadial;
    pub struct Tabs;
    pub struct Popup;
    pub struct Modal;
    pub struct Toast;
    pub struct Drawer;
}

pub mod typographic {
    use crate::ColorOption;

    pub struct H1;
    pub struct H2;
    pub struct H3;
    pub struct H4;
    pub struct H5;
    pub struct H6;

    /// # Pararaph
    pub struct P;
    /// # Italic
    pub struct I;
    /// # Bold
    pub struct B;


    pub enum Size {
        XS,
        SM,
        MD,
        LG,
        XL,
        XL2,
        XL3,
        XL4,
        XL5,
        XL6,
        XL7,
        Custom(f32)
    }


    pub struct Opacity(pub f32);
    pub struct FontColor(pub ColorOption);
    pub enum Rounded {
        None,
        XS,
        SM,
        MD,
        LG,
        XL,
        XL2,
        XL3,
        XL4,
        XL5,
        XL6,
        XL7,
        Full,
        Custom(f32)
    }

}


pub struct StyleColor {
    pub color: Color,
}
impl StyleColor {
    pub fn v(self, l: f32) -> Color {
        self.color.with_l((1000.0 - l)/1000.0)
    }
    pub fn v50(self) -> Color {
        self.color.with_l(0.95)
    }
    pub fn v100(self) -> Color {
        self.color.with_l(0.9)
    }
    pub fn v200(self) -> Color {
        self.color.with_l(0.8)
    }
    pub fn v300(self) -> Color {
        self.color.with_l(0.7)
    }
    pub fn v400(self) -> Color {
        self.color.with_l(0.6)
    }
    pub fn v500(self) -> Color {
        self.color.with_l(0.5)
    }
    pub fn v600(self) -> Color {
        self.color.with_l(0.4)
    }
    pub fn v700(self) -> Color {
        self.color.with_l(0.3)
    }
    pub fn v800(self) -> Color {
        self.color.with_l(0.2)
    }
    pub fn v900(self) -> Color {
        self.color.with_l(0.1)
    }
}

pub struct StyleTheme {
    primary   : StyleColor,
    secondary : StyleColor,
    tertiary  : StyleColor,
    quaternery: StyleColor,
    info   : StyleColor,
    warning: StyleColor,
    success: StyleColor,
    error  : StyleColor,
    surface: StyleColor,

    rounding_container: f32,
    rounding_base: f32,
    border_width: f32,
    border_color: ColorOption,
}





pub enum ColorOption {
    Primary(f32),
    Secondary(f32),
    Tertiary(f32),
    Quaternery(f32),
    Info(f32),
    Warning(f32),
    Success(f32),
    Error(f32),
    Surface(f32),
    Neutral(f32),
}
impl ColorOption {
    pub const PRIMARY_50: ColorOption = ColorOption::Primary(50.0);
    pub const PRIMARY_100: ColorOption = ColorOption::Primary(100.0);
    pub const PRIMARY_200: ColorOption = ColorOption::Primary(200.0);
    pub const PRIMARY_300: ColorOption = ColorOption::Primary(300.0);
    pub const PRIMARY_400: ColorOption = ColorOption::Primary(400.0);
    pub const PRIMARY_500: ColorOption = ColorOption::Primary(500.0);
    pub const PRIMARY_600: ColorOption = ColorOption::Primary(600.0);
    pub const PRIMARY_700: ColorOption = ColorOption::Primary(700.0);
    pub const PRIMARY_800: ColorOption = ColorOption::Primary(800.0);
    pub const PRIMARY_900: ColorOption = ColorOption::Primary(900.0);

    pub const SECONDARY_50: ColorOption = ColorOption::Secondary(50.0);
    pub const SECONDARY_100: ColorOption = ColorOption::Secondary(100.0);
    pub const SECONDARY_200: ColorOption = ColorOption::Secondary(200.0);
    pub const SECONDARY_300: ColorOption = ColorOption::Secondary(300.0);
    pub const SECONDARY_400: ColorOption = ColorOption::Secondary(400.0);
    pub const SECONDARY_500: ColorOption = ColorOption::Secondary(500.0);
    pub const SECONDARY_600: ColorOption = ColorOption::Secondary(600.0);
    pub const SECONDARY_700: ColorOption = ColorOption::Secondary(700.0);
    pub const SECONDARY_800: ColorOption = ColorOption::Secondary(800.0);
    pub const SECONDARY_900: ColorOption = ColorOption::Secondary(900.0);

    pub const TERTIARY_50: ColorOption = ColorOption::Tertiary(50.0);
    pub const TERTIARY_100: ColorOption = ColorOption::Tertiary(100.0);
    pub const TERTIARY_200: ColorOption = ColorOption::Tertiary(200.0);
    pub const TERTIARY_300: ColorOption = ColorOption::Tertiary(300.0);
    pub const TERTIARY_400: ColorOption = ColorOption::Tertiary(400.0);
    pub const TERTIARY_500: ColorOption = ColorOption::Tertiary(500.0);
    pub const TERTIARY_600: ColorOption = ColorOption::Tertiary(600.0);
    pub const TERTIARY_700: ColorOption = ColorOption::Tertiary(700.0);
    pub const TERTIARY_800: ColorOption = ColorOption::Tertiary(800.0);
    pub const TERTIARY_900: ColorOption = ColorOption::Tertiary(900.0);

    pub const QUATERNARY_50: ColorOption = ColorOption::Quaternery(50.0);
    pub const QUATERNARY_100: ColorOption = ColorOption::Quaternery(100.0);
    pub const QUATERNARY_200: ColorOption = ColorOption::Quaternery(200.0);
    pub const QUATERNARY_300: ColorOption = ColorOption::Quaternery(300.0);
    pub const QUATERNARY_400: ColorOption = ColorOption::Quaternery(400.0);
    pub const QUATERNARY_500: ColorOption = ColorOption::Quaternery(500.0);
    pub const QUATERNARY_600: ColorOption = ColorOption::Quaternery(600.0);
    pub const QUATERNARY_700: ColorOption = ColorOption::Quaternery(700.0);
    pub const QUATERNARY_800: ColorOption = ColorOption::Quaternery(800.0);
    pub const QUATERNARY_900: ColorOption = ColorOption::Quaternery(900.0);

    pub const INFO_50: ColorOption = ColorOption::Info(50.0);
    pub const INFO_100: ColorOption = ColorOption::Info(100.0);
    pub const INFO_200: ColorOption = ColorOption::Info(200.0);
    pub const INFO_300: ColorOption = ColorOption::Info(300.0);
    pub const INFO_400: ColorOption = ColorOption::Info(400.0);
    pub const INFO_500: ColorOption = ColorOption::Info(500.0);
    pub const INFO_600: ColorOption = ColorOption::Info(600.0);
    pub const INFO_700: ColorOption = ColorOption::Info(700.0);
    pub const INFO_800: ColorOption = ColorOption::Info(800.0);
    pub const INFO_900: ColorOption = ColorOption::Info(900.0);

    pub const WARNING_50: ColorOption = ColorOption::Warning(50.0);
    pub const WARNING_100: ColorOption = ColorOption::Warning(100.0);
    pub const WARNING_200: ColorOption = ColorOption::Warning(200.0);
    pub const WARNING_300: ColorOption = ColorOption::Warning(300.0);
    pub const WARNING_400: ColorOption = ColorOption::Warning(400.0);
    pub const WARNING_500: ColorOption = ColorOption::Warning(500.0);
    pub const WARNING_600: ColorOption = ColorOption::Warning(600.0);
    pub const WARNING_700: ColorOption = ColorOption::Warning(700.0);
    pub const WARNING_800: ColorOption = ColorOption::Warning(800.0);
    pub const WARNING_900: ColorOption = ColorOption::Warning(900.0);

    pub const SUCCESS_50: ColorOption = ColorOption::Success(50.0);
    pub const SUCCESS_100: ColorOption = ColorOption::Success(100.0);
    pub const SUCCESS_200: ColorOption = ColorOption::Success(200.0);
    pub const SUCCESS_300: ColorOption = ColorOption::Success(300.0);
    pub const SUCCESS_400: ColorOption = ColorOption::Success(400.0);
    pub const SUCCESS_500: ColorOption = ColorOption::Success(500.0);
    pub const SUCCESS_600: ColorOption = ColorOption::Success(600.0);
    pub const SUCCESS_700: ColorOption = ColorOption::Success(700.0);
    pub const SUCCESS_800: ColorOption = ColorOption::Success(800.0);
    pub const SUCCESS_900: ColorOption = ColorOption::Success(900.0);

    pub const ERROR_50: ColorOption = ColorOption::Error(50.0);
    pub const ERROR_100: ColorOption = ColorOption::Error(100.0);
    pub const ERROR_200: ColorOption = ColorOption::Error(200.0);
    pub const ERROR_300: ColorOption = ColorOption::Error(300.0);
    pub const ERROR_400: ColorOption = ColorOption::Error(400.0);
    pub const ERROR_500: ColorOption = ColorOption::Error(500.0);
    pub const ERROR_600: ColorOption = ColorOption::Error(600.0);
    pub const ERROR_700: ColorOption = ColorOption::Error(700.0);
    pub const ERROR_800: ColorOption = ColorOption::Error(800.0);
    pub const ERROR_900: ColorOption = ColorOption::Error(900.0);

    pub const SURFACE_50: ColorOption = ColorOption::Surface(50.0);
    pub const SURFACE_100: ColorOption = ColorOption::Surface(100.0);
    pub const SURFACE_200: ColorOption = ColorOption::Surface(200.0);
    pub const SURFACE_300: ColorOption = ColorOption::Surface(300.0);
    pub const SURFACE_400: ColorOption = ColorOption::Surface(400.0);
    pub const SURFACE_500: ColorOption = ColorOption::Surface(500.0);
    pub const SURFACE_600: ColorOption = ColorOption::Surface(600.0);
    pub const SURFACE_700: ColorOption = ColorOption::Surface(700.0);
    pub const SURFACE_800: ColorOption = ColorOption::Surface(800.0);
    pub const SURFACE_900: ColorOption = ColorOption::Surface(900.0);

    pub const NEUTRAL_50: ColorOption = ColorOption::Neutral(50.0);
    pub const NEUTRAL_100: ColorOption = ColorOption::Neutral(100.0);
    pub const NEUTRAL_200: ColorOption = ColorOption::Neutral(200.0);
    pub const NEUTRAL_300: ColorOption = ColorOption::Neutral(300.0);
    pub const NEUTRAL_400: ColorOption = ColorOption::Neutral(400.0);
    pub const NEUTRAL_500: ColorOption = ColorOption::Neutral(500.0);
    pub const NEUTRAL_600: ColorOption = ColorOption::Neutral(600.0);
    pub const NEUTRAL_700: ColorOption = ColorOption::Neutral(700.0);
    pub const NEUTRAL_800: ColorOption = ColorOption::Neutral(800.0);
    pub const NEUTRAL_900: ColorOption = ColorOption::Neutral(900.0);
}