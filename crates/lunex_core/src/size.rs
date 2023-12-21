use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;  // For * f32
use std::ops::MulAssign;
use bevy::prelude::{Vec2, Vec3, Vec4};


// #====================#
// #=== UNIT TESTING ===#

#[cfg(test)]
mod test {
    use super::{Abs, Prc, Rem, Measurement};
    #[test]
    fn all () {
        assert_eq!(Measurement::new().with_abs(Abs(5)) + Abs(5) + Abs(5), Measurement::new().with_abs(Abs(15)));
        assert_eq!(Measurement::new().with_prc(Prc(5)) + Prc(5) + Prc(5), Measurement::new().with_prc(Prc(15)));
        assert_eq!(Measurement::new().with_rem(Rem(5)) + Rem(5) + Rem(5), Measurement::new().with_rem(Rem(15)));

        let amount = Abs(5) + Prc(10) + Rem(15);
        assert_eq!(amount, Measurement::new().with_abs(Abs(5)).with_prc(Prc(10)).with_rem(Rem(15)));

        let mut new_amount = amount + Abs(20);
        assert_eq!(new_amount, Measurement::new().with_abs(Abs(25)).with_prc(Prc(10)).with_rem(Rem(15)));

        new_amount += Prc(20);
        assert_eq!(new_amount, Measurement::new().with_abs(Abs(25)).with_prc(Prc(30)).with_rem(Rem(15)));

        new_amount += amount;
        assert_eq!(new_amount, Measurement::new().with_abs(Abs(30)).with_prc(Prc(40)).with_rem(Rem(30)));
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
/// A struct holding size measurement data used in UI.
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
/// # use lunex_core::{Measurement, Abs, Rem};
/// let a: Measurement<f32> = Abs(4.0) + Rem(16.0); // -> 4px + (16rem == 256px with font size 16px)
/// let b: Measurement<f32> = Prc(50.0).into();   // -> 50%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Measurement<T> {
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

impl <T> Measurement<T> {
    /// ## With
    /// Replaces the value of appropriate units with the new value.
    pub fn with(mut self, other: Measurement<T>) -> Self {
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
    pub fn set(&mut self, other: Measurement<T>) {
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

// # Impl into `Abs(T) -> Measurement(T)`
impl <T> Into<Measurement<T>> for Abs<T> {
    fn into(self) -> Measurement<T> {
        Measurement::new().with_abs(self)
    }
}
// # Impl into `Prc(T) -> Measurement(T)`
impl <T> Into<Measurement<T>> for Prc<T> {
    fn into(self) -> Measurement<T> {
        Measurement::new().with_prc(self)
    }
}
// # Impl into `Rem(T) -> Measurement(T)`
impl <T> Into<Measurement<T>> for Rem<T> {
    fn into(self) -> Measurement<T> {
        Measurement::new().with_rem(self)
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
    type Output = Measurement<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        Measurement::new().with_abs(self).with_prc(other)
    }
}
// # Impl `Abs(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Abs<T> {
    type Output = Measurement<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        Measurement::new().with_abs(self).with_rem(other)
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
    type Output = Measurement<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        Measurement::new().with_prc(self).with_abs(other)
    }
}
// # Impl `Prc(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Prc<T> {
    type Output = Measurement<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        Measurement::new().with_prc(self).with_rem(other)
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
    type Output = Measurement<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        Measurement::new().with_rem(self).with_abs(other)
    }
}
// # Impl `Rem(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Rem<T> {
    type Output = Measurement<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        Measurement::new().with_rem(self).with_prc(other)
    }
}

// # Impl `Measurement(T) + Measurement(T)`
impl<T: Add<Output = T> + Add> Add for Measurement<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {

        let mut output = Measurement::new();

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
// # Impl `Measurement(T) + Abs(T)`
impl<T: Add<Output = T> + Add> Add<Abs<T>> for Measurement<T> {
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
// # Impl `Measurement(T) + Prc(T)`
impl<T: Add<Output = T> + Add> Add<Prc<T>> for Measurement<T> {
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
// # Impl `Measurement(T) + Rem(T)`
impl<T: Add<Output = T> + Add> Add<Rem<T>> for Measurement<T> {
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

// # Impl `Measurement(T) += Measurement(T)`
impl<T: Add<Output = T> + Copy> AddAssign for Measurement<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
// # Impl `Measurement(T) += Abs(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Abs<T>> for Measurement<T> {
    fn add_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.set_abs(Abs(v + rhs.0)),
            None => self.set_abs(rhs),
        }
    }
}
// # Impl `Measurement(T) += Prc(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Prc<T>> for Measurement<T> {
    fn add_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.set_prc(Prc(v + rhs.0)),
            None => self.set_prc(rhs),
        }
    }
}
// # Impl `Measurement(T) += Rem(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Rem<T>> for Measurement<T> {
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
impl Measurement<f32> {
    /// ## From Standard
    /// Creates new measurement from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: f32) -> Measurement<f32> {
        Rem(size * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl Measurement<Vec2> {
    /// ## From Standard
    /// Creates new measurement from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec2>) -> Measurement<Vec2> {
        Rem(size.into() * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl Measurement<Vec3> {
    /// ## From Standard
    /// Creates new measurement from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec3>) -> Measurement<Vec3> {
        Rem(size.into() * 0.25).into()
    }
}
// # Impl from_standard TailwindCSS scale
impl Measurement<Vec4> {
    /// ## From Standard
    /// Creates new measurement from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<Vec4>) -> Measurement<Vec4> {
        Rem(size.into() * 0.25).into()
    }
}

/// ## Measurement Evaluate
/// Trait for implementing evaluation logic for (T)
pub trait MeasurementEvaluate<T> {
    /// ## Evaluate
    /// Evaluates the measurement for (T)
    fn evaluate(&self, parent_size: T, font_size: T) -> T;
}

impl MeasurementEvaluate<f32> for Measurement<f32> {
    fn evaluate(&self, parent_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl MeasurementEvaluate<Vec2> for Measurement<Vec2> {
    fn evaluate(&self, parent_size: Vec2, font_size: Vec2) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl MeasurementEvaluate<Vec3> for Measurement<Vec3> {
    fn evaluate(&self, parent_size: Vec3, font_size: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl MeasurementEvaluate<Vec4> for Measurement<Vec4> {
    fn evaluate(&self, parent_size: Vec4, font_size: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}

impl Measurement<Vec2> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
}
impl Measurement<Vec3> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
    }
}
impl Measurement<Vec4> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.x = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.x = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.x = v2 } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.y = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.y = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.y = v2 } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.z = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.z = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.z = v2 } }
    }
    /// ## With W
    /// Replaces the W value of appropriate units with the new value.
    pub fn with_w(self, other: Measurement<f32>) -> Self {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.w = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.w = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.w = v2 } }
        self
    }
    /// ## Set W
    /// Sets the W value of appropriate units with the new value.
    pub fn set_w(&mut self, other: Measurement<f32>) {
        if let Some(mut v1) = self.abs { if let Some(v2) = other.abs { v1.w = v2 } }
        if let Some(mut v1) = self.prc { if let Some(v2) = other.prc { v1.w = v2 } }
        if let Some(mut v1) = self.rem { if let Some(v2) = other.rem { v1.w = v2 } }
    }
}


// #================================#
// #=== CONSTANT IMPLEMENTATIONS ===#

impl <T> Measurement<T> {
    /// ## New
    /// Creates new empty measurement
    pub const fn new() -> Self {
        Measurement {
            abs: None,
            prc: None,
            rem: None,
        }
    }
    /// ## From absolute
    /// Creates new measurement
    pub const fn from_abs(abs: T) -> Measurement<T> {
        Measurement {
            abs: Some(abs),
            prc: None,
            rem: None,
        }
    }
    /// ## From percentage
    /// Creates new measurement
    pub const fn from_prc(prc: T) -> Measurement<T> {
        Measurement {
            abs: None,
            prc: Some(prc),
            rem: None,
        }
    }
    /// ## From rem
    /// Creates new measurement
    pub const fn from_rem(rem: T) -> Measurement<T> {
        Measurement {
            abs: None,
            prc: None,
            rem: Some(rem),
        }
    }
    /// ## From absolute & percentage
    /// Creates new measurement
    pub const fn from_abs_prc(abs: T, prc: T) -> Measurement<T> {
        Measurement {
            abs: Some(abs),
            prc: Some(prc),
            rem: None,
        }
    }
    /// ## From absolute & rem
    /// Creates new measurement
    pub const fn from_abs_rem(abs: T, rem: T) -> Measurement<T> {
        Measurement {
            abs: Some(abs),
            prc: None,
            rem: Some(rem),
        }
    }
    /// ## From percentage & rem
    /// Creates new measurement
    pub const fn from_prc_rem(prc: T, rem: T) -> Measurement<T> {
        Measurement {
            abs: None,
            prc: Some(prc),
            rem: Some(rem),
        }
    }
    /// ## From absolute & percentage & rem
    /// Creates new measurement
    pub const fn from_abs_prc_rem(abs: T, prc: T, rem: T) -> Measurement<T> {
        Measurement {
            abs: Some(abs),
            prc: Some(prc),
            rem: Some(rem),
        }
    }
}
