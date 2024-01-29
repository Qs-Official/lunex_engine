#[cfg(feature = "bevy")]
use bevy::prelude::Component;

use crate::{import::*, Div};
use crate::{NiceDisplay, NodeSize};

use super::{Window, Solid};


/// Type used for aligning subnodes inside nodes.
/// 
/// _Range_ : `-1.0 for START to 1.0 for END`
/// * [`Align::START`]
/// * [`Align::CENTER`]
/// * [`Align::END`]
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

/// Defines how a container is scaled relative to it's parent container
/// * [`Cover::Horizontal`]
/// * [`Cover::Vertical`]
/// * [`Cover::Contain`]
/// * [`Cover::Full`]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Cover {
    /// ## Horizontal cover
    /// Stretches the container so that it always fully covers the horizontal axis of the parent.
    Horizontal,
    /// ## Vertical cover
    /// Stretches the container so that it always fully covers the vertical axis of the parent.
    Vertical,
    /// ## Contain
    /// Stretches the container so that it is fully contained within the parent.
    #[default] Contain,
    // ## Full
    /// Stretches the container so that it fully covers the parent.
    Full,
}
impl NiceDisplay for Cover {
    fn to_nicestr(&self) -> String {
        match self {
            Cover::Horizontal => format!("{}", "Horizontal".bold()),
            Cover::Vertical => format!("{}", "Vertical".bold()),
            Cover::Contain => format!("{}", "Contain".bold()),
            Cover::Full => format!("{}", "Full".bold()),
        }
    }
}





/// Enum holding the node layout
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    Window(Window),
    Solid(Solid),
    Div(Div),
    //Window3D
    //Div
    //Br
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
            Layout::Div(layout) => format!("{} {}", "Div".bold().bright_cyan(), layout.to_nicestr()),
        }
    }
}





#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Sizing {
    /// Minimal with forced wrapping.
    Minimal,
    ///Minimal with no wrap unless reached max size.
    #[default]
    Normal,
    /// Stretches until it can't.
    Maximal,
}





#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum StackDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum LinePlacement {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StackOptions {
    /// Dictates on which axis should the nodes be stacked.
    pub direction: StackDirection,
    /// Dictates if the stacking direction should be flipped (flip around Y axis).
    pub flipped: bool,
    /// Dictates if the stacking direction should be inverted (flip around X axis).
    pub inverted: bool,
    /// Dictates how should the nodes be positioned within one line.
    pub placement: LinePlacement,
    /// Minimal gap between subnodes and lines.
    pub gap: NodeSize<Vec2>,
    /// Default alignment of nodes within lines.
    pub node_alignment: Align,
}
impl Default for StackOptions {
    fn default() -> Self {
        StackOptions {
            direction: Default::default(),
            flipped: Default::default(),
            inverted: Default::default(),
            placement: Default::default(),
            gap: Default::default(),
            node_alignment: Align::START,
        }
    }
}
impl StackOptions {
    pub fn new() -> Self {
        Default::default()
    }
    /// ## With direction
    /// Replaces the direction with the new value.
    pub fn direction(mut self, direction: StackDirection) -> Self {
        self.direction = direction;
        self
    }
    /// ## As flipped
    /// Replaces the flipped value with the new value.
    pub fn flipped(mut self, value: bool) -> Self {
        self.flipped = value;
        self
    }
    /// ## As inverted
    /// Replaces the inversion value with the new value.
    pub fn inverted(mut self, value: bool) -> Self {
        self.inverted = value;
        self
    }
    /// ## With placement
    /// Replaces the placement with the new value.
    pub fn placement(mut self, placement: LinePlacement) -> Self {
        self.placement = placement;
        self
    }
    /// ## With gap
    /// Replaces the gap with the new value.
    pub fn gap(mut self, gap: impl Into<NodeSize<Vec2>>) -> Self {
        self.gap = gap.into();
        self
    }
    /// ## With gap horizontal
    /// Replaces the horizontal gap with the new value.
    pub fn gap_x(mut self, gap: impl Into<NodeSize<f32>>) -> Self {
        self.gap.set_x(gap);
        self
    }
    /// ## With gap vertical
    /// Replaces the vertical gap with the new value.
    pub fn gap_y(mut self, gap: impl Into<NodeSize<f32>>) -> Self {
        self.gap.set_y(gap);
        self
    }
}