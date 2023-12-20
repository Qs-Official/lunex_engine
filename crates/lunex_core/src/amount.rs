use std::ops::Add;
use std::ops::AddAssign;

use bevy::prelude::*;

// # TEST
#[cfg(test)]
mod test {
    use super::{Abs, Prc, Rem, Amount};
    #[test]
    fn all () {

        assert_eq!(Amount::new().with_abs(Abs(5)) + Abs(5) + Abs(5), Amount::new().with_abs(Abs(15)));
        assert_eq!(Amount::new().with_prc(Prc(5)) + Prc(5) + Prc(5), Amount::new().with_prc(Prc(15)));
        assert_eq!(Amount::new().with_rem(Rem(5)) + Rem(5) + Rem(5), Amount::new().with_rem(Rem(15)));

        let amount = Abs(5) + Prc(10) + Rem(15); //??? now working another ADD
        assert_eq!(amount, Amount::new().with_abs(Abs(5)).with_prc(Prc(10)).with_rem(Rem(15)));

        let mut new_amount = amount + Abs(20);

        new_amount += Prc(20);
        new_amount += amount;

        assert_eq!(new_amount, Amount::new().with_abs(Abs(30)).with_prc(Prc(40)).with_rem(Rem(30)));
    }
}




/// ## Absolute
/// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Abs<T>(T);
/// ## Percentage
/// `0% to 100%`. Overflow allowed.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Prc<T>(T);
/// ## Rem
/// Size of symbol `M` which is `16px` with `font size 16px` and so on.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rem<T>(T);

// # Impl `Abs(T) -> Amount(T)`
impl <T> Into<Amount<T>> for Abs<T> {
    fn into(self) -> Amount<T> {
        Amount::new().with_abs(self)
    }
}
// # Impl `Prc(T) -> Amount(T)`
impl <T> Into<Amount<T>> for Prc<T> {
    fn into(self) -> Amount<T> {
        Amount::new().with_prc(self)
    }
}
// # Impl `Rem(T) -> Amount(T)`
impl <T> Into<Amount<T>> for Rem<T> {
    fn into(self) -> Amount<T> {
        Amount::new().with_rem(self)
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
    type Output = Amount<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        Amount::new().with_abs(self).with_prc(other)
    }
}
// # Impl `Abs(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Abs<T> {
    type Output = Amount<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        Amount::new().with_abs(self).with_rem(other)
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
    type Output = Amount<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        Amount::new().with_prc(self).with_abs(other)
    }
}
// # Impl `Prc(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Prc<T> {
    type Output = Amount<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        Amount::new().with_prc(self).with_rem(other)
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
    type Output = Amount<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        Amount::new().with_rem(self).with_abs(other)
    }
}
// # Impl `Rem(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Rem<T> {
    type Output = Amount<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        Amount::new().with_rem(self).with_prc(other)
    }
}



/// # Amount
/// A struct holding size measurment data and how to calculate them.
/// It represents sum of different units used in UI.
/// * `Rem` => _Size of one "M" char_
/// * `Prc` => _Ratio (Percentage)_
/// * `RemPrc` => _Em + Rt_
/// 
/// size 1 = 0.25rem = 4px
/// ## Example
/// ```
/// # use lunex_core::{Amount, ToAmount};
/// let size: Amount<f32> = Amount::Rem(12.0);
/// let size: Amount<f32> = Amount::RemPrc(12.0, 5.0);
/// let size: Amount<f32> = 12.0.to_em() + 5.0.to_rt();
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Amount<T> {
    /// ## Absolute
    /// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
    abs: Option<T>,
    /// ## Percentage
    /// `0% to 100%`. Overflow allowed.
    prc: Option<T>,
    /// ## Rem
    /// Size of symbol `M` which is `16px` with `font size 16px` and so on.
    rem: Option<T>,
}
impl <T> Amount<T> {
    /// # New
    /// Creates new empty amount
    pub fn new() -> Self {
        Amount {
            abs: None,
            prc: None,
            rem: None,
        }
    }
    pub fn with_abs(mut self, abs: Abs<T>) -> Self {
        self.abs = Some(abs.0);
        self
    }
    pub fn with_prc(mut self, prc: Prc<T>) -> Self {
        self.prc = Some(prc.0);
        self
    }
    pub fn with_rem(mut self, rem: Rem<T>) -> Self {
        self.rem = Some(rem.0);
        self
    }

    pub fn set_abs(&mut self, abs: Abs<T>) {
        self.abs = Some(abs.0);
    }
    pub fn set_prc(&mut self, prc: Prc<T>) {
        self.prc = Some(prc.0);
    }
    pub fn set_rem(&mut self, rem: Rem<T>) {
        self.rem = Some(rem.0);
    }
}

// # Impl `Amount(T) + Amount(T)`
impl<T: Add<Output = T> + Add> Add for Amount<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {

        let mut output = Amount::new();

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
// # Impl `Amount(T) + Abs(T)`
impl<T: Add<Output = T> + Add> Add<Abs<T>> for Amount<T> {
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
// # Impl `Amount(T) + Prc(T)`
impl<T: Add<Output = T> + Add> Add<Prc<T>> for Amount<T> {
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
// # Impl `Amount(T) + Rem(T)`
impl<T: Add<Output = T> + Add> Add<Rem<T>> for Amount<T> {
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

// # Impl `Amount(T) += Amount(T)`
impl<T: Add<Output = T> + Copy> AddAssign for Amount<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
// # Impl `Amount(T) += Abs(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Abs<T>> for Amount<T> {
    fn add_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.set_abs(Abs(v + rhs.0)),
            None => self.set_abs(rhs),
        }
    }
}
// # Impl `Amount(T) += Prc(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Prc<T>> for Amount<T> {
    fn add_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.set_prc(Prc(v + rhs.0)),
            None => self.set_prc(rhs),
        }
    }
}
// # Impl `Amount(T) += Rem(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Rem<T>> for Amount<T> {
    fn add_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.set_rem(Rem(v + rhs.0)),
            None => self.set_rem(rhs),
        }
    }
}



/*


// # IMPLEMENTS EVALUATE
// The code that actually computes the final measurment
impl Amount<f32> {
    /// # Evaluate
    /// * `em`: size of one 'M' char from the font with size "12"
    /// * `rt`: true size of 100%
    pub fn evaluate(&self, em: f32, rt: f32) -> f32 {
        match self {
            Amount::Rem(v1) => {
                (*v1/12.0) * em
            },
            Amount::Prc(v1) => {
                (*v1/100.0) * rt
            },
            Amount::RemPrc(v1, v2) => {
                (*v1/12.0) * em + (*v2/100.0) * rt
            },
        }
    }
}
impl Amount<Vec2> {
    /// # Evaluate
    /// * `em`: size of one 'M' char from the font with size "12"
    /// * `rt`: true size of 100%
    pub fn evaluate(&self, em: f32, rt: Vec2) -> Vec2 {
        match self {
            Amount::Rem(v1) => {
                (*v1/12.0) * em
            },
            Amount::Prc(v1) => {
                (*v1/100.0) * rt
            },
            Amount::RemPrc(v1, v2) => {
                (*v1/12.0) * em + (*v2/100.0) * rt
            },
        }
    }
}
impl Amount<Vec3> {
    /// # Evaluate
    /// * `em`: size of one 'M' char from the font with size "12"
    /// * `rt`: true size of 100%
    pub fn evaluate(&self, em: f32, rt: Vec3) -> Vec3 {
        match self {
            Amount::Rem(v1) => {
                (*v1/12.0) * em
            },
            Amount::Prc(v1) => {
                (*v1/100.0) * rt
            },
            Amount::RemPrc(v1, v2) => {
                (*v1/12.0) * em + (*v2/100.0) * rt
            },
        }
    }
}

// # IMPLEMENTS ADD
// Allows adding same generic Amounts together
impl<T: Add<Output = T>> Add for Amount<T> {
    type Output = Self;
    /// # Add
    /// Adds two amounts together.
    /// 
    /// `12em + 20%` + `5%` => `12em + 25%`
    /// ```
    /// # use lunex_core::Amount;
    /// let val = Amount::RemPrc(12.0, 20.0) + Amount::Prc(5.0);
    /// assert_eq!(Amount::RemPrc(12.0, 25.0), val);
    /// ```
    fn add(self, other: Self) -> Self::Output {
        match self {
            Amount::Rem(x1) => match other {
                Amount::Rem(y1) => Amount::Rem(x1 + y1),
                Amount::Prc(y1) => Amount::RemPrc(x1, y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(x1 + y1, y2),
            },
            Amount::Prc(x1) => match other {
                Amount::Rem(y1) => Amount::RemPrc(y1, x1),
                Amount::Prc(y1) => Amount::Prc(x1 + y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(y1, x1 + y2),
            },
            Amount::RemPrc(x1, x2) => match other {
                Amount::Rem(y1) => Amount::RemPrc(x1 + y1, x2),
                Amount::Prc(y1) => Amount::RemPrc(x1, x2 + y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(x1 + y1, x2 + y2),
            },
        }
    }
}

// # IMPLEMENTS SET
// Complex match logic for settings property values.
impl <T> Amount<T> {
    /// # Set
    /// Overwrites the value of appropriate unit.
    /// 
    /// `12em + 20%` set to `5%` => `12em + 5%`
    /// ```
    /// use lunex_core::Amount;
    /// 
    /// let val = Amount::RemPrc(12.0, 20.0).set(Amount::Prc(5.0));
    /// assert_eq!(Amount::RemPrc(12.0, 5.0), val);
    /// ```
    pub fn set(self, other: Amount<T>) -> Self {
        match self {
            Amount::Rem(x1) => match other {
                Amount::Rem(y1) => Amount::Rem(y1),
                Amount::Prc(y1) => Amount::RemPrc(x1, y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(y1, y2),
            },
            Amount::Prc(x1) => match other {
                Amount::Rem(y1) => Amount::RemPrc(y1, x1),
                Amount::Prc(y1) => Amount::Prc(y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(y1, y2),
            },
            Amount::RemPrc(x1, x2) => match other {
                Amount::Rem(y1) => Amount::RemPrc(y1, x2),
                Amount::Prc(y1) => Amount::RemPrc(x1, y1),
                Amount::RemPrc(y1, y2) => Amount::RemPrc(y1, y2),
            },
        }
    }
}
impl Amount<Vec2> {
    /// # Set X
    /// Overwrites the X value of appropriate unit.
    pub fn set_x(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Rem(mut v1) => match other {
                Amount::Rem(w1) => {
                    v1.x = w1;
                    Amount::Rem(v1)
                },
                Amount::Prc(w1) => {
                    Amount::RemPrc(v1, Vec2::new(w1, 0.0))
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w1;
                    Amount::RemPrc(v1, Vec2::new(w2, 0.0))
                },
            },
            Amount::Prc(mut v1) => match other {
                Amount::Rem(w1) => {
                    Amount::RemPrc(Vec2::new(w1, 0.0), v1)
                },
                Amount::Prc(w1) => {
                    v1.x = w1;
                    Amount::Prc(v1)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w2;
                    Amount::RemPrc(Vec2::new(w1, 0.0), v1)
                },
            },
            Amount::RemPrc(mut v1, mut v2) => match other {
                Amount::Rem(w1) => {
                    v1.x = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::Prc(w1) => {
                    v2.x = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w1;
                    v2.x = w2;
                    Amount::RemPrc(v1, v2)
                },
            },
        }
    }
    /// # Set Y
    /// Overwrites the Y value of appropriate unit.
    pub fn set_y(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Rem(mut v1) => match other {
                Amount::Rem(w1) => {
                    v1.y = w1;
                    Amount::Rem(v1)
                },
                Amount::Prc(w1) => {
                    Amount::RemPrc(v1, Vec2::new(0.0, w1))
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w1;
                    Amount::RemPrc(v1, Vec2::new(0.0, w2))
                },
            },
            Amount::Prc(mut v1) => match other {
                Amount::Rem(w1) => {
                    Amount::RemPrc(Vec2::new(0.0, w1), v1)
                },
                Amount::Prc(w1) => {
                    v1.y = w1;
                    Amount::Prc(v1)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w2;
                    Amount::RemPrc(Vec2::new(0.0, w1), v1)
                },
            },
            Amount::RemPrc(mut v1, mut v2) => match other {
                Amount::Rem(w1) => {
                    v1.y = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::Prc(w1) => {
                    v2.y = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w1;
                    v2.y = w2;
                    Amount::RemPrc(v1, v2)
                },
            },
        }
    }
}
impl Amount<Vec3> {
    /// # Set X
    /// Overwrites the X value of appropriate unit.
    pub fn set_x(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Rem(mut v1) => match other {
                Amount::Rem(w1) => {
                    v1.x = w1;
                    Amount::Rem(v1)
                },
                Amount::Prc(w1) => {
                    Amount::RemPrc(v1, Vec3::new(w1, 0.0, 0.0))
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w1;
                    Amount::RemPrc(v1, Vec3::new(w2, 0.0, 0.0))
                },
            },
            Amount::Prc(mut v1) => match other {
                Amount::Rem(w1) => {
                    Amount::RemPrc(Vec3::new(w1, 0.0, 0.0), v1)
                },
                Amount::Prc(w1) => {
                    v1.x = w1;
                    Amount::Prc(v1)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w2;
                    Amount::RemPrc(Vec3::new(w1, 0.0, 0.0), v1)
                },
            },
            Amount::RemPrc(mut v1, mut v2) => match other {
                Amount::Rem(w1) => {
                    v1.x = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::Prc(w1) => {
                    v2.x = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.x = w1;
                    v2.x = w2;
                    Amount::RemPrc(v1, v2)
                },
            },
        }
    }
    /// # Set Y
    /// Overwrites the Y value of appropriate unit.
    pub fn set_y(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Rem(mut v1) => match other {
                Amount::Rem(w1) => {
                    v1.y = w1;
                    Amount::Rem(v1)
                },
                Amount::Prc(w1) => {
                    Amount::RemPrc(v1, Vec3::new(0.0, w1, 0.0))
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w1;
                    Amount::RemPrc(v1, Vec3::new(0.0, w2, 0.0))
                },
            },
            Amount::Prc(mut v1) => match other {
                Amount::Rem(w1) => {
                    Amount::RemPrc(Vec3::new(0.0, w1, 0.0), v1)
                },
                Amount::Prc(w1) => {
                    v1.y = w1;
                    Amount::Prc(v1)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w2;
                    Amount::RemPrc(Vec3::new(0.0, w1, 0.0), v1)
                },
            },
            Amount::RemPrc(mut v1, mut v2) => match other {
                Amount::Rem(w1) => {
                    v1.y = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::Prc(w1) => {
                    v2.y = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.y = w1;
                    v2.y = w2;
                    Amount::RemPrc(v1, v2)
                },
            },
        }
    }
    /// # Set Z
    /// Overwrites the Z value of appropriate unit.
    pub fn set_z(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Rem(mut v1) => match other {
                Amount::Rem(w1) => {
                    v1.z = w1;
                    Amount::Rem(v1)
                },
                Amount::Prc(w1) => {
                    Amount::RemPrc(v1, Vec3::new(0.0, 0.0, w1))
                },
                Amount::RemPrc(w1, w2) => {
                    v1.z = w1;
                    Amount::RemPrc(v1, Vec3::new(0.0, 0.0, w2))
                },
            },
            Amount::Prc(mut v1) => match other {
                Amount::Rem(w1) => {
                    Amount::RemPrc(Vec3::new(0.0, 0.0, w1), v1)
                },
                Amount::Prc(w1) => {
                    v1.z = w1;
                    Amount::Prc(v1)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.z = w2;
                    Amount::RemPrc(Vec3::new(0.0, 0.0, w1), v1)
                },
            },
            Amount::RemPrc(mut v1, mut v2) => match other {
                Amount::Rem(w1) => {
                    v1.z = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::Prc(w1) => {
                    v2.z = w1;
                    Amount::RemPrc(v1, v2)
                },
                Amount::RemPrc(w1, w2) => {
                    v1.z = w1;
                    v2.z = w2;
                    Amount::RemPrc(v1, v2)
                },
            },
        }
    }
}
// Implement for Vec4 maybe?

// # IMPLEMENTS ToAmount
// Allows for quick conversion for the used types.
pub trait ToAmount<T> {
    fn to_em(self) -> Amount<Self> where Self: Sized {
        Amount::Rem(self)
    }
    fn to_rt(self) -> Amount<Self> where Self: Sized {
        Amount::Prc(self)
    }
}
pub trait ToAmountArray<T> {
    fn to_emrt(self) -> Amount<T>;
}



impl ToAmount<f32> for f32 {
    fn to_em(self) -> Amount<f32> {
        Amount::Rem(self)
    }
    fn to_rt(self) -> Amount<f32> {
        Amount::Prc(self)
    }
}
impl ToAmountArray<f32> for [f32; 2] {
    fn to_emrt(self) -> Amount<f32> {
        Amount::RemPrc(self[0], self[1])
    }
}
impl ToAmount<Vec2> for Vec2 {
    fn to_em(self) -> Amount<Vec2> {
        Amount::Rem(self)
    }
    fn to_rt(self) -> Amount<Vec2> {
        Amount::Prc(self)
    }
}
impl ToAmountArray<Vec2> for [Vec2; 2] {
    fn to_emrt(self) -> Amount<Vec2> {
        Amount::RemPrc(self[0], self[1])
    }
}
impl ToAmount<Vec3> for Vec3 {
    fn to_em(self) -> Amount<Vec3> {
        Amount::Rem(self)
    }
    fn to_rt(self) -> Amount<Vec3> {
        Amount::Prc(self)
    }
}
impl ToAmountArray<Vec3> for [Vec3; 2] {
    fn to_emrt(self) -> Amount<Vec3> {
        Amount::RemPrc(self[0], self[1])
    }
}*/