use glam::Vec4;

use crate::NodeSize;

use super::{Align, DivSize, DivItemOrientation};



// I should be able to recreate Solid functionality with Div
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Div { // Most basic type, basically every div is List 
    pub width: DivSize<f32>,
    pub min_width: Option<NodeSize<f32>>,
    pub max_width: Option<NodeSize<f32>>,
    pub align_x: Option<Align>,

    pub height: DivSize<f32>,
    pub min_height: Option<NodeSize<f32>>,
    pub max_height: Option<NodeSize<f32>>,
    pub align_y: Option<Align>,

    pub padding: NodeSize<Vec4>,
    pub margin: NodeSize<Vec4>,

    pub breaks: bool,


    // Item positioning

    pub item_orientation: DivItemOrientation,
    pub item_flipped: bool,
    pub item_next_gap: NodeSize<f32>,
    pub item_line_gap: NodeSize<f32>,



    //Content GRID || LIST
}

//pub struct Break; // Just div that skips to new grid line
//pub struct List; //Ver or Hor
//pub struct Grid; //Ver or Hor