use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use crate::import::*;
use super::NiceDisplay;


// #====================#
// #=== UNIT TESTING ===#

#[cfg(test)]
mod test {
    use super::{Abs, Prc, Rem, NodeSize, Vec2};
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

        let node: NodeSize<Vec2> = Rem(Vec2::new(10.0, 12.0)).into();
        assert_eq!(node, NodeSize::<Vec2>::new().with_x(Rem(10.0)).with_y(Rem(12.0)));

        let _: NodeSize<Vec2> = NodeSize::from_standard((1.0, 2.0));

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
/// * [`Abs`] = Absolute units
/// * [`Prc`] = Percentage units
/// * [`Rem`] = Rem units
/// 
/// ### Support
/// First class implementations for (`T`)
/// * [`f32`]
/// * [`Vec2`]
/// * [`Vec3`]
/// * [`Vec4`]
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

// # Impl `with_abs` and `set_abs` ...
impl<T> NodeSize<T> {
    /// ## With
    /// Replaces the value of appropriate units with the new value.
    pub fn with(mut self, other: NodeSize<T>) -> Self {
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { *v1 = v2 } else { self.abs = Some(v2) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { *v1 = v2 } else { self.prc = Some(v2) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { *v1 = v2 } else { self.rem = Some(v2) } }
        self
    }
    /// ## With Absolute
    /// Replaces the value with the new `absolute` value.
    pub fn with_abs(mut self, abs: Abs<T>) -> Self {
        self.abs = Some(abs.0);
        self
    }
    /// ## With Percentage
    /// Replaces the value with the new `percentage` value.
    pub fn with_prc(mut self, prc: Prc<T>) -> Self {
        self.prc = Some(prc.0);
        self
    }
    /// ## With Rem
    /// Replaces the value with the new `rem` value.
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
    /// Sets the value to the new `absolute` value.
    pub fn set_abs(&mut self, abs: Abs<T>) {
        self.abs = Some(abs.0);
    }
    /// ## Set Percentage
    /// Sets the value to the new `percentage` value.
    pub fn set_prc(&mut self, prc: Prc<T>) {
        self.prc = Some(prc.0);
    }
    /// ## Set Rem
    /// Sets the value to the new `rem` value.
    pub fn set_rem(&mut self, rem: Rem<T>) {
        self.rem = Some(rem.0);
    }
}

// # Impl `from_standard` Tailwind scale
impl<T: Mul<f32, Output = T>> NodeSize<T> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<T>) -> NodeSize<T> {
        Rem(size.into() * 0.25).into()
    }
}


// CONVERSION ======

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


// ADDITION ======

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
                Some(v2) => output.abs = Some(v1 + v2),
                None => output.abs = Some(v1),
            }
        }
        if let Some(v1) = self.prc {
            match other.prc {
                Some(v2) => output.prc = Some(v1 + v2),
                None => output.prc = Some(v1),
            }
        }
        if let Some(v1) = self.rem {
            match other.rem {
                Some(v2) => output.rem = Some(v1 + v2),
                None => output.rem = Some(v1),
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
            Some(v) => self.abs = Some(v + rhs.0),
            None => self.abs = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) += Prc(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Prc<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.prc = Some(v + rhs.0),
            None => self.prc = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) += Rem(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Rem<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.rem = Some(v + rhs.0),
            None => self.rem = Some(rhs.0),
        }
    }
}

// NEGATION ======

impl<T: Neg<Output = T>> Neg for Abs<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Abs(-self.0)
    }
}
impl<T: Neg<Output = T>> Neg for Prc<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Prc(-self.0)
    }
}
impl<T: Neg<Output = T>> Neg for Rem<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Rem(-self.0)
    }
}


// SUBTRACTION ======

// # Impl `Abs(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub for Abs<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Abs(self.0 - other.0)
    }
}
// # Impl `Abs(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub<Prc<T>> for Abs<T> where Prc<T>: Neg<Output = Prc<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Prc<T>) -> Self::Output {
        NodeSize::new().with_abs(self).with_prc(-other)
    }
}
// # Impl `Abs(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub<Rem<T>> for Abs<T> where Rem<T>: Neg<Output = Rem<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Rem<T>) -> Self::Output {
        NodeSize::new().with_abs(self).with_rem(-other)
    }
}

// # Impl `Prc(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub for Prc<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Prc(self.0 - other.0)
    }
}
// # Impl `Prc(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub<Abs<T>> for Prc<T> where Abs<T>: Neg<Output = Abs<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Abs<T>) -> Self::Output {
        NodeSize::new().with_prc(self).with_abs(-other)
    }
}
// # Impl `Prc(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub<Rem<T>> for Prc<T> where Rem<T>: Neg<Output = Rem<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Rem<T>) -> Self::Output {
        NodeSize::new().with_prc(self).with_rem(-other)
    }
}

// # Impl `Rem(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub for Rem<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Rem(self.0 - other.0)
    }
}
// # Impl `Rem(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub<Abs<T>> for Rem<T> where Abs<T>: Neg<Output = Abs<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Abs<T>) -> Self::Output {
        NodeSize::new().with_rem(self).with_abs(-other)
    }
}
// # Impl `Rem(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub<Prc<T>> for Rem<T> where Prc<T>: Neg<Output = Prc<T>> {
    type Output = NodeSize<T>;
    fn sub(self, other: Prc<T>) -> Self::Output {
        NodeSize::new().with_rem(self).with_prc(-other)
    }
}

// # Impl `NodeSize(T) - NodeSize(T)`
impl<T: Sub<Output = T> + Sub + Neg<Output = T>> Sub for NodeSize<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let mut output = NodeSize::new();

        if let Some(v1) = self.abs {
            match other.abs {
                Some(v2) => output.abs = Some(v1 - v2),
                None => output.abs = Some(v1),
            }
        } else {
            if let Some(v2) = other.abs {
                output.abs = Some(-v2)
            }
        }

        if let Some(v1) = self.prc {
            match other.prc {
                Some(v2) => output.prc = Some(v1 - v2),
                None => output.prc = Some(v1),
            }
        } else {
            if let Some(v2) = other.prc {
                output.prc = Some(-v2)
            }
        }

        if let Some(v1) = self.rem {
            match other.rem {
                Some(v2) => output.rem = Some(v1 - v2),
                None => output.rem = Some(v1),
            }
        } else {
            if let Some(v2) = other.rem {
                output.rem = Some(-v2)
            }
        }

        output
    }
}
// # Impl `NodeSize(T) - Abs(T)`
impl<T: Sub<Output = T> + Sub> Sub<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Abs<T>) -> Self::Output {
        match self.abs {
            Some(v) => {
                self.abs = Some(v - other.0);
                self
            },
            None => self.with_abs(other),
        }
    }
}
// # Impl `NodeSize(T) - Prc(T)`
impl<T: Sub<Output = T> + Sub> Sub<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Prc<T>) -> Self::Output {
        match self.prc {
            Some(v) => {
                self.prc = Some(v - other.0);
                self
            },
            None => self.with_prc(other),
        }
    }
}
// # Impl `NodeSize(T) - Rem(T)`
impl<T: Sub<Output = T> + Sub> Sub<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Rem<T>) -> Self::Output {
        match self.rem {
            Some(v) => {
                self.rem = Some(v - other.0);
                self
            },
            None => self.with_rem(other),
        }
    }
}

// # Impl `NodeSize(T) -= NodeSize(T)`
impl<T: Sub<Output = T> + Copy + Neg<Output = T>> SubAssign for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
// # Impl `NodeSize(T) -= Abs(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Abs<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.abs = Some(v - rhs.0),
            None => self.abs = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) -= Prc(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Prc<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.prc = Some(v - rhs.0),
            None => self.prc = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) -= Rem(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Rem<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.rem = Some(v - rhs.0),
            None => self.rem = Some(rhs.0),
        }
    }
}


// MULTIPLICATION ======

// # Impl `Abs(T) * Abs(T)`
impl<T: Mul<Output = T>> Mul for Abs<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Abs(self.0 * other.0)
    }
}
// # Impl `Prc(T) * Prc(T)`
impl<T: Mul<Output = T>> Mul for Prc<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Prc(self.0 * other.0)
    }
}
// # Impl `Rem(T) * Rem(T)`
impl<T: Mul<Output = T>> Mul for Rem<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Rem(self.0 * other.0)
    }
}

// # Impl `NodeSize(T) * NodeSize(T)`
impl<T: Mul<Output = T> + Mul> Mul for NodeSize<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut output = NodeSize::new();
        if let Some(v1) = self.abs {
            if let Some(v2) = other.abs {
                output.abs = Some(v1 * v2);
            }
        }
        if let Some(v1) = self.prc {
            if let Some(v2) = other.prc {
                output.prc = Some(v1 * v2);
            }
        }
        if let Some(v1) = self.rem {
            if let Some(v2) = other.rem {
                output.rem = Some(v1 * v2);
            }
        }
        output
    }
}
// # Impl `NodeSize(T) * Abs(T)`
impl<T: Mul<Output = T> + Mul> Mul<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Abs<T>) -> Self::Output {
        if let Some(v) = self.abs {
            self.abs = Some(v * other.0);
        }
        self
    }
}
// # Impl `NodeSize(T) * Prc(T)`
impl<T: Mul<Output = T> + Mul> Mul<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Prc<T>) -> Self::Output {
        if let Some(v) = self.prc {
            self.prc = Some(v * other.0);
        }
        self
    }
}
// # Impl `NodeSize(T) * Rem(T)`
impl<T: Mul<Output = T> + Mul> Mul<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Rem<T>) -> Self::Output {
        if let Some(v) = self.rem {
            self.rem = Some(v * other.0);
        }
        self
    }
}

// # Impl `NodeSize(T) *= NodeSize(T)`
impl<T: Mul<Output = T> + Copy> MulAssign for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
// # Impl `NodeSize(T) *= Abs(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Abs<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Abs<T>) {
        if let Some(v) = self.abs {
            self.abs = Some(v * rhs.0);
        }
    }
}
// # Impl `NodeSize(T) *= Prc(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Prc<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Prc<T>) {
        if let Some(v) = self.prc {
            self.prc = Some(v * rhs.0);
        }
    }
}
// # Impl `NodeSize(T) *= Rem(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Rem<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Rem<T>) {
        if let Some(v) = self.rem {
            self.rem = Some(v * rhs.0);
        }
    }
}


// MULTIPLICATION with F32 ======

// # Impl `NodeSize(T) * f32 = NodeSize(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for NodeSize<T> {
    type Output = NodeSize<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = NodeSize::new();
        if let Some(v) = self.abs {
            output.abs = Some(v * rhs);
        }
        if let Some(v) = self.prc {
            output.prc = Some(v * rhs);
        }
        if let Some(v) = self.rem {
            output.rem = Some(v * rhs);
        }
        output
    }
}
// # Impl `Abs(T) * f32 = Abs(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Abs<T> {
    type Output = Abs<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Abs(self.0 * rhs)
    }
}
// # Impl `Prc(T) * f32 = Prc(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Prc<T> {
    type Output = Prc<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Prc(self.0 * rhs)
    }
}
// # Impl `Rem(T) * f32 = Rem(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Rem<T> {
    type Output = Rem<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Rem(self.0 * rhs)
    }
}

// # Impl `NodeSize(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}
// # Impl `Abs(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Abs<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Abs(self.0 * rhs);
    }
}
// # Impl `Prc(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Prc<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Prc(self.0 * rhs);
    }
}
// # Impl `Rem(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Rem<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Rem(self.0 * rhs);
    }
}


// #================================#
// #=== SPECIFIC IMPLEMENTATIONS ===#

// # Impl `splat2`
impl Abs<Vec2> {
    /// ### Same as
    /// ```no_run
    /// Abs(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Abs(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Abs<Vec3> {
    /// ### Same as
    /// ```no_run
    /// Abs(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Abs(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Abs<Vec4> {
    /// ### Same as
    /// ```no_run
    /// Abs(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Abs(Vec4::splat(v))
    }
}

// # Impl `splat2`
impl Prc<Vec2> {
    /// ### Same as
    /// ```no_run
    /// Prc(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Prc(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Prc<Vec3> {
    /// ### Same as
    /// ```no_run
    /// Prc(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Prc(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Prc<Vec4> {
    /// ### Same as
    /// ```no_run
    /// Prc(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Prc(Vec4::splat(v))
    }
}

// # Impl `splat2`
impl Rem<Vec2> {
    /// ### Same as
    /// ```no_run
    /// Rem(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Rem(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Rem<Vec3> {
    /// ### Same as
    /// ```no_run
    /// Rem(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Rem(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Rem<Vec4> {
    /// ### Same as
    /// ```no_run
    /// Rem(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Rem(Vec4::splat(v))
    }
}


// # Impl `with_x` and `set_x` ...
impl NodeSize<Vec2> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec2::new(v2, 0.0)) } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec2::new(v2, 0.0)) } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec2::new(0.0, v2)) } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec2::new(0.0, v2)) } }
    }
}
// # Impl `with_x` and `set_x` ...
impl NodeSize<Vec3> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec3::new(v2, 0.0, 0.0)) } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec3::new(v2, 0.0, 0.0)) } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec3::new(0.0, v2, 0.0)) } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec3::new(0.0, v2, 0.0)) } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec3::new(0.0, 0.0, v2)) } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec3::new(0.0, 0.0, v2)) } }
    }
}
// # Impl `with_x` and `set_x` ...
impl NodeSize<Vec4> {
    /// ## With X
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        self
    }
    /// ## Set X
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
    }
    /// ## With Y
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        self
    }
    /// ## Set Y
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
    }
    /// ## With Z
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        self
    }
    /// ## Set Z
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
    }
    /// ## With W
    /// Replaces the W value of appropriate units with the new value.
    pub fn with_w(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.w = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.w = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.w = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        self
    }
    /// ## Set W
    /// Sets the W value of appropriate units with the new value.
    pub fn set_w(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.w = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.w = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.w = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
    }
}



/// ## NodeSize Evaluate
/// Trait for implementing evaluation logic for (T)
pub trait NodeSizeEvaluate<T, TT> {
    /// ## Evaluate
    /// Evaluates the NodeSize for (T)
    fn evaluate(&self, parent_size: T, font_size: TT) -> T;
}

impl NodeSizeEvaluate<f32, f32> for NodeSize<f32> {
    fn evaluate(&self, parent_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec2, f32> for NodeSize<Vec2> {
    fn evaluate(&self, parent_size: Vec2, font_size: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec3, f32> for NodeSize<Vec3> {
    fn evaluate(&self, parent_size: Vec3, font_size: f32) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec4, f32> for NodeSize<Vec4> {
    fn evaluate(&self, parent_size: Vec4, font_size: f32) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
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

// # MOST LIKELY NOT NEEDED AND DEPRACTED!
impl NodeSize<f32> {
    /// ## Abs - Extra-small
    pub const A_XS: NodeSize<f32> = NodeSize::A_4;
    /// ## Abs - Small
    pub const A_SM: NodeSize<f32> = NodeSize::A_8;
    /// ## Abs - Medium
    pub const A_MD: NodeSize<f32> = NodeSize::A_12;
    /// ## Abs - Large
    pub const A_LG: NodeSize<f32> = NodeSize::A_16;
    /// ## Abs - Extra-large
    pub const A_XL: NodeSize<f32> = NodeSize::A_24;
    /// ## Abs - Extra-large 2
    pub const A_XL2: NodeSize<f32> = NodeSize::A_32;
    /// ## Abs - Extra-large 3
    pub const A_XL3: NodeSize<f32> = NodeSize::A_40;
    /// ## Abs - Extra-large 4
    pub const A_XL4: NodeSize<f32> = NodeSize::A_48;
    /// ## Abs - Extra-large 5
    pub const A_XL5: NodeSize<f32> = NodeSize::A_56;
    /// ## Abs - Extra-large 6
    pub const A_XL6: NodeSize<f32> = NodeSize::A_64;
    /// ## Abs - Extra-large 7
    pub const A_XL7: NodeSize<f32> = NodeSize::A_72;

    /// ## Abs - Standard Size 0
    pub const A_0: NodeSize<f32> = NodeSize::from_abs(0.0);
    /// ## Abs - Standard Size 1
    pub const A_1: NodeSize<f32> = NodeSize::from_abs(1.0 * 4.0);
    /// ## Abs - Standard Size 2
    pub const A_2: NodeSize<f32> = NodeSize::from_abs(2.0 * 4.0);
    /// ## Abs - Standard Size 3
    pub const A_3: NodeSize<f32> = NodeSize::from_abs(3.0 * 4.0);
    /// ## Abs - Standard Size 4
    pub const A_4: NodeSize<f32> = NodeSize::from_abs(4.0 * 4.0);
    /// ## Abs - Standard Size 5
    pub const A_5: NodeSize<f32> = NodeSize::from_abs(5.0 * 4.0);
    /// ## Abs - Standard Size 6
    pub const A_6: NodeSize<f32> = NodeSize::from_abs(6.0 * 4.0);
    /// ## Abs - Standard Size 7
    pub const A_7: NodeSize<f32> = NodeSize::from_abs(7.0 * 4.0);
    /// ## Abs - Standard Size 8
    pub const A_8: NodeSize<f32> = NodeSize::from_abs(8.0 * 4.0);
    /// ## Abs - Standard Size 9
    pub const A_9: NodeSize<f32> = NodeSize::from_abs(9.0 * 4.0);
    /// ## Abs - Standard Size 10
    pub const A_10: NodeSize<f32> = NodeSize::from_abs(10.0 * 4.0);
    /// ## Abs - Standard Size 11
    pub const A_11: NodeSize<f32> = NodeSize::from_abs(11.0 * 4.0);
    /// ## Abs - Standard Size 12
    pub const A_12: NodeSize<f32> = NodeSize::from_abs(12.0 * 4.0);
    /// ## Abs - Standard Size 13
    pub const A_13: NodeSize<f32> = NodeSize::from_abs(13.0 * 4.0);
    /// ## Abs - Standard Size 14
    pub const A_14: NodeSize<f32> = NodeSize::from_abs(14.0 * 4.0);
    /// ## Abs - Standard Size 15
    pub const A_15: NodeSize<f32> = NodeSize::from_abs(15.0 * 4.0);
    /// ## Abs - Standard Size 16
    pub const A_16: NodeSize<f32> = NodeSize::from_abs(16.0 * 4.0);
    /// ## Abs - Standard Size 17
    pub const A_17: NodeSize<f32> = NodeSize::from_abs(17.0 * 4.0);
    /// ## Abs - Standard Size 18
    pub const A_18: NodeSize<f32> = NodeSize::from_abs(18.0 * 4.0);
    /// ## Abs - Standard Size 19
    pub const A_19: NodeSize<f32> = NodeSize::from_abs(19.0 * 4.0);
    /// ## Abs - Standard Size 20
    pub const A_20: NodeSize<f32> = NodeSize::from_abs(20.0 * 4.0);
    /// ## Abs - Standard Size 21
    pub const A_21: NodeSize<f32> = NodeSize::from_abs(21.0 * 4.0);
    /// ## Abs - Standard Size 22
    pub const A_22: NodeSize<f32> = NodeSize::from_abs(22.0 * 4.0);
    /// ## Abs - Standard Size 23
    pub const A_23: NodeSize<f32> = NodeSize::from_abs(23.0 * 4.0);
    /// ## Abs - Standard Size 24
    pub const A_24: NodeSize<f32> = NodeSize::from_abs(24.0 * 4.0);
    /// ## Abs - Standard Size 25
    pub const A_25: NodeSize<f32> = NodeSize::from_abs(25.0 * 4.0);
    /// ## Abs - Standard Size 26
    pub const A_26: NodeSize<f32> = NodeSize::from_abs(26.0 * 4.0);
    /// ## Abs - Standard Size 27
    pub const A_27: NodeSize<f32> = NodeSize::from_abs(27.0 * 4.0);
    /// ## Abs - Standard Size 28
    pub const A_28: NodeSize<f32> = NodeSize::from_abs(28.0 * 4.0);
    /// ## Abs - Standard Size 29
    pub const A_29: NodeSize<f32> = NodeSize::from_abs(29.0 * 4.0);
    /// ## Abs - Standard Size 30
    pub const A_30: NodeSize<f32> = NodeSize::from_abs(30.0 * 4.0);
    /// ## Abs - Standard Size 31
    pub const A_31: NodeSize<f32> = NodeSize::from_abs(31.0 * 4.0);
    /// ## Abs - Standard Size 32
    pub const A_32: NodeSize<f32> = NodeSize::from_abs(32.0 * 4.0);
    /// ## Abs - Standard Size 33
    pub const A_33: NodeSize<f32> = NodeSize::from_abs(33.0 * 4.0);
    /// ## Abs - Standard Size 34
    pub const A_34: NodeSize<f32> = NodeSize::from_abs(34.0 * 4.0);
    /// ## Abs - Standard Size 35
    pub const A_35: NodeSize<f32> = NodeSize::from_abs(35.0 * 4.0);
    /// ## Abs - Standard Size 36
    pub const A_36: NodeSize<f32> = NodeSize::from_abs(36.0 * 4.0);
    /// ## Abs - Standard Size 37
    pub const A_37: NodeSize<f32> = NodeSize::from_abs(37.0 * 4.0);
    /// ## Abs - Standard Size 38
    pub const A_38: NodeSize<f32> = NodeSize::from_abs(38.0 * 4.0);
    /// ## Abs - Standard Size 39
    pub const A_39: NodeSize<f32> = NodeSize::from_abs(39.0 * 4.0);
    /// ## Abs - Standard Size 40
    pub const A_40: NodeSize<f32> = NodeSize::from_abs(40.0 * 4.0);
    /// ## Abs - Standard Size 41
    pub const A_41: NodeSize<f32> = NodeSize::from_abs(41.0 * 4.0);
    /// ## Abs - Standard Size 42
    pub const A_42: NodeSize<f32> = NodeSize::from_abs(42.0 * 4.0);
    /// ## Abs - Standard Size 43
    pub const A_43: NodeSize<f32> = NodeSize::from_abs(43.0 * 4.0);
    /// ## Abs - Standard Size 44
    pub const A_44: NodeSize<f32> = NodeSize::from_abs(44.0 * 4.0);
    /// ## Abs - Standard Size 45
    pub const A_45: NodeSize<f32> = NodeSize::from_abs(45.0 * 4.0);
    /// ## Abs - Standard Size 46
    pub const A_46: NodeSize<f32> = NodeSize::from_abs(46.0 * 4.0);
    /// ## Abs - Standard Size 47
    pub const A_47: NodeSize<f32> = NodeSize::from_abs(47.0 * 4.0);
    /// ## Abs - Standard Size 48
    pub const A_48: NodeSize<f32> = NodeSize::from_abs(48.0 * 4.0);
    /// ## Abs - Standard Size 49
    pub const A_49: NodeSize<f32> = NodeSize::from_abs(49.0 * 4.0);
    /// ## Abs - Standard Size 50
    pub const A_50: NodeSize<f32> = NodeSize::from_abs(50.0 * 4.0);
    /// ## Abs - Standard Size 51
    pub const A_51: NodeSize<f32> = NodeSize::from_abs(51.0 * 4.0);
    /// ## Abs - Standard Size 52
    pub const A_52: NodeSize<f32> = NodeSize::from_abs(52.0 * 4.0);
    /// ## Abs - Standard Size 53
    pub const A_53: NodeSize<f32> = NodeSize::from_abs(53.0 * 4.0);
    /// ## Abs - Standard Size 54
    pub const A_54: NodeSize<f32> = NodeSize::from_abs(54.0 * 4.0);
    /// ## Abs - Standard Size 55
    pub const A_55: NodeSize<f32> = NodeSize::from_abs(55.0 * 4.0);
    /// ## Abs - Standard Size 56
    pub const A_56: NodeSize<f32> = NodeSize::from_abs(56.0 * 4.0);
    /// ## Abs - Standard Size 57
    pub const A_57: NodeSize<f32> = NodeSize::from_abs(57.0 * 4.0);
    /// ## Abs - Standard Size 58
    pub const A_58: NodeSize<f32> = NodeSize::from_abs(58.0 * 4.0);
    /// ## Abs - Standard Size 59
    pub const A_59: NodeSize<f32> = NodeSize::from_abs(59.0 * 4.0);
    /// ## Abs - Standard Size 64
    pub const A_60: NodeSize<f32> = NodeSize::from_abs(60.0 * 4.0);
    /// ## Abs - Standard Size 61
    pub const A_61: NodeSize<f32> = NodeSize::from_abs(61.0 * 4.0);
    /// ## Abs - Standard Size 62
    pub const A_62: NodeSize<f32> = NodeSize::from_abs(62.0 * 4.0);
    /// ## Abs - Standard Size 63
    pub const A_63: NodeSize<f32> = NodeSize::from_abs(63.0 * 4.0);
    /// ## Abs - Standard Size 64
    pub const A_64: NodeSize<f32> = NodeSize::from_abs(64.0 * 4.0);
    /// ## Abs - Standard Size 65
    pub const A_65: NodeSize<f32> = NodeSize::from_abs(65.0 * 4.0);
    /// ## Abs - Standard Size 66
    pub const A_66: NodeSize<f32> = NodeSize::from_abs(66.0 * 4.0);
    /// ## Abs - Standard Size 67
    pub const A_67: NodeSize<f32> = NodeSize::from_abs(67.0 * 4.0);
    /// ## Abs - Standard Size 68
    pub const A_68: NodeSize<f32> = NodeSize::from_abs(68.0 * 4.0);
    /// ## Abs - Standard Size 69
    pub const A_69: NodeSize<f32> = NodeSize::from_abs(69.0 * 4.0);
    /// ## Abs - Standard Size 70
    pub const A_70: NodeSize<f32> = NodeSize::from_abs(70.0 * 4.0);
    /// ## Abs - Standard Size 71
    pub const A_71: NodeSize<f32> = NodeSize::from_abs(71.0 * 4.0);
    /// ## Abs - Standard Size 72
    pub const A_72: NodeSize<f32> = NodeSize::from_abs(72.0 * 4.0);
    /// ## Abs - Standard Size 73
    pub const A_73: NodeSize<f32> = NodeSize::from_abs(73.0 * 4.0);
    /// ## Abs - Standard Size 74
    pub const A_74: NodeSize<f32> = NodeSize::from_abs(74.0 * 4.0);
    /// ## Abs - Standard Size 75
    pub const A_75: NodeSize<f32> = NodeSize::from_abs(75.0 * 4.0);
    /// ## Abs - Standard Size 76
    pub const A_76: NodeSize<f32> = NodeSize::from_abs(76.0 * 4.0);
    /// ## Abs - Standard Size 77
    pub const A_77: NodeSize<f32> = NodeSize::from_abs(77.0 * 4.0);
    /// ## Abs - Standard Size 78
    pub const A_78: NodeSize<f32> = NodeSize::from_abs(78.0 * 4.0);
    /// ## Abs - Standard Size 79
    pub const A_79: NodeSize<f32> = NodeSize::from_abs(79.0 * 4.0);
    /// ## Abs - Standard Size 80
    pub const A_80: NodeSize<f32> = NodeSize::from_abs(80.0 * 4.0);
    /// ## Abs - Standard Size 81
    pub const A_81: NodeSize<f32> = NodeSize::from_abs(81.0 * 4.0);
    /// ## Abs - Standard Size 82
    pub const A_82: NodeSize<f32> = NodeSize::from_abs(82.0 * 4.0);
    /// ## Abs - Standard Size 83
    pub const A_83: NodeSize<f32> = NodeSize::from_abs(83.0 * 4.0);
    /// ## Abs - Standard Size 84
    pub const A_84: NodeSize<f32> = NodeSize::from_abs(84.0 * 4.0);
    /// ## Abs - Standard Size 85
    pub const A_85: NodeSize<f32> = NodeSize::from_abs(85.0 * 4.0);
    /// ## Abs - Standard Size 86
    pub const A_86: NodeSize<f32> = NodeSize::from_abs(86.0 * 4.0);
    /// ## Abs - Standard Size 87
    pub const A_87: NodeSize<f32> = NodeSize::from_abs(87.0 * 4.0);
    /// ## Abs - Standard Size 88
    pub const A_88: NodeSize<f32> = NodeSize::from_abs(88.0 * 4.0);
    /// ## Abs - Standard Size 89
    pub const A_89: NodeSize<f32> = NodeSize::from_abs(89.0 * 4.0);
    /// ## Abs - Standard Size 90
    pub const A_90: NodeSize<f32> = NodeSize::from_abs(90.0 * 4.0);
    /// ## Abs - Standard Size 91
    pub const A_91: NodeSize<f32> = NodeSize::from_abs(91.0 * 4.0);
    /// ## Abs - Standard Size 92
    pub const A_92: NodeSize<f32> = NodeSize::from_abs(92.0 * 4.0);
    /// ## Abs - Standard Size 93
    pub const A_93: NodeSize<f32> = NodeSize::from_abs(93.0 * 4.0);
    /// ## Abs - Standard Size 94
    pub const A_94: NodeSize<f32> = NodeSize::from_abs(94.0 * 4.0);
    /// ## Abs - Standard Size 95
    pub const A_95: NodeSize<f32> = NodeSize::from_abs(95.0 * 4.0);
    /// ## Abs - Standard Size 96
    pub const A_96: NodeSize<f32> = NodeSize::from_abs(96.0 * 4.0);
    /// ## Abs - Standard Size 97
    pub const A_97: NodeSize<f32> = NodeSize::from_abs(97.0 * 4.0);
    /// ## Abs - Standard Size 98
    pub const A_98: NodeSize<f32> = NodeSize::from_abs(98.0 * 4.0);
    /// ## Abs - Standard Size 99
    pub const A_99: NodeSize<f32> = NodeSize::from_abs(99.0 * 4.0);
    /// ## Abs - Standard Size 100
    pub const A_100: NodeSize<f32> = NodeSize::from_abs(100.0 * 4.0);

    /// ## Rem - Extra-small
    pub const R_XS: NodeSize<f32> = NodeSize::R_4;
    /// ## Rem - Small
    pub const R_SM: NodeSize<f32> = NodeSize::R_8;
    /// ## Rem - Medium
    pub const R_MD: NodeSize<f32> = NodeSize::R_12;
    /// ## Rem - Large
    pub const R_LG: NodeSize<f32> = NodeSize::R_16;
    /// ## Rem - Extra-large
    pub const R_XL: NodeSize<f32> = NodeSize::R_24;
    /// ## Rem - Extra-large 2
    pub const R_XL2: NodeSize<f32> = NodeSize::R_32;
    /// ## Rem - Extra-large 3
    pub const R_XL3: NodeSize<f32> = NodeSize::R_40;
    /// ## Rem - Extra-large 4
    pub const R_XL4: NodeSize<f32> = NodeSize::R_48;
    /// ## Rem - Extra-large 5
    pub const R_XL5: NodeSize<f32> = NodeSize::R_56;
    /// ## Rem - Extra-large 6
    pub const R_XL6: NodeSize<f32> = NodeSize::R_64;
    /// ## Rem - Extra-large 7
    pub const R_XL7: NodeSize<f32> = NodeSize::R_72;

    /// ## Rem - Standard Size 0
    pub const R_0: NodeSize<f32> = NodeSize::from_rem(0.0);
    /// ## Rem - Standard Size 1
    pub const R_1: NodeSize<f32> = NodeSize::from_rem(1.0 * 0.25);
    /// ## Rem - Standard Size 2
    pub const R_2: NodeSize<f32> = NodeSize::from_rem(2.0 * 0.25);
    /// ## Rem - Standard Size 3
    pub const R_3: NodeSize<f32> = NodeSize::from_rem(3.0 * 0.25);
    /// ## Rem - Standard Size 4
    pub const R_4: NodeSize<f32> = NodeSize::from_rem(4.0 * 0.25);
    /// ## Rem - Standard Size 5
    pub const R_5: NodeSize<f32> = NodeSize::from_rem(5.0 * 0.25);
    /// ## Rem - Standard Size 6
    pub const R_6: NodeSize<f32> = NodeSize::from_rem(6.0 * 0.25);
    /// ## Rem - Standard Size 7
    pub const R_7: NodeSize<f32> = NodeSize::from_rem(7.0 * 0.25);
    /// ## Rem - Standard Size 8
    pub const R_8: NodeSize<f32> = NodeSize::from_rem(8.0 * 0.25);
    /// ## Rem - Standard Size 9
    pub const R_9: NodeSize<f32> = NodeSize::from_rem(9.0 * 0.25);
    /// ## Rem - Standard Size 10
    pub const R_10: NodeSize<f32> = NodeSize::from_rem(10.0 * 0.25);
    /// ## Rem - Standard Size 11
    pub const R_11: NodeSize<f32> = NodeSize::from_rem(11.0 * 0.25);
    /// ## Rem - Standard Size 12
    pub const R_12: NodeSize<f32> = NodeSize::from_rem(12.0 * 0.25);
    /// ## Rem - Standard Size 13
    pub const R_13: NodeSize<f32> = NodeSize::from_rem(13.0 * 0.25);
    /// ## Rem - Standard Size 14
    pub const R_14: NodeSize<f32> = NodeSize::from_rem(14.0 * 0.25);
    /// ## Rem - Standard Size 15
    pub const R_15: NodeSize<f32> = NodeSize::from_rem(15.0 * 0.25);
    /// ## Rem - Standard Size 16
    pub const R_16: NodeSize<f32> = NodeSize::from_rem(16.0 * 0.25);
    /// ## Rem - Standard Size 17
    pub const R_17: NodeSize<f32> = NodeSize::from_rem(17.0 * 0.25);
    /// ## Rem - Standard Size 18
    pub const R_18: NodeSize<f32> = NodeSize::from_rem(18.0 * 0.25);
    /// ## Rem - Standard Size 19
    pub const R_19: NodeSize<f32> = NodeSize::from_rem(19.0 * 0.25);
    /// ## Rem - Standard Size 20
    pub const R_20: NodeSize<f32> = NodeSize::from_rem(20.0 * 0.25);
    /// ## Rem - Standard Size 21
    pub const R_21: NodeSize<f32> = NodeSize::from_rem(21.0 * 0.25);
    /// ## Rem - Standard Size 22
    pub const R_22: NodeSize<f32> = NodeSize::from_rem(22.0 * 0.25);
    /// ## Rem - Standard Size 23
    pub const R_23: NodeSize<f32> = NodeSize::from_rem(23.0 * 0.25);
    /// ## Rem - Standard Size 24
    pub const R_24: NodeSize<f32> = NodeSize::from_rem(24.0 * 0.25);
    /// ## Rem - Standard Size 25
    pub const R_25: NodeSize<f32> = NodeSize::from_rem(25.0 * 0.25);
    /// ## Rem - Standard Size 26
    pub const R_26: NodeSize<f32> = NodeSize::from_rem(26.0 * 0.25);
    /// ## Rem - Standard Size 27
    pub const R_27: NodeSize<f32> = NodeSize::from_rem(27.0 * 0.25);
    /// ## Rem - Standard Size 28
    pub const R_28: NodeSize<f32> = NodeSize::from_rem(28.0 * 0.25);
    /// ## Rem - Standard Size 29
    pub const R_29: NodeSize<f32> = NodeSize::from_rem(29.0 * 0.25);
    /// ## Rem - Standard Size 30
    pub const R_30: NodeSize<f32> = NodeSize::from_rem(30.0 * 0.25);
    /// ## Rem - Standard Size 31
    pub const R_31: NodeSize<f32> = NodeSize::from_rem(31.0 * 0.25);
    /// ## Rem - Standard Size 32
    pub const R_32: NodeSize<f32> = NodeSize::from_rem(32.0 * 0.25);
    /// ## Rem - Standard Size 33
    pub const R_33: NodeSize<f32> = NodeSize::from_rem(33.0 * 0.25);
    /// ## Rem - Standard Size 34
    pub const R_34: NodeSize<f32> = NodeSize::from_rem(34.0 * 0.25);
    /// ## Rem - Standard Size 35
    pub const R_35: NodeSize<f32> = NodeSize::from_rem(35.0 * 0.25);
    /// ## Rem - Standard Size 36
    pub const R_36: NodeSize<f32> = NodeSize::from_rem(36.0 * 0.25);
    /// ## Rem - Standard Size 37
    pub const R_37: NodeSize<f32> = NodeSize::from_rem(37.0 * 0.25);
    /// ## Rem - Standard Size 38
    pub const R_38: NodeSize<f32> = NodeSize::from_rem(38.0 * 0.25);
    /// ## Rem - Standard Size 39
    pub const R_39: NodeSize<f32> = NodeSize::from_rem(39.0 * 0.25);
    /// ## Rem - Standard Size 40
    pub const R_40: NodeSize<f32> = NodeSize::from_rem(40.0 * 0.25);
    /// ## Rem - Standard Size 41
    pub const R_41: NodeSize<f32> = NodeSize::from_rem(41.0 * 0.25);
    /// ## Rem - Standard Size 42
    pub const R_42: NodeSize<f32> = NodeSize::from_rem(42.0 * 0.25);
    /// ## Rem - Standard Size 43
    pub const R_43: NodeSize<f32> = NodeSize::from_rem(43.0 * 0.25);
    /// ## Rem - Standard Size 44
    pub const R_44: NodeSize<f32> = NodeSize::from_rem(44.0 * 0.25);
    /// ## Rem - Standard Size 45
    pub const R_45: NodeSize<f32> = NodeSize::from_rem(45.0 * 0.25);
    /// ## Rem - Standard Size 46
    pub const R_46: NodeSize<f32> = NodeSize::from_rem(46.0 * 0.25);
    /// ## Rem - Standard Size 47
    pub const R_47: NodeSize<f32> = NodeSize::from_rem(47.0 * 0.25);
    /// ## Rem - Standard Size 48
    pub const R_48: NodeSize<f32> = NodeSize::from_rem(48.0 * 0.25);
    /// ## Rem - Standard Size 49
    pub const R_49: NodeSize<f32> = NodeSize::from_rem(49.0 * 0.25);
    /// ## Rem - Standard Size 50
    pub const R_50: NodeSize<f32> = NodeSize::from_rem(50.0 * 0.25);
    /// ## Rem - Standard Size 51
    pub const R_51: NodeSize<f32> = NodeSize::from_rem(51.0 * 0.25);
    /// ## Rem - Standard Size 52
    pub const R_52: NodeSize<f32> = NodeSize::from_rem(52.0 * 0.25);
    /// ## Rem - Standard Size 53
    pub const R_53: NodeSize<f32> = NodeSize::from_rem(53.0 * 0.25);
    /// ## Rem - Standard Size 54
    pub const R_54: NodeSize<f32> = NodeSize::from_rem(54.0 * 0.25);
    /// ## Rem - Standard Size 55
    pub const R_55: NodeSize<f32> = NodeSize::from_rem(55.0 * 0.25);
    /// ## Rem - Standard Size 56
    pub const R_56: NodeSize<f32> = NodeSize::from_rem(56.0 * 0.25);
    /// ## Rem - Standard Size 57
    pub const R_57: NodeSize<f32> = NodeSize::from_rem(57.0 * 0.25);
    /// ## Rem - Standard Size 58
    pub const R_58: NodeSize<f32> = NodeSize::from_rem(58.0 * 0.25);
    /// ## Rem - Standard Size 59
    pub const R_59: NodeSize<f32> = NodeSize::from_rem(59.0 * 0.25);
    /// ## Rem - Standard Size 64
    pub const R_60: NodeSize<f32> = NodeSize::from_rem(60.0 * 0.25);
    /// ## Rem - Standard Size 61
    pub const R_61: NodeSize<f32> = NodeSize::from_rem(61.0 * 0.25);
    /// ## Rem - Standard Size 62
    pub const R_62: NodeSize<f32> = NodeSize::from_rem(62.0 * 0.25);
    /// ## Rem - Standard Size 63
    pub const R_63: NodeSize<f32> = NodeSize::from_rem(63.0 * 0.25);
    /// ## Rem - Standard Size 64
    pub const R_64: NodeSize<f32> = NodeSize::from_rem(64.0 * 0.25);
    /// ## Rem - Standard Size 65
    pub const R_65: NodeSize<f32> = NodeSize::from_rem(65.0 * 0.25);
    /// ## Rem - Standard Size 66
    pub const R_66: NodeSize<f32> = NodeSize::from_rem(66.0 * 0.25);
    /// ## Rem - Standard Size 67
    pub const R_67: NodeSize<f32> = NodeSize::from_rem(67.0 * 0.25);
    /// ## Rem - Standard Size 68
    pub const R_68: NodeSize<f32> = NodeSize::from_rem(68.0 * 0.25);
    /// ## Rem - Standard Size 69
    pub const R_69: NodeSize<f32> = NodeSize::from_rem(69.0 * 0.25);
    /// ## Rem - Standard Size 70
    pub const R_70: NodeSize<f32> = NodeSize::from_rem(70.0 * 0.25);
    /// ## Rem - Standard Size 71
    pub const R_71: NodeSize<f32> = NodeSize::from_rem(71.0 * 0.25);
    /// ## Rem - Standard Size 72
    pub const R_72: NodeSize<f32> = NodeSize::from_rem(72.0 * 0.25);
    /// ## Rem - Standard Size 73
    pub const R_73: NodeSize<f32> = NodeSize::from_rem(73.0 * 0.25);
    /// ## Rem - Standard Size 74
    pub const R_74: NodeSize<f32> = NodeSize::from_rem(74.0 * 0.25);
    /// ## Rem - Standard Size 75
    pub const R_75: NodeSize<f32> = NodeSize::from_rem(75.0 * 0.25);
    /// ## Rem - Standard Size 76
    pub const R_76: NodeSize<f32> = NodeSize::from_rem(76.0 * 0.25);
    /// ## Rem - Standard Size 77
    pub const R_77: NodeSize<f32> = NodeSize::from_rem(77.0 * 0.25);
    /// ## Rem - Standard Size 78
    pub const R_78: NodeSize<f32> = NodeSize::from_rem(78.0 * 0.25);
    /// ## Rem - Standard Size 79
    pub const R_79: NodeSize<f32> = NodeSize::from_rem(79.0 * 0.25);
    /// ## Rem - Standard Size 80
    pub const R_80: NodeSize<f32> = NodeSize::from_rem(80.0 * 0.25);
    /// ## Rem - Standard Size 81
    pub const R_81: NodeSize<f32> = NodeSize::from_rem(81.0 * 0.25);
    /// ## Rem - Standard Size 82
    pub const R_82: NodeSize<f32> = NodeSize::from_rem(82.0 * 0.25);
    /// ## Rem - Standard Size 83
    pub const R_83: NodeSize<f32> = NodeSize::from_rem(83.0 * 0.25);
    /// ## Rem - Standard Size 84
    pub const R_84: NodeSize<f32> = NodeSize::from_rem(84.0 * 0.25);
    /// ## Rem - Standard Size 85
    pub const R_85: NodeSize<f32> = NodeSize::from_rem(85.0 * 0.25);
    /// ## Rem - Standard Size 86
    pub const R_86: NodeSize<f32> = NodeSize::from_rem(86.0 * 0.25);
    /// ## Rem - Standard Size 87
    pub const R_87: NodeSize<f32> = NodeSize::from_rem(87.0 * 0.25);
    /// ## Rem - Standard Size 88
    pub const R_88: NodeSize<f32> = NodeSize::from_rem(88.0 * 0.25);
    /// ## Rem - Standard Size 89
    pub const R_89: NodeSize<f32> = NodeSize::from_rem(89.0 * 0.25);
    /// ## Rem - Standard Size 90
    pub const R_90: NodeSize<f32> = NodeSize::from_rem(90.0 * 0.25);
    /// ## Rem - Standard Size 91
    pub const R_91: NodeSize<f32> = NodeSize::from_rem(91.0 * 0.25);
    /// ## Rem - Standard Size 92
    pub const R_92: NodeSize<f32> = NodeSize::from_rem(92.0 * 0.25);
    /// ## Rem - Standard Size 93
    pub const R_93: NodeSize<f32> = NodeSize::from_rem(93.0 * 0.25);
    /// ## Rem - Standard Size 94
    pub const R_94: NodeSize<f32> = NodeSize::from_rem(94.0 * 0.25);
    /// ## Rem - Standard Size 95
    pub const R_95: NodeSize<f32> = NodeSize::from_rem(95.0 * 0.25);
    /// ## Rem - Standard Size 96
    pub const R_96: NodeSize<f32> = NodeSize::from_rem(96.0 * 0.25);
    /// ## Rem - Standard Size 97
    pub const R_97: NodeSize<f32> = NodeSize::from_rem(97.0 * 0.25);
    /// ## Rem - Standard Size 98
    pub const R_98: NodeSize<f32> = NodeSize::from_rem(98.0 * 0.25);
    /// ## Rem - Standard Size 99
    pub const R_99: NodeSize<f32> = NodeSize::from_rem(99.0 * 0.25);
    /// ## Rem - Standard Size 100
    pub const R_100: NodeSize<f32> = NodeSize::from_rem(100.0 * 0.25);
}
impl NodeSize<Vec2> {
    /// ## Abs - Extra-small
    pub const A_XS_VEC2: NodeSize<Vec2> = NodeSize::A_4_VEC2;
    /// ## Abs - Small
    pub const A_SM_VEC2: NodeSize<Vec2> = NodeSize::A_8_VEC2;
    /// ## Abs - Medium
    pub const A_MD_VEC2: NodeSize<Vec2> = NodeSize::A_12_VEC2;
    /// ## Abs - Large
    pub const A_LG_VEC2: NodeSize<Vec2> = NodeSize::A_16_VEC2;
    /// ## Abs - Extra-large
    pub const A_XL_VEC2: NodeSize<Vec2> = NodeSize::A_24_VEC2;
    /// ## Abs - Extra-large 2
    pub const A_XL2_VEC2: NodeSize<Vec2> = NodeSize::A_32_VEC2;
    /// ## Abs - Extra-large 3
    pub const A_XL3_VEC2: NodeSize<Vec2> = NodeSize::A_40_VEC2;
    /// ## Abs - Extra-large 4
    pub const A_XL4_VEC2: NodeSize<Vec2> = NodeSize::A_48_VEC2;
    /// ## Abs - Extra-large 5
    pub const A_XL5_VEC2: NodeSize<Vec2> = NodeSize::A_56_VEC2;
    /// ## Abs - Extra-large 6
    pub const A_XL6_VEC2: NodeSize<Vec2> = NodeSize::A_64_VEC2;
    /// ## Abs - Extra-large 7
    pub const A_XL7_VEC2: NodeSize<Vec2> = NodeSize::A_72_VEC2;

    /// ## Abs - Standard Size 0
    pub const A_0_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::ZERO);
    /// ## Abs - Standard Size 1
    pub const A_1_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(1.0 * 4.0));
    /// ## Abs - Standard Size 2
    pub const A_2_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(2.0 * 4.0));
    /// ## Abs - Standard Size 3
    pub const A_3_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(3.0 * 4.0));
    /// ## Abs - Standard Size 4
    pub const A_4_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(4.0 * 4.0));
    /// ## Abs - Standard Size 5
    pub const A_5_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(5.0 * 4.0));
    /// ## Abs - Standard Size 6
    pub const A_6_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(6.0 * 4.0));
    /// ## Abs - Standard Size 7
    pub const A_7_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(7.0 * 4.0));
    /// ## Abs - Standard Size 8
    pub const A_8_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(8.0 * 4.0));
    /// ## Abs - Standard Size 9
    pub const A_9_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(9.0 * 4.0));
    /// ## Abs - Standard Size 10
    pub const A_10_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(10.0 * 4.0));
    /// ## Abs - Standard Size 11
    pub const A_11_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(11.0 * 4.0));
    /// ## Abs - Standard Size 12
    pub const A_12_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(12.0 * 4.0));
    /// ## Abs - Standard Size 13
    pub const A_13_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(13.0 * 4.0));
    /// ## Abs - Standard Size 14
    pub const A_14_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(14.0 * 4.0));
    /// ## Abs - Standard Size 15
    pub const A_15_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(15.0 * 4.0));
    /// ## Abs - Standard Size 16
    pub const A_16_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(16.0 * 4.0));
    /// ## Abs - Standard Size 17
    pub const A_17_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(17.0 * 4.0));
    /// ## Abs - Standard Size 18
    pub const A_18_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(18.0 * 4.0));
    /// ## Abs - Standard Size 19
    pub const A_19_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(19.0 * 4.0));
    /// ## Abs - Standard Size 20
    pub const A_20_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(20.0 * 4.0));
    /// ## Abs - Standard Size 21
    pub const A_21_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(21.0 * 4.0));
    /// ## Abs - Standard Size 22
    pub const A_22_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(22.0 * 4.0));
    /// ## Abs - Standard Size 23
    pub const A_23_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(23.0 * 4.0));
    /// ## Abs - Standard Size 24
    pub const A_24_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(24.0 * 4.0));
    /// ## Abs - Standard Size 25
    pub const A_25_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(25.0 * 4.0));
    /// ## Abs - Standard Size 26
    pub const A_26_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(26.0 * 4.0));
    /// ## Abs - Standard Size 27
    pub const A_27_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(27.0 * 4.0));
    /// ## Abs - Standard Size 28
    pub const A_28_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(28.0 * 4.0));
    /// ## Abs - Standard Size 29
    pub const A_29_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(29.0 * 4.0));
    /// ## Abs - Standard Size 30
    pub const A_30_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(30.0 * 4.0));
    /// ## Abs - Standard Size 31
    pub const A_31_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(31.0 * 4.0));
    /// ## Abs - Standard Size 32
    pub const A_32_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(32.0 * 4.0));
    /// ## Abs - Standard Size 33
    pub const A_33_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(33.0 * 4.0));
    /// ## Abs - Standard Size 34
    pub const A_34_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(34.0 * 4.0));
    /// ## Abs - Standard Size 35
    pub const A_35_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(35.0 * 4.0));
    /// ## Abs - Standard Size 36
    pub const A_36_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(36.0 * 4.0));
    /// ## Abs - Standard Size 37
    pub const A_37_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(37.0 * 4.0));
    /// ## Abs - Standard Size 38
    pub const A_38_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(38.0 * 4.0));
    /// ## Abs - Standard Size 39
    pub const A_39_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(39.0 * 4.0));
    /// ## Abs - Standard Size 40
    pub const A_40_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(40.0 * 4.0));
    /// ## Abs - Standard Size 41
    pub const A_41_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(41.0 * 4.0));
    /// ## Abs - Standard Size 42
    pub const A_42_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(42.0 * 4.0));
    /// ## Abs - Standard Size 43
    pub const A_43_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(43.0 * 4.0));
    /// ## Abs - Standard Size 44
    pub const A_44_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(44.0 * 4.0));
    /// ## Abs - Standard Size 45
    pub const A_45_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(45.0 * 4.0));
    /// ## Abs - Standard Size 46
    pub const A_46_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(46.0 * 4.0));
    /// ## Abs - Standard Size 47
    pub const A_47_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(47.0 * 4.0));
    /// ## Abs - Standard Size 48
    pub const A_48_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(48.0 * 4.0));
    /// ## Abs - Standard Size 49
    pub const A_49_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(49.0 * 4.0));
    /// ## Abs - Standard Size 50
    pub const A_50_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(50.0 * 4.0));
    /// ## Abs - Standard Size 51
    pub const A_51_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(51.0 * 4.0));
    /// ## Abs - Standard Size 52
    pub const A_52_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(52.0 * 4.0));
    /// ## Abs - Standard Size 53
    pub const A_53_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(53.0 * 4.0));
    /// ## Abs - Standard Size 54
    pub const A_54_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(54.0 * 4.0));
    /// ## Abs - Standard Size 55
    pub const A_55_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(55.0 * 4.0));
    /// ## Abs - Standard Size 56
    pub const A_56_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(56.0 * 4.0));
    /// ## Abs - Standard Size 57
    pub const A_57_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(57.0 * 4.0));
    /// ## Abs - Standard Size 58
    pub const A_58_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(58.0 * 4.0));
    /// ## Abs - Standard Size 59
    pub const A_59_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(59.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_60_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(60.0 * 4.0));
    /// ## Abs - Standard Size 61
    pub const A_61_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(61.0 * 4.0));
    /// ## Abs - Standard Size 62
    pub const A_62_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(62.0 * 4.0));
    /// ## Abs - Standard Size 63
    pub const A_63_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(63.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_64_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(64.0 * 4.0));
    /// ## Abs - Standard Size 65
    pub const A_65_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(65.0 * 4.0));
    /// ## Abs - Standard Size 66
    pub const A_66_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(66.0 * 4.0));
    /// ## Abs - Standard Size 67
    pub const A_67_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(67.0 * 4.0));
    /// ## Abs - Standard Size 68
    pub const A_68_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(68.0 * 4.0));
    /// ## Abs - Standard Size 69
    pub const A_69_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(69.0 * 4.0));
    /// ## Abs - Standard Size 70
    pub const A_70_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(70.0 * 4.0));
    /// ## Abs - Standard Size 71
    pub const A_71_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(71.0 * 4.0));
    /// ## Abs - Standard Size 72
    pub const A_72_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(72.0 * 4.0));
    /// ## Abs - Standard Size 73
    pub const A_73_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(73.0 * 4.0));
    /// ## Abs - Standard Size 74
    pub const A_74_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(74.0 * 4.0));
    /// ## Abs - Standard Size 75
    pub const A_75_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(75.0 * 4.0));
    /// ## Abs - Standard Size 76
    pub const A_76_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(76.0 * 4.0));
    /// ## Abs - Standard Size 77
    pub const A_77_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(77.0 * 4.0));
    /// ## Abs - Standard Size 78
    pub const A_78_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(78.0 * 4.0));
    /// ## Abs - Standard Size 79
    pub const A_79_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(79.0 * 4.0));
    /// ## Abs - Standard Size 80
    pub const A_80_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(80.0 * 4.0));
    /// ## Abs - Standard Size 81
    pub const A_81_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(81.0 * 4.0));
    /// ## Abs - Standard Size 82
    pub const A_82_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(82.0 * 4.0));
    /// ## Abs - Standard Size 83
    pub const A_83_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(83.0 * 4.0));
    /// ## Abs - Standard Size 84
    pub const A_84_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(84.0 * 4.0));
    /// ## Abs - Standard Size 85
    pub const A_85_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(85.0 * 4.0));
    /// ## Abs - Standard Size 86
    pub const A_86_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(86.0 * 4.0));
    /// ## Abs - Standard Size 87
    pub const A_87_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(87.0 * 4.0));
    /// ## Abs - Standard Size 88
    pub const A_88_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(88.0 * 4.0));
    /// ## Abs - Standard Size 89
    pub const A_89_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(89.0 * 4.0));
    /// ## Abs - Standard Size 90
    pub const A_90_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(90.0 * 4.0));
    /// ## Abs - Standard Size 91
    pub const A_91_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(91.0 * 4.0));
    /// ## Abs - Standard Size 92
    pub const A_92_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(92.0 * 4.0));
    /// ## Abs - Standard Size 93
    pub const A_93_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(93.0 * 4.0));
    /// ## Abs - Standard Size 94
    pub const A_94_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(94.0 * 4.0));
    /// ## Abs - Standard Size 95
    pub const A_95_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(95.0 * 4.0));
    /// ## Abs - Standard Size 96
    pub const A_96_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(96.0 * 4.0));
    /// ## Abs - Standard Size 97
    pub const A_97_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(97.0 * 4.0));
    /// ## Abs - Standard Size 98
    pub const A_98_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(98.0 * 4.0));
    /// ## Abs - Standard Size 99
    pub const A_99_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(99.0 * 4.0));
    /// ## Abs - Standard Size 100
    pub const A_100_VEC2: NodeSize<Vec2> = NodeSize::from_abs(Vec2::splat(100.0 * 4.0));

    /// ## Rem - Extra-small
    pub const R_XS_VEC2: NodeSize<Vec2> = NodeSize::R_4_VEC2;
    /// ## Rem - Small
    pub const R_SM_VEC2: NodeSize<Vec2> = NodeSize::R_8_VEC2;
    /// ## Rem - Medium
    pub const R_MD_VEC2: NodeSize<Vec2> = NodeSize::R_12_VEC2;
    /// ## Rem - Large
    pub const R_LG_VEC2: NodeSize<Vec2> = NodeSize::R_16_VEC2;
    /// ## Rem - Extra-large
    pub const R_XL_VEC2: NodeSize<Vec2> = NodeSize::R_24_VEC2;
    /// ## Rem - Extra-large 2
    pub const R_XL2_VEC2: NodeSize<Vec2> = NodeSize::R_32_VEC2;
    /// ## Rem - Extra-large 3
    pub const R_XL3_VEC2: NodeSize<Vec2> = NodeSize::R_40_VEC2;
    /// ## Rem - Extra-large 4
    pub const R_XL4_VEC2: NodeSize<Vec2> = NodeSize::R_48_VEC2;
    /// ## Rem - Extra-large 5
    pub const R_XL5_VEC2: NodeSize<Vec2> = NodeSize::R_56_VEC2;
    /// ## Rem - Extra-large 6
    pub const R_XL6_VEC2: NodeSize<Vec2> = NodeSize::R_64_VEC2;
    /// ## Rem - Extra-large 7
    pub const R_XL7_VEC2: NodeSize<Vec2> = NodeSize::R_72_VEC2;

    /// ## Rem - Standard Size 0
    pub const R_0_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::ZERO);
    /// ## Rem - Standard Size 1
    pub const R_1_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(1.0 * 0.25));
    /// ## Rem - Standard Size 2
    pub const R_2_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(2.0 * 0.25));
    /// ## Rem - Standard Size 3
    pub const R_3_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(3.0 * 0.25));
    /// ## Rem - Standard Size 4
    pub const R_4_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(4.0 * 0.25));
    /// ## Rem - Standard Size 5
    pub const R_5_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(5.0 * 0.25));
    /// ## Rem - Standard Size 6
    pub const R_6_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(6.0 * 0.25));
    /// ## Rem - Standard Size 7
    pub const R_7_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(7.0 * 0.25));
    /// ## Rem - Standard Size 8
    pub const R_8_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(8.0 * 0.25));
    /// ## Rem - Standard Size 9
    pub const R_9_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(9.0 * 0.25));
    /// ## Rem - Standard Size 10
    pub const R_10_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(10.0 * 0.25));
    /// ## Rem - Standard Size 11
    pub const R_11_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(11.0 * 0.25));
    /// ## Rem - Standard Size 12
    pub const R_12_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(12.0 * 0.25));
    /// ## Rem - Standard Size 13
    pub const R_13_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(13.0 * 0.25));
    /// ## Rem - Standard Size 14
    pub const R_14_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(14.0 * 0.25));
    /// ## Rem - Standard Size 15
    pub const R_15_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(15.0 * 0.25));
    /// ## Rem - Standard Size 16
    pub const R_16_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(16.0 * 0.25));
    /// ## Rem - Standard Size 17
    pub const R_17_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(17.0 * 0.25));
    /// ## Rem - Standard Size 18
    pub const R_18_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(18.0 * 0.25));
    /// ## Rem - Standard Size 19
    pub const R_19_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(19.0 * 0.25));
    /// ## Rem - Standard Size 20
    pub const R_20_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(20.0 * 0.25));
    /// ## Rem - Standard Size 21
    pub const R_21_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(21.0 * 0.25));
    /// ## Rem - Standard Size 22
    pub const R_22_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(22.0 * 0.25));
    /// ## Rem - Standard Size 23
    pub const R_23_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(23.0 * 0.25));
    /// ## Rem - Standard Size 24
    pub const R_24_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(24.0 * 0.25));
    /// ## Rem - Standard Size 25
    pub const R_25_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(25.0 * 0.25));
    /// ## Rem - Standard Size 26
    pub const R_26_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(26.0 * 0.25));
    /// ## Rem - Standard Size 27
    pub const R_27_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(27.0 * 0.25));
    /// ## Rem - Standard Size 28
    pub const R_28_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(28.0 * 0.25));
    /// ## Rem - Standard Size 29
    pub const R_29_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(29.0 * 0.25));
    /// ## Rem - Standard Size 30
    pub const R_30_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(30.0 * 0.25));
    /// ## Rem - Standard Size 31
    pub const R_31_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(31.0 * 0.25));
    /// ## Rem - Standard Size 32
    pub const R_32_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(32.0 * 0.25));
    /// ## Rem - Standard Size 33
    pub const R_33_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(33.0 * 0.25));
    /// ## Rem - Standard Size 34
    pub const R_34_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(34.0 * 0.25));
    /// ## Rem - Standard Size 35
    pub const R_35_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(35.0 * 0.25));
    /// ## Rem - Standard Size 36
    pub const R_36_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(36.0 * 0.25));
    /// ## Rem - Standard Size 37
    pub const R_37_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(37.0 * 0.25));
    /// ## Rem - Standard Size 38
    pub const R_38_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(38.0 * 0.25));
    /// ## Rem - Standard Size 39
    pub const R_39_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(39.0 * 0.25));
    /// ## Rem - Standard Size 40
    pub const R_40_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(40.0 * 0.25));
    /// ## Rem - Standard Size 41
    pub const R_41_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(41.0 * 0.25));
    /// ## Rem - Standard Size 42
    pub const R_42_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(42.0 * 0.25));
    /// ## Rem - Standard Size 43
    pub const R_43_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(43.0 * 0.25));
    /// ## Rem - Standard Size 44
    pub const R_44_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(44.0 * 0.25));
    /// ## Rem - Standard Size 45
    pub const R_45_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(45.0 * 0.25));
    /// ## Rem - Standard Size 46
    pub const R_46_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(46.0 * 0.25));
    /// ## Rem - Standard Size 47
    pub const R_47_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(47.0 * 0.25));
    /// ## Rem - Standard Size 48
    pub const R_48_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(48.0 * 0.25));
    /// ## Rem - Standard Size 49
    pub const R_49_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(49.0 * 0.25));
    /// ## Rem - Standard Size 50
    pub const R_50_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(50.0 * 0.25));
    /// ## Rem - Standard Size 51
    pub const R_51_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(51.0 * 0.25));
    /// ## Rem - Standard Size 52
    pub const R_52_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(52.0 * 0.25));
    /// ## Rem - Standard Size 53
    pub const R_53_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(53.0 * 0.25));
    /// ## Rem - Standard Size 54
    pub const R_54_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(54.0 * 0.25));
    /// ## Rem - Standard Size 55
    pub const R_55_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(55.0 * 0.25));
    /// ## Rem - Standard Size 56
    pub const R_56_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(56.0 * 0.25));
    /// ## Rem - Standard Size 57
    pub const R_57_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(57.0 * 0.25));
    /// ## Rem - Standard Size 58
    pub const R_58_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(58.0 * 0.25));
    /// ## Rem - Standard Size 59
    pub const R_59_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(59.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_60_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(60.0 * 0.25));
    /// ## Rem - Standard Size 61
    pub const R_61_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(61.0 * 0.25));
    /// ## Rem - Standard Size 62
    pub const R_62_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(62.0 * 0.25));
    /// ## Rem - Standard Size 63
    pub const R_63_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(63.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_64_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(64.0 * 0.25));
    /// ## Rem - Standard Size 65
    pub const R_65_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(65.0 * 0.25));
    /// ## Rem - Standard Size 66
    pub const R_66_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(66.0 * 0.25));
    /// ## Rem - Standard Size 67
    pub const R_67_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(67.0 * 0.25));
    /// ## Rem - Standard Size 68
    pub const R_68_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(68.0 * 0.25));
    /// ## Rem - Standard Size 69
    pub const R_69_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(69.0 * 0.25));
    /// ## Rem - Standard Size 70
    pub const R_70_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(70.0 * 0.25));
    /// ## Rem - Standard Size 71
    pub const R_71_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(71.0 * 0.25));
    /// ## Rem - Standard Size 72
    pub const R_72_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(72.0 * 0.25));
    /// ## Rem - Standard Size 73
    pub const R_73_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(73.0 * 0.25));
    /// ## Rem - Standard Size 74
    pub const R_74_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(74.0 * 0.25));
    /// ## Rem - Standard Size 75
    pub const R_75_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(75.0 * 0.25));
    /// ## Rem - Standard Size 76
    pub const R_76_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(76.0 * 0.25));
    /// ## Rem - Standard Size 77
    pub const R_77_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(77.0 * 0.25));
    /// ## Rem - Standard Size 78
    pub const R_78_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(78.0 * 0.25));
    /// ## Rem - Standard Size 79
    pub const R_79_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(79.0 * 0.25));
    /// ## Rem - Standard Size 80
    pub const R_80_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(80.0 * 0.25));
    /// ## Rem - Standard Size 81
    pub const R_81_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(81.0 * 0.25));
    /// ## Rem - Standard Size 82
    pub const R_82_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(82.0 * 0.25));
    /// ## Rem - Standard Size 83
    pub const R_83_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(83.0 * 0.25));
    /// ## Rem - Standard Size 84
    pub const R_84_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(84.0 * 0.25));
    /// ## Rem - Standard Size 85
    pub const R_85_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(85.0 * 0.25));
    /// ## Rem - Standard Size 86
    pub const R_86_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(86.0 * 0.25));
    /// ## Rem - Standard Size 87
    pub const R_87_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(87.0 * 0.25));
    /// ## Rem - Standard Size 88
    pub const R_88_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(88.0 * 0.25));
    /// ## Rem - Standard Size 89
    pub const R_89_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(89.0 * 0.25));
    /// ## Rem - Standard Size 90
    pub const R_90_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(90.0 * 0.25));
    /// ## Rem - Standard Size 91
    pub const R_91_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(91.0 * 0.25));
    /// ## Rem - Standard Size 92
    pub const R_92_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(92.0 * 0.25));
    /// ## Rem - Standard Size 93
    pub const R_93_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(93.0 * 0.25));
    /// ## Rem - Standard Size 94
    pub const R_94_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(94.0 * 0.25));
    /// ## Rem - Standard Size 95
    pub const R_95_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(95.0 * 0.25));
    /// ## Rem - Standard Size 96
    pub const R_96_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(96.0 * 0.25));
    /// ## Rem - Standard Size 97
    pub const R_97_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(97.0 * 0.25));
    /// ## Rem - Standard Size 98
    pub const R_98_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(98.0 * 0.25));
    /// ## Rem - Standard Size 99
    pub const R_99_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(99.0 * 0.25));
    /// ## Rem - Standard Size 100
    pub const R_100_VEC2: NodeSize<Vec2> = NodeSize::from_rem(Vec2::splat(100.0 * 0.25));
}
impl NodeSize<Vec3> {
    /// ## Abs - Extra-small
    pub const A_XS_VEC3: NodeSize<Vec3> = NodeSize::A_4_VEC3;
    /// ## Abs - Small
    pub const A_SM_VEC3: NodeSize<Vec3> = NodeSize::A_8_VEC3;
    /// ## Abs - Medium
    pub const A_MD_VEC3: NodeSize<Vec3> = NodeSize::A_12_VEC3;
    /// ## Abs - Large
    pub const A_LG_VEC3: NodeSize<Vec3> = NodeSize::A_16_VEC3;
    /// ## Abs - Extra-large
    pub const A_XL_VEC3: NodeSize<Vec3> = NodeSize::A_24_VEC3;
    /// ## Abs - Extra-large 2
    pub const A_XL2_VEC3: NodeSize<Vec3> = NodeSize::A_32_VEC3;
    /// ## Abs - Extra-large 3
    pub const A_XL3_VEC3: NodeSize<Vec3> = NodeSize::A_40_VEC3;
    /// ## Abs - Extra-large 4
    pub const A_XL4_VEC3: NodeSize<Vec3> = NodeSize::A_48_VEC3;
    /// ## Abs - Extra-large 5
    pub const A_XL5_VEC3: NodeSize<Vec3> = NodeSize::A_56_VEC3;
    /// ## Abs - Extra-large 6
    pub const A_XL6_VEC3: NodeSize<Vec3> = NodeSize::A_64_VEC3;
    /// ## Abs - Extra-large 7
    pub const A_XL7_VEC3: NodeSize<Vec3> = NodeSize::A_72_VEC3;

    /// ## Abs - Standard Size 0
    pub const A_0_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::ZERO);
    /// ## Abs - Standard Size 1
    pub const A_1_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(1.0 * 4.0));
    /// ## Abs - Standard Size 2
    pub const A_2_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(2.0 * 4.0));
    /// ## Abs - Standard Size 3
    pub const A_3_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(3.0 * 4.0));
    /// ## Abs - Standard Size 4
    pub const A_4_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(4.0 * 4.0));
    /// ## Abs - Standard Size 5
    pub const A_5_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(5.0 * 4.0));
    /// ## Abs - Standard Size 6
    pub const A_6_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(6.0 * 4.0));
    /// ## Abs - Standard Size 7
    pub const A_7_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(7.0 * 4.0));
    /// ## Abs - Standard Size 8
    pub const A_8_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(8.0 * 4.0));
    /// ## Abs - Standard Size 9
    pub const A_9_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(9.0 * 4.0));
    /// ## Abs - Standard Size 10
    pub const A_10_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(10.0 * 4.0));
    /// ## Abs - Standard Size 11
    pub const A_11_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(11.0 * 4.0));
    /// ## Abs - Standard Size 12
    pub const A_12_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(12.0 * 4.0));
    /// ## Abs - Standard Size 13
    pub const A_13_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(13.0 * 4.0));
    /// ## Abs - Standard Size 14
    pub const A_14_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(14.0 * 4.0));
    /// ## Abs - Standard Size 15
    pub const A_15_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(15.0 * 4.0));
    /// ## Abs - Standard Size 16
    pub const A_16_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(16.0 * 4.0));
    /// ## Abs - Standard Size 17
    pub const A_17_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(17.0 * 4.0));
    /// ## Abs - Standard Size 18
    pub const A_18_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(18.0 * 4.0));
    /// ## Abs - Standard Size 19
    pub const A_19_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(19.0 * 4.0));
    /// ## Abs - Standard Size 20
    pub const A_20_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(20.0 * 4.0));
    /// ## Abs - Standard Size 21
    pub const A_21_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(21.0 * 4.0));
    /// ## Abs - Standard Size 22
    pub const A_22_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(22.0 * 4.0));
    /// ## Abs - Standard Size 23
    pub const A_23_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(23.0 * 4.0));
    /// ## Abs - Standard Size 24
    pub const A_24_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(24.0 * 4.0));
    /// ## Abs - Standard Size 25
    pub const A_25_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(25.0 * 4.0));
    /// ## Abs - Standard Size 26
    pub const A_26_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(26.0 * 4.0));
    /// ## Abs - Standard Size 27
    pub const A_27_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(27.0 * 4.0));
    /// ## Abs - Standard Size 28
    pub const A_28_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(28.0 * 4.0));
    /// ## Abs - Standard Size 29
    pub const A_29_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(29.0 * 4.0));
    /// ## Abs - Standard Size 30
    pub const A_30_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(30.0 * 4.0));
    /// ## Abs - Standard Size 31
    pub const A_31_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(31.0 * 4.0));
    /// ## Abs - Standard Size 32
    pub const A_32_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(32.0 * 4.0));
    /// ## Abs - Standard Size 33
    pub const A_33_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(33.0 * 4.0));
    /// ## Abs - Standard Size 34
    pub const A_34_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(34.0 * 4.0));
    /// ## Abs - Standard Size 35
    pub const A_35_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(35.0 * 4.0));
    /// ## Abs - Standard Size 36
    pub const A_36_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(36.0 * 4.0));
    /// ## Abs - Standard Size 37
    pub const A_37_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(37.0 * 4.0));
    /// ## Abs - Standard Size 38
    pub const A_38_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(38.0 * 4.0));
    /// ## Abs - Standard Size 39
    pub const A_39_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(39.0 * 4.0));
    /// ## Abs - Standard Size 40
    pub const A_40_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(40.0 * 4.0));
    /// ## Abs - Standard Size 41
    pub const A_41_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(41.0 * 4.0));
    /// ## Abs - Standard Size 42
    pub const A_42_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(42.0 * 4.0));
    /// ## Abs - Standard Size 43
    pub const A_43_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(43.0 * 4.0));
    /// ## Abs - Standard Size 44
    pub const A_44_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(44.0 * 4.0));
    /// ## Abs - Standard Size 45
    pub const A_45_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(45.0 * 4.0));
    /// ## Abs - Standard Size 46
    pub const A_46_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(46.0 * 4.0));
    /// ## Abs - Standard Size 47
    pub const A_47_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(47.0 * 4.0));
    /// ## Abs - Standard Size 48
    pub const A_48_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(48.0 * 4.0));
    /// ## Abs - Standard Size 49
    pub const A_49_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(49.0 * 4.0));
    /// ## Abs - Standard Size 50
    pub const A_50_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(50.0 * 4.0));
    /// ## Abs - Standard Size 51
    pub const A_51_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(51.0 * 4.0));
    /// ## Abs - Standard Size 52
    pub const A_52_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(52.0 * 4.0));
    /// ## Abs - Standard Size 53
    pub const A_53_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(53.0 * 4.0));
    /// ## Abs - Standard Size 54
    pub const A_54_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(54.0 * 4.0));
    /// ## Abs - Standard Size 55
    pub const A_55_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(55.0 * 4.0));
    /// ## Abs - Standard Size 56
    pub const A_56_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(56.0 * 4.0));
    /// ## Abs - Standard Size 57
    pub const A_57_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(57.0 * 4.0));
    /// ## Abs - Standard Size 58
    pub const A_58_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(58.0 * 4.0));
    /// ## Abs - Standard Size 59
    pub const A_59_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(59.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_60_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(60.0 * 4.0));
    /// ## Abs - Standard Size 61
    pub const A_61_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(61.0 * 4.0));
    /// ## Abs - Standard Size 62
    pub const A_62_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(62.0 * 4.0));
    /// ## Abs - Standard Size 63
    pub const A_63_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(63.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_64_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(64.0 * 4.0));
    /// ## Abs - Standard Size 65
    pub const A_65_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(65.0 * 4.0));
    /// ## Abs - Standard Size 66
    pub const A_66_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(66.0 * 4.0));
    /// ## Abs - Standard Size 67
    pub const A_67_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(67.0 * 4.0));
    /// ## Abs - Standard Size 68
    pub const A_68_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(68.0 * 4.0));
    /// ## Abs - Standard Size 69
    pub const A_69_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(69.0 * 4.0));
    /// ## Abs - Standard Size 70
    pub const A_70_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(70.0 * 4.0));
    /// ## Abs - Standard Size 71
    pub const A_71_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(71.0 * 4.0));
    /// ## Abs - Standard Size 72
    pub const A_72_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(72.0 * 4.0));
    /// ## Abs - Standard Size 73
    pub const A_73_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(73.0 * 4.0));
    /// ## Abs - Standard Size 74
    pub const A_74_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(74.0 * 4.0));
    /// ## Abs - Standard Size 75
    pub const A_75_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(75.0 * 4.0));
    /// ## Abs - Standard Size 76
    pub const A_76_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(76.0 * 4.0));
    /// ## Abs - Standard Size 77
    pub const A_77_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(77.0 * 4.0));
    /// ## Abs - Standard Size 78
    pub const A_78_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(78.0 * 4.0));
    /// ## Abs - Standard Size 79
    pub const A_79_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(79.0 * 4.0));
    /// ## Abs - Standard Size 80
    pub const A_80_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(80.0 * 4.0));
    /// ## Abs - Standard Size 81
    pub const A_81_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(81.0 * 4.0));
    /// ## Abs - Standard Size 82
    pub const A_82_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(82.0 * 4.0));
    /// ## Abs - Standard Size 83
    pub const A_83_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(83.0 * 4.0));
    /// ## Abs - Standard Size 84
    pub const A_84_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(84.0 * 4.0));
    /// ## Abs - Standard Size 85
    pub const A_85_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(85.0 * 4.0));
    /// ## Abs - Standard Size 86
    pub const A_86_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(86.0 * 4.0));
    /// ## Abs - Standard Size 87
    pub const A_87_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(87.0 * 4.0));
    /// ## Abs - Standard Size 88
    pub const A_88_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(88.0 * 4.0));
    /// ## Abs - Standard Size 89
    pub const A_89_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(89.0 * 4.0));
    /// ## Abs - Standard Size 90
    pub const A_90_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(90.0 * 4.0));
    /// ## Abs - Standard Size 91
    pub const A_91_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(91.0 * 4.0));
    /// ## Abs - Standard Size 92
    pub const A_92_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(92.0 * 4.0));
    /// ## Abs - Standard Size 93
    pub const A_93_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(93.0 * 4.0));
    /// ## Abs - Standard Size 94
    pub const A_94_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(94.0 * 4.0));
    /// ## Abs - Standard Size 95
    pub const A_95_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(95.0 * 4.0));
    /// ## Abs - Standard Size 96
    pub const A_96_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(96.0 * 4.0));
    /// ## Abs - Standard Size 97
    pub const A_97_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(97.0 * 4.0));
    /// ## Abs - Standard Size 98
    pub const A_98_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(98.0 * 4.0));
    /// ## Abs - Standard Size 99
    pub const A_99_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(99.0 * 4.0));
    /// ## Abs - Standard Size 100
    pub const A_100_VEC3: NodeSize<Vec3> = NodeSize::from_abs(Vec3::splat(100.0 * 4.0));

    /// ## Rem - Extra-small
    pub const R_XS_VEC3: NodeSize<Vec3> = NodeSize::R_4_VEC3;
    /// ## Rem - Small
    pub const R_SM_VEC3: NodeSize<Vec3> = NodeSize::R_8_VEC3;
    /// ## Rem - Medium
    pub const R_MD_VEC3: NodeSize<Vec3> = NodeSize::R_12_VEC3;
    /// ## Rem - Large
    pub const R_LG_VEC3: NodeSize<Vec3> = NodeSize::R_16_VEC3;
    /// ## Rem - Extra-large
    pub const R_XL_VEC3: NodeSize<Vec3> = NodeSize::R_24_VEC3;
    /// ## Rem - Extra-large 2
    pub const R_XL2_VEC3: NodeSize<Vec3> = NodeSize::R_32_VEC3;
    /// ## Rem - Extra-large 3
    pub const R_XL3_VEC3: NodeSize<Vec3> = NodeSize::R_40_VEC3;
    /// ## Rem - Extra-large 4
    pub const R_XL4_VEC3: NodeSize<Vec3> = NodeSize::R_48_VEC3;
    /// ## Rem - Extra-large 5
    pub const R_XL5_VEC3: NodeSize<Vec3> = NodeSize::R_56_VEC3;
    /// ## Rem - Extra-large 6
    pub const R_XL6_VEC3: NodeSize<Vec3> = NodeSize::R_64_VEC3;
    /// ## Rem - Extra-large 7
    pub const R_XL7_VEC3: NodeSize<Vec3> = NodeSize::R_72_VEC3;

    /// ## Rem - Standard Size 0
    pub const R_0_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::ZERO);
    /// ## Rem - Standard Size 1
    pub const R_1_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(1.0 * 0.25));
    /// ## Rem - Standard Size 2
    pub const R_2_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(2.0 * 0.25));
    /// ## Rem - Standard Size 3
    pub const R_3_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(3.0 * 0.25));
    /// ## Rem - Standard Size 4
    pub const R_4_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(4.0 * 0.25));
    /// ## Rem - Standard Size 5
    pub const R_5_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(5.0 * 0.25));
    /// ## Rem - Standard Size 6
    pub const R_6_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(6.0 * 0.25));
    /// ## Rem - Standard Size 7
    pub const R_7_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(7.0 * 0.25));
    /// ## Rem - Standard Size 8
    pub const R_8_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(8.0 * 0.25));
    /// ## Rem - Standard Size 9
    pub const R_9_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(9.0 * 0.25));
    /// ## Rem - Standard Size 10
    pub const R_10_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(10.0 * 0.25));
    /// ## Rem - Standard Size 11
    pub const R_11_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(11.0 * 0.25));
    /// ## Rem - Standard Size 12
    pub const R_12_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(12.0 * 0.25));
    /// ## Rem - Standard Size 13
    pub const R_13_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(13.0 * 0.25));
    /// ## Rem - Standard Size 14
    pub const R_14_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(14.0 * 0.25));
    /// ## Rem - Standard Size 15
    pub const R_15_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(15.0 * 0.25));
    /// ## Rem - Standard Size 16
    pub const R_16_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(16.0 * 0.25));
    /// ## Rem - Standard Size 17
    pub const R_17_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(17.0 * 0.25));
    /// ## Rem - Standard Size 18
    pub const R_18_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(18.0 * 0.25));
    /// ## Rem - Standard Size 19
    pub const R_19_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(19.0 * 0.25));
    /// ## Rem - Standard Size 20
    pub const R_20_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(20.0 * 0.25));
    /// ## Rem - Standard Size 21
    pub const R_21_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(21.0 * 0.25));
    /// ## Rem - Standard Size 22
    pub const R_22_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(22.0 * 0.25));
    /// ## Rem - Standard Size 23
    pub const R_23_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(23.0 * 0.25));
    /// ## Rem - Standard Size 24
    pub const R_24_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(24.0 * 0.25));
    /// ## Rem - Standard Size 25
    pub const R_25_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(25.0 * 0.25));
    /// ## Rem - Standard Size 26
    pub const R_26_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(26.0 * 0.25));
    /// ## Rem - Standard Size 27
    pub const R_27_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(27.0 * 0.25));
    /// ## Rem - Standard Size 28
    pub const R_28_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(28.0 * 0.25));
    /// ## Rem - Standard Size 29
    pub const R_29_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(29.0 * 0.25));
    /// ## Rem - Standard Size 30
    pub const R_30_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(30.0 * 0.25));
    /// ## Rem - Standard Size 31
    pub const R_31_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(31.0 * 0.25));
    /// ## Rem - Standard Size 32
    pub const R_32_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(32.0 * 0.25));
    /// ## Rem - Standard Size 33
    pub const R_33_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(33.0 * 0.25));
    /// ## Rem - Standard Size 34
    pub const R_34_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(34.0 * 0.25));
    /// ## Rem - Standard Size 35
    pub const R_35_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(35.0 * 0.25));
    /// ## Rem - Standard Size 36
    pub const R_36_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(36.0 * 0.25));
    /// ## Rem - Standard Size 37
    pub const R_37_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(37.0 * 0.25));
    /// ## Rem - Standard Size 38
    pub const R_38_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(38.0 * 0.25));
    /// ## Rem - Standard Size 39
    pub const R_39_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(39.0 * 0.25));
    /// ## Rem - Standard Size 40
    pub const R_40_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(40.0 * 0.25));
    /// ## Rem - Standard Size 41
    pub const R_41_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(41.0 * 0.25));
    /// ## Rem - Standard Size 42
    pub const R_42_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(42.0 * 0.25));
    /// ## Rem - Standard Size 43
    pub const R_43_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(43.0 * 0.25));
    /// ## Rem - Standard Size 44
    pub const R_44_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(44.0 * 0.25));
    /// ## Rem - Standard Size 45
    pub const R_45_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(45.0 * 0.25));
    /// ## Rem - Standard Size 46
    pub const R_46_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(46.0 * 0.25));
    /// ## Rem - Standard Size 47
    pub const R_47_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(47.0 * 0.25));
    /// ## Rem - Standard Size 48
    pub const R_48_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(48.0 * 0.25));
    /// ## Rem - Standard Size 49
    pub const R_49_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(49.0 * 0.25));
    /// ## Rem - Standard Size 50
    pub const R_50_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(50.0 * 0.25));
    /// ## Rem - Standard Size 51
    pub const R_51_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(51.0 * 0.25));
    /// ## Rem - Standard Size 52
    pub const R_52_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(52.0 * 0.25));
    /// ## Rem - Standard Size 53
    pub const R_53_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(53.0 * 0.25));
    /// ## Rem - Standard Size 54
    pub const R_54_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(54.0 * 0.25));
    /// ## Rem - Standard Size 55
    pub const R_55_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(55.0 * 0.25));
    /// ## Rem - Standard Size 56
    pub const R_56_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(56.0 * 0.25));
    /// ## Rem - Standard Size 57
    pub const R_57_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(57.0 * 0.25));
    /// ## Rem - Standard Size 58
    pub const R_58_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(58.0 * 0.25));
    /// ## Rem - Standard Size 59
    pub const R_59_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(59.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_60_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(60.0 * 0.25));
    /// ## Rem - Standard Size 61
    pub const R_61_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(61.0 * 0.25));
    /// ## Rem - Standard Size 62
    pub const R_62_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(62.0 * 0.25));
    /// ## Rem - Standard Size 63
    pub const R_63_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(63.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_64_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(64.0 * 0.25));
    /// ## Rem - Standard Size 65
    pub const R_65_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(65.0 * 0.25));
    /// ## Rem - Standard Size 66
    pub const R_66_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(66.0 * 0.25));
    /// ## Rem - Standard Size 67
    pub const R_67_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(67.0 * 0.25));
    /// ## Rem - Standard Size 68
    pub const R_68_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(68.0 * 0.25));
    /// ## Rem - Standard Size 69
    pub const R_69_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(69.0 * 0.25));
    /// ## Rem - Standard Size 70
    pub const R_70_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(70.0 * 0.25));
    /// ## Rem - Standard Size 71
    pub const R_71_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(71.0 * 0.25));
    /// ## Rem - Standard Size 72
    pub const R_72_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(72.0 * 0.25));
    /// ## Rem - Standard Size 73
    pub const R_73_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(73.0 * 0.25));
    /// ## Rem - Standard Size 74
    pub const R_74_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(74.0 * 0.25));
    /// ## Rem - Standard Size 75
    pub const R_75_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(75.0 * 0.25));
    /// ## Rem - Standard Size 76
    pub const R_76_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(76.0 * 0.25));
    /// ## Rem - Standard Size 77
    pub const R_77_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(77.0 * 0.25));
    /// ## Rem - Standard Size 78
    pub const R_78_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(78.0 * 0.25));
    /// ## Rem - Standard Size 79
    pub const R_79_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(79.0 * 0.25));
    /// ## Rem - Standard Size 80
    pub const R_80_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(80.0 * 0.25));
    /// ## Rem - Standard Size 81
    pub const R_81_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(81.0 * 0.25));
    /// ## Rem - Standard Size 82
    pub const R_82_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(82.0 * 0.25));
    /// ## Rem - Standard Size 83
    pub const R_83_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(83.0 * 0.25));
    /// ## Rem - Standard Size 84
    pub const R_84_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(84.0 * 0.25));
    /// ## Rem - Standard Size 85
    pub const R_85_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(85.0 * 0.25));
    /// ## Rem - Standard Size 86
    pub const R_86_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(86.0 * 0.25));
    /// ## Rem - Standard Size 87
    pub const R_87_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(87.0 * 0.25));
    /// ## Rem - Standard Size 88
    pub const R_88_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(88.0 * 0.25));
    /// ## Rem - Standard Size 89
    pub const R_89_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(89.0 * 0.25));
    /// ## Rem - Standard Size 90
    pub const R_90_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(90.0 * 0.25));
    /// ## Rem - Standard Size 91
    pub const R_91_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(91.0 * 0.25));
    /// ## Rem - Standard Size 92
    pub const R_92_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(92.0 * 0.25));
    /// ## Rem - Standard Size 93
    pub const R_93_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(93.0 * 0.25));
    /// ## Rem - Standard Size 94
    pub const R_94_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(94.0 * 0.25));
    /// ## Rem - Standard Size 95
    pub const R_95_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(95.0 * 0.25));
    /// ## Rem - Standard Size 96
    pub const R_96_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(96.0 * 0.25));
    /// ## Rem - Standard Size 97
    pub const R_97_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(97.0 * 0.25));
    /// ## Rem - Standard Size 98
    pub const R_98_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(98.0 * 0.25));
    /// ## Rem - Standard Size 99
    pub const R_99_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(99.0 * 0.25));
    /// ## Rem - Standard Size 100
    pub const R_100_VEC3: NodeSize<Vec3> = NodeSize::from_rem(Vec3::splat(100.0 * 0.25));
}
impl NodeSize<Vec4> {
    /// ## Abs - Extra-small
    pub const A_XS_VEC4: NodeSize<Vec4> = NodeSize::A_4_VEC4;
    /// ## Abs - Small
    pub const A_SM_VEC4: NodeSize<Vec4> = NodeSize::A_8_VEC4;
    /// ## Abs - Medium
    pub const A_MD_VEC4: NodeSize<Vec4> = NodeSize::A_12_VEC4;
    /// ## Abs - Large
    pub const A_LG_VEC4: NodeSize<Vec4> = NodeSize::A_16_VEC4;
    /// ## Abs - Extra-large
    pub const A_XL_VEC4: NodeSize<Vec4> = NodeSize::A_24_VEC4;
    /// ## Abs - Extra-large 2
    pub const A_XL2_VEC4: NodeSize<Vec4> = NodeSize::A_32_VEC4;
    /// ## Abs - Extra-large 3
    pub const A_XL3_VEC4: NodeSize<Vec4> = NodeSize::A_40_VEC4;
    /// ## Abs - Extra-large 4
    pub const A_XL4_VEC4: NodeSize<Vec4> = NodeSize::A_48_VEC4;
    /// ## Abs - Extra-large 5
    pub const A_XL5_VEC4: NodeSize<Vec4> = NodeSize::A_56_VEC4;
    /// ## Abs - Extra-large 6
    pub const A_XL6_VEC4: NodeSize<Vec4> = NodeSize::A_64_VEC4;
    /// ## Abs - Extra-large 7
    pub const A_XL7_VEC4: NodeSize<Vec4> = NodeSize::A_72_VEC4;

    /// ## Abs - Standard Size 0
    pub const A_0_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::ZERO);
    /// ## Abs - Standard Size 1
    pub const A_1_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(1.0 * 4.0));
    /// ## Abs - Standard Size 2
    pub const A_2_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(2.0 * 4.0));
    /// ## Abs - Standard Size 3
    pub const A_3_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(3.0 * 4.0));
    /// ## Abs - Standard Size 4
    pub const A_4_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(4.0 * 4.0));
    /// ## Abs - Standard Size 5
    pub const A_5_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(5.0 * 4.0));
    /// ## Abs - Standard Size 6
    pub const A_6_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(6.0 * 4.0));
    /// ## Abs - Standard Size 7
    pub const A_7_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(7.0 * 4.0));
    /// ## Abs - Standard Size 8
    pub const A_8_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(8.0 * 4.0));
    /// ## Abs - Standard Size 9
    pub const A_9_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(9.0 * 4.0));
    /// ## Abs - Standard Size 10
    pub const A_10_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(10.0 * 4.0));
    /// ## Abs - Standard Size 11
    pub const A_11_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(11.0 * 4.0));
    /// ## Abs - Standard Size 12
    pub const A_12_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(12.0 * 4.0));
    /// ## Abs - Standard Size 13
    pub const A_13_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(13.0 * 4.0));
    /// ## Abs - Standard Size 14
    pub const A_14_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(14.0 * 4.0));
    /// ## Abs - Standard Size 15
    pub const A_15_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(15.0 * 4.0));
    /// ## Abs - Standard Size 16
    pub const A_16_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(16.0 * 4.0));
    /// ## Abs - Standard Size 17
    pub const A_17_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(17.0 * 4.0));
    /// ## Abs - Standard Size 18
    pub const A_18_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(18.0 * 4.0));
    /// ## Abs - Standard Size 19
    pub const A_19_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(19.0 * 4.0));
    /// ## Abs - Standard Size 20
    pub const A_20_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(20.0 * 4.0));
    /// ## Abs - Standard Size 21
    pub const A_21_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(21.0 * 4.0));
    /// ## Abs - Standard Size 22
    pub const A_22_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(22.0 * 4.0));
    /// ## Abs - Standard Size 23
    pub const A_23_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(23.0 * 4.0));
    /// ## Abs - Standard Size 24
    pub const A_24_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(24.0 * 4.0));
    /// ## Abs - Standard Size 25
    pub const A_25_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(25.0 * 4.0));
    /// ## Abs - Standard Size 26
    pub const A_26_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(26.0 * 4.0));
    /// ## Abs - Standard Size 27
    pub const A_27_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(27.0 * 4.0));
    /// ## Abs - Standard Size 28
    pub const A_28_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(28.0 * 4.0));
    /// ## Abs - Standard Size 29
    pub const A_29_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(29.0 * 4.0));
    /// ## Abs - Standard Size 30
    pub const A_30_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(30.0 * 4.0));
    /// ## Abs - Standard Size 31
    pub const A_31_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(31.0 * 4.0));
    /// ## Abs - Standard Size 32
    pub const A_32_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(32.0 * 4.0));
    /// ## Abs - Standard Size 33
    pub const A_33_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(33.0 * 4.0));
    /// ## Abs - Standard Size 34
    pub const A_34_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(34.0 * 4.0));
    /// ## Abs - Standard Size 35
    pub const A_35_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(35.0 * 4.0));
    /// ## Abs - Standard Size 36
    pub const A_36_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(36.0 * 4.0));
    /// ## Abs - Standard Size 37
    pub const A_37_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(37.0 * 4.0));
    /// ## Abs - Standard Size 38
    pub const A_38_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(38.0 * 4.0));
    /// ## Abs - Standard Size 39
    pub const A_39_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(39.0 * 4.0));
    /// ## Abs - Standard Size 40
    pub const A_40_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(40.0 * 4.0));
    /// ## Abs - Standard Size 41
    pub const A_41_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(41.0 * 4.0));
    /// ## Abs - Standard Size 42
    pub const A_42_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(42.0 * 4.0));
    /// ## Abs - Standard Size 43
    pub const A_43_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(43.0 * 4.0));
    /// ## Abs - Standard Size 44
    pub const A_44_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(44.0 * 4.0));
    /// ## Abs - Standard Size 45
    pub const A_45_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(45.0 * 4.0));
    /// ## Abs - Standard Size 46
    pub const A_46_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(46.0 * 4.0));
    /// ## Abs - Standard Size 47
    pub const A_47_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(47.0 * 4.0));
    /// ## Abs - Standard Size 48
    pub const A_48_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(48.0 * 4.0));
    /// ## Abs - Standard Size 49
    pub const A_49_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(49.0 * 4.0));
    /// ## Abs - Standard Size 50
    pub const A_50_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(50.0 * 4.0));
    /// ## Abs - Standard Size 51
    pub const A_51_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(51.0 * 4.0));
    /// ## Abs - Standard Size 52
    pub const A_52_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(52.0 * 4.0));
    /// ## Abs - Standard Size 53
    pub const A_53_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(53.0 * 4.0));
    /// ## Abs - Standard Size 54
    pub const A_54_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(54.0 * 4.0));
    /// ## Abs - Standard Size 55
    pub const A_55_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(55.0 * 4.0));
    /// ## Abs - Standard Size 56
    pub const A_56_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(56.0 * 4.0));
    /// ## Abs - Standard Size 57
    pub const A_57_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(57.0 * 4.0));
    /// ## Abs - Standard Size 58
    pub const A_58_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(58.0 * 4.0));
    /// ## Abs - Standard Size 59
    pub const A_59_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(59.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_60_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(60.0 * 4.0));
    /// ## Abs - Standard Size 61
    pub const A_61_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(61.0 * 4.0));
    /// ## Abs - Standard Size 62
    pub const A_62_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(62.0 * 4.0));
    /// ## Abs - Standard Size 63
    pub const A_63_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(63.0 * 4.0));
    /// ## Abs - Standard Size 64
    pub const A_64_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(64.0 * 4.0));
    /// ## Abs - Standard Size 65
    pub const A_65_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(65.0 * 4.0));
    /// ## Abs - Standard Size 66
    pub const A_66_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(66.0 * 4.0));
    /// ## Abs - Standard Size 67
    pub const A_67_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(67.0 * 4.0));
    /// ## Abs - Standard Size 68
    pub const A_68_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(68.0 * 4.0));
    /// ## Abs - Standard Size 69
    pub const A_69_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(69.0 * 4.0));
    /// ## Abs - Standard Size 70
    pub const A_70_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(70.0 * 4.0));
    /// ## Abs - Standard Size 71
    pub const A_71_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(71.0 * 4.0));
    /// ## Abs - Standard Size 72
    pub const A_72_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(72.0 * 4.0));
    /// ## Abs - Standard Size 73
    pub const A_73_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(73.0 * 4.0));
    /// ## Abs - Standard Size 74
    pub const A_74_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(74.0 * 4.0));
    /// ## Abs - Standard Size 75
    pub const A_75_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(75.0 * 4.0));
    /// ## Abs - Standard Size 76
    pub const A_76_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(76.0 * 4.0));
    /// ## Abs - Standard Size 77
    pub const A_77_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(77.0 * 4.0));
    /// ## Abs - Standard Size 78
    pub const A_78_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(78.0 * 4.0));
    /// ## Abs - Standard Size 79
    pub const A_79_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(79.0 * 4.0));
    /// ## Abs - Standard Size 80
    pub const A_80_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(80.0 * 4.0));
    /// ## Abs - Standard Size 81
    pub const A_81_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(81.0 * 4.0));
    /// ## Abs - Standard Size 82
    pub const A_82_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(82.0 * 4.0));
    /// ## Abs - Standard Size 83
    pub const A_83_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(83.0 * 4.0));
    /// ## Abs - Standard Size 84
    pub const A_84_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(84.0 * 4.0));
    /// ## Abs - Standard Size 85
    pub const A_85_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(85.0 * 4.0));
    /// ## Abs - Standard Size 86
    pub const A_86_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(86.0 * 4.0));
    /// ## Abs - Standard Size 87
    pub const A_87_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(87.0 * 4.0));
    /// ## Abs - Standard Size 88
    pub const A_88_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(88.0 * 4.0));
    /// ## Abs - Standard Size 89
    pub const A_89_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(89.0 * 4.0));
    /// ## Abs - Standard Size 90
    pub const A_90_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(90.0 * 4.0));
    /// ## Abs - Standard Size 91
    pub const A_91_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(91.0 * 4.0));
    /// ## Abs - Standard Size 92
    pub const A_92_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(92.0 * 4.0));
    /// ## Abs - Standard Size 93
    pub const A_93_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(93.0 * 4.0));
    /// ## Abs - Standard Size 94
    pub const A_94_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(94.0 * 4.0));
    /// ## Abs - Standard Size 95
    pub const A_95_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(95.0 * 4.0));
    /// ## Abs - Standard Size 96
    pub const A_96_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(96.0 * 4.0));
    /// ## Abs - Standard Size 97
    pub const A_97_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(97.0 * 4.0));
    /// ## Abs - Standard Size 98
    pub const A_98_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(98.0 * 4.0));
    /// ## Abs - Standard Size 99
    pub const A_99_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(99.0 * 4.0));
    /// ## Abs - Standard Size 100
    pub const A_100_VEC4: NodeSize<Vec4> = NodeSize::from_abs(Vec4::splat(100.0 * 4.0));

    /// ## Rem - Extra-small
    pub const R_XS_VEC4: NodeSize<Vec4> = NodeSize::R_4_VEC4;
    /// ## Rem - Small
    pub const R_SM_VEC4: NodeSize<Vec4> = NodeSize::R_8_VEC4;
    /// ## Rem - Medium
    pub const R_MD_VEC4: NodeSize<Vec4> = NodeSize::R_12_VEC4;
    /// ## Rem - Large
    pub const R_LG_VEC4: NodeSize<Vec4> = NodeSize::R_16_VEC4;
    /// ## Rem - Extra-large
    pub const R_XL_VEC4: NodeSize<Vec4> = NodeSize::R_24_VEC4;
    /// ## Rem - Extra-large 2
    pub const R_XL2_VEC4: NodeSize<Vec4> = NodeSize::R_32_VEC4;
    /// ## Rem - Extra-large 3
    pub const R_XL3_VEC4: NodeSize<Vec4> = NodeSize::R_40_VEC4;
    /// ## Rem - Extra-large 4
    pub const R_XL4_VEC4: NodeSize<Vec4> = NodeSize::R_48_VEC4;
    /// ## Rem - Extra-large 5
    pub const R_XL5_VEC4: NodeSize<Vec4> = NodeSize::R_56_VEC4;
    /// ## Rem - Extra-large 6
    pub const R_XL6_VEC4: NodeSize<Vec4> = NodeSize::R_64_VEC4;
    /// ## Rem - Extra-large 7
    pub const R_XL7_VEC4: NodeSize<Vec4> = NodeSize::R_72_VEC4;

    /// ## Rem - Standard Size 0
    pub const R_0_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::ZERO);
    /// ## Rem - Standard Size 1
    pub const R_1_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(1.0 * 0.25));
    /// ## Rem - Standard Size 2
    pub const R_2_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(2.0 * 0.25));
    /// ## Rem - Standard Size 3
    pub const R_3_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(3.0 * 0.25));
    /// ## Rem - Standard Size 4
    pub const R_4_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(4.0 * 0.25));
    /// ## Rem - Standard Size 5
    pub const R_5_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(5.0 * 0.25));
    /// ## Rem - Standard Size 6
    pub const R_6_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(6.0 * 0.25));
    /// ## Rem - Standard Size 7
    pub const R_7_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(7.0 * 0.25));
    /// ## Rem - Standard Size 8
    pub const R_8_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(8.0 * 0.25));
    /// ## Rem - Standard Size 9
    pub const R_9_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(9.0 * 0.25));
    /// ## Rem - Standard Size 10
    pub const R_10_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(10.0 * 0.25));
    /// ## Rem - Standard Size 11
    pub const R_11_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(11.0 * 0.25));
    /// ## Rem - Standard Size 12
    pub const R_12_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(12.0 * 0.25));
    /// ## Rem - Standard Size 13
    pub const R_13_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(13.0 * 0.25));
    /// ## Rem - Standard Size 14
    pub const R_14_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(14.0 * 0.25));
    /// ## Rem - Standard Size 15
    pub const R_15_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(15.0 * 0.25));
    /// ## Rem - Standard Size 16
    pub const R_16_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(16.0 * 0.25));
    /// ## Rem - Standard Size 17
    pub const R_17_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(17.0 * 0.25));
    /// ## Rem - Standard Size 18
    pub const R_18_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(18.0 * 0.25));
    /// ## Rem - Standard Size 19
    pub const R_19_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(19.0 * 0.25));
    /// ## Rem - Standard Size 20
    pub const R_20_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(20.0 * 0.25));
    /// ## Rem - Standard Size 21
    pub const R_21_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(21.0 * 0.25));
    /// ## Rem - Standard Size 22
    pub const R_22_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(22.0 * 0.25));
    /// ## Rem - Standard Size 23
    pub const R_23_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(23.0 * 0.25));
    /// ## Rem - Standard Size 24
    pub const R_24_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(24.0 * 0.25));
    /// ## Rem - Standard Size 25
    pub const R_25_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(25.0 * 0.25));
    /// ## Rem - Standard Size 26
    pub const R_26_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(26.0 * 0.25));
    /// ## Rem - Standard Size 27
    pub const R_27_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(27.0 * 0.25));
    /// ## Rem - Standard Size 28
    pub const R_28_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(28.0 * 0.25));
    /// ## Rem - Standard Size 29
    pub const R_29_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(29.0 * 0.25));
    /// ## Rem - Standard Size 30
    pub const R_30_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(30.0 * 0.25));
    /// ## Rem - Standard Size 31
    pub const R_31_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(31.0 * 0.25));
    /// ## Rem - Standard Size 32
    pub const R_32_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(32.0 * 0.25));
    /// ## Rem - Standard Size 33
    pub const R_33_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(33.0 * 0.25));
    /// ## Rem - Standard Size 34
    pub const R_34_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(34.0 * 0.25));
    /// ## Rem - Standard Size 35
    pub const R_35_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(35.0 * 0.25));
    /// ## Rem - Standard Size 36
    pub const R_36_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(36.0 * 0.25));
    /// ## Rem - Standard Size 37
    pub const R_37_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(37.0 * 0.25));
    /// ## Rem - Standard Size 38
    pub const R_38_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(38.0 * 0.25));
    /// ## Rem - Standard Size 39
    pub const R_39_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(39.0 * 0.25));
    /// ## Rem - Standard Size 40
    pub const R_40_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(40.0 * 0.25));
    /// ## Rem - Standard Size 41
    pub const R_41_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(41.0 * 0.25));
    /// ## Rem - Standard Size 42
    pub const R_42_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(42.0 * 0.25));
    /// ## Rem - Standard Size 43
    pub const R_43_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(43.0 * 0.25));
    /// ## Rem - Standard Size 44
    pub const R_44_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(44.0 * 0.25));
    /// ## Rem - Standard Size 45
    pub const R_45_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(45.0 * 0.25));
    /// ## Rem - Standard Size 46
    pub const R_46_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(46.0 * 0.25));
    /// ## Rem - Standard Size 47
    pub const R_47_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(47.0 * 0.25));
    /// ## Rem - Standard Size 48
    pub const R_48_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(48.0 * 0.25));
    /// ## Rem - Standard Size 49
    pub const R_49_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(49.0 * 0.25));
    /// ## Rem - Standard Size 50
    pub const R_50_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(50.0 * 0.25));
    /// ## Rem - Standard Size 51
    pub const R_51_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(51.0 * 0.25));
    /// ## Rem - Standard Size 52
    pub const R_52_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(52.0 * 0.25));
    /// ## Rem - Standard Size 53
    pub const R_53_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(53.0 * 0.25));
    /// ## Rem - Standard Size 54
    pub const R_54_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(54.0 * 0.25));
    /// ## Rem - Standard Size 55
    pub const R_55_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(55.0 * 0.25));
    /// ## Rem - Standard Size 56
    pub const R_56_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(56.0 * 0.25));
    /// ## Rem - Standard Size 57
    pub const R_57_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(57.0 * 0.25));
    /// ## Rem - Standard Size 58
    pub const R_58_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(58.0 * 0.25));
    /// ## Rem - Standard Size 59
    pub const R_59_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(59.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_60_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(60.0 * 0.25));
    /// ## Rem - Standard Size 61
    pub const R_61_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(61.0 * 0.25));
    /// ## Rem - Standard Size 62
    pub const R_62_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(62.0 * 0.25));
    /// ## Rem - Standard Size 63
    pub const R_63_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(63.0 * 0.25));
    /// ## Rem - Standard Size 64
    pub const R_64_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(64.0 * 0.25));
    /// ## Rem - Standard Size 65
    pub const R_65_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(65.0 * 0.25));
    /// ## Rem - Standard Size 66
    pub const R_66_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(66.0 * 0.25));
    /// ## Rem - Standard Size 67
    pub const R_67_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(67.0 * 0.25));
    /// ## Rem - Standard Size 68
    pub const R_68_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(68.0 * 0.25));
    /// ## Rem - Standard Size 69
    pub const R_69_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(69.0 * 0.25));
    /// ## Rem - Standard Size 70
    pub const R_70_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(70.0 * 0.25));
    /// ## Rem - Standard Size 71
    pub const R_71_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(71.0 * 0.25));
    /// ## Rem - Standard Size 72
    pub const R_72_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(72.0 * 0.25));
    /// ## Rem - Standard Size 73
    pub const R_73_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(73.0 * 0.25));
    /// ## Rem - Standard Size 74
    pub const R_74_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(74.0 * 0.25));
    /// ## Rem - Standard Size 75
    pub const R_75_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(75.0 * 0.25));
    /// ## Rem - Standard Size 76
    pub const R_76_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(76.0 * 0.25));
    /// ## Rem - Standard Size 77
    pub const R_77_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(77.0 * 0.25));
    /// ## Rem - Standard Size 78
    pub const R_78_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(78.0 * 0.25));
    /// ## Rem - Standard Size 79
    pub const R_79_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(79.0 * 0.25));
    /// ## Rem - Standard Size 80
    pub const R_80_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(80.0 * 0.25));
    /// ## Rem - Standard Size 81
    pub const R_81_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(81.0 * 0.25));
    /// ## Rem - Standard Size 82
    pub const R_82_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(82.0 * 0.25));
    /// ## Rem - Standard Size 83
    pub const R_83_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(83.0 * 0.25));
    /// ## Rem - Standard Size 84
    pub const R_84_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(84.0 * 0.25));
    /// ## Rem - Standard Size 85
    pub const R_85_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(85.0 * 0.25));
    /// ## Rem - Standard Size 86
    pub const R_86_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(86.0 * 0.25));
    /// ## Rem - Standard Size 87
    pub const R_87_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(87.0 * 0.25));
    /// ## Rem - Standard Size 88
    pub const R_88_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(88.0 * 0.25));
    /// ## Rem - Standard Size 89
    pub const R_89_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(89.0 * 0.25));
    /// ## Rem - Standard Size 90
    pub const R_90_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(90.0 * 0.25));
    /// ## Rem - Standard Size 91
    pub const R_91_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(91.0 * 0.25));
    /// ## Rem - Standard Size 92
    pub const R_92_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(92.0 * 0.25));
    /// ## Rem - Standard Size 93
    pub const R_93_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(93.0 * 0.25));
    /// ## Rem - Standard Size 94
    pub const R_94_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(94.0 * 0.25));
    /// ## Rem - Standard Size 95
    pub const R_95_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(95.0 * 0.25));
    /// ## Rem - Standard Size 96
    pub const R_96_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(96.0 * 0.25));
    /// ## Rem - Standard Size 97
    pub const R_97_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(97.0 * 0.25));
    /// ## Rem - Standard Size 98
    pub const R_98_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(98.0 * 0.25));
    /// ## Rem - Standard Size 99
    pub const R_99_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(99.0 * 0.25));
    /// ## Rem - Standard Size 100
    pub const R_100_VEC4: NodeSize<Vec4> = NodeSize::from_rem(Vec4::splat(100.0 * 0.25));
}

// # Impl CONSTS
impl Abs<f32> {
    /// ## Zero
    pub const ZERO: Abs<f32> = Abs(0.0);
    /// ## One
    pub const ONE: Abs<f32> = Abs(1.0);
    /// ## Extra-small
    pub const XS: Abs<f32> = Abs(1.0 * 16.0);
    /// ## Small
    pub const SM: Abs<f32> = Abs(2.0 * 16.0);
    /// ## Medium
    pub const MD: Abs<f32> = Abs(3.0 * 16.0);
    /// ## Large
    pub const LG: Abs<f32> = Abs(4.0 * 16.0);
    /// ## Extra-large
    pub const XL: Abs<f32> = Abs(6.0 * 16.0);
    /// ## Extra-large 2
    pub const XL2: Abs<f32> = Abs(8.0 * 16.0);
    /// ## Extra-large 3
    pub const XL3: Abs<f32> = Abs(10.0 * 16.0);
    /// ## Extra-large 4
    pub const XL4: Abs<f32> = Abs(12.0 * 16.0);
    /// ## Extra-large 5
    pub const XL5: Abs<f32> = Abs(14.0 * 16.0);
    /// ## Extra-large 6
    pub const XL6: Abs<f32> = Abs(16.0 * 16.0);
    /// ## Extra-large 7
    pub const XL7: Abs<f32> = Abs(18.0 * 16.0);
}
impl Prc<f32> {
    /// ## Zero
    pub const ZERO: Prc<f32> = Prc(0.0);
    /// ## One
    pub const ONE: Prc<f32> = Prc(1.0);
    /// ## Full
    pub const FULL: Prc<f32> = Prc(100.0);
    /// ## Half
    pub const HALF: Prc<f32> = Prc(100.0 / 2.0);
    /// ## Third
    pub const THIRD: Prc<f32> = Prc(100.0 / 3.0);
    /// ## Quarter
    pub const QUARTER: Prc<f32> = Prc(100.0 / 4.0);
    /// ## Fifth
    pub const FIFTH: Prc<f32> = Prc(100.0 / 5.0);
    /// ## Sixth
    pub const SIXTH: Prc<f32> = Prc(100.0 / 6.0);
    /// ## Seventh
    pub const SEVENTH: Prc<f32> = Prc(100.0 / 7.0);
    /// ## Eighth
    pub const EIGHTH: Prc<f32> = Prc(100.0 / 8.0);
    /// ## Ninth
    pub const NINTH: Prc<f32> = Prc(100.0 / 9.0);
    /// ## Tenth
    pub const TENTH: Prc<f32> = Prc(100.0 / 10.0);
    /// ## Eleventh
    pub const ELEVENTH: Prc<f32> = Prc(100.0 / 11.0);
    /// ## Twelfth
    pub const TWELFTH: Prc<f32> = Prc(100.0 / 12.0);
    /// ## Thirteenth
    pub const THIRTEENTH: Prc<f32> = Prc(100.0 / 13.0);
    /// ## Fourteenth
    pub const FOURTEENTH: Prc<f32> = Prc(100.0 / 14.0);
    /// ## Fifteenth
    pub const FIFTEENTH: Prc<f32> = Prc(100.0 / 15.0);
    /// ## Sixteenth
    pub const SIXTEENTH: Prc<f32> = Prc(100.0 / 16.0);
    /// ## Seventeenth
    pub const SEVENTEENTH: Prc<f32> = Prc(100.0 / 17.0);
    /// ## Eighteenth
    pub const EIGHTEENTH: Prc<f32> = Prc(100.0 / 18.0);
    /// ## Nineteenth
    pub const NINETEENTH: Prc<f32> = Prc(100.0 / 19.0);
    /// ## Twentieth
    pub const TWENTIETH: Prc<f32> = Prc(100.0 / 20.0);
}
impl Rem<f32> {
    /// ## Zero
    pub const ZERO: Rem<f32> = Rem(0.0);
    /// ## One
    pub const ONE: Rem<f32> = Rem(1.0);
    /// ## Extra-small
    pub const XS: Rem<f32> = Rem(1.0);
    /// ## Small
    pub const SM: Rem<f32> = Rem(2.0);
    /// ## Medium
    pub const MD: Rem<f32> = Rem(3.0);
    /// ## Large
    pub const LG: Rem<f32> = Rem(4.0);
    /// ## Extra-large
    pub const XL: Rem<f32> = Rem(6.0);
    /// ## Extra-large 2
    pub const XL2: Rem<f32> = Rem(8.0);
    /// ## Extra-large 3
    pub const XL3: Rem<f32> = Rem(10.0);
    /// ## Extra-large 4
    pub const XL4: Rem<f32> = Rem(12.0);
    /// ## Extra-large 5
    pub const XL5: Rem<f32> = Rem(14.0);
    /// ## Extra-large 6
    pub const XL6: Rem<f32> = Rem(16.0);
    /// ## Extra-large 7
    pub const XL7: Rem<f32> = Rem(18.0);
}

impl Abs<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Abs<Vec2> = Abs(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Abs<Vec2> = Abs(Vec2::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC2: Abs<Vec2> = Abs(Vec2::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC2: Abs<Vec2> = Abs(Vec2::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC2: Abs<Vec2> = Abs(Vec2::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC2: Abs<Vec2> = Abs(Vec2::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC2: Abs<Vec2> = Abs(Vec2::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC2: Abs<Vec2> = Abs(Vec2::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC2: Abs<Vec2> = Abs(Vec2::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC2: Abs<Vec2> = Abs(Vec2::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC2: Abs<Vec2> = Abs(Vec2::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC2: Abs<Vec2> = Abs(Vec2::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC2: Abs<Vec2> = Abs(Vec2::splat(18.0 * 16.0));
}
impl Prc<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Prc<Vec2> = Prc(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Prc<Vec2> = Prc(Vec2::splat(1.0));
    /// ## Full
    pub const FULL_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0));
    /// ## Half
    pub const HALF_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 20.0));
}
impl Rem<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Rem<Vec2> = Rem(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Rem<Vec2> = Rem(Vec2::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC2: Rem<Vec2> = Rem(Vec2::splat(1.0));
    /// ## Small
    pub const SM_VEC2: Rem<Vec2> = Rem(Vec2::splat(2.0));
    /// ## Medium
    pub const MD_VEC2: Rem<Vec2> = Rem(Vec2::splat(3.0));
    /// ## Large
    pub const LG_VEC2: Rem<Vec2> = Rem(Vec2::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC2: Rem<Vec2> = Rem(Vec2::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC2: Rem<Vec2> = Rem(Vec2::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC2: Rem<Vec2> = Rem(Vec2::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC2: Rem<Vec2> = Rem(Vec2::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC2: Rem<Vec2> = Rem(Vec2::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC2: Rem<Vec2> = Rem(Vec2::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC2: Rem<Vec2> = Rem(Vec2::splat(18.0));
}

impl Abs<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Abs<Vec3> = Abs(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Abs<Vec3> = Abs(Vec3::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC3: Abs<Vec3> = Abs(Vec3::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC3: Abs<Vec3> = Abs(Vec3::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC3: Abs<Vec3> = Abs(Vec3::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC3: Abs<Vec3> = Abs(Vec3::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC3: Abs<Vec3> = Abs(Vec3::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC3: Abs<Vec3> = Abs(Vec3::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC3: Abs<Vec3> = Abs(Vec3::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC3: Abs<Vec3> = Abs(Vec3::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC3: Abs<Vec3> = Abs(Vec3::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC3: Abs<Vec3> = Abs(Vec3::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC3: Abs<Vec3> = Abs(Vec3::splat(18.0 * 16.0));
}
impl Prc<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Prc<Vec3> = Prc(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Prc<Vec3> = Prc(Vec3::splat(1.0));
    /// ## Full
    pub const FULL_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0));
    /// ## Half
    pub const HALF_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 20.0));
}
impl Rem<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Rem<Vec3> = Rem(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Rem<Vec3> = Rem(Vec3::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC3: Rem<Vec3> = Rem(Vec3::splat(1.0));
    /// ## Small
    pub const SM_VEC3: Rem<Vec3> = Rem(Vec3::splat(2.0));
    /// ## Medium
    pub const MD_VEC3: Rem<Vec3> = Rem(Vec3::splat(3.0));
    /// ## Large
    pub const LG_VEC3: Rem<Vec3> = Rem(Vec3::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC3: Rem<Vec3> = Rem(Vec3::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC3: Rem<Vec3> = Rem(Vec3::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC3: Rem<Vec3> = Rem(Vec3::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC3: Rem<Vec3> = Rem(Vec3::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC3: Rem<Vec3> = Rem(Vec3::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC3: Rem<Vec3> = Rem(Vec3::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC3: Rem<Vec3> = Rem(Vec3::splat(18.0));
}

impl Abs<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Abs<Vec4> = Abs(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Abs<Vec4> = Abs(Vec4::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC4: Abs<Vec4> = Abs(Vec4::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC4: Abs<Vec4> = Abs(Vec4::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC4: Abs<Vec4> = Abs(Vec4::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC4: Abs<Vec4> = Abs(Vec4::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC4: Abs<Vec4> = Abs(Vec4::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC4: Abs<Vec4> = Abs(Vec4::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC4: Abs<Vec4> = Abs(Vec4::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC4: Abs<Vec4> = Abs(Vec4::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC4: Abs<Vec4> = Abs(Vec4::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC4: Abs<Vec4> = Abs(Vec4::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC4: Abs<Vec4> = Abs(Vec4::splat(18.0 * 16.0));
}
impl Prc<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Prc<Vec4> = Prc(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Prc<Vec4> = Prc(Vec4::splat(1.0));
    /// ## Full
    pub const FULL_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0));
    /// ## Half
    pub const HALF_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 20.0));
}
impl Rem<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Rem<Vec4> = Rem(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Rem<Vec4> = Rem(Vec4::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC4: Rem<Vec4> = Rem(Vec4::splat(1.0));
    /// ## Small
    pub const SM_VEC4: Rem<Vec4> = Rem(Vec4::splat(2.0));
    /// ## Medium
    pub const MD_VEC4: Rem<Vec4> = Rem(Vec4::splat(3.0));
    /// ## Large
    pub const LG_VEC4: Rem<Vec4> = Rem(Vec4::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC4: Rem<Vec4> = Rem(Vec4::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC4: Rem<Vec4> = Rem(Vec4::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC4: Rem<Vec4> = Rem(Vec4::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC4: Rem<Vec4> = Rem(Vec4::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC4: Rem<Vec4> = Rem(Vec4::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC4: Rem<Vec4> = Rem(Vec4::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC4: Rem<Vec4> = Rem(Vec4::splat(18.0));
}

pub type Size = NodeSize<f32>;