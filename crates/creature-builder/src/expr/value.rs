use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExprValue(pub f32);

impl From<ExprValue> for f32 {
    fn from(val: ExprValue) -> Self {
        val.0
    }
}

impl ExprValue {
    pub fn atan(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0.atan2(rhs.0)))
    }

    pub fn modulo(self, rhs: Self) -> Option<Self> {
        let v = self.0.rem_euclid(rhs.0);
        if v.is_finite() {
            Some(Self(v))
        } else {
            None
        }
    }

    pub fn gt(self, rhs: Self) -> Option<Self> {
        Some(Self(if self.0 > rhs.0 { 1.0 } else { -1.0 }))
    }

    pub fn min(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0.min(rhs.0)))
    }

    pub fn max(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0.max(rhs.0)))
    }

    pub fn sigmoid(self) -> Option<Self> {
        let v = 1.0 / (1.0 + (-self.0).exp());
        if v.is_finite() {
            Some(Self(v))
        } else {
            None
        }
    }

    pub fn if_else(self, a: Self, b: Self) -> Option<Self> {
        if self.0 > 0.0 {
            Some(a)
        } else {
            Some(b)
        }
    }

    pub fn lerp(self, b: Self, t: Self) -> Option<Self> {
        Some(Self(self.0 + (b.0 - self.0) * t.0))
    }
}


macro_rules! impl_un_op {
    ($fun:ident) => {
        impl ExprValue {
            pub fn $fun(self) -> Option<ExprValue> {
                let v = self.0.$fun();
                if v.is_finite() {
                    Some(ExprValue(v))
                } else {
                    None
                }
            }
        }
    };
}


impl_un_op!(signum);
impl_un_op!(abs);
impl_un_op!(sin);
impl_un_op!(cos);
impl_un_op!(ln);
impl_un_op!(exp);


macro_rules! impl_bin_op {
    ($op:ident, $fun:ident) => {
        impl $op<ExprValue> for ExprValue {
            type Output = Option<ExprValue>;

            fn $fun(self, rhs: ExprValue) -> Self::Output {
                let val = self.0.$fun(rhs.0);
                if val.is_finite() {
                    Some(ExprValue(val))
                } else {
                    None
                }
            }
        }
    };
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);
