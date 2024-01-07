/// ## Nice display
/// Trait for types to implement so they can be nicely printed in terminal.
/// Used by [`crate::NodeDisplayTrait::tree`] for displaying custom node data.
pub trait NiceDisplay {
    /// ## To nice string
    /// Used when you want to convert type into nicely formatted string
    /// for display in the terminal. Only important data for the user should be shown.
    /// Use `colorise` crate for nice colors.
    fn to_nicestr(&self) -> String;
}