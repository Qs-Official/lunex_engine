use std::borrow::Borrow;

use bevy::{prelude::*, render::primitives::Aabb};
use lunex_engine::prelude::*;


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MovableByCamera;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Element;


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UiLink {
    pub path: String,
}
impl UiLink {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink { path: path.borrow().to_string() }
    }
    pub fn add( &self, path: impl Borrow<str>) -> Self {
        UiLink { path: format!("{}/{}", self.path, path.borrow()) }
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
pub struct UiTreeBundle <M: Default + Component, N: Default + Component, T: Component> {
    pub tree: UiTree<M, N>,
    pub marker: T,
    pub transform: Transform,
    pub dimension: Dimension,

    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
}
impl <M: Default + Component, N: Default + Component, T: Component + Default> From<UiTree<M, N>> for UiTreeBundle<M, N, T> {
    fn from(value: UiTree<M, N>) -> Self {
        UiTreeBundle::<M, N, T> {
            tree: value,
            ..default()
        }
    }
}



#[derive(Bundle, Debug, Default, Clone)]
pub struct UiMaterial3dBundle {
    pub element: Element,
    pub mesh: Handle<Mesh>,

    pub material: Handle<StandardMaterial>,

    pub dimension: Dimension,
    pub transform: Transform,

    pub aabb: Aabb,

    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
impl From<Handle<StandardMaterial>> for UiMaterial3dBundle {
    fn from(value: Handle<StandardMaterial>) -> Self {
        UiMaterial3dBundle {
            material: value,
            ..default()
        }
    }
}


#[derive(Bundle, Clone, Debug, Default)]
pub struct UiImage2dBundle {
    pub element: Element,
    pub sprite: Sprite,

    pub texture: Handle<Image>,

    pub dimension: Dimension,
    pub transform: Transform,

    pub aabb: Aabb,

    pub visibility: Visibility,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
impl From<Handle<Image>> for UiImage2dBundle {
    fn from(value: Handle<Image>) -> Self {
        UiImage2dBundle {
            texture: value,
            ..default()
        }
    }
}
