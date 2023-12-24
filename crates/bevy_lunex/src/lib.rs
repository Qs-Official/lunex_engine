pub use lunex::prelude::*;

pub mod prelude {
    pub use super::{NodeTrait, NodeTraitPrint};
    pub use super::Interface;
}

use bevy::prelude::*;
use ahash::AHashMap;
use lunex::{Node, Container};



#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension(pub Vec2);



#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNodeMap {
    id_map: AHashMap<String, Entity>
}
impl ShadowNodeMap {
    pub fn build_set(cmd: &mut Commands, ui: Interface, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
        let shadownode = cmd.spawn((

            msh.add(shape::Quad { size: Vec2::splat(4.0), flip: false }.into()),
            mat.add(Color::rgb(0.5, 1.0, 0.5).into()),

            ShadowNodeMap::default(),
            ShadowNode::default(),
            Dimension::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),

        )).id();
        for (_, node) in &ui.node.nodes {
            ShadowNode::build(cmd, node, shadownode, msh, mat);
        }
        //ShadowNode::default(); // Needs to be inserted as component to the realnode
    }
}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNode {}
impl ShadowNode {
    fn build(cmd: &mut Commands, ui: &Node<Container>, parent_id: Entity, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
        let shadownode = cmd.spawn((

            msh.add(shape::Quad { size: Vec2::splat(4.0), flip: false }.into()),
            mat.add(Color::rgb(0.5, 0.5, 0.5).into()),

            ShadowNode::default(),
            Dimension::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),

        )).id();
        cmd.entity(parent_id).push_children(&[shadownode]);
        for (_, node) in &ui.nodes {
            ShadowNode::build(cmd, node, shadownode, msh, mat);
        }
    }
}
