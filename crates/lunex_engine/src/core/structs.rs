use crate::{Rect3D, NiceDisplay};
use bevy::ecs::component::Component;
use colored::Colorize;

use crate::nodes::prelude::*;
use crate::layout::Layout;

//pub type UI<T> = UiTree<T>;
pub type UiTree<T = NoData> = NodeTree<InterfaceData, Container<T>>;
pub type UiNode<T = NoData> = Node<Container<T>>;



/// ## No data
/// Empty type to tell the compiler that there is no data stored in the separete nodes.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NoData;



#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct InterfaceData {
    //pub themes: Theme,
}

/// ## Container
/// A struct holding all UI data appended to [`UiNode`]. Responsible for storing layout, custom data, cache, etc.
/// Every [`UiNode`] needs to have this to work properly.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Container<T: Default + Component> {
    pub data: Option<T>,
    pub rect: Rect3D,
    pub layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    //depth: f32,
}
impl <T:Default + Component> Container<T> {
    pub fn new() -> Container<T> {
        Container::default()
    }
}
impl <T: Default + Component> NiceDisplay for Container<T> {
    fn to_nicestr(&self) -> String {
        format!("{} {} {}", self.layout.to_nicestr(), "|||".black(), self.rect.to_nicestr())
    }
}