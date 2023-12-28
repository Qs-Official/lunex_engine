use crate::import::*;
use crate::{NiceDisplay, NodeSize, Rect3D};

use super::{Window, Solid};


/// ## Align
/// Type used for aligning items in parametric containers.
/// 
/// _Range_ : `-1.0 for START to 1.0 for END`
/// * `Align::START`
/// * `Align::CENTER`
/// * `Align::END`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Align (pub f32);
impl Align {
    pub const START: Align = Align(-1.0);
    pub const LEFT: Align = Align(-1.0);
    pub const CENTER: Align = Align(0.0);
    pub const MIDDLE: Align = Align(0.0);
    pub const END: Align = Align(1.0);
    pub const RIGHT: Align = Align(1.0);
}
impl NiceDisplay for Align {
    fn to_nicestr(&self) -> String {
        format!("{}", self.0.to_string().bold())
    }
}


/// ## Layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    Window(Window),
    Solid(Solid),
    //Window3D
    //Div
    //Br
}
impl Layout {
    pub fn compute(&self, parent: Rect3D, font_size: f32) -> Rect3D {
        match &self {
            Layout::Window(l) => l.compute(parent.into(), font_size).into(),
            Layout::Solid(_) => todo!(),
        }
    }
}
impl Default for Layout {
    fn default() -> Self {
        Window::FULL.into()
    }
}
impl NiceDisplay for Layout {
    fn to_nicestr(&self) -> String {
        match self {
            Layout::Solid(layout) => format!("{} {}", "Solid".bold().bright_cyan(), layout.to_nicestr()),
            Layout::Window(layout) => format!("{} {}", "Window".bold().bright_cyan(), layout.to_nicestr()),
        }
    }
}


/// ## Div Size
/// 
/// * [DivSize::Min]
/// * [DivSize::Max]
/// * [DivSize::Exact]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DivSize<T> {
    /// ## Min
    /// Div will be as little as it can be while still encapsulating it's content and following set rules.
    #[default] Min,
    /// ## Max
    /// Div will be as big as it can be while following set rules. If `max_width` is not set then fill `100%` of the parent layout.
    Max,
    /// ## Exact
    /// Div will be sized as specified while still following set rules.
    Exact(NodeSize<T>),
}


/// ## Div Item Orientation
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DivItemOrientation {
    /// ## Horizontal
    #[default] Horizontal,
    /// ## Vertical
    Vertical,
}