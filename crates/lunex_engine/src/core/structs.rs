use crate::{import::*, NiceDisplay, Rectangle3D, StackOptions};
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
    /// Mandatory data the user can uppend which all nodes have shared access to.
    pub data: M,
    /// Scale of the [`Abs`] unit.
    pub abs_scale: f32,
    /// Default font size for all subnodes to use (Rem unit scaling).
    pub font_size: f32,
}
impl <M: Default + Component> Default for MasterData<M> {
    fn default() -> Self {
        MasterData {
            data: Default::default(),
            abs_scale: 1.0,
            font_size: 16.0,
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
    /// Optional data the user can append.
    pub data: Option<N>,
    /// Calculated rectangle from layout.
    pub rectangle: Rectangle3D,
    /// Layout of this node.
    pub layout: Layout,
    /// Layout of subnodes and how to stack them.
    pub stack: StackOptions,
    /// Optional font size to overwrite the inherited master font size.
    pub font_size: Option<f32>,
    /// Size of the content to wrap around. Affects this node's size only if the layout is parametric (Div).
    pub content_size: Vec2,
}
impl <N:Default + Component> NodeData<N> {
    pub fn new() -> NodeData<N> {
        NodeData::default()
    }
}
impl <N: Default + Component> NiceDisplay for NodeData<N> {
    fn to_nicestr(&self) -> String {
        format!("{} {} {}", self.layout.to_nicestr(), "|||".black(), self.rectangle.to_nicestr())
    }
}