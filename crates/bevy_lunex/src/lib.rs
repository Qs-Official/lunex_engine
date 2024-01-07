// #======================#
// #=== PRELUDE EXPORT ===#

//pub use lunex_engine::prelude::*;

pub mod prelude {
    pub use lunex_engine::common::prelude::*;
    pub use lunex_engine::core::prelude::*;
    pub use lunex_engine::layout;
    
    pub use super::{ShadowNodeTree, ShadowNode};
    pub use lunex_engine::nodes::prelude::{ NodeGeneralTrait, NodeDisplayTrait };


}

// #=======================#
// #=== RE-EXPORT TYPES ===#

use bevy::prelude::*;
use ahash::AHashMap;
pub use lunex_engine::{UINode, UINodeTree};



#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension(pub Vec2);



#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNodeTree {
    id_map: AHashMap<String, Entity>
}
impl ShadowNodeTree {
    pub fn build_set(cmd: &mut Commands, ui: UINodeTree, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
        let shadownode = cmd.spawn((

            msh.add(shape::Quad { size: Vec2::splat(4.0), flip: false }.into()),
            mat.add(Color::rgb(0.5, 1.0, 0.5).into()),

            ShadowNodeTree::default(),
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
    fn build(cmd: &mut Commands, ui: &UINode, parent_id: Entity, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
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
