use bevy::{prelude::*, text::Text2dBounds};
use bevy_lunex::prelude::*;  


//################################################################################
//# == Image Update ==
//# This is a universal system that does the synchronization magic. It pulls relevant data from Hierarchy and updates all widgets that contain images.
//# This system will NOT be hard-coded so people can have more control over how they want the layout capabilities of Bevy_Lunex to handle (Maybe 3D?)

#[derive(Component)]
pub struct UIPlacement {
    pub offset: Vec2,
}
pub fn image_update(mut systems: Query<(&mut Hierarchy, &mut UIPlacement)>, mut query: Query<(&mut Widget, &Handle<Image>, &mut Transform)>, assets: Res<Assets<Image>>) {

    let (mut system, mut placement) = systems.get_single_mut().unwrap();     //get the single hiearchy struct
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
                    placement.offset.x = -system.width/2.0;
                    placement.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch_position(&mut system, "").unwrap().invert_y();      //The widget will locate itself inside the hierarchy
                    transform.translation.x = pos.point_1.x + placement.offset.x;
                    transform.translation.y = pos.point_1.y + placement.offset.y;

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


pub fn _text_update(mut systems: Query<(&mut Hierarchy, &mut UIPlacement)>, mut query: Query<(&mut Widget, &Text2dBounds, &mut Transform)>) {

    let (mut system, mut placement) = systems.get_single_mut().unwrap();     //get the single hiearchy struct
    for (widget, _bounds, mut transform) in &mut query {
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
                    placement.offset.x = -system.width/2.0;
                    placement.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch_position(&mut system, "").unwrap().invert_y();      //The widget will locate itself inside the hierarchy
                    transform.translation.x = pos.point_1.x + placement.offset.x;
                    transform.translation.y = pos.point_1.y + placement.offset.y;

                    //transform.scale.x = pos.width/bounds.size.x;
                    //transform.scale.y = pos.height/bounds.size.y;

                }
            }
        };
    }
}


pub fn element_update(mut systems: Query<(&mut Hierarchy, &mut UIPlacement)>, mut query: Query<(&mut Widget, &Element, &mut Transform)>) {

    let (mut system, mut placement) = systems.get_single_mut().unwrap();
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
                    placement.offset.x = -system.width/2.0;
                    placement.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch_position(&mut system, "").unwrap().invert_y();
                    let vec = pos.get_pos_y_inverted(element.relative);
                    transform.translation.x = vec.x + placement.offset.x;
                    transform.translation.y = vec.y + placement.offset.y;

                    //let scale = pos.width/element.size.x.min(pos.height/element.size.y);
                    //transform.scale.x = scale;
                    //transform.scale.y = scale;

                }
            }
        };
    }
}