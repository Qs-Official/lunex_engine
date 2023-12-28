use crate::{Rect3D, NiceDisplay};
use colored::Colorize;

use crate::nodes::prelude::*;
use crate::layout::Layout;


pub type UINodeTree<P = ()> = NodeTree<InterfaceData, Container<P>>;
pub type UINode<P = ()> = Node<Container<P>>;


pub struct InterfaceData {
    //pub themes: Theme,
}

#[derive(Debug, Default)]
pub struct Container<P> {
    pub data: Option<P>,
    pub rect: Rect3D,
    pub layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    //depth: f32,

    //roll: f32,
    //yaw: f32,
    //pitch: f32
}

impl <P:Default> Container<P> {
    pub fn new() -> Container<P> {
        Container::default()
    }
}

impl <P> NiceDisplay for Container<P> {
    fn to_nicestr(&self) -> String {
        format!("{} {} {}", self.layout.to_nicestr(), "|||".black(), self.rect.to_nicestr())
    }
}