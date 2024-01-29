use std::borrow::Borrow;

use bevy::{prelude::*, render::primitives::Aabb, sprite::Anchor, text::{Text2dBounds, TextLayoutInfo}};
use lunex_engine::prelude::*;

pub type UiStack = StackOptions;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct UiContent {
    pub size: Vec2,
}
impl UiContent {
    pub fn new(size: impl Into<Vec2>) -> Self {
        UiContent { size: size.into() }
    }
}


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
    pub fn new( &self) -> Self {
        UiLink { path: format!("{}/", self.path) }
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





#[derive(Bundle, Clone, Debug, Default)]
pub struct UiText3dBundle {
    /// Contains the text.
    pub text: Text,
    /// How the text is positioned relative to its transform.
    pub text_anchor: bevy::sprite::Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: bevy::text::Text2dBounds,
    /// The transform of the text.
    pub transform: Transform,
    /// The global transform of the text.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the text.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
    pub text_layout_info: bevy::text::TextLayoutInfo,


    pub element: Element,
    pub sprite: Sprite,

    pub dimension: Dimension,

    pub aabb: Aabb,
}


#[derive(Bundle, Clone, Debug, Default)]
pub struct UiText2dBundle {
    pub element: Element,
    pub dimension: Dimension,

    /// Contains the text.
    pub text: Text,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: Text2dBounds,
    /// The transform of the text.
    pub transform: Transform,
    /// The global transform of the text.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the text.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
    pub text_layout_info: TextLayoutInfo,
}
