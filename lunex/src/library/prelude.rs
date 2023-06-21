#![allow(non_snake_case)]
pub mod Ui {
    pub use crate::library::ui_widget::Widget;


    pub use crate::library::ui_core::Hiearchy;
    pub use crate::library::ui_core::hiearchy_update;

    //pub use crate::core::style::Style;
    //pub use crate::core::style::Align;
    //pub use crate::core::style::Sprite;

    pub use crate::library::ui_container::PositionType;
    pub use crate::library::ui_container::SolidSize;

    pub mod Pos {
        pub use crate::library::ui_container::Relative;
        pub use crate::library::ui_container::Window;
        pub use crate::library::ui_container::Solid;
    }

    //pub use crate::library::ui::preset as Templates;
    //pub use crate::template as Template;
}
pub use crate::library::general::Outcome;
pub use crate::library::general::Timer;
pub use crate::library::general::MString;

pub use ahash::AHashMap as HashMap;