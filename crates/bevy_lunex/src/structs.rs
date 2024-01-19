use bevy::prelude::*;
use lunex_engine::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct MovableByCamera;


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
pub struct UiTreeBundle<T: Default + Component, M: Component> {
    pub marker: M,
    pub ui_tree: UiTree<T>,
    pub transform: Transform,
    pub dimenstion: Dimension,
}
impl <T: Default + Component, M: Default + Component> From<UiTree<T>> for UiTreeBundle<T, M> {
    fn from(value: UiTree<T>) -> Self {
        UiTreeBundle::<T, M> {
            ui_tree: value,
            ..default()
        }
    }
}
impl <T: Default + Component, M: Default + Component> From<Result<UiTree<T>, UiError>> for UiTreeBundle<T, M> {
    fn from(value: Result<UiTree<T>, UiError>) -> Self {
        match value {
            Ok(val) => UiTreeBundle::<T, M> {
                ui_tree: val,
                ..default()
            },
            Err(e) => {
                error!("Panicted when constructing UiTreeBundle from Err: {}", e);
                panic!("Panicted when constructing UiTreeBundle from Err: {}", e);
            }
        }
    }
}