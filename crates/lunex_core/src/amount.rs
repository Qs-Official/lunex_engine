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
        assert_eq!(Amount::EmRt(1.0, 5.0), val);

        assert_eq!(Amount::Em(1.0), Amount::Em(2.0).set(Amount::Em(1.0)));
        assert_eq!(Amount::Rt(1.0), Amount::Rt(2.0).set(Amount::Rt(1.0)));
        assert_eq!(Amount::EmRt(1.0, 2.0), Amount::Em(1.0).set(Amount::Rt(2.0)));
        assert_eq!(Amount::EmRt(1.0, 2.0), Amount::Rt(2.0).set(Amount::Em(1.0)));

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


/// # Amount
/// A struct holding size measurment data and how to calculate them.
/// It represents sum of different units used in UI.
/// * `Em` => _Size of one "M" char_
/// * `Rt` => _Ratio (Percentage)_
/// * `EmRt` => _Em + Rt_
/// ## Example
/// ```
/// # use lunex_core::{Amount, ToAmount};
/// let size: Amount<f32> = Amount::Em(12.0);
/// let size: Amount<f32> = Amount::EmRt(12.0, 5.0);
/// let size: Amount<f32> = 12.0.to_em() + 5.0.to_rt();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Amount<T> {
    Em(T),
    Rt(T),
    EmRt(T, T),
}

// Just default implementation
impl <T: Default> Default for Amount<T> {
    fn default() -> Self {
        Amount::Em(T::default())
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
            Amount::Em(v1) => {
                (*v1/12.0) * em
            },
            Amount::Rt(v1) => {
                (*v1/100.0) * rt
            },
            Amount::EmRt(v1, v2) => {
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
            Amount::Em(v1) => {
                (*v1/12.0) * em
            },
            Amount::Rt(v1) => {
                (*v1/100.0) * rt
            },
            Amount::EmRt(v1, v2) => {
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
            Amount::Em(v1) => {
                (*v1/12.0) * em
            },
            Amount::Rt(v1) => {
                (*v1/100.0) * rt
            },
            Amount::EmRt(v1, v2) => {
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
    /// let val = Amount::EmRt(12.0, 20.0) + Amount::Rt(5.0);
    /// assert_eq!(Amount::EmRt(12.0, 25.0), val);
    /// ```
    fn add(self, other: Self) -> Self::Output {
        match self {
            Amount::Em(x1) => match other {
                Amount::Em(y1) => Amount::Em(x1 + y1),
                Amount::Rt(y1) => Amount::EmRt(x1, y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(x1 + y1, y2),
            },
            Amount::Rt(x1) => match other {
                Amount::Em(y1) => Amount::EmRt(y1, x1),
                Amount::Rt(y1) => Amount::Rt(x1 + y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(y1, x1 + y2),
            },
            Amount::EmRt(x1, x2) => match other {
                Amount::Em(y1) => Amount::EmRt(x1 + y1, x2),
                Amount::Rt(y1) => Amount::EmRt(x1, x2 + y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(x1 + y1, x2 + y2),
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
    /// let val = Amount::EmRt(12.0, 20.0).set(Amount::Rt(5.0));
    /// assert_eq!(Amount::EmRt(12.0, 5.0), val);
    /// ```
    pub fn set(self, other: Amount<T>) -> Self {
        match self {
            Amount::Em(x1) => match other {
                Amount::Em(y1) => Amount::Em(y1),
                Amount::Rt(y1) => Amount::EmRt(x1, y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(y1, y2),
            },
            Amount::Rt(x1) => match other {
                Amount::Em(y1) => Amount::EmRt(y1, x1),
                Amount::Rt(y1) => Amount::Rt(y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(y1, y2),
            },
            Amount::EmRt(x1, x2) => match other {
                Amount::Em(y1) => Amount::EmRt(y1, x2),
                Amount::Rt(y1) => Amount::EmRt(x1, y1),
                Amount::EmRt(y1, y2) => Amount::EmRt(y1, y2),
            },
        }
    }
}
impl Amount<Vec2> {
    /// # Set X
    /// Overwrites the X value of appropriate unit.
    pub fn set_x(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Em(mut v1) => match other {
                Amount::Em(w1) => {
                    v1.x = w1;
                    Amount::Em(v1)
                },
                Amount::Rt(w1) => {
                    Amount::EmRt(v1, Vec2::new(w1, 0.0))
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w1;
                    Amount::EmRt(v1, Vec2::new(w2, 0.0))
                },
            },
            Amount::Rt(mut v1) => match other {
                Amount::Em(w1) => {
                    Amount::EmRt(Vec2::new(w1, 0.0), v1)
                },
                Amount::Rt(w1) => {
                    v1.x = w1;
                    Amount::Rt(v1)
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w2;
                    Amount::EmRt(Vec2::new(w1, 0.0), v1)
                },
            },
            Amount::EmRt(mut v1, mut v2) => match other {
                Amount::Em(w1) => {
                    v1.x = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::Rt(w1) => {
                    v2.x = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w1;
                    v2.x = w2;
                    Amount::EmRt(v1, v2)
                },
            },
        }
    }
    /// # Set Y
    /// Overwrites the Y value of appropriate unit.
    pub fn set_y(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Em(mut v1) => match other {
                Amount::Em(w1) => {
                    v1.y = w1;
                    Amount::Em(v1)
                },
                Amount::Rt(w1) => {
                    Amount::EmRt(v1, Vec2::new(0.0, w1))
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w1;
                    Amount::EmRt(v1, Vec2::new(0.0, w2))
                },
            },
            Amount::Rt(mut v1) => match other {
                Amount::Em(w1) => {
                    Amount::EmRt(Vec2::new(0.0, w1), v1)
                },
                Amount::Rt(w1) => {
                    v1.y = w1;
                    Amount::Rt(v1)
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w2;
                    Amount::EmRt(Vec2::new(0.0, w1), v1)
                },
            },
            Amount::EmRt(mut v1, mut v2) => match other {
                Amount::Em(w1) => {
                    v1.y = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::Rt(w1) => {
                    v2.y = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w1;
                    v2.y = w2;
                    Amount::EmRt(v1, v2)
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
            Amount::Em(mut v1) => match other {
                Amount::Em(w1) => {
                    v1.x = w1;
                    Amount::Em(v1)
                },
                Amount::Rt(w1) => {
                    Amount::EmRt(v1, Vec3::new(w1, 0.0, 0.0))
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w1;
                    Amount::EmRt(v1, Vec3::new(w2, 0.0, 0.0))
                },
            },
            Amount::Rt(mut v1) => match other {
                Amount::Em(w1) => {
                    Amount::EmRt(Vec3::new(w1, 0.0, 0.0), v1)
                },
                Amount::Rt(w1) => {
                    v1.x = w1;
                    Amount::Rt(v1)
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w2;
                    Amount::EmRt(Vec3::new(w1, 0.0, 0.0), v1)
                },
            },
            Amount::EmRt(mut v1, mut v2) => match other {
                Amount::Em(w1) => {
                    v1.x = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::Rt(w1) => {
                    v2.x = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::EmRt(w1, w2) => {
                    v1.x = w1;
                    v2.x = w2;
                    Amount::EmRt(v1, v2)
                },
            },
        }
    }
    /// # Set Y
    /// Overwrites the Y value of appropriate unit.
    pub fn set_y(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Em(mut v1) => match other {
                Amount::Em(w1) => {
                    v1.y = w1;
                    Amount::Em(v1)
                },
                Amount::Rt(w1) => {
                    Amount::EmRt(v1, Vec3::new(0.0, w1, 0.0))
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w1;
                    Amount::EmRt(v1, Vec3::new(0.0, w2, 0.0))
                },
            },
            Amount::Rt(mut v1) => match other {
                Amount::Em(w1) => {
                    Amount::EmRt(Vec3::new(0.0, w1, 0.0), v1)
                },
                Amount::Rt(w1) => {
                    v1.y = w1;
                    Amount::Rt(v1)
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w2;
                    Amount::EmRt(Vec3::new(0.0, w1, 0.0), v1)
                },
            },
            Amount::EmRt(mut v1, mut v2) => match other {
                Amount::Em(w1) => {
                    v1.y = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::Rt(w1) => {
                    v2.y = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::EmRt(w1, w2) => {
                    v1.y = w1;
                    v2.y = w2;
                    Amount::EmRt(v1, v2)
                },
            },
        }
    }
    /// # Set Z
    /// Overwrites the Z value of appropriate unit.
    pub fn set_z(self, other: Amount<f32>) -> Self {
        match self {
            Amount::Em(mut v1) => match other {
                Amount::Em(w1) => {
                    v1.z = w1;
                    Amount::Em(v1)
                },
                Amount::Rt(w1) => {
                    Amount::EmRt(v1, Vec3::new(0.0, 0.0, w1))
                },
                Amount::EmRt(w1, w2) => {
                    v1.z = w1;
                    Amount::EmRt(v1, Vec3::new(0.0, 0.0, w2))
                },
            },
            Amount::Rt(mut v1) => match other {
                Amount::Em(w1) => {
                    Amount::EmRt(Vec3::new(0.0, 0.0, w1), v1)
                },
                Amount::Rt(w1) => {
                    v1.z = w1;
                    Amount::Rt(v1)
                },
                Amount::EmRt(w1, w2) => {
                    v1.z = w2;
                    Amount::EmRt(Vec3::new(0.0, 0.0, w1), v1)
                },
            },
            Amount::EmRt(mut v1, mut v2) => match other {
                Amount::Em(w1) => {
                    v1.z = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::Rt(w1) => {
                    v2.z = w1;
                    Amount::EmRt(v1, v2)
                },
                Amount::EmRt(w1, w2) => {
                    v1.z = w1;
                    v2.z = w2;
                    Amount::EmRt(v1, v2)
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
        Amount::Em(self)
    }
    fn to_rt(self) -> Amount<Self> where Self: Sized {
        Amount::Rt(self)
    }
}
pub trait ToAmountArray<T> {
    fn to_emrt(self) -> Amount<T>;
}



impl ToAmount<f32> for f32 {
    fn to_em(self) -> Amount<f32> {
        Amount::Em(self)
    }
    fn to_rt(self) -> Amount<f32> {
        Amount::Rt(self)
    }
}
impl ToAmountArray<f32> for [f32; 2] {
    fn to_emrt(self) -> Amount<f32> {
        Amount::EmRt(self[0], self[1])
    }
}
impl ToAmount<Vec2> for Vec2 {
    fn to_em(self) -> Amount<Vec2> {
        Amount::Em(self)
    }
    fn to_rt(self) -> Amount<Vec2> {
        Amount::Rt(self)
    }
}
impl ToAmountArray<Vec2> for [Vec2; 2] {
    fn to_emrt(self) -> Amount<Vec2> {
        Amount::EmRt(self[0], self[1])
    }
}
impl ToAmount<Vec3> for Vec3 {
    fn to_em(self) -> Amount<Vec3> {
        Amount::Em(self)
    }
    fn to_rt(self) -> Amount<Vec3> {
        Amount::Rt(self)
    }
}
impl ToAmountArray<Vec3> for [Vec3; 2] {
    fn to_emrt(self) -> Amount<Vec3> {
        Amount::EmRt(self[0], self[1])
    }
}