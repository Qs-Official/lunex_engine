use bevy::prelude::*;
use bevy_lunex::prelude::*;  

// ===========================================================
// === SPAWN COMMADS ===

pub fn spawn_text (commands: &mut Commands, widget: Widget, text: &str, text_style: TextStyle) {
    commands.spawn (
        TextElementBundle {
            widget,
            element: Element {
                relative: Vec2::new(50.0, 50.0),
                boundary: text_compute_size_simple(text, text_style.font_size),
                scale: 40.0,
                ..default()
            },
            text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::Center),
            ..Default::default()
        }
    );
}

pub fn spawn_image (commands: &mut Commands, asset_server: &Res<AssetServer>, widget: Widget, path: &str) {
    commands.spawn (
        ImageElementBundle {
            widget,
            element: Element {
                relative: Vec2::new(0.0, 0.0),
                scale: 100.0,
                ..default()
            },
            texture: asset_server.load(path),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..Default::default()
        }
    );
    
}

pub fn spawn_image_with_text (commands: &mut Commands, asset_server: &Res<AssetServer>, widget: Widget, imgpath: &str, text: &str, text_pos: Vec2, text_style: TextStyle) {
    commands.spawn (
        ImageElementBundle {
            widget,
            element: Element {
                relative: Vec2::new(0.0, 0.0),
                scale: 100.0,
                ..default()
            },
            texture: asset_server.load(imgpath),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..Default::default()
        }
    ).with_children(|builder| {
        builder.spawn(Text2dBundle {
            text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: text_pos.x, y: text_pos.y, z: 5. }, ..default() },
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            ..default()
        });
    });
}


// ===========================================================
// === LUNEX SYNC TO ENTITIES ===
//# This function is the main system that is behind aligning text and images. It querries through entities with widgets

#[derive(Component)]
pub struct UserInterface {
    pub offset: Vec2,
}

//OUTDATED, NEEDS TO RUN AFTER ELEMENT_UPDATE TO FIX ALL IMAGES DUE TO 0 FOR THER BOUNDARY
pub fn image_update(mut systems: Query<(&mut Hierarchy, &mut UserInterface)>, mut query: Query<(&mut Widget, &Handle<Image>, &mut Transform)>, assets: Res<Assets<Image>>) {

    let (mut system, mut ui) = systems.get_single_mut().unwrap();     //get the single hiearchy struct
    for (widget, image_handle, mut transform) in &mut query {
        match widget.fetch(&system, "") {
            Result::Err(..) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            },
            Result::Ok(branch) => {
                if !branch.is_visible() {
                    transform.translation.x = -10000.0;
                    transform.translation.y = -10000.0;
                } else {
                    ui.offset.x = -system.width/2.0;
                    ui.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch(&mut system, "").unwrap().container_get().position_get().invert_y();      //The widget will locate itself inside the hierarchy
                    transform.translation.x = pos.point_1.x + ui.offset.x;
                    transform.translation.y = pos.point_1.y + ui.offset.y;

                    match assets.get(image_handle) {
                        Option::Some(image) => {
                            let image_dimensions = image.size();
                            transform.scale.x = pos.width/image_dimensions.x;
                            transform.scale.y = pos.height/image_dimensions.y;
                        },
                        Option::None => {},
                    }
                }
            }
        };
    }
}

pub fn element_update(mut systems: Query<(&mut Hierarchy, &mut UserInterface)>, mut query: Query<(&mut Widget, &Element, &mut Transform)>) {

    let (mut system, mut ui) = systems.get_single_mut().unwrap();
    for (widget, element, mut transform) in &mut query {
        match widget.fetch(&system, "") {
            Result::Err(..) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            },
            Result::Ok(branch) => {
                if !branch.is_visible() {
                    transform.translation.x = -10000.0;
                    transform.translation.y = -10000.0;
                } else {
                    ui.offset.x = -system.width/2.0;
                    ui.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch(&mut system, "").unwrap().container_get().position_get().invert_y();
                    let vec = pos.get_pos_y_inverted(element.relative);
                    transform.translation.x = vec.x + ui.offset.x;
                    transform.translation.y = vec.y + ui.offset.y;

                    let scale = f32::min(pos.width/element.boundary.x, pos.height/element.boundary.y) * element.scale/100.0;
                    transform.scale.x = scale;
                    transform.scale.y = scale;

                }
            }
        };
    }
}

pub struct AlignPlugin;
impl Plugin for AlignPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (element_update, image_update).chain());
    }
}