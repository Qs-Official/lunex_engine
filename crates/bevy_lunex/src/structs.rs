use std::borrow::Borrow;

use bevy::prelude::*;
use lunex_engine::prelude::*;


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MovableByCamera;


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UiLink {
    pub path: String,
}
impl UiLink {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink { path: path.borrow().to_string() }
    }
}


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension {
    pub size: Vec2,
}
impl Dimension {
    pub fn new(size: impl Into<Vec2>) -> Self {
        Dimension {
            size: size.into()
        }
    }
}


#[derive(Bundle, Debug, Default, Clone, PartialEq)]
pub struct UiTreeBundle <T: Default + Component, M: Component> {
    pub tree: UiTree<T>,
    pub marker: M,
    pub transform: Transform,
    pub dimension: Dimension,

    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
}
impl <T: Default + Component, M: Default + Component> From<UiTree<T>> for UiTreeBundle<T, M> {
    fn from(value: UiTree<T>) -> Self {
        UiTreeBundle::<T, M> {
            tree: value,
            ..default()
        }
    }
}
impl <T: Default + Component, M: Default + Component> From<Result<UiTree<T>, UiError>> for UiTreeBundle<T, M> {
    fn from(value: Result<UiTree<T>, UiError>) -> Self {
        match value {
            Ok(val) => UiTreeBundle::<T, M> {
                tree: val,
                ..default()
            },
            Err(e) => {
                error!("Panicted when constructing UiTreeBundle from Err: {}", e);
                panic!("Panicted when constructing UiTreeBundle from Err: {}", e);
            }
        }
    }
}



#[derive(Bundle, Debug, Default, Clone)]
pub struct UiImageBundle {
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub transform: Transform,
    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}