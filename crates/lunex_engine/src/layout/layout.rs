use lunex_types::NodeSize;
use lunex_types::NiceDisplay;
use colored::Colorize;


pub mod prelude {
    pub use super::declarative;
    pub use super::parametric;

    pub use super::lui;

    pub use super::Align;
    pub use super::Layout;
    pub use super::DivSize;
}

/// # Lunex UI
/// Contains all containers available in
/// ### Declarative
/// * [Window]
/// * [Solid]
/// ### Parametric
/// * [Div]
/// * [List]
/// * [Grid]
pub mod lui {
    pub use crate::declarative::*;
    pub use crate::parametric::*;
}



/// ## Declarative Layouts
/// Contains declarative type of layouts.
/// You define their exact position. They don't rely on context.
/// They are the primitives of this library.
/// * [Window]
/// * [Solid]
pub mod declarative {}



/// ## Parametric Layouts
/// Contains parametric type of layouts.
/// You define how they behave based on their neighboring nodes.
/// They rely on context. They work similarly as HTML (padding, margin, etc.)
/// * [Div]
/// * [List]
/// * [Grid]
pub mod parametric {}