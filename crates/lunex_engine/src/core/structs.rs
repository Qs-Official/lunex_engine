use crate::{Rect3D, NiceDisplay};
use bevy::ecs::component::Component;
use colored::Colorize;

use crate::nodes::prelude::*;
use crate::layout::Layout;


pub type UiTree<M = NoData, N = NoData> = NodeTree<MasterData<M>, NodeData<N>>;
pub type UiNode<N = NoData> = Node<NodeData<N>>;



/// Empty type to tell the compiler that there is no data stored in the node.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NoData;



/// A struct holding all data appended to [`UiTree`]. Responsible for storing settings, scaling, theme, etc.
/// Every [`UiTree`] needs to have this to work properly.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct MasterData<M: Default + Component> {
    pub data: M,

    pub abs_scale: f32,
}
impl <M: Default + Component> Default for MasterData<M> {
    fn default() -> Self {
        MasterData {
            data: Default::default(),
            abs_scale: 1.0,
        }
    }
}
impl <M: Default + Component> NiceDisplay for MasterData<M> {
    fn to_nicestr(&self) -> String {
        format!("{}", self.abs_scale)
    }
}


/// A struct holding all data appended to [`UiNode`]. Responsible for storing layout, custom data, cache, etc.
/// Every [`UiNode`] needs to have this to work properly.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NodeData<N: Default + Component> {
    pub data: Option<N>,
    pub rect: Rect3D,
    pub layout: Layout,
}
impl <N:Default + Component> NodeData<N> {
    pub fn new() -> NodeData<N> {
        NodeData::default()
    }
}
impl <N: Default + Component> NiceDisplay for NodeData<N> {
    fn to_nicestr(&self) -> String {
        format!("{} {} {}", self.layout.to_nicestr(), "|||".black(), self.rect.to_nicestr())
    }
}