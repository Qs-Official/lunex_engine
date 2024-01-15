// #======================#
// #=== PRELUDE EXPORT ===#

//pub use lunex_engine::prelude::*;

pub mod macros;
pub mod systems;

pub mod prelude {

    // BEVY-LUNEX SPECIFIC
    pub use super::systems::*;


    //pub use lunex_engine::common::prelude::*;
    //pub use lunex_engine::core::prelude::*;
    //pub use lunex_engine::layout;
    
    //pub use super::{ShadowNodeTree, ShadowNode};
    //pub use lunex_engine::nodes::prelude::{ NodeGeneralTrait, NodeDisplayTrait };

    pub use lunex_engine::core::prelude::*;
    pub use lunex_engine::NoData;
    pub use super::UiLink;
    //pub use lunex_engine::UI;

    //pub use lunex_engine::*;
}

// #=======================#
// #=== RE-EXPORT TYPES ===#

use std::borrow::Borrow;

use bevy::prelude::*;
use ahash::AHashMap;
pub use lunex_engine::{UiNode, UiTree};


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UiLink {
    path: String,
}
impl UiLink {
    pub fn path( path: impl Borrow<str>) -> Self {
        UiLink { path: path.borrow().to_string() }
    }
}


#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimension(pub Vec2);



#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ShadowNodeTree {
    id_map: AHashMap<String, Entity>
}
impl ShadowNodeTree {
    pub fn build_set(cmd: &mut Commands, ui: UiTree, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
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
    fn build(cmd: &mut Commands, ui: &UiNode, parent_id: Entity, msh: &mut ResMut<Assets<Mesh>>, mat: &mut ResMut<Assets<StandardMaterial>>) {
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
