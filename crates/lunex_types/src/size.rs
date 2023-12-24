use std::ops::Add;
use std::ops::AddAssign;
//use std::ops::Mul;  // For * f32
//use std::ops::MulAssign;
use glam::f32::{Vec2, Vec3, Vec4};
use colored::Colorize;

use crate::NiceDisplay;


// #====================#
// #=== UNIT TESTING ===#

#[cfg(test)]
mod test {
    use super::{Abs, Prc, Rem, Size, NodeSize};
    #[test]
    fn all () {
        assert_eq!(NodeSize::new().with_abs(Abs(5)) + Abs(5) + Abs(5), NodeSize::new().with_abs(Abs(15)));
        assert_eq!(NodeSize::new().with_prc(Prc(5)) + Prc(5) + Prc(5), NodeSize::new().with_prc(Prc(15)));
        assert_eq!(NodeSize::new().with_rem(Rem(5)) + Rem(5) + Rem(5), NodeSize::new().with_rem(Rem(15)));

        let amount = Abs(5) + Prc(10) + Rem(15);
        assert_eq!(amount, NodeSize::new().with_abs(Abs(5)).with_prc(Prc(10)).with_rem(Rem(15)));

        let mut new_amount = amount + Abs(20);
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(25)).with_prc(Prc(10)).with_rem(Rem(15)));

        new_amount += Prc(20);
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(25)).with_prc(Prc(30)).with_rem(Rem(15)));

        new_amount += amount;
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(30)).with_prc(Prc(40)).with_rem(Rem(30)));
        let _ = Size::S_65;
    }
}


// #========================#
// #=== TYPE DEFINITIONS ===#

/// ## Absolute unit
/// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
/// ### Example
/// ```
/// # use lunex_core::Abs;
/// let a: Abs<f32> = Abs(4.0) + Abs(6.0); // -> 10px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Abs<T>(pub T);
/// ## Percentage unit
/// `0% to 100%`. Overflow allowed.
/// ### Example
/// ```
/// # use lunex_core::Prc;
/// let a: Prc<f32> = Prc(25.0) + Prc(40.0); // -> 65%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Prc<T>(pub T);
/// ## Rem unit
/// Size of 1 symbol `M` which is `16px` with `font size 16px` and so on.
/// ### Example
/// ```
/// # use lunex_core::Rem;
/// let a: Rem<f32> = Rem(1.0) + Rem(2.0); // -> 3rem == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rem<T>(pub T);

/// # Node Size
/// A struct holding size NodeSize data used in UI.
/// It can be constructed from the following units:
/// * [Abs]
/// * [Prc]
/// * [Rem]
/// 
/// size 1 = 0.25rem = 4px
/// ### Support
/// First class implementations for (T)
/// * [f32]
/// * [Vec2]
/// * [Vec3]
/// * [Vec4]
/// ### Example
/// ```
/// # use lunex_core::{NodeSize, Abs, Rem};
/// let a: NodeSize<f32> = Abs(4.0) + Rem(16.0); // -> 4px + (16rem == 256px with font size 16px)
/// let b: NodeSize<f32> = Prc(50.0).into();   // -> 50%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NodeSize<T> {
    /// ## Absolute
    /// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
    pub abs: Option<T>,
    /// ## Percentage
    /// `0% to 100%`. Overflow allowed.
    pub prc: Option<T>,
    /// ## Rem
    /// Size of symbol `M` which is `16px` with `font size 16px` and so on.
    pub rem: Option<T>,
}


// #===============================#
// #=== GENERIC IMPLEMENTATIONS ===#

impl <T> NodeSize<T> {
    /// ## With
    /// Replaces the value of appropriate units with the new value.
    pub fn with(mut self, other: NodeSize<T>) -> Self {
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { *v1 = v2 } else { self.abs = Some(v2) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { *v1 = v2 } else { self.prc = Some(v2) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { *v1 = v2 } else { self.rem = Some(v2) } }
        self
    }
    /// ## With Absolute
    /// Replaces the value with the new absolute value.
    pub fn with_abs(mut self, abs: Abs<T>) -> Self {
        self.abs = Some(abs.0);
        self
    }
    /// ## With Percentage
    /// Replaces the value with the new percentage value.
    pub fn with_prc(mut self, prc: Prc<T>) -> Self {
        self.prc = Some(prc.0);
        self
    }
    /// ## With Rem
    /// Replaces the value with the new rem value.
    pub fn with_rem(mut self, rem: Rem<T>) -> Self {
        self.rem = Some(rem.0);
        self
    }
    /// ## Set
    /// Sets the value of appropriate units to the new value.
    pub fn set(&mut self, other: NodeSize<T>) {
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { *v1 = v2 } else { self.abs = Some(v2) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { *v1 = v2 } else { self.prc = Some(v2) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { *v1 = v2 } else { self.rem = Some(v2) } }
    }
    /// ## Set Absolute
    /// Sets the value to the new absolute value.
    pub fn set_abs(&mut self, abs: Abs<T>) {
        self.abs = Some(abs.0);
    }
    /// ## Set Percentage
    /// Sets the value to the new percentage value.
    pub fn set_prc(&mut self, prc: Prc<T>) {
        self.prc = Some(prc.0);
    }
    /// ## Set Rem
    /// Sets the value to the new rem value.
    pub fn set_rem(&mut self, rem: Rem<T>) {
        self.rem = Some(rem.0);
    }
}

// # Impl into `Abs(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Abs<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_abs(self)
    }
}
// # Impl into `Prc(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Prc<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_prc(self)
    }
}
// # Impl into `Rem(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Rem<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_rem(self)
    }
}

// # Impl `Abs(T) + Abs(T)`
impl<T: Add<Output = T>> Add for Abs<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Abs(self.0 + other.0)
    }
}
// # Impl `Abs(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Abs<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        NodeSize::new().with_abs(self).with_prc(other)
    }
}
// # Impl `Abs(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Abs<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        NodeSize::new().with_abs(self).with_rem(other)
    }
}

// # Impl `Prc(T) + Prc(T)`
impl<T: Add<Output = T>> Add for Prc<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Prc(self.0 + other.0)
    }
}
// # Impl `Prc(T) + Abs(T)`
impl<T: Add<Output = T>> Add<Abs<T>> for Prc<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        NodeSize::new().with_prc(self).with_abs(other)
    }
}
// # Impl `Prc(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Prc<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        NodeSize::new().with_prc(self).with_rem(other)
    }
}

// # Impl `Rem(T) + Rem(T)`
impl<T: Add<Output = T>> Add for Rem<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Rem(self.0 + other.0)
    }
}
// # Impl `Rem(T) + Abs(T)`
impl<T: Add<Output = T>> Add<Abs<T>> for Rem<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        NodeSize::new().with_rem(self).with_abs(other)
    }
}
// # Impl `Rem(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Rem<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        NodeSize::new().with_rem(self).with_prc(other)
    }
}

// # Impl `NodeSize(T) + NodeSize(T)`
impl<T: Add<Output = T> + Add> Add for NodeSize<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {

        let mut output = NodeSize::new();

        if let Some(v1) = self.abs {
            match other.abs {
                Some(v2) => output.set_abs(Abs(v1 + v2)),
                None => output.set_abs(Abs(v1)),
            }
        }

        if let Some(v1) = self.prc {
            match other.prc {
                Some(v2) => output.set_prc(Prc(v1 + v2)),
                None => output.set_prc(Prc(v1)),
            }
        }

        if let Some(v1) = self.rem {
            match other.rem {
                Some(v2) => output.set_rem(Rem(v1 + v2)),
                None => output.set_rem(Rem(v1)),
            }
        }
        
        output
    }
}
// # Impl `NodeSize(T) + Abs(T)`
impl<T: Add<Output = T> + Add> Add<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Abs<T>) -> Self::Output {
        match self.abs {
            Some(v) => {
                self.abs = Some(v + other.0);
                self
            },
            None => self.with_abs(other),
        }
    }
}
// # Impl `NodeSize(T) + Prc(T)`
impl<T: Add<Output = T> + Add> Add<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Prc<T>) -> Self::Output {
        match self.prc {
            Some(v) => {
                self.prc = Some(v + other.0);
                self
            },
            None => self.with_prc(other),
        }
    }
}
// # Impl `NodeSize(T) + Rem(T)`
impl<T: Add<Output = T> + Add> Add<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Rem<T>) -> Self::Output {
        match self.rem {
            Some(v) => {
                self.rem = Some(v + other.0);
                self
            },
            None => self.with_rem(other),
        }
    }
}

// # Impl `NodeSize(T) += NodeSize(T)`
impl<T: Add<Output = T> + Copy> AddAssign for NodeSize<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
// # Impl `NodeSize(T) += Abs(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Abs<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.set_abs(Abs(v + rhs.0)),
            None => self.set_abs(rhs),
        }
    }
}
// # Impl `NodeSize(T) += Prc(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Prc<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.set_prc(Prc(v + rhs.0)),
            None => self.set_prc(rhs),
        }
    }
}
// # Impl `NodeSize(T) += Rem(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Rem<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.set_rem(Rem(v + rhs.0)),
            None => self.set_rem(rhs),
        }
    }
}


// #================================#
// #=== SPECIFIC IMPLEMENTATIONS ===#

// # Impl from_standard TailwindCSS scale
impl NodeSize<f32> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: f32) -> NodeSize<f32> {
        Rem(size * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl NodeSize<Vec2> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec2>) -> NodeSize<Vec2> {
        Rem(size.into() * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl NodeSize<Vec3> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec3>) -> NodeSize<Vec3> {
        Rem(size.into() * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl NodeSize<Vec4> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec4>) -> NodeSize<Vec4> {
        Rem(size.into() * 0.25).into()
    }
}

/// ## NodeSize Evaluate
/// Trait for implementing evaluation logic for (T)
pub trait NodeSizeEvaluate<T> {
    /// ## Evaluate
    /// Evaluates the NodeSize for (T)
    fn evaluate(&self, parent_size: T, font_size: T) -> T;
}

impl NodeSizeEvaluate<f32> for NodeSize<f32> {
    fn evaluate(&self, parent_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec2> for NodeSize<Vec2> {
    fn evaluate(&self, parent_size: Vec2, font_size: Vec2) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec3> for NodeSize<Vec3> {
    fn evaluate(&self, parent_size: Vec3, font_size: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec4> for NodeSize<Vec4> {
    fn evaluate(&self, parent_size: Vec4, font_size: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}

impl NodeSize<Vec2> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
}
impl NodeSize<Vec3> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
    }
}
impl NodeSize<Vec4> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
    }
    /// ## With W
    /// Replaces the W value of appropriate units with the new value.
    pub fn with_w(self, other: NodeSize<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.w = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.w = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.w = v2 } }
        self
    }
    /// ## Set W
    /// Sets the W value of appropriate units with the new value.
    pub fn set_w(&mut self, other: NodeSize<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.w = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.w = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.w = v2 } }
    }
}

impl NiceDisplay for NodeSize<f32> {
    fn to_nicestr(&self) -> String {
        let mut t = String::new();
        if let Some(v) = self.abs {
            if v != 0.0 {
                t = format!("{}", v.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_red(), "m".bright_red());
            }
        }
        if t.is_empty() { t = format!("{}", "0".bright_blue()); };
        format!("{}", t.black())
    }
}
impl NiceDisplay for NodeSize<Vec2> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {}", tx.black(), ty.black())
    }
}
impl NiceDisplay for NodeSize<Vec3> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", v.z.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{}", tx.black(), ty.black(), tz.black())
    }
}
impl NiceDisplay for NodeSize<Vec4> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        let mut tw = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", v.z.to_string().bright_blue());
            }
            if v.w != 0.0 {
                tw = format!("{}", v.w.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "m".bright_red());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        if tw.is_empty() { tw = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{} w:{}", tx.black(), ty.black(), tz.black(), tw.black())
    }
}

// #================================#
// #=== CONSTANT IMPLEMENTATIONS ===#

// # Impl Constructors
impl <T> NodeSize<T> {
    /// ## New
    /// Creates new empty NodeSize
    pub const fn new() -> Self {
        NodeSize {
            abs: None,
            prc: None,
            rem: None,
        }
    }
    /// ## From absolute
    /// Creates new NodeSize
    pub const fn from_abs(abs: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: None,
            rem: None,
        }
    }
    /// ## From percentage
    /// Creates new NodeSize
    pub const fn from_prc(prc: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: Some(prc),
            rem: None,
        }
    }
    /// ## From rem
    /// Creates new NodeSize
    pub const fn from_rem(rem: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: None,
            rem: Some(rem),
        }
    }
    /// ## From absolute & percentage
    /// Creates new NodeSize
    pub const fn from_abs_prc(abs: T, prc: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: Some(prc),
            rem: None,
        }
    }
    /// ## From absolute & rem
    /// Creates new NodeSize
    pub const fn from_abs_rem(abs: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: None,
            rem: Some(rem),
        }
    }
    /// ## From percentage & rem
    /// Creates new NodeSize
    pub const fn from_prc_rem(prc: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: Some(prc),
            rem: Some(rem),
        }
    }
    /// ## From absolute & percentage & rem
    /// Creates new NodeSize
    pub const fn from_abs_prc_rem(abs: T, prc: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: Some(prc),
            rem: Some(rem),
        }
    }
}

// # Impl CONSTS
impl NodeSize<f32> {
    /// ## Extra-small
    pub const XS: NodeSize<f32> = NodeSize::from_rem(1.0);
    /// ## Small
    pub const SM: NodeSize<f32> = NodeSize::from_rem(2.0);
    /// ## Medium
    pub const MD: NodeSize<f32> = NodeSize::from_rem(3.0);
    /// ## Large
    pub const LG: NodeSize<f32> = NodeSize::from_rem(4.0);
    /// ## Extra-large
    pub const XL: NodeSize<f32> = NodeSize::from_rem(6.0);
    /// ## Extra-large 2
    pub const XL2: NodeSize<f32> = NodeSize::from_rem(8.0);
    /// ## Extra-large 3
    pub const XL3: NodeSize<f32> = NodeSize::from_rem(10.0);
    /// ## Extra-large 4
    pub const XL4: NodeSize<f32> = NodeSize::from_rem(12.0);
    /// ## Extra-large 5
    pub const XL5: NodeSize<f32> = NodeSize::from_rem(14.0);
    /// ## Extra-large 6
    pub const XL6: NodeSize<f32> = NodeSize::from_rem(16.0);
    /// ## Extra-large 7
    pub const XL7: NodeSize<f32> = NodeSize::from_rem(18.0);

    /// ## Standard Size 0
    pub const S_0: NodeSize<f32> = NodeSize::from_rem(0.0);
    /// ## Standard Size 1
    pub const S_1: NodeSize<f32> = NodeSize::from_rem(1.0 * 0.25);
    /// ## Standard Size 2
    pub const S_2: NodeSize<f32> = NodeSize::from_rem(2.0 * 0.25);
    /// ## Standard Size 3
    pub const S_3: NodeSize<f32> = NodeSize::from_rem(3.0 * 0.25);
    /// ## Standard Size 4
    pub const S_4: NodeSize<f32> = NodeSize::from_rem(4.0 * 0.25);
    /// ## Standard Size 5
    pub const S_5: NodeSize<f32> = NodeSize::from_rem(5.0 * 0.25);
    /// ## Standard Size 6
    pub const S_6: NodeSize<f32> = NodeSize::from_rem(6.0 * 0.25);
    /// ## Standard Size 7
    pub const S_7: NodeSize<f32> = NodeSize::from_rem(7.0 * 0.25);
    /// ## Standard Size 8
    pub const S_8: NodeSize<f32> = NodeSize::from_rem(8.0 * 0.25);
    /// ## Standard Size 9
    pub const S_9: NodeSize<f32> = NodeSize::from_rem(9.0 * 0.25);
    /// ## Standard Size 10
    pub const S_10: NodeSize<f32> = NodeSize::from_rem(10.0 * 0.25);
    /// ## Standard Size 11
    pub const S_11: NodeSize<f32> = NodeSize::from_rem(11.0 * 0.25);
    /// ## Standard Size 12
    pub const S_12: NodeSize<f32> = NodeSize::from_rem(12.0 * 0.25);
    /// ## Standard Size 13
    pub const S_13: NodeSize<f32> = NodeSize::from_rem(13.0 * 0.25);
    /// ## Standard Size 14
    pub const S_14: NodeSize<f32> = NodeSize::from_rem(14.0 * 0.25);
    /// ## Standard Size 15
    pub const S_15: NodeSize<f32> = NodeSize::from_rem(15.0 * 0.25);
    /// ## Standard Size 16
    pub const S_16: NodeSize<f32> = NodeSize::from_rem(16.0 * 0.25);
    /// ## Standard Size 17
    pub const S_17: NodeSize<f32> = NodeSize::from_rem(17.0 * 0.25);
    /// ## Standard Size 18
    pub const S_18: NodeSize<f32> = NodeSize::from_rem(18.0 * 0.25);
    /// ## Standard Size 19
    pub const S_19: NodeSize<f32> = NodeSize::from_rem(19.0 * 0.25);
    /// ## Standard Size 20
    pub const S_20: NodeSize<f32> = NodeSize::from_rem(20.0 * 0.25);
    /// ## Standard Size 21
    pub const S_21: NodeSize<f32> = NodeSize::from_rem(21.0 * 0.25);
    /// ## Standard Size 22
    pub const S_22: NodeSize<f32> = NodeSize::from_rem(22.0 * 0.25);
    /// ## Standard Size 23
    pub const S_23: NodeSize<f32> = NodeSize::from_rem(23.0 * 0.25);
    /// ## Standard Size 24
    pub const S_24: NodeSize<f32> = NodeSize::from_rem(24.0 * 0.25);
    /// ## Standard Size 25
    pub const S_25: NodeSize<f32> = NodeSize::from_rem(25.0 * 0.25);
    /// ## Standard Size 26
    pub const S_26: NodeSize<f32> = NodeSize::from_rem(26.0 * 0.25);
    /// ## Standard Size 27
    pub const S_27: NodeSize<f32> = NodeSize::from_rem(27.0 * 0.25);
    /// ## Standard Size 28
    pub const S_28: NodeSize<f32> = NodeSize::from_rem(28.0 * 0.25);
    /// ## Standard Size 29
    pub const S_29: NodeSize<f32> = NodeSize::from_rem(29.0 * 0.25);
    /// ## Standard Size 30
    pub const S_30: NodeSize<f32> = NodeSize::from_rem(30.0 * 0.25);
    /// ## Standard Size 31
    pub const S_31: NodeSize<f32> = NodeSize::from_rem(31.0 * 0.25);
    /// ## Standard Size 32
    pub const S_32: NodeSize<f32> = NodeSize::from_rem(32.0 * 0.25);
    /// ## Standard Size 33
    pub const S_33: NodeSize<f32> = NodeSize::from_rem(33.0 * 0.25);
    /// ## Standard Size 34
    pub const S_34: NodeSize<f32> = NodeSize::from_rem(34.0 * 0.25);
    /// ## Standard Size 35
    pub const S_35: NodeSize<f32> = NodeSize::from_rem(35.0 * 0.25);
    /// ## Standard Size 36
    pub const S_36: NodeSize<f32> = NodeSize::from_rem(36.0 * 0.25);
    /// ## Standard Size 37
    pub const S_37: NodeSize<f32> = NodeSize::from_rem(37.0 * 0.25);
    /// ## Standard Size 38
    pub const S_38: NodeSize<f32> = NodeSize::from_rem(38.0 * 0.25);
    /// ## Standard Size 39
    pub const S_39: NodeSize<f32> = NodeSize::from_rem(39.0 * 0.25);
    /// ## Standard Size 40
    pub const S_40: NodeSize<f32> = NodeSize::from_rem(40.0 * 0.25);
    /// ## Standard Size 41
    pub const S_41: NodeSize<f32> = NodeSize::from_rem(41.0 * 0.25);
    /// ## Standard Size 42
    pub const S_42: NodeSize<f32> = NodeSize::from_rem(42.0 * 0.25);
    /// ## Standard Size 43
    pub const S_43: NodeSize<f32> = NodeSize::from_rem(43.0 * 0.25);
    /// ## Standard Size 44
    pub const S_44: NodeSize<f32> = NodeSize::from_rem(44.0 * 0.25);
    /// ## Standard Size 45
    pub const S_45: NodeSize<f32> = NodeSize::from_rem(45.0 * 0.25);
    /// ## Standard Size 46
    pub const S_46: NodeSize<f32> = NodeSize::from_rem(46.0 * 0.25);
    /// ## Standard Size 47
    pub const S_47: NodeSize<f32> = NodeSize::from_rem(47.0 * 0.25);
    /// ## Standard Size 48
    pub const S_48: NodeSize<f32> = NodeSize::from_rem(48.0 * 0.25);
    /// ## Standard Size 49
    pub const S_49: NodeSize<f32> = NodeSize::from_rem(49.0 * 0.25);
    /// ## Standard Size 50
    pub const S_50: NodeSize<f32> = NodeSize::from_rem(50.0 * 0.25);
    /// ## Standard Size 51
    pub const S_51: NodeSize<f32> = NodeSize::from_rem(51.0 * 0.25);
    /// ## Standard Size 52
    pub const S_52: NodeSize<f32> = NodeSize::from_rem(52.0 * 0.25);
    /// ## Standard Size 53
    pub const S_53: NodeSize<f32> = NodeSize::from_rem(53.0 * 0.25);
    /// ## Standard Size 54
    pub const S_54: NodeSize<f32> = NodeSize::from_rem(54.0 * 0.25);
    /// ## Standard Size 55
    pub const S_55: NodeSize<f32> = NodeSize::from_rem(55.0 * 0.25);
    /// ## Standard Size 56
    pub const S_56: NodeSize<f32> = NodeSize::from_rem(56.0 * 0.25);
    /// ## Standard Size 57
    pub const S_57: NodeSize<f32> = NodeSize::from_rem(57.0 * 0.25);
    /// ## Standard Size 58
    pub const S_58: NodeSize<f32> = NodeSize::from_rem(58.0 * 0.25);
    /// ## Standard Size 59
    pub const S_59: NodeSize<f32> = NodeSize::from_rem(59.0 * 0.25);
    /// ## Standard Size 64
    pub const S_60: NodeSize<f32> = NodeSize::from_rem(60.0 * 0.25);
    /// ## Standard Size 61
    pub const S_61: NodeSize<f32> = NodeSize::from_rem(61.0 * 0.25);
    /// ## Standard Size 62
    pub const S_62: NodeSize<f32> = NodeSize::from_rem(62.0 * 0.25);
    /// ## Standard Size 63
    pub const S_63: NodeSize<f32> = NodeSize::from_rem(63.0 * 0.25);
    /// ## Standard Size 64
    pub const S_64: NodeSize<f32> = NodeSize::from_rem(64.0 * 0.25);
    /// ## Standard Size 65
    pub const S_65: NodeSize<f32> = NodeSize::from_rem(65.0 * 0.25);
    /// ## Standard Size 66
    pub const S_66: NodeSize<f32> = NodeSize::from_rem(66.0 * 0.25);
    /// ## Standard Size 67
    pub const S_67: NodeSize<f32> = NodeSize::from_rem(67.0 * 0.25);
    /// ## Standard Size 68
    pub const S_68: NodeSize<f32> = NodeSize::from_rem(68.0 * 0.25);
    /// ## Standard Size 69
    pub const S_69: NodeSize<f32> = NodeSize::from_rem(69.0 * 0.25);
    /// ## Standard Size 70
    pub const S_70: NodeSize<f32> = NodeSize::from_rem(70.0 * 0.25);
    /// ## Standard Size 71
    pub const S_71: NodeSize<f32> = NodeSize::from_rem(71.0 * 0.25);
    /// ## Standard Size 72
    pub const S_72: NodeSize<f32> = NodeSize::from_rem(72.0 * 0.25);
    /// ## Standard Size 73
    pub const S_73: NodeSize<f32> = NodeSize::from_rem(73.0 * 0.25);
    /// ## Standard Size 74
    pub const S_74: NodeSize<f32> = NodeSize::from_rem(74.0 * 0.25);
    /// ## Standard Size 75
    pub const S_75: NodeSize<f32> = NodeSize::from_rem(75.0 * 0.25);
    /// ## Standard Size 76
    pub const S_76: NodeSize<f32> = NodeSize::from_rem(76.0 * 0.25);
    /// ## Standard Size 77
    pub const S_77: NodeSize<f32> = NodeSize::from_rem(77.0 * 0.25);
    /// ## Standard Size 78
    pub const S_78: NodeSize<f32> = NodeSize::from_rem(78.0 * 0.25);
    /// ## Standard Size 79
    pub const S_79: NodeSize<f32> = NodeSize::from_rem(79.0 * 0.25);
    /// ## Standard Size 80
    pub const S_80: NodeSize<f32> = NodeSize::from_rem(80.0 * 0.25);
    /// ## Standard Size 81
    pub const S_81: NodeSize<f32> = NodeSize::from_rem(81.0 * 0.25);
    /// ## Standard Size 82
    pub const S_82: NodeSize<f32> = NodeSize::from_rem(82.0 * 0.25);
    /// ## Standard Size 83
    pub const S_83: NodeSize<f32> = NodeSize::from_rem(83.0 * 0.25);
    /// ## Standard Size 84
    pub const S_84: NodeSize<f32> = NodeSize::from_rem(84.0 * 0.25);
    /// ## Standard Size 85
    pub const S_85: NodeSize<f32> = NodeSize::from_rem(85.0 * 0.25);
    /// ## Standard Size 86
    pub const S_86: NodeSize<f32> = NodeSize::from_rem(86.0 * 0.25);
    /// ## Standard Size 87
    pub const S_87: NodeSize<f32> = NodeSize::from_rem(87.0 * 0.25);
    /// ## Standard Size 88
    pub const S_88: NodeSize<f32> = NodeSize::from_rem(88.0 * 0.25);
    /// ## Standard Size 89
    pub const S_89: NodeSize<f32> = NodeSize::from_rem(89.0 * 0.25);
    /// ## Standard Size 90
    pub const S_90: NodeSize<f32> = NodeSize::from_rem(90.0 * 0.25);
    /// ## Standard Size 91
    pub const S_91: NodeSize<f32> = NodeSize::from_rem(91.0 * 0.25);
    /// ## Standard Size 92
    pub const S_92: NodeSize<f32> = NodeSize::from_rem(92.0 * 0.25);
    /// ## Standard Size 93
    pub const S_93: NodeSize<f32> = NodeSize::from_rem(93.0 * 0.25);
    /// ## Standard Size 94
    pub const S_94: NodeSize<f32> = NodeSize::from_rem(94.0 * 0.25);
    /// ## Standard Size 95
    pub const S_95: NodeSize<f32> = NodeSize::from_rem(95.0 * 0.25);
    /// ## Standard Size 96
    pub const S_96: NodeSize<f32> = NodeSize::from_rem(96.0 * 0.25);
    /// ## Standard Size 97
    pub const S_97: NodeSize<f32> = NodeSize::from_rem(97.0 * 0.25);
    /// ## Standard Size 98
    pub const S_98: NodeSize<f32> = NodeSize::from_rem(98.0 * 0.25);
    /// ## Standard Size 99
    pub const S_99: NodeSize<f32> = NodeSize::from_rem(99.0 * 0.25);
    /// ## Standard Size 100
    pub const S_100: NodeSize<f32> = NodeSize::from_rem(100.0 * 0.25);
}

pub type Size = NodeSize<f32>;