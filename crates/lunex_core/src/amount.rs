use std::ops::Add;
use bevy::prelude::*;

// # TEST
#[cfg(test)]
mod test {
    use super::{Amount, ToAmount, ToAmountArray};
    use super::Vec2;
    #[test]
    fn simple () {
        let val = 1.0.to_em() + 2.0.to_rt() + 3.0.to_rt();
        assert_eq!(Amount::RemPrc(1.0, 5.0), val);

        assert_eq!(Amount::Rem(1.0), Amount::Rem(2.0).set(Amount::Rem(1.0)));
        assert_eq!(Amount::Prc(1.0), Amount::Prc(2.0).set(Amount::Prc(1.0)));
        assert_eq!(Amount::RemPrc(1.0, 2.0), Amount::Rem(1.0).set(Amount::Prc(2.0)));
        assert_eq!(Amount::RemPrc(1.0, 2.0), Amount::Prc(2.0).set(Amount::Rem(1.0)));

        let val = Vec2::splat(1.0).to_em() + Vec2::splat(2.0).to_rt() + Vec2::splat(3.0).to_rt();
        assert_eq!([Vec2::splat(1.0), Vec2::splat(5.0)].to_emrt(), val);

        assert_eq!(Vec2::splat(1.0).to_em(), Vec2::splat(2.0).to_em().set(Vec2::splat(1.0).to_em()));
        assert_eq!(Vec2::splat(1.0).to_rt(), Vec2::splat(2.0).to_rt().set(Vec2::splat(1.0).to_rt()));
        assert_eq!([Vec2::splat(1.0), Vec2::splat(2.0)].to_emrt(), Vec2::splat(1.0).to_em().set(Vec2::splat(2.0).to_rt()));
        assert_eq!([Vec2::splat(1.0), Vec2::splat(2.0)].to_emrt(), Vec2::splat(2.0).to_rt().set(Vec2::splat(1.0).to_em()));
    }
    #[test]
    fn complex () {

        let val1 = [Vec2::new(1., 2.), Vec2::new(7., 3.)].to_emrt();
        let val2 = [Vec2::new(4., 2.), Vec2::new(5., 3.)].to_emrt();
        assert_eq!(val1, val2.set_x([1_f32, 7_f32].to_emrt()));

        let val1 = [Vec2::new(1., 2.), Vec2::new(1., 3.)].to_emrt();
        let val2 = [Vec2::new(1., 4.), Vec2::new(1., 5.)].to_emrt();
        assert_eq!(val1, val2.set_y([2_f32, 3_f32].to_emrt()));

    }

}




/// ## Absolute
/// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
pub struct Abs<T>(T);
/// ## Percentage
/// `0% to 100%`. Overflow allowed.
pub struct Prc<T>(T);
/// ## Rem
/// Size of symbol `M` which is `16px` with `font size 16px` and so on.
pub struct Rem<T>(T);






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
#[derive(Debug, Clone, Copy, PartialEq)]
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
// Impl add for Amount
// impl add for Abs, Prc, Rem so `Abs(10.0) + Rem(5) = Amount`

// Just default implementation
impl <T: Default> Default for Amount<T> {
    fn default() -> Self {
        Amount::Rem(T::default())
    }
}

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
}