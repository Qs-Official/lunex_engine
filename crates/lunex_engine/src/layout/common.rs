#[cfg(feature = "bevy")]
use bevy::prelude::Component;

use crate::{import::*, Div};
use crate::{NiceDisplay, NodeSize};

use super::{Window, Solid};


/// ## Align
/// Type used for aligning Ui items inside containers.
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

/// ## Cover
/// Defines how a container is scaled relative to it's parent container
/// * [`Cover::Horizonal`]
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







/// ## Layout
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
pub enum StackOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum StackPlacement {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct StackOptions {
    /// Dictates on which axis should the nodes be stacked
    orientation: StackOrientation,
    /// Dictates if the stacking direction should be flipped
    flipped: bool,
    /// Dictates how should the nodes be positioned within one line
    placement: StackPlacement,
    /// Minimal gap between subnodes
    item_gap: NodeSize<f32>,
}